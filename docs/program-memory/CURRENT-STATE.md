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
- **T001**: **PASS WITH NOTES** — evidence `docs/program-memory/baseline/R0-repo-baseline.md` (baseline recorded against tasks commit `797a35ac78b2599b7ef231dbd0a49b3d586ce245`)

## Fehrest / DeepMed-AI

- Fehrest: empty (0 commits); clean; untouched by T001
- DeepMed-AI: HEAD `796168f` README-only; clean; untouched by T001

## Manifest contradictions (from T001)

- Root `Cargo.toml`, `go.work`, `pnpm-workspace.yaml` reference missing active paths; related evidence under `_archived/`

## Environment notes (T001 — accepted)

- `cargo 1.97.1` eventually succeeded after rustup recovery
- `rustc` still failed (rename/download errors under `%USERPROFILE%\.rustup`)
- Honestly recorded environment limitation — **do not** treat Rust toolchain as healthy
- Rust implementation remains blocked until a later authorized environment/toolchain task proves `rustc` operational
- Other tools: git 2.55.0, gh 2.96.0, uv 0.11.24, specify 0.14.0, Node v22.23.1, pnpm 9.0.0, Python 3.11.15, graphify 0.9.25

## Remaining unresolved operational details

- Named operational release custodian
- Hardware-backed key vs approved secure signing service selection
- Exact OpenMed PyPI pin + per-model license manifest (ADR-09/tasks)
- Fehrest vs DeepMed init sequencing if R3 resource-constrained
- App vs updater signing key separation details where applicable
