# CURRENT-STATE

Verified local facts only. Unresolved items are marked explicitly.

## Fanatir

- Branch: `docs/f0-planning-memory-bootstrap` (local only; no upstream)
- Spec Kit: `specs/001-fanatir-repository-and-architecture-reconstitution`
- **T001–T004**: PASS WITH NOTES
- **T005–T006**: PASS WITH NON-BLOCKING NOTES (Tier A complete)
- **T007**: PASS WITH NOTES — commit `1115f181b689e136839493c480469fa4258f6768`
- **T008**: **PASS WITH NOTES** — `docs/program-memory/baseline/R0-archived-assets.md`
  - `_archived/crates`, `apps-desktop`, `services-operations-go` = **non-authoritative scaffolds**
  - Root Cargo/go/pnpm still reference missing live paths (archive counterparts exist)
  - Tauri v2 stub; empty capabilities; bundle targets app/dmg (not Windows-first)
  - Go ops skeleton: archive only; not Alpha-required
  - Do not reactivate without ADR-gated reconstitution

## Fehrest / DeepMed-AI

- Unchanged; clean

## Remaining

- Spec 002; ADR-01/02/05/06/09/10/14/15 implementation drafts
- Authorized afia-ui install; signing custody
