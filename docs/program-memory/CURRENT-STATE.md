# CURRENT-STATE

Verified local facts only. Unresolved items are marked explicitly.

## Ecosystem root

- Path (local machine fact): `C:\Projects\Fanatir-Ecosystem`
- Workspace file: `Fanatir-Ecosystem.code-workspace`
- Graphify output (local, uncommitted): `graphify-out/`
- Note: absolute paths below are local verified facts for this checkout, not portable constitution requirements.

## Fanatir

- Remote: `https://github.com/IamShehri/Fanatir.git` (PUBLIC via `gh`)
- Local path: `C:\Projects\Fanatir-Ecosystem\Fanatir`
- Branch (local planning): `docs/f0-planning-memory-bootstrap` (not pushed; no upstream)
- Spec Kit feature: `specs/001-fanatir-repository-and-architecture-reconstitution`
- Spec / Plan / Tasks: **Accepted**
- **T001**: **PASS WITH NOTES** — `docs/program-memory/baseline/R0-repo-baseline.md` (parent of T002)
- **T002**: **PASS WITH NOTES** — `docs/program-memory/baseline/R0-build-commands.md` (commit `5f968c164b14c4afb0e14836b8f1b42ebb7675f2`)
- **T003**: **PASS WITH NOTES** — `docs/program-memory/baseline/R0-tooling.md`
  - Node `v22.23.1`; PATH pnpm `9.0.0` vs afia-ui `pnpm@10.4.1`; Corepack `0.34.6` available but not enabled
  - `rustc`/`cargo` **1.97.1** healthy at T003 time (T001 rustc failure was historical); workspace members still missing
  - Python `3.11.15` (Hermes venv first on PATH); `uv 0.11.24`; no Fanatir project venv
  - Go / R / DuckDB CLI / Tauri CLI / global Vite: unavailable

## Fehrest / DeepMed-AI

- Fehrest: empty (0 commits); clean; untouched through T003
- DeepMed-AI: HEAD `796168f` README-only; clean; untouched through T003

## Manifest contradictions

- Root `Cargo.toml`, `go.work`, `pnpm-workspace.yaml` reference missing active paths; related evidence under `_archived/`
- Root pnpm workspace does **not** include `afia-ui/`

## Remaining unresolved operational details

- Named operational release custodian
- Hardware-backed key vs approved secure signing service selection
- Exact OpenMed PyPI pin + per-model license manifest (ADR-09/tasks)
- Fehrest vs DeepMed init sequencing if R3 resource-constrained
- App vs updater signing key separation details where applicable
- Authorized afia-ui dependency install + pnpm 10.4.1 reconciliation
