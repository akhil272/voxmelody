# Handover

Last updated: 2026-11-25

## Current state
- Phase 0 complete: GO. See `VERIFICATION.md`.
- Phase 1 complete: branded SvelteKit app shell (D011). Windows release build launches; navigation and window buttons tested. Generation is not connected, so its controls are disabled. macOS is untested.

## Next
1. Phase 2 model manager: manifest with mirror URLs, sizes and SHA-256 (D013); separate voice and music packs; resumable downloads; disk-space check; remove.
2. CI that builds acestep.cpp from commit `973517b` for Windows (Vulkan) and macOS (Metal).
3. Owner listens to the verification samples in `.verify/out` and confirms quality is acceptable.

## Watch out
- Read the known issues and unverified list in `VERIFICATION.md` first.
- `panic = "abort"` is on in release: handle errors with `Result`, never rely on catching panics.
- `src-tauri/tauri.macos.conf.json` repeats the window settings; keep both files in sync.
- Do not upgrade sherpa-onnx to 2.x (D003) or add MusicGen (D004).
- `.verify/` is local test data and is never committed.
