# Handover

Last updated: 2026-11-25

## Current state
- Phase 0 complete: GO. See `VERIFICATION.md`.
- Phases 1 to 3 complete on Windows: app shell, model manager, Voice Studio. Installer 6.7 MB. Music controls are disabled.
- Tests: 25 in `src-tauri`, 6 in `engines/voice`.

## Next
1. Phase 4 Music Studio: build acestep.cpp commit `973517b` (Vulkan and CPU), run it as a process, follow D018.
2. Fix music endings: generate longer, trim silence, fade, cut to exact length; normalise loudness.
3. Report the GPU in the hardware check from the engine's device list.
4. Owner confirms the voice and music samples sound good enough.

## Testing
- `cargo test` in `src-tauri` covers downloads and moves.
- Set `VOXMELODY_TEST_ENGINE` and `VOXMELODY_TEST_VOICE` to run the voice tests against a real engine; without them those tests pass without checking anything.
- Debug builds read `VOXMELODY_DATA_DIR` to keep test data away from real data.

## Watch out
- Read the known issues and unverified list in `VERIFICATION.md` first.
- `panic = "abort"` is on in release: handle errors with `Result`, never rely on catching panics.
- `src-tauri/tauri.macos.conf.json` repeats the window settings; keep both files in sync.
- Do not upgrade sherpa-onnx to 2.x (D003) or add MusicGen (D004).
- `.verify/` is local test data and is never committed.
