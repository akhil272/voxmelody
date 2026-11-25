# Handover

Last updated: 2026-11-25

## Current state
- Phase 0 complete: GO. See `VERIFICATION.md`.
- Phase 1: app shell and brand system. Generation is not connected yet, so its controls are disabled.

## Next
1. Phase 2 model manager: manifest with pinned URLs, sizes and SHA-256; resumable downloads; disk-space check; remove.
2. CI that builds acestep.cpp from commit `973517b` for Windows (Vulkan) and macOS (Metal).
3. Owner listens to the verification samples in `.verify/out` and confirms quality is acceptable.

## Watch out
- Read the known issues and unverified list in `VERIFICATION.md` first.
- Do not upgrade sherpa-onnx to 2.x (D003) or add MusicGen (D004).
- `.verify/` is local test data and is never committed.
