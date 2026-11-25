# Handover

Last updated: 2026-11-25

## Current state
- Phase 0 complete: GO. See `VERIFICATION.md`.
- Phases 1 to 5 complete on Windows: app shell, model manager, Voice Studio, Music Studio, Library. Installer 11.9 MiB.
- Tests: 49 in `src-tauri`, 6 in `engines/voice`.
- Phase 5 is on `feature/library`, not yet merged to `dev`.

## Next
1. Owner listens to music and voice samples and confirms they sound good enough.
2. Phase 6 Mix: choose a voice and a music recording from the Library and mix them. Add a limiter so every track reaches −14 LUFS (D021).
3. Test music on AMD, Intel and 8 GB RAM PCs; decide on integrated GPUs (D022).

## Testing
- `cargo test` in `src-tauri` covers downloads and moves.
- Set `VOXMELODY_TEST_ENGINE` and `VOXMELODY_TEST_VOICE` to run the voice tests against a real engine; without them those tests pass without checking anything.
- Set `VOXMELODY_TEST_MUSIC_ENGINE` (its DLLs must sit beside it) and `VOXMELODY_TEST_MUSIC` for the real music tests.
- With `VOXMELODY_DATA_DIR` set, debug builds also keep recordings there. Set `WEBVIEW2_USER_DATA_FOLDER` too when the installed app is open.
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
