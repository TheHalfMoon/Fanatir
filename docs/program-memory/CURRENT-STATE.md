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
- Spec Kit: initialized with integration `cursor-agent` (specify-cli 0.14.0)
- Constitution: **v1.0.0 ratified 2026-07-23** at `.specify/memory/constitution.md`
- Active reconstitution spec: `specs/001-fanatir-repository-and-architecture-reconstitution/`
- Spec status: **Accepted** — founder-approved reconstitution specification (local acceptance commit pending/created)

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
- Alpha requirement (founder Q6): bounded DeepMed pipeline required; OpenMed import **not** authorized in 001

## Graphify local build (verified 2026-07-23)

- Output: `C:\Projects\Fanatir-Ecosystem\graphify-out\`
- Scale: 1698 nodes, 4107 edges, 199 communities
- Repository representation: Fanatir code nodes present; Fehrest empty; DeepMed-AI README-only under `--code-only`
- MCP config (ecosystem parent): `.cursor/mcp.json` server `fanatir-ecosystem-graph`

## Explicitly unresolved (post Q1-Q9)

- Duplicate Supabase migration filesystem canonicalization
- Shared primitive schemas
- Fehrest/DeepMed release packaging/version channels
- Interim packaging detail for first vertical slice vs minimal trusted host
