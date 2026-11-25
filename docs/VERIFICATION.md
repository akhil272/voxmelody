# Phase 0: Verification

Date: 2026-11-25. **Verdict: GO.**

The source brief proposed Python, FastAPI and an 8–12 GB GPU. The models are viable, but that packaging is not suitable for the public. Native engines make it work on ordinary PCs.

Test machine: Windows 11, i9-9900K, 64 GB RAM, GTX 1060 6 GB (2016).

## Voice: sherpa-onnx 1.13.8, CPU, 4 threads

| Model | Download | Audio | Time | Real-time factor |
| --- | --- | --- | --- | --- |
| Kokoro 82M fp32 | 350 MB | 19.1 s | 6.2 s | 0.33 |
| Kokoro 82M int8 | 132 MB | 19.0 s | 21.2 s | 1.11 |
| Piper lessac-medium | 67 MB | 20.0 s | 0.85 s | 0.04 |

The fp32 (one female and one male voice) and Piper outputs were transcribed back with Whisper: every word, price, date and number was present. int8 was not transcribed.

**sherpa-onnx 2.0.3 fails this test.** It removed espeak-ng, so Kokoro silently drops unknown words and numbers ("VoxMelody", "$49.99", "20") and Piper produced no audio. Any upgrade must repeat the transcription test.

## Music: acestep.cpp 973517b, ACE-Step 1.5 turbo Q4_K_M

Models total 3.28 GB: DiT 1.45, LM 0.6B 0.71, text encoder 0.78, VAE 0.34.

| Requested | Device | Time | Delivered | Peak VRAM (whole system) |
| --- | --- | --- | --- | --- |
| 30 s | Vulkan, GTX 1060 | 57 s | 29.6 s | 4.4 GB |
| 60 s | Vulkan, GTX 1060 | 52 s | 59.6 s | 5.2 GB |
| 90 s | Vulkan, GTX 1060 | 53 s | 86.8 s | 5.3 GB |
| 30 s | CPU synth only (LM output reused from GPU run) | 55 s | 29.6 s | none |

VRAM figures include about 1.3 GB of desktop use. Signal analysis showed tonal, rhythmic content (about 172 BPM detected, roughly double time of the requested 84). Musical quality still needs a human listening check.

Voice plus ducked music, normalised to about −14 LUFS, stayed fully intelligible by transcription.

## Licences

| Item | Code | Weights | Ship? |
| --- | --- | --- | --- |
| Kokoro-82M | Apache-2.0 | Apache-2.0 | Yes |
| ACE-Step 1.5 | MIT | MIT | Yes |
| acestep.cpp | MIT | n/a | Yes |
| sherpa-onnx 1.x | Apache-2.0, bundles GPL-3.0 espeak-ng | n/a | Yes, as a separate process with source offer |
| Qwen3-TTS 0.6B | Apache-2.0 | Apache-2.0 | Later (voice cloning, not tested) |
| Piper voices | MIT or GPL-3.0 | Varies by voice | Only voices vetted one by one |
| MusicGen | MIT | **CC-BY-NC-4.0** | **No** |

- The mockup's "Aria (MusicGen)" option must become ACE-Step.
- The Piper lessac voice used for speed testing has its own dataset licence terms, so it is not approved for shipping.

## Phase 2 additions (2026-11-25)

- Full CPU-only music pipeline, 60 s track: LM 14 s plus synth 114 s; peak memory 2.75 GB (LM) and 3.61 GB (synth). On the GTX 1060, synth peaked at 1.36 GB of RAM.
- Voice pack trimmed from 378 to 30 files (367 MB) covering English, Spanish, French, Hindi, Italian and Portuguese. English, Spanish and French ran. Spanish phoneme tokens matched the full set exactly; audio differs slightly on every run even with identical input. Chinese and Japanese files are not included.
- In the app: download, pause, resume, forced kill then resume, proxy failure, both packs installed from Hugging Face, and a cross-drive move. Voice and music then generated from those files with the engine tools.
- Cross-drive move of 3.6 GB took 456 s in an unoptimised debug build; release speed is not measured.

## Phase 3 additions (2026-11-25)

- The stock sherpa-onnx command-line tool garbles accents, fails in folders with non-ASCII names, and reads a script such as "--help" as an option. Our own engine process passes all three.
- 41 voices in 7 language options ran. English, Spanish, French, Italian, Portuguese and Hindi were transcribed back; differences were spelling or transcriber errors, confirmed for Spanish and for a 111-word unpunctuated sentence (111 of 111 words).
- 5,000 characters produced 245 s of audio in 82 s. A warm one-line request takes about 0.5 s; a cold start adds about 2 s. Peak engine memory 480 MB on a short script.
- Words in scripts the voice does not support, such as Japanese in an English voice, may be skipped. Emoji are read aloud by name. Speeds above 1.5× blur words, so the app stops there.
- The engine imports no networking library and had no open sockets while running.
- In the app: preview, generate, play, seek, language change, export, cancel (0.1 s), and removing the voice during generation.

## Phase 4 additions (2026-11-25)

Same test machine. Times are the engine process alone, including model loading; audio includes two spare seconds.

| Audio | Device | Time |
| --- | --- | --- |
| 32 s | Vulkan, GTX 1060 (tile 1024) | 36 s |
| 92 s | Vulkan, GTX 1060 (tile 256) | 50 s |
| 242 s | Vulkan, GTX 1060 (tile 256) | 125 s |
| 32 s | CPU (tile 1024) | 75 s |
| 62 s | CPU (tile 1024) | 149 s |

- A 242 s track crashed on the GPU with tile 1024, where whole-system VRAM peaked at 5.8 of 6 GB. With tile 256 it peaked at 3.3 to 3.4 GB, including about 1.7 GB of desktop use.
- Seven of eight raw tracks had sound to the end; one faded out about three seconds early, which the app now detects and composes again.
- In the app: 30.00 s files, peaks at −1.0 dBFS, loudness −16.6 to −18.9 LUFS (limited by peaks, D021). Progress, play, cancel (0.1 s), both studios at once, and removing the model during generation.
- Killing the GPU process mid-track restarted it on the CPU and delivered the track.
- Engine and pack in folders with non-ASCII names work. With the Vulkan library removed the engine reports the CPU.
- Requested 84 BPM; four GPU tracks measured about 86, one of them as double time. The key was passed to the model but not measured.
- The engine and its libraries import no networking library and no Visual C++ runtime.

## Phase 5 additions (2026-11-25)

- In the app: automatic saving, live update while the Library is open, playback, rename, cancel rename, delete, restart, files moved away, files put back, and deleting a missing recording (the moved file is kept).
- Native dialogs: Export wrote a byte-identical file; Relink refused another WAV and accepted the renamed original in a folder with a non-ASCII name. Exporting a recording onto itself is guarded in code but was not driven through the dialog.
- Unit tests cover a damaged index, an unusable Music folder, two windows saving at once, and same-sized tracks.

## Known issues to handle

1. Solved in Phase 4: exact length without a silent ending (D021).
2. Solved in Phase 4: tracks are normalised from unclipped samples.
3. Solved in Phase 4: tile 256 cut VAE decode of a 92 s track from 32 s to 21 s.
4. Solved in Phase 3: common English abbreviations are expanded before speech.
5. acestep.cpp has no release binaries; `pnpm engines` builds it, and CI must do the same in Phase 8.

## Not verified

- macOS: no Mac available. The project supports Metal, but it needs a CI build and a real-device run.
- AMD and Intel GPUs, and PCs with 8 GB RAM.
- Qwen3-TTS.
- Generation with the network adapter physically off (neither engine imports network code).
- Microsoft Store acceptance.
