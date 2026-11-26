# VoxMelody

Generate voice. Create music. All offline, on your machine.

VoxMelody is a free desktop app that turns text into natural speech and descriptions into original music and songs. Everything runs on your computer: no account, no uploads, no subscription and no tracking.

## Features

- **Voice Studio**: natural speech in more than 30 languages, with adjustable speed and pauses.
- **My Voices**: create a voice from your own recording, with a recorded consent step. Recordings made with it are marked as cloned.
- **Music Studio**: instrumentals from a description, or songs from your own lyrics, from 10 seconds to 4 minutes.
- **Mix**: put a voice over music with automatic ducking, intro and outro, mastered to streaming loudness.
- **Library**: every recording is saved as a WAV file in a folder you choose, with tabs for voice, music and mixes.
- **Models**: download only the models you want, keep them on any drive, and see each model's licence first.
- **Claude Code connection**: generate speech, music and mixes from Claude Code, still on your computer.

## Download

| System | Download |
| --- | --- |
| Windows 10 and 11 (64-bit) | Installer (`.exe`) |
| macOS, Apple silicon (M series) | Disk image (`.dmg`) |
| macOS, Intel | Disk image (`.dmg`) |

Download links are on [builtbyakhil.com](https://builtbyakhil.com). Models download inside the app the first time you need them.

## Privacy

VoxMelody only goes online when you download a model. Your scripts, lyrics, voices and recordings never leave your computer.

## Develop

Requires Node 22, pnpm, Rust, and the [Tauri prerequisites](https://tauri.app/start/prerequisites/). The music engine also needs CMake and Ninja; on Windows, use Visual Studio with the C++ workload and set `VULKAN_SDK`.

```sh
pnpm install
pnpm engines
pnpm tauri dev
```

Build the Windows installer with `pnpm tauri build --bundles nsis`.

See [docs/PHASES.md](docs/PHASES.md) for the roadmap, [docs/DECISIONS.md](docs/DECISIONS.md) for settled choices and [docs/VERIFICATION.md](docs/VERIFICATION.md) for model test results.

## Support

VoxMelody is free. If it helped you create something, you can [buy me a coffee](https://buymeacoffee.com/builtbyakhil).

## Licence

GPL-3.0. Models keep their own licences; see [THIRD_PARTY_LICENSES.md](THIRD_PARTY_LICENSES.md).
