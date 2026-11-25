# VoxMelody

Open-source Tauri desktop app that generates voice and music locally and offline, free for everyone on Windows and macOS.

Read `docs/HANDOVER.md` (what is next) and `docs/DECISIONS.md` (what is settled) before starting work. Check `docs/PHASES.md` for scope.

## Stack

- Tauri 2 (Rust) shell, Next.js static export, TypeScript, Tailwind, shadcn/ui
- Native engines run as separate processes: sherpa-onnx (voice), acestep.cpp (music). No Python at runtime.

## UI rules

- Customize every shadcn/ui component to the brand guidelines; never ship its default look.
- Write standard, production-ready UI microcopy. Never explain backend mechanics, browser storage, or internal logic.
- Use established product conventions (e.g. Relink, Restore, Placeholder).
- Keep explanations to a single sentence focused on user benefit.
- Use short, imperative button labels (1–3 words).

## Code rules

- Reuse components wherever possible; check `src/components` before creating one.
- Do not define inline helpers. Put them in `src/lib/utils` and reuse them.
- UI text lives in `src/lib/constants`; colours come from tokens in `src/app/globals.css`, never raw hex in components.
- The UI must build as a static export: no server actions, route handlers, or runtime Node APIs.
- No network calls except user-started model downloads. No telemetry.
- Verify each model's weight licence (not only its code licence) before adding it.

## Brand

| Token | Value | Use |
| --- | --- | --- |
| Teal | `#0F766E` | Primary, voice |
| Cobalt | `#1D4EDB` | Secondary, music |
| Slate | `#0F172A` | Text |
| Muted | `#64748B` | Muted text |

Headings use Space Grotesk; body and UI use Inter. Keep clear space around the logo, use approved colours, and maintain its aspect ratio.

## Git workflow

- `main` is released code; `dev` is integration.
- Branch every feature from `dev` (`feature/<name>`) and merge it back to `dev`.
- At release time, open a PR from `dev` to `main`. Never commit directly to `main`.

## Before finishing any task

1. `pnpm lint`, `pnpm build`, and `cargo check` in `src-tauri` pass.
2. Update `docs/HANDOVER.md`.
3. Record major decisions in `docs/DECISIONS.md`.
