# Phases

A phase starts only when the previous exit criteria are met.

| # | Phase | Scope | Exit criteria | Status |
| --- | --- | --- | --- | --- |
| 0 | Verification | Models, licences, real generation | Go/no-go recorded | Done: GO |
| 1 | Foundation | Repo, docs, Tauri + SvelteKit, brand, app shell | Desktop build runs | Done |
| 2 | Model manager | Separate voice and music packs; mirrors, checksum, resume, remove; hardware check; custom models folder | Models install and move between drives | Done on Windows |
| 3 | Voice Studio | Kokoro engine, voices, speed, preview, export | All words spoken; works with network off | Done on Windows |
| 4 | Music Studio | ACE-Step engine, prompt, length, BPM, key, cancel | Exact length, no silent tail; GPU and CPU | Done on Windows |
| 5 | Library | History, playback, rename, export, delete, Relink | Survives restart and moved files | Next |
| 6 | Mix | Duck music under voice, loudness, fades | −14 LUFS, voice intelligible | Planned |
| 7 | Model catalog | Browse and download many models, recommended by default but not restricted; favourites; choose a model per generation; filter by type, size and licence | Several models per type install side by side | Planned |
| 8 | Release | CI, macOS testing, signing, Store and DMG, updater | Installs on clean Windows and Mac | Planned |
| 9 | Later | Voice cloning with consent safeguards, MCP for Claude Code, more models | Scoped separately | Backlog |

macOS and 8 GB RAM checks should start during Phases 3 and 4, not wait for release.
