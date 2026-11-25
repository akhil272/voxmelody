// Reads one JSON request line on stdin, writes JSON events to stdout, then exits.
// Exiting after each track returns all memory to the computer.

#include "backend.h"
#include "model-store.h"
#include "pipeline-lm.h"
#include "pipeline-synth.h"
#include "request.h"
#include "yyjson.h"

#include <algorithm>
#include <cmath>
#include <cstdint>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

#ifdef _WIN32
#  include <windows.h>
#endif

namespace fs = std::filesystem;

static const char * LM_FILE   = "acestep-5Hz-lm-0.6B-Q8_0.gguf";
static const char * TEXT_FILE = "Qwen3-Embedding-0.6B-Q8_0.gguf";
static const char * DIT_FILE  = "acestep-v15-turbo-Q4_K_M.gguf";
static const char * VAE_FILE  = "vae-BF16.gguf";

static const int    CODES_PER_SECOND  = 5;
static const int    FRAMES_PER_CODE   = 5;
static const int    LM_ATTEMPTS       = 3;
static const int    EXTRA_SECONDS     = 15;
static const int    PROMPT_TOKENS     = 3072;
static const int    METADATA_TOKENS   = 150;
// Ten identical codes are two seconds of nothing happening.
static const size_t FROZEN_CODES      = 10;
// Below this the models do not fit in dedicated memory.
static const size_t MIN_GPU_MEMORY    = 3500ull * 1024 * 1024;
static const int    DIT_STEPS         = 8;
// Small tiles decode faster and keep the peak under 2 GB of graphics memory.
static const int    VAE_CHUNK         = 256;

static const float LM_SHARE  = 0.25f;
static const float DIT_SHARE = 0.35f;

static void emit(const std::string & json) {
    std::cout << json << std::endl;
}

static int fail(const char * code) {
    emit(std::string("{\"event\":\"error\",\"code\":\"") + code + "\"}");
    return 1;
}

static std::string quote(const std::string & text) {
    yyjson_mut_doc * doc = yyjson_mut_doc_new(nullptr);
    yyjson_mut_doc_set_root(doc, yyjson_mut_strncpy(doc, text.data(), text.size()));
    char *      json = yyjson_mut_write(doc, 0, nullptr);
    std::string out  = json ? json : "\"\"";
    free(json);
    yyjson_mut_doc_free(doc);
    return out;
}

// The engines poll for cancellation once per unit of work, which also measures progress.
struct Progress {
    float start;
    float share;
    float units;
    float done = 0;
    int   last = -1;
};

static bool tick(void * data) {
    auto *    p     = static_cast<Progress *>(data);
    const int value = (int) (1000 * (p->start + p->share * std::min(1.0f, p->done / p->units)));
    p->done += 1;
    if (value != p->last) {
        p->last = value;
        emit("{\"event\":\"progress\",\"value\":" + std::to_string(value / 1000.0) + "}");
    }
    return false;
}

// The largest dedicated GPU that can hold the models, or null for the CPU.
static ggml_backend_dev_t find_gpu() {
    ggml_backend_dev_t best        = nullptr;
    size_t             best_memory = MIN_GPU_MEMORY;
    for (size_t i = 0; i < ggml_backend_dev_count(); i++) {
        ggml_backend_dev_t dev = ggml_backend_dev_get(i);
        if (ggml_backend_dev_type(dev) != GGML_BACKEND_DEVICE_TYPE_GPU) {
            continue;
        }
        size_t free_memory = 0, total = 0;
        ggml_backend_dev_memory(dev, &free_memory, &total);
        if (total >= best_memory) {
            best        = dev;
            best_memory = total;
        }
    }
    return best;
}

static std::vector<std::string> split_codes(const std::string & text) {
    std::vector<std::string> codes;
    size_t                   start = 0;
    while (start < text.size()) {
        const size_t end = std::min(text.find(',', start), text.size());
        codes.push_back(text.substr(start, end - start));
        start = end + 1;
    }
    return codes;
}

// Codes before the music stops.
static size_t playing_codes(const std::vector<std::string> & codes) {
    size_t run = 1;
    for (size_t i = 1; i < codes.size(); i++) {
        run = codes[i] == codes[i - 1] ? run + 1 : 1;
        if (run == FROZEN_CODES) {
            return i + 1 - FROZEN_CODES;
        }
    }
    return codes.size();
}

// Composes past the requested length, then keeps only what is needed, so the
// track never ends early and no time is spent on audio that is cut away.
static bool compose(ModelStore * store, const AceRequest & request, int seconds, AceRequest * plan) {
    AceLmParams params;
    ace_lm_default_params(&params);
    params.model_path = LM_FILE;
    params.max_batch  = 1;

    const size_t             needed = (size_t) seconds * CODES_PER_SECOND;
    std::vector<std::string> best;
    for (int attempt = 0; attempt < LM_ATTEMPTS && best.size() < needed; attempt++) {
        AceRequest in = request;
        in.duration   = (float) (seconds + (EXTRA_SECONDS + seconds / 4) * (attempt + 1));
        in.lm_seed    = -1;
        // A cache sized to the request saves over 1 GB of graphics memory.
        params.max_seq = PROMPT_TOKENS + (int) in.duration * CODES_PER_SECOND;
        request_resolve_lm_seed(&in);

        AceLm * lm = ace_lm_load(store, &params);
        if (!lm) {
            return false;
        }
        Progress   progress = { 0, LM_SHARE, in.duration * CODES_PER_SECOND + METADATA_TOKENS };
        AceRequest out;
        request_init(&out);
        const int rc = ace_lm_generate(lm, &in, 1, &out, nullptr, nullptr, tick, &progress);
        ace_lm_free(lm);
        if (rc != 0) {
            return false;
        }
        std::vector<std::string> codes = split_codes(out.audio_codes);
        codes.resize(playing_codes(codes));
        if (codes.size() > best.size()) {
            best  = std::move(codes);
            *plan = out;
        }
    }
    if (best.empty()) {
        return false;
    }
    best.resize(std::min(best.size(), needed));
    plan->audio_codes.clear();
    for (const auto & code : best) {
        plan->audio_codes += (plan->audio_codes.empty() ? "" : ",") + code;
    }
    plan->duration = (float) best.size() / CODES_PER_SECOND;
    return true;
}

static bool synthesize(ModelStore * store, AceRequest plan, AceAudio * audio) {
    AceSynthParams params;
    ace_synth_default_params(&params);
    params.text_encoder_path = TEXT_FILE;
    params.dit_path          = DIT_FILE;
    params.vae_path          = VAE_FILE;
    params.vae_chunk         = VAE_CHUNK;

    plan.seed = -1;
    request_resolve_seed(&plan);
    AceSynth * synth = ace_synth_load(store, &params);
    if (!synth) {
        return false;
    }
    Progress      dit = { LM_SHARE, DIT_SHARE, DIT_STEPS };
    AceSynthJob * job = ace_synth_job_run_dit(synth, &plan, nullptr, 0, nullptr, 0, nullptr, 0, nullptr, 0, 1, tick, &dit);
    bool          ok  = job != nullptr;
    if (ok) {
        const float frames = plan.duration * CODES_PER_SECOND * FRAMES_PER_CODE;
        const float tiles  = std::ceil(frames / (float) (params.vae_chunk - 2 * params.vae_overlap));
        Progress    vae    = { LM_SHARE + DIT_SHARE, 1 - LM_SHARE - DIT_SHARE, std::max(1.0f, tiles) };
        ok                 = ace_synth_job_run_vae(synth, job, audio, tick, &vae) == 0 && audio->samples;
        ace_synth_job_free(job);
    }
    ace_synth_free(synth);
    return ok;
}

// Raw interleaved stereo float32; the app finishes the track.
static bool write_audio(const fs::path & path, const AceAudio & audio) {
    const size_t       count = (size_t) audio.n_samples;
    std::vector<float> interleaved(count * 2);
    for (size_t i = 0; i < count; i++) {
        interleaved[i * 2]     = audio.samples[i];
        interleaved[i * 2 + 1] = audio.samples[count + i];
    }
    std::ofstream file(path, std::ios::binary);
    file.write(reinterpret_cast<const char *>(interleaved.data()), (std::streamsize) (interleaved.size() * sizeof(float)));
    file.close();
    return file.good();
}

static int run(yyjson_val * root) {
    const auto text = [root](const char * key) {
        const char * value = yyjson_get_str(yyjson_obj_get(root, key));
        return std::string(value ? value : "");
    };
    const std::string op = text("op");
    if (op != "compose" && op != "devices") {
        return fail("request");
    }

    backend_load();
    ggml_backend_dev_t gpu = yyjson_get_bool(yyjson_obj_get(root, "cpuOnly")) ? nullptr : find_gpu();
    if (op == "devices") {
        emit("{\"event\":\"device\",\"gpu\":" + (gpu ? quote(ggml_backend_dev_description(gpu)) : std::string("null")) + "}");
        return 0;
    }

    AceRequest request;
    request_init(&request);
    request.caption           = text("prompt");
    request.lyrics            = "[Instrumental]";
    request.keyscale          = text("key");
    request.bpm               = yyjson_get_int(yyjson_obj_get(root, "bpm"));
    const int         seconds = yyjson_get_int(yyjson_obj_get(root, "seconds"));
    const std::string output  = text("output");
    if (request.caption.empty() || output.empty() || seconds < 1) {
        return fail("request");
    }
    if (!backend_set_device(gpu ? ggml_backend_dev_name(gpu) : "CPU")) {
        return fail("engine");
    }
    emit("{\"event\":\"device\",\"gpu\":" + (gpu ? quote(ggml_backend_dev_description(gpu)) : std::string("null")) + "}");

    // Model files open by relative name, so any folder name works. Backends are
    // loaded by now: libraries are never searched for in the models folder.
    std::error_code error;
    fs::current_path(fs::u8path(text("models")), error);
    if (error) {
        return fail("model");
    }
    for (const char * file : { LM_FILE, TEXT_FILE, DIT_FILE, VAE_FILE }) {
        if (!fs::exists(file, error)) {
            return fail("model");
        }
    }

    ModelStore * store = store_create(EVICT_STRICT);
    AceRequest   plan;
    request_init(&plan);
    AceAudio audio = {};
    if (!compose(store, request, seconds, &plan) || !synthesize(store, plan, &audio)) {
        return fail("engine");
    }
    if (!write_audio(fs::u8path(output), audio)) {
        return fail("output");
    }
    emit("{\"event\":\"done\",\"rate\":" + std::to_string(audio.sample_rate) + "}");
    return 0;
}

int main() {
#ifdef _WIN32
    // A crash must end the process quietly, without a system dialog.
    SetErrorMode(SEM_FAILCRITICALERRORS | SEM_NOGPFAULTERRORBOX);
    _set_abort_behavior(0, _WRITE_ABORT_MSG | _CALL_REPORTFAULT);
#endif
    std::string line;
    std::getline(std::cin, line);
    yyjson_doc * doc  = yyjson_read(line.data(), line.size(), 0);
    yyjson_val * root = yyjson_doc_get_root(doc);
    const int    code = yyjson_is_obj(root) ? run(root) : fail("request");
    // Skips driver teardown, which can hang; the system reclaims everything.
    std::cout.flush();
    std::_Exit(code);
}
