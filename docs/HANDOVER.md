# Handover

Last updated: 2026-11-25

## Current state
- Phase 0 complete: GO. See `VERIFICATION.md`.
- Phases 1 to 4 complete on Windows: app shell, model manager, Voice Studio, Music Studio. Installer 11.9 MiB.
- Tests: 38 in `src-tauri`, 6 in `engines/voice`.
- Phase 4 is on `feature/music-studio`, not yet merged to `dev`.

## Next
1. Owner listens to music and voice samples and confirms they sound good enough.
2. Phase 5 Library: make takes permanent (D019, D024).
3. Test music on AMD, Intel and 8 GB RAM PCs; decide on integrated GPUs (D022).
4. Phase 6 adds a limiter so every track reaches −14 LUFS (D021).

## Testing
- `cargo test` in `src-tauri` covers downloads and moves.
- Set `VOXMELODY_TEST_ENGINE` and `VOXMELODY_TEST_VOICE` to run the voice tests against a real engine; without them those tests pass without checking anything.
- Set `VOXMELODY_TEST_MUSIC_ENGINE` (its DLLs must sit beside it) and `VOXMELODY_TEST_MUSIC` for the real music tests.
- Debug builds read `VOXMELODY_DATA_DIR` to keep test data away from real data.

## Watch out
- Read the known issues and unverified list in `VERIFICATION.md` first.
- `panic = "abort"` is on in release: handle errors with `Result`, never rely on catching panics.
- `src-tauri/tauri.macos.conf.json` repeats the window settings; keep both files in sync.
- `pnpm engines` needs `VULKAN_SDK`, Visual Studio and about 1 GB free; the first build takes about four minutes.
- CMake output lives in `engines/music/.build`; the dot keeps svelte-check from scanning it.
- `src-tauri/tauri.windows.conf.json` bundles the music DLLs beside the app.
- Do not upgrade sherpa-onnx to 2.x (D003) or add MusicGen (D004).
- `.verify/` is local test data and is never committed.
