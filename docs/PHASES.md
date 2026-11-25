# Phases

A phase starts only when the previous exit criteria are met.

| # | Phase | Scope | Exit criteria | Status |
| --- | --- | --- | --- | --- |
| 8 | Release | CI, macOS testing, signing, Store and DMG, updater | Installs on clean Windows and Mac | Next |
| 9 | Later | More models (voice cloning and MCP are done) | Scoped separately | Backlog |

Completed phases are removed; `VERIFICATION.md` keeps their test results. macOS and 8 GB RAM checks should not wait for release.
