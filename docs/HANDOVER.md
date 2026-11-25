# Handover

Last updated: 2026-11-25

## Current state
- Phase 0 complete: GO. See `VERIFICATION.md`.
- Phases 1 to 5 complete on Windows and merged to local `dev`: app shell, model manager, Voice Studio, Music Studio, Library. Not pushed. Installer 11.9 MiB.
- Tests: 49 in `src-tauri`, 6 in `engines/voice`.

## Next
1. Phase 6 Mix, on `feature/mix` from `dev`. Exit: −14 LUFS with intelligible voice (`PHASES.md`).
   - Choose a voice and a music recording from the Library, duck the music under the voice, fade, export, and keep the mix in the Library.
   - Decide first: whether the mix gets its own page, and music longer or shorter than the voice.
   - Add a limiter so mixes, and music tracks, reach −14 LUFS (D021).
   - Reuse `src-tauri/src/audio` (loudness, WAV) and `music/master.rs` (trim, fade, crossfade). `wav.rs` writes stereo but has no WAV reader yet.
   - Voice is mono, 16-bit, usually 24 kHz; music is stereo, 16-bit, 48 kHz. Voice needs resampling.
   - Library `Kind` in Rust and `PackId` in the UI only know voice and music; a mix needs a third kind that has no model pack.
   - Check intelligibility by transcription, as in Phase 0.
2. Owner listens to voice and music samples and confirms they sound good enough.
3. Test music on AMD, Intel and 8 GB RAM PCs; decide on integrated GPUs (D022).

## Testing
- `cargo test` in `src-tauri` covers downloads, moves, mastering and the Library.
- Set `VOXMELODY_TEST_ENGINE` and `VOXMELODY_TEST_VOICE` to run the voice tests against a real engine; without them those tests pass without checking anything.
- Set `VOXMELODY_TEST_MUSIC_ENGINE` (its DLLs must sit beside it) and `VOXMELODY_TEST_MUSIC` for the real music tests.
- Debug builds read `VOXMELODY_DATA_DIR` to keep test data and recordings away from real data. Set `WEBVIEW2_USER_DATA_FOLDER` too when the installed app is open.
- A debug build shows the built UI only with `cargo build --features tauri/custom-protocol`, after `pnpm build`.
- The owner tests an installed copy on this PC. Stop test processes by path or id, never by name.

## Watch out
- Read the known issues and unverified list in `VERIFICATION.md` first.
- `panic = "abort"` is on in release: handle errors with `Result`, never rely on catching panics.
- `src-tauri/tauri.macos.conf.json` repeats the window settings; keep both files in sync.
- `pnpm engines` needs `VULKAN_SDK`, Visual Studio and about 1 GB free; the first build takes about four minutes. On this PC the SDK is `.verify\vk\sdk\Library`.
- CMake output lives in `engines/music/.build`; the dot keeps svelte-check from scanning it.
- `src-tauri/tauri.windows.conf.json` bundles the music DLLs beside the app.
- Do not upgrade sherpa-onnx to 2.x (D003) or add MusicGen (D004).
- `.verify/` is local test data and is never committed. `.verify\VoxMelody Models` is the installed app's models folder; do not delete it.
