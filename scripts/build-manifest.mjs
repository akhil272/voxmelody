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

const PACKS = [
  {
    id: "voice",
    repo: "csukuangfj/kokoro-multi-lang-v1_0",
    commit: "f7b96bb6bef5c5da4d3aa4f4e0498fbbf62dc78b",
    recommendedMemory: 4 * 1024 ** 3,
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
    id: "music",
    repo: "Serveurperso/ACE-Step-1.5-GGUF",
    commit: "666ac70204440867d8c01ba4b119cc79c95b370a",
    recommendedMemory: 8 * 1024 ** 3,
    files: [
      "LICENSE",
      "vae-BF16.gguf",
      "acestep-5Hz-lm-0.6B-Q8_0.gguf",
      "Qwen3-Embedding-0.6B-Q8_0.gguf",
      "acestep-v15-turbo-Q4_K_M.gguf",
    ],
  },
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
for (const pack of PACKS) {
  const base = `${HOST}/${pack.repo}/resolve/${pack.commit}/`;
  const tree = await listTree(pack);
  const files = [];
  for (const path of pack.files) {
    const entry = tree.get(path);
    if (!entry) throw new Error(`Missing ${path} in ${pack.repo}`);
    files.push(await describeFile(base, entry));
  }
  packs.push({ id: pack.id, recommendedMemory: pack.recommendedMemory, sources: [base], files });
}

writeFileSync(new URL("../src-tauri/models.json", import.meta.url), `${JSON.stringify({ packs }, null, 2)}\n`);
