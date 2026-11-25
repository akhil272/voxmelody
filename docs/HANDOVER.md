# Handover

Last updated: 2026-11-25

## Current state
- Phase 0 complete: GO. See `VERIFICATION.md`.
- Phase 1 complete: branded SvelteKit app shell (D011). Windows release build launches; navigation and window buttons tested. Generation is not connected, so its controls are disabled.
- Phase 2 complete on Windows: model manager and custom models folder (D014, D015). Installer 1.6 MB. 18 Rust tests. macOS is untested.

## Next
1. Phase 3 Voice Studio: run sherpa-onnx as a child process against the installed voice pack; voices, speed, preview, export.
2. Bundle the sherpa-onnx engine with its licence and source offer (D003, D007).
3. CI that builds acestep.cpp from commit `973517b` for Windows (Vulkan) and macOS (Metal).
4. Owner confirms the voice and music samples sound good enough.

## Testing
- `cargo test` in `src-tauri` covers downloads and moves.
- Debug builds read `VOXMELODY_DATA_DIR` to keep test data away from real data.

## Watch out
- Read the known issues and unverified list in `VERIFICATION.md` first.
- `panic = "abort"` is on in release: handle errors with `Result`, never rely on catching panics.
- `src-tauri/tauri.macos.conf.json` repeats the window settings; keep both files in sync.
- Do not upgrade sherpa-onnx to 2.x (D003) or add MusicGen (D004).
- `.verify/` is local test data and is never committed.
