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

## Known issues to handle

1. Music can end early: the 30 s track had 4.9 s of trailing silence. Generate longer, trim silence, fade, and cut to length.
2. Raw music peaks at 0 dBFS with flat peaks; normalise on export.
3. VAE decode was the slowest logged synth stage (25.7 s of the 90 s run); tune VAE chunk size.
4. Kokoro reads "Dr." as a sentence end; normalise abbreviations first.
5. acestep.cpp has no GitHub release binaries; build them in CI from a pinned commit.

## Not verified

- macOS: no Mac available. The project supports Metal, but it needs a CI build and a real-device run.
- AMD and Intel GPUs, and PCs with 8 GB RAM.
- Qwen3-TTS.
- Generation with the network adapter physically off.
- Microsoft Store acceptance.
