# Verification

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
| Kitten TTS 0.8 (Nano, Micro, Mini) | Apache-2.0 | Apache-2.0 | Yes |
| Supertonic 3 | MIT | OpenRAIL-M (use restrictions, including disclosing machine-made content) | Yes, labelled |
| Pocket TTS | MIT | CC-BY-4.0, with prohibited uses | Yes, labelled (D034) |
| ZipVoice | Apache-2.0 | Apache-2.0, trained on CC-BY-NC data | No |
| Qwen3-TTS 0.6B | Apache-2.0 | Apache-2.0 | Later (not tested) |
| Piper voices | MIT or GPL-3.0 | Varies by voice | Only voices vetted one by one |
| MusicGen | MIT | **CC-BY-NC-4.0** | **No** |

- The mockup's "Aria (MusicGen)" option must become ACE-Step.
- The Piper lessac voice used for speed testing has its own dataset licence terms, so it is not approved for shipping.

## Model manager (2026-11-25)

- Full CPU-only music pipeline, 60 s track: LM 14 s plus synth 114 s; peak memory 2.75 GB (LM) and 3.61 GB (synth). On the GTX 1060, synth peaked at 1.36 GB of RAM.
- Voice pack trimmed from 378 to 30 files (367 MB) covering English, Spanish, French, Hindi, Italian and Portuguese. English, Spanish and French ran. Spanish phoneme tokens matched the full set exactly; audio differs slightly on every run even with identical input. Chinese and Japanese files are not included.
- In the app: download, pause, resume, forced kill then resume, proxy failure, both packs installed from Hugging Face, and a cross-drive move. Voice and music then generated from those files with the engine tools.
- Cross-drive move of 3.6 GB took 456 s in an unoptimised debug build; release speed is not measured.

## Voice Studio (2026-11-25)

- The stock sherpa-onnx command-line tool garbles accents, fails in folders with non-ASCII names, and reads a script such as "--help" as an option. Our own engine process passes all three.
- 41 voices in 7 language options ran. English, Spanish, French, Italian, Portuguese and Hindi were transcribed back; differences were spelling or transcriber errors, confirmed for Spanish and for a 111-word unpunctuated sentence (111 of 111 words).
- 5,000 characters produced 245 s of audio in 82 s. A warm one-line request takes about 0.5 s; a cold start adds about 2 s. Peak engine memory 480 MB on a short script.
- Words in scripts the voice does not support, such as Japanese in an English voice, may be skipped. Emoji are read aloud by name. Speeds above 1.5× blur words, so the app stops there.
- The engine imports no networking library and had no open sockets while running.
- In the app: preview, generate, play, seek, language change, export, cancel (0.1 s), and removing the voice during generation.

## Music Studio (2026-11-25)

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

## Library (2026-11-25)

- In the app: automatic saving, live update while the Library is open, playback, rename, cancel rename, delete, restart, files moved away, files put back, and deleting a missing recording (the moved file is kept).
- Native dialogs: Export wrote a byte-identical file; Relink refused another WAV and accepted the renamed original in a folder with a non-ASCII name. Exporting a recording onto itself is guarded in code but was not driven through the dialog.
- Unit tests cover a damaged index, an unusable Music folder, two windows saving at once, and same-sized tracks.

## Mix (2026-11-25)

- Nine mixes of real recordings measured by ffmpeg: −14.0 LUFS (one −14.1) with true peaks of −0.9 dBTP or lower. A dry voice at −25.4 LUFS and a fresh music track also reached −14.0.
- An app-made voice with a date, price and number transcribed identically dry, at 50% and at 100% music volume (34 of 34 words).
- An older sample that already loses words when dry gave the same 49 words in five mixes, apart from one word swap ("and" or "in").
- In the app: empty page, long titles, create, play, Library entry, a source moved away while the page was open, and deleting the mix shown on the page. A 13 s voice mixed in 3 s in a debug build.
- Not checked: Export through the dialog for a mix (shared code), mixes of the longest scripts, and listening by a person.

## Model catalog (2026-11-25)

Release dates and authors come from the model cards; ACE-Step and Kitten dates are when the weights appeared on Hugging Face.

| Voice model | Download | Time for about 13 s of English | Transcription |
| --- | --- | --- | --- |
| Kitten Nano, Micro, Mini | 61, 45, 82 MB | 1.3, 4.4, 8.1 s | 24 samples: every date, number and time right; one price heard wrongly (Mini, "Bruno") |
| Supertonic 3 | 145 MB | 1.5 s | Reads "$49.99" wrongly, so English prices are spelled out first; then 10 of 11 prices and 10 of 11 dates were right |

- Supertonic was run once in each of 31 languages. Latvian, Estonian, Lithuanian, Vietnamese and Hungarian had the most differences; Whisper small is also weak there, so these need native listeners.
- Extra quality steps did not improve Supertonic. Pauses affect Kokoro and Kitten only.

| Music model (GTX 1060) | 32 s track | Peak VRAM (whole system) |
| --- | --- | --- |
| SFT, 50 steps | 58 s | 2.9 GB |
| SFT, 30 steps, prompt strength 4 | 63 s | 2.9 GB |
| Turbo HQ | 31 s | 4.2 GB |
| XL Turbo | 40 s; 242 s track 213 s | 4.2; 4.7 GB |

- The same seed gave identical audio twice and another seed did not. Creativity and steps changed the result.
- In the app: simple and advanced pages, filters, sorting, search, favourites, download, a Resume that linked shared files instead of fetching them, generating with three voice models and SFT, an invalid seed, switching back to simple, and removing one model without harming others.
- Not checked: Turbo HQ and XL on the CPU, the memory recommendations for them, musical quality, and moving shared files between two real drives (unit test only).

## Voice cloning (2026-11-25)

- Pitch followed the sample: 198 Hz gave 174 to 203 Hz, and 117 Hz gave 103 to 110 Hz. Likeness was not judged by a person.
- Dates, times, numbers and names transcribed correctly in every sample. One price was read wrongly, so prices are spelled out as for Supertonic.
- A 1,900-character script made 102 s of audio in 36 s, including loading the model.
- With the original samples, "Chapter one" became "After one" in most tries, and another voice added "And" before sentences. With silence around the sample and an end at a pause, 12 of 12 openings were right across both voices; a stray first word remains possible.
- Speed has no effect with this model, so the control is hidden. More steps did not help.
- In the app, with a fake microphone: record, too short, silence, a broken file, an MP3 file, consent required, consent asked again for another owner, names with accents and Chinese, use, generate, Cloned label, play, delete. Creating a voice without consent is refused by Rust.
- Not checked: a real microphone and its permission prompt, renaming, the long-run quality of many voices, and macOS.

## Songs (2026-11-25)

Share of the sung words recognised by Whisper small, one try each on a GTX 1060 (recommended model; about 30 s for 45 s of song):

| Words recognised | Languages |
| --- | --- |
| 98% or more | English (3 songs), Polish |
| 90 to 95% | Portuguese, Italian, Spanish, French, German |
| 83 to 89% | Korean, Turkish |
| Below 60% | Arabic, Japanese, Chinese, Hindi, Russian; Telugu 0% |

- Whisper is weak on sung Hindi, Telugu and Chinese, so these need listeners. Turbo HQ scored higher for Hindi (72%), Japanese (78%) and Arabic (73%), and lower for French, Korean, Polish and Turkish.
- Most songs ended on their own; Japanese stopped mid-music, which the app now composes again.
- Display driver resets (event 4101) appeared 19 times in one hour, for instrumentals and for the older engine too. After D041 there were none in more than 20 tracks.
- In the app: both modes, section buttons, lyrics required, too long, languages, generate, Library label, not offered in Mix. Over MCP: a song, missing and bad lyrics, cancel.
- Not checked: listening, songs longer than 137 s, SFT and XL, and songs on the CPU.

## Claude Code connection (2026-11-25)

- Over real pipes: handshake, tools, speech (2.4 s), a 12 s track (20 s), a mix, cancel, wrong recordings, a cloned voice refused, out-of-range and undocumented arguments, and closing (0.1 s, no processes left).
- Not checked: inside a Claude Code session.

## Known issues to handle

1. Solved: exact length without a silent ending (D021).
2. Solved: tracks are normalised from unclipped samples.
3. Solved: tile 256 cut VAE decode of a 92 s track from 32 s to 21 s.
4. Solved: common English abbreviations are expanded before speech.
5. acestep.cpp has no release binaries; `pnpm engines` builds it, and CI must do the same in Phase 8.

## Not verified

- macOS: no Mac available. The project supports Metal, but it needs a CI build and a real-device run.
- AMD and Intel GPUs, and PCs with 8 GB RAM.
- Qwen3-TTS.
- Generation with the network adapter physically off (neither engine imports network code).
- Microsoft Store acceptance.
