// Rebuilds src-tauri/models.json from pinned Hugging Face commits.
// Usage: node scripts/build-manifest.mjs
import { createHash } from "node:crypto";
import { writeFileSync } from "node:fs";

const HOST = "https://huggingface.co";
const ESPEAK = "espeak-ng-data";
const ESPEAK_FILES = [
  ...["intonations", "phondata", "phonindex", "phontab"],
  ...["en", "es", "fr", "hi", "it", "pt"].map((language) => `${language}_dict`),
  ...["en", "en-029", "en-GB-scotland", "en-GB-x-gbclan", "en-GB-x-gbcwmd", "en-GB-x-rp", "en-US", "en-US-nyc"].map(
    (voice) => `lang/gmw/${voice}`,
  ),
  ...["lang/inc/hi", "lang/roa/es", "lang/roa/fr", "lang/roa/it", "lang/roa/pt", "lang/roa/pt-BR"],
];

const GIB = 1024 ** 3;
const ENGLISH_ESPEAK = ESPEAK_FILES.filter((file) => !/_dict$|^lang\/(?!gmw)/.test(file) || file === "en_dict");
const SUPERTONIC_LANGUAGES =
  "en ar bg cs da de el es et fi fr hi hr hu id it ja ko lt lv nl pl pt ro ru sk sl sv tr uk vi".split(" ");

const APACHE = { licence: "Apache-2.0", restricted: false };
const ACE_STEP = {
  kind: "music",
  author: "ACE-Step",
  licence: "MIT",
  restricted: false,
  languages: [],
  repo: "Serveurperso/ACE-Step-1.5-GGUF",
  commit: "666ac70204440867d8c01ba4b119cc79c95b370a",
};

const kitten = (size, details) => ({
  id: `kitten-${size}`,
  kind: "voice",
  name: `Kitten ${size[0].toUpperCase()}${size.slice(1)} 0.8`,
  author: "KittenML",
  released: "2026-02-19",
  ...APACHE,
  languages: ["en-us"],
  recommendedMemory: 4 * GIB,
  files: ["LICENSE", "tokens.txt", ...ENGLISH_ESPEAK.map((file) => `${ESPEAK}/${file}`), "voices.bin", details.model],
  ...details,
  model: undefined,
  voice: { family: "kitten", file: details.model },
});

const aceStep = ({ lm, dit, ...details }) => ({
  ...ACE_STEP,
  ...details,
  files: ["LICENSE", "vae-BF16.gguf", lm, "Qwen3-Embedding-0.6B-Q8_0.gguf", dit],
  music: { lm, dit, ...details.music },
});

const SMALL_LM = "acestep-5Hz-lm-0.6B-Q8_0.gguf";
const MEDIUM_LM = "acestep-5Hz-lm-1.7B-Q8_0.gguf";

// The first pack of each kind is the recommended one; its id is also its folder name.
const PACKS = [
  {
    id: "voice",
    kind: "voice",
    name: "Kokoro 1.0",
    author: "hexgrad",
    released: "2025-01-27",
    ...APACHE,
    page: "hexgrad/Kokoro-82M",
    parameters: "82M",
    precision: "32-bit",
    languages: ["en-us", "en-gb", "es", "fr", "hi", "it", "pt-br"],
    voice: { family: "kokoro", file: "model.onnx" },
    repo: "csukuangfj/kokoro-multi-lang-v1_0",
    commit: "f7b96bb6bef5c5da4d3aa4f4e0498fbbf62dc78b",
    recommendedMemory: 4 * GIB,
    files: [
      "LICENSE",
      "tokens.txt",
      "lexicon-us-en.txt",
      "lexicon-gb-en.txt",
      ...ESPEAK_FILES.map((file) => `${ESPEAK}/${file}`),
      "voices.bin",
      "model.onnx",
    ],
  },
  {
    id: "supertonic-3",
    kind: "voice",
    name: "Supertonic 3",
    author: "Supertone",
    released: "2026-04-29",
    licence: "OpenRAIL-M",
    restricted: true,
    page: "Supertone/supertonic-3",
    parameters: "99M",
    precision: "8-bit",
    languages: SUPERTONIC_LANGUAGES,
    voice: { family: "supertonic", file: "" },
    repo: "csukuangfj2/sherpa-onnx-supertonic-3-tts-int8-2026-05-11",
    commit: "cca5a0e6c96e1d2c720986bf7e75fcc81dee3ae4",
    recommendedMemory: 4 * GIB,
    files: [
      "duration_predictor.int8.onnx",
      "text_encoder.int8.onnx",
      "tts.json",
      "unicode_indexer.bin",
      "vector_estimator.int8.onnx",
      "vocoder.int8.onnx",
      "voice.bin",
    ],
  },
  kitten("mini", {
    page: "KittenML/kitten-tts-mini-0.8",
    parameters: "80M",
    precision: "Mixed",
    repo: "csukuangfj2/kitten-mini-en-v0_8",
    commit: "5e67e1579d5119f606e95632a4852b6af936e765",
    model: "model.onnx",
  }),
  kitten("micro", {
    page: "KittenML/kitten-tts-micro-0.8",
    parameters: "40M",
    precision: "Mixed",
    repo: "csukuangfj2/kitten-micro-en-v0_8",
    commit: "7131b31e5fbe7da21e32f84899d1d273a1477e15",
    model: "model.onnx",
  }),
  kitten("nano", {
    page: "KittenML/kitten-tts-nano-0.8-fp32",
    parameters: "15M",
    precision: "32-bit",
    repo: "csukuangfj2/kitten-nano-en-v0_8-fp32",
    commit: "eb4fe4f70801b2eb1c3f45be046ccd553c7575e7",
    model: "model.fp32.onnx",
  }),
  {
    id: "pocket-tts",
    kind: "voice",
    name: "Pocket TTS",
    author: "Kyutai",
    released: "2026-01-13",
    licence: "CC-BY-4.0",
    restricted: true,
    page: "kyutai/pocket-tts",
    parameters: "100M",
    precision: "8-bit",
    languages: ["en-us"],
    voice: { family: "pocket", file: "" },
    repo: "csukuangfj2/sherpa-onnx-pocket-tts-int8-2026-01-26",
    commit: "4145e1e8ec6c81dc73a28641c6ab031ab57ea5b3",
    recommendedMemory: 4 * GIB,
    files: [
      "LICENSE",
      "decoder.int8.onnx",
      "encoder.onnx",
      "lm_flow.int8.onnx",
      "lm_main.int8.onnx",
      "text_conditioner.onnx",
      "token_scores.json",
      "vocab.json",
    ],
  },
  aceStep({
    id: "music",
    name: "ACE-Step 1.5 Turbo",
    released: "2026-01-23",
    page: "ACE-Step/Ace-Step1.5",
    parameters: "2B",
    precision: "4-bit",
    recommendedMemory: 8 * GIB,
    lm: SMALL_LM,
    dit: "acestep-v15-turbo-Q4_K_M.gguf",
    music: { steps: 8, guidance: false, graphicsMemory: 3.5 * GIB },
  }),
  aceStep({
    id: "ace-step-turbo-hq",
    name: "ACE-Step 1.5 Turbo HQ",
    released: "2026-01-23",
    page: "ACE-Step/Ace-Step1.5",
    parameters: "2B",
    precision: "8-bit",
    recommendedMemory: 12 * GIB,
    lm: MEDIUM_LM,
    dit: "acestep-v15-turbo-Q8_0.gguf",
    music: { steps: 8, guidance: false, graphicsMemory: 5.5 * GIB },
  }),
  aceStep({
    id: "ace-step-sft",
    name: "ACE-Step 1.5 SFT",
    released: "2026-01-23",
    page: "ACE-Step/acestep-v15-sft",
    parameters: "2B",
    precision: "4-bit",
    recommendedMemory: 8 * GIB,
    lm: SMALL_LM,
    dit: "acestep-v15-sft-Q4_K_M.gguf",
    music: { steps: 50, guidance: true, graphicsMemory: 3.5 * GIB },
  }),
  aceStep({
    id: "ace-step-xl",
    name: "ACE-Step 1.5 XL Turbo",
    released: "2026-04-02",
    page: "ACE-Step/acestep-v15-xl-turbo",
    parameters: "4B",
    precision: "4-bit",
    recommendedMemory: 12 * GIB,
    lm: MEDIUM_LM,
    dit: "acestep-v15-xl-turbo-Q4_K_M.gguf",
    music: { steps: 8, guidance: false, graphicsMemory: 5.5 * GIB },
  }),
];

async function fetchOk(url) {
  const response = await fetch(url);
  if (!response.ok) throw new Error(`${response.status} ${url}`);
  return response;
}

async function listTree({ repo, commit }) {
  const entries = [];
  let url = `${HOST}/api/models/${repo}/tree/${commit}?recursive=true`;
  while (url) {
    const response = await fetchOk(url);
    entries.push(...(await response.json()));
    url = response.headers.get("link")?.match(/<([^>]+)>;\s*rel="next"/)?.[1];
  }
  return new Map(entries.map((entry) => [entry.path, entry]));
}

async function describeFile(base, entry) {
  if (entry.lfs) return { path: entry.path, size: entry.lfs.size, sha256: entry.lfs.oid };
  const data = Buffer.from(await (await fetchOk(base + entry.path)).arrayBuffer());
  return { path: entry.path, size: data.length, sha256: createHash("sha256").update(data).digest("hex") };
}

const packs = [];
for (const { repo, commit, page, files: paths, ...info } of PACKS) {
  const base = `${HOST}/${repo}/resolve/${commit}/`;
  const tree = await listTree({ repo, commit });
  const files = [];
  for (const path of paths) {
    const entry = tree.get(path);
    if (!entry) throw new Error(`Missing ${path} in ${repo}`);
    files.push(await describeFile(base, entry));
  }
  packs.push({ ...info, pageUrl: `${HOST}/${page}`, sources: [base], files });
}

writeFileSync(new URL("../src-tauri/models.json", import.meta.url), `${JSON.stringify({ packs }, null, 2)}\n`);
