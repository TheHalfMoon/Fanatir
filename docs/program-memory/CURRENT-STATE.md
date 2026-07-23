# CURRENT-STATE

Verified local facts only. Unresolved items are marked explicitly.

## Ecosystem root

- Path (local machine fact): `C:\Projects\Fanatir-Ecosystem`
- Workspace file: `Fanatir-Ecosystem.code-workspace`
- Graphify output (local, uncommitted): `graphify-out/`
- Note: absolute paths below are local verified facts for this checkout, not portable constitution requirements.

## Fanatir

- Remote: `https://github.com/IamShehri/Fanatir.git`
- Local path: `C:\Projects\Fanatir-Ecosystem\Fanatir`
- Branch (local planning): `docs/f0-planning-memory-bootstrap` (not pushed)
- Bootstrap commit: `aa5db3f7cb0d8fac414d6ec0a1247c387422b682`
- Constitution commit: `1ee7c42ea0e06f182318522c232f598680268d2a`
- Spec acceptance commit: `46f55c4e9a6b69aecbd85007e98688141939f869`
- Plan acceptance commit: `ac5c777e91b74fc30903a364f03337a5ac8a63f6` (`docs(plan): define Fanatir reconstitution program`)
- Spec Kit: initialized with integration `cursor-agent` (specify-cli 0.14.0)
- Constitution: **v1.0.0 ratified 2026-07-23** at `.specify/memory/constitution.md`
- Active reconstitution: `specs/001-fanatir-repository-and-architecture-reconstitution/`
- Spec status: **Accepted**
- Plan status: **Accepted**
- Tasks status: **Accepted** (founder-approved execution program; local tasks-acceptance commit in this change set)
- Task counts: 70 total — R0=11, R1=19, R2=13, R3=14, R4=5, R5=8
- Founder gates: T030, T042, T055, T060, T066
- First executable task: **T001**

## Fehrest / DeepMed-AI

- Fehrest: empty clone (no commits)
- DeepMed-AI: README-only tip `796168f`
- OpenMed/Graphify fork/import: **not** authorized under 001

## Remaining unresolved operational details

- Named operational release custodian
- Hardware-backed key vs approved secure signing service selection
- Exact OpenMed PyPI pin + per-model license manifest (ADR-09/tasks)
- Fehrest vs DeepMed init sequencing if R3 resource-constrained
- App vs updater signing key separation details where applicable
