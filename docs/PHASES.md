# Phases

A phase starts only when the previous exit criteria are met.

| # | Phase | Scope | Exit criteria | Status |
| --- | --- | --- | --- | --- |
| 0 | Verification | Models, licences, real generation | Go/no-go recorded | Done: GO |
| 1 | Foundation | Repo, docs, Tauri + Next.js, brand, app shell | Desktop build runs | Done |
| 2 | Model manager | Download, checksum, resume, remove; hardware check | Models install on a clean PC | Next |
| 3 | Voice Studio | Kokoro engine, voices, speed, preview, export | All words spoken; works with network off | Planned |
| 4 | Music Studio | ACE-Step engine, prompt, length, BPM, key, cancel | Exact length, no silent tail; GPU and CPU | Planned |
| 5 | Library | History, playback, rename, export, delete, Relink | Survives restart and moved files | Planned |
| 6 | Mix | Duck music under voice, loudness, fades | −14 LUFS, voice intelligible | Planned |
| 7 | Release | CI, macOS testing, signing, Store and DMG, updater | Installs on clean Windows and Mac | Planned |
| 8 | Later | Voice cloning with consent safeguards, MCP for Claude Code, more models | Scoped separately | Backlog |

macOS and 8 GB RAM checks should start during Phases 3 and 4, not wait for release.
