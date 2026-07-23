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
- **T001**: **PASS WITH NOTES** — `docs/program-memory/baseline/R0-repo-baseline.md` (commit `ea2c3f66c62758e058fbe89a2781473054e2fe8e`)
- **T002**: **PASS WITH NOTES** — `docs/program-memory/baseline/R0-build-commands.md`
  - Active app root: `afia-ui/`
  - Authority: pnpm via `afia-ui/package.json` + `afia-ui/pnpm-lock.yaml` (declares pnpm@10.4.1)
  - `pnpm run build` / `pnpm run dev`: exit **1** — `node_modules` missing (`vite` not found); install withheld by T002 policy
  - Root workspace still points at missing `apps/desktop/ui`

## Fehrest / DeepMed-AI

- Fehrest: empty (0 commits); clean; untouched by T001–T002
- DeepMed-AI: HEAD `796168f` README-only; clean; untouched by T001–T002

## Manifest contradictions

- Root `Cargo.toml`, `go.work`, `pnpm-workspace.yaml` reference missing active paths; related evidence under `_archived/`
- Root pnpm workspace does **not** include `afia-ui/`

## Environment notes

- T001: `cargo 1.97.1` after rustup recovery; `rustc` still failed — Rust implementation blocked until authorized toolchain task
- T002 host tools: Node `v22.23.1`, PATH pnpm `9.0.0`, npm `10.9.8`
- Other tools (T001): git 2.55.0, gh 2.96.0, uv 0.11.24, specify 0.14.0, Python 3.11.15, graphify 0.9.25

## Remaining unresolved operational details

- Named operational release custodian
- Hardware-backed key vs approved secure signing service selection
- Exact OpenMed PyPI pin + per-model license manifest (ADR-09/tasks)
- Fehrest vs DeepMed init sequencing if R3 resource-constrained
- App vs updater signing key separation details where applicable
- Authorized dependency install for `afia-ui` (out of T002 scope)
