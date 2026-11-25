# Decisions

Append new entries; supersede old ones instead of editing them.

## D001: Native engines, no Python runtime (2026-11-25)
Ship sherpa-onnx and acestep.cpp as child processes managed by Rust. The brief's FastAPI and Docker stack would require large installs and favours NVIDIA cards. It stays an option only for models without native ports.

## D002: Kokoro fp32 is the default voice model (2026-11-25)
int8 was 3.4× slower on the test CPU. Revisit for low-memory machines.

## D003: Pin sherpa-onnx to 1.13.8 (2026-11-25)
2.x silently drops numbers and unknown words. Upgrading requires a phonemizer and a passing transcription test.

## D004: ACE-Step replaces MusicGen (2026-11-25)
MusicGen weights are non-commercial. Default: ACE-Step 1.5 turbo Q4_K_M with the 0.6B LM.

## D005: Vulkan on Windows, Metal on macOS, CPU fallback (2026-11-25)
One Windows backend covers NVIDIA, AMD and Intel, and needs no CUDA install.

## D006: Piper is not a default (2026-11-25)
It is fast, but each voice carries its own licence. Add voices only after individual review.

## D007: GPL-3.0 app licence (2026-11-25)
Compatible with the bundled GPL-3.0 espeak-ng component and keeps forks open. Owner may revisit before outside contributions arrive.

## D008: Models download after install (2026-11-25)
Weights total about 3.6 GB. Installers stay small; generation works offline once models are installed. Downloads are pinned and checksum-verified.

## D009: Next.js static export (2026-11-25)
Tauri has no Node server. All privileged work goes through typed Rust commands.

## D010: Branching (2026-11-25)
`feature/*` from `dev`, merged to `dev`; releases are PRs from `dev` to `main`.
