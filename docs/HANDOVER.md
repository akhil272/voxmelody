# Handover

Last updated: 2026-11-26

## Current state
- The model catalog, Voice Studio, My Voices (cloning), Music Studio (instrumentals and songs), Library, Mix and the Claude Code connection work on Windows and are merged to `dev`. `dev` is pushed. Only `main` and `dev` branches remain.
- The repository is public and GPL-3.0 (D043). Version is 1.0.0 (D048).
- New on `dev`: violet brand colour, About with support and social links, recordings folder in Settings, Library tabs. Checked in the browser preview and by tests; not yet in an installed copy.
- Do not open a pull request to `main` unless the owner asks.
- Tests: 88 in `src-tauri`, 8 in `engines/voice`.

## Next
1. Owner tests the installed copy: My Voices with a real microphone, songs in Music Studio, and the command in Settings > Connections. Act on that feedback first.
2. Owner confirms the Pocket TTS download source (D034).
3. Phase 8 Release (D049): owner creates the R2 bucket, public domain and token, and fills `.env.release`. Then tag `v1.0.0` on the release commit and push the tag; when the macOS workflow passes, run `node .private/release.mjs`. The first macOS build has not run yet. The website listing is a separate task in the builtbyakhil repository. Unsigned builds warn on first open; Apple notarization needs a paid Developer ID.
4. Owner listens to voice, music and mix samples from every model. Mixing levels are constants in `src-tauri/src/mix/render.rs`.
5. Native listeners check Supertonic's weaker languages (`VERIFICATION.md`).
6. Test music on AMD, Intel and 8 GB RAM PCs; decide on integrated GPUs (D022).

## Testing
- `cargo test` in `src-tauri` covers downloads, moves, mastering and the Library.
- Set `VOXMELODY_TEST_ENGINE` and `VOXMELODY_TEST_VOICE` to run the voice tests against a real engine; without them those tests pass without checking anything.
- Set `VOXMELODY_TEST_MUSIC_ENGINE` (its DLLs must sit beside it) and `VOXMELODY_TEST_MUSIC` for the real music tests, and run them with `--test-threads=1`: two engines at once overload the graphics card.
- Set `VOXMELODY_TEST_MIX_VOICE`, `_MUSIC` and `_OUTPUT` (and optionally `_VOLUME`, 0 to 1) to render a real mix in the tests.
- Debug builds read `VOXMELODY_DATA_DIR` to keep test data and recordings away from real data. Set `WEBVIEW2_USER_DATA_FOLDER` too when the installed app is open.
- Run a debug build with `mcp` as its argument and send JSON-RPC lines to test the Claude Code connection.
- A fake microphone for tests: `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--use-fake-ui-for-media-stream --use-fake-device-for-media-stream --use-file-for-fake-audio-capture=<wav>`.
- A debug build shows the built UI only with `cargo build --features tauri/custom-protocol`, after `pnpm build`.
- The owner tests an installed copy on this PC. Stop test processes by path or id, never by name.

## Watch out
- To add a model: edit `scripts/build-manifest.mjs`, run it, add its description in `src/lib/constants/copy.ts`, its licence in `THIRD_PARTY_LICENSES.md`, and voices in `src-tauri/src/voice/voices.rs`.
- Development and the installed app share the models in `.verify\VoxMelody Models`. Set `VOXMELODY_DATA_DIR` to `.verify\dev`, whose settings point there. Moving or removing models in either affects both. SFT and XL were deleted; download them again to test.
- `.private/release.mjs` is not in git. Back it up; without it, rebuild it from D049.
- Read the known issues and unverified list in `VERIFICATION.md` first.
- `panic = "abort"` is on in release: handle errors with `Result`, never rely on catching panics.
- `src-tauri/tauri.macos.conf.json` repeats the window settings; keep both files in sync.
- `pnpm engines` needs `VULKAN_SDK`, Visual Studio and about 1 GB free; the first build takes about four minutes. On this PC the SDK is `.verify\vk\sdk\Library`.
- CMake output lives in `engines/music/.build`; the dot keeps svelte-check from scanning it.
- `src-tauri/tauri.windows.conf.json` bundles the music DLLs beside the app.
- Builds from before Songs cannot read a library that contains a song, as with mixes below.
- Builds from before Mix cannot read a library that contains a mix; they set it aside as `library.damaged`. Update the installed copy before making mixes.
- A connected Claude Code session keeps `voxmelody.exe mcp` running; installing or removing a model in use can fail until it ends.
- macOS microphone access needs the text in `src-tauri/Info.plist`, and a signed build also needs the audio input entitlement.
- A mix is held in memory: expect about 1 GB for a 20-minute voice.
- Do not upgrade sherpa-onnx to 2.x (D003) or add MusicGen (D004).
- `.verify/` is local test data and is never committed. `.verify\VoxMelody Models` is the installed app's models folder; do not delete it.
