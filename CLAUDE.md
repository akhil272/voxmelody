# VoxMelody

Open-source Tauri desktop app that generates voice and music locally and offline, free for everyone on Windows and macOS.

Read `docs/HANDOVER.md` (what is next) and `docs/DECISIONS.md` (what is settled) before starting work. Check `docs/PHASES.md` for scope.

## Stack

- Tauri 2 (Rust) shell, SvelteKit (static, Svelte 5 runes), TypeScript, Tailwind, shadcn/ui via shadcn-svelte
- Native engines run as separate processes: sherpa-onnx (voice), acestep.cpp (music). No Python at runtime.

## UI rules

- Customize every shadcn/ui component to the brand guidelines; never ship its default look.
- Write standard, production-ready UI microcopy. Never explain backend mechanics, browser storage, or internal logic.
- Use established product conventions (e.g. Relink, Restore, Placeholder).
- Keep explanations to a single sentence focused on user benefit.
- Use short, imperative button labels (1–3 words).

## Code rules

- Reuse components wherever possible; check `src/lib/components` before creating one.
- Do not define inline helpers. Put them in `src/lib/utils` and reuse them.
- UI text lives in `src/lib/constants`; colours come from tokens in `src/routes/layout.css`, never raw hex in components.
- The UI must build as a static client-only app: no server routes, form actions, server load functions, or runtime Node APIs.
- Import project code through the `#lib` alias, and Lucide icons by path (`@lucide/svelte/icons/<name>`).
- Keep the installer small: models are never bundled, and new dependencies must justify their size.
- No network calls except user-started model downloads. No telemetry.
- Rust returns stable error codes; map them to user copy in `src/lib/constants`.
- Regenerate `src-tauri/models.json` with `scripts/build-manifest.mjs`; never edit it by hand.
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

1. `pnpm lint`, `pnpm build`, `cargo test` and `cargo clippy` in `src-tauri` pass.
2. Update `docs/HANDOVER.md`.
3. Record major decisions in `docs/DECISIONS.md`.
