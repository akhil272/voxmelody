# Handover

Last updated: 2026-11-25

## Current state
- The model manager, Voice Studio, Music Studio, Library and Mix work on Windows and are merged to local `dev`. Not pushed. Only `main` and `dev` branches remain.
- Tests: 62 in `src-tauri`, 6 in `engines/voice`.

## Next
1. Phase 7 Model catalog, on `feature/catalog` from `dev` (`PHASES.md`, D016).
2. Owner listens to voice, music and mix samples and confirms they sound good enough. Mixing levels are constants in `src-tauri/src/mix/render.rs`.
3. Test music on AMD, Intel and 8 GB RAM PCs; decide on integrated GPUs (D022).
4. Music tracks now pass through the limiter (D028); tracks made earlier keep their old loudness.

## Testing
- `cargo test` in `src-tauri` covers downloads, moves, mastering and the Library.
- Set `VOXMELODY_TEST_ENGINE` and `VOXMELODY_TEST_VOICE` to run the voice tests against a real engine; without them those tests pass without checking anything.
- Set `VOXMELODY_TEST_MUSIC_ENGINE` (its DLLs must sit beside it) and `VOXMELODY_TEST_MUSIC` for the real music tests.
- Set `VOXMELODY_TEST_MIX_VOICE`, `_MUSIC` and `_OUTPUT` (and optionally `_VOLUME`, 0 to 1) to render a real mix in the tests.
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
- Builds from before Mix cannot read a library that contains a mix; they set it aside as `library.damaged`. Update the installed copy before making mixes.
- A mix is held in memory: expect about 1 GB for a 20-minute voice.
- Do not upgrade sherpa-onnx to 2.x (D003) or add MusicGen (D004).
- `.verify/` is local test data and is never committed. `.verify\VoxMelody Models` is the installed app's models folder; do not delete it.
