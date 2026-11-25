# VoxMelody

Generate voice. Create music. All offline, on your machine.

VoxMelody is a free, open-source desktop app for Windows and macOS. It is in early development; voice generation works on Windows, and music is next.

## Develop

Requires Node 22, pnpm, Rust, and the [Tauri prerequisites](https://tauri.app/start/prerequisites/).

```sh
pnpm install
pnpm tauri dev
```

Build the Windows installer with `pnpm tauri build --bundles nsis`.

See [docs/PHASES.md](docs/PHASES.md) for the roadmap and [docs/VERIFICATION.md](docs/VERIFICATION.md) for model test results.

## Licence

GPL-3.0. Models keep their own licences.
