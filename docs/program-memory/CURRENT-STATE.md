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
- Spec acceptance commit: `46f55c4e9a6b69aecbd85007e98688141939f869` (`docs(spec): define Fanatir architecture reconstitution`)
- Spec Kit: initialized with integration `cursor-agent` (specify-cli 0.14.0)
- Constitution: **v1.0.0 ratified 2026-07-23** at `.specify/memory/constitution.md`
- Active reconstitution spec: `specs/001-fanatir-repository-and-architecture-reconstitution/`
- Spec status: **Accepted** (acceptance commit `46f55c4e9a6b69aecbd85007e98688141939f869`)
- Plan status: **Accepted** (founder-approved reconstitution plan; local plan-acceptance commit in this change set)

## Fehrest

- Remote: `https://github.com/IamShehri/Fehrest.git`
- Local path: `C:\Projects\Fanatir-Ecosystem\Fehrest`
- Branch: `main`
- Commits: **none** (empty repository clone)
- Alpha requirement (founder Q5): bounded independent + embedded Fehrest Alpha — **not initialized yet**

## DeepMed-AI

- Remote: `https://github.com/IamShehri/DeepMed-AI.git`
- Local path: `C:\Projects\Fanatir-Ecosystem\DeepMed-AI`
- Branch: `main`
- HEAD: `796168f821ff468e533234c5f05b74a1b8cc407f`
- Tip commit message: `Update project description in README.md`
- Alpha: bounded DeepMed required; OpenMed **fork/import** not authorized in 001; **Decision B** allows temporary R3 OpenMed **PyPI** runtime use via ADR-09

## Graphify local build (verified 2026-07-23)

- Output: `C:\Projects\Fanatir-Ecosystem\graphify-out\`
- Scale: 1698 nodes, 4107 edges, 199 communities
- Repository representation: Fanatir code nodes present; Fehrest empty; DeepMed-AI README-only under `--code-only`
- MCP config (ecosystem parent): `.cursor/mcp.json` server `fanatir-ecosystem-graph`

## Founder decisions recorded in plan (2026-07-23)

- **A / ADR-15**: Rust-first polyglot; ADR-15 before R2 implementation; constrains ADR-01/02/04/05/06/08/09/10/13/14
- **B**: R3 DeepMed may use OpenMed PyPI temporarily (not fork/import) under ADR-09 rules
- **C**: Content/PHI/Artifact SoT → Rust Artifact Store; freeze `documents-crypto`; future **002-supabase-local-first-and-migration-canonicalization** authorized (not created)
- **D**: Founder = signing authority owner; custody controls before R5 distribution; unsigned builds internal-only

## Remaining unresolved operational details

- Named operational release custodian
- Hardware-backed key vs approved secure signing service selection
- Exact OpenMed PyPI pin + per-model license manifest (ADR-09/tasks)
- Fehrest vs DeepMed init sequencing if R3 resource-constrained
- App vs updater signing key separation details where applicable
