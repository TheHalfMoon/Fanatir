# CURRENT-STATE

Verified local facts only. Unresolved items are marked explicitly.

## Ecosystem root

- Path: `C:\Projects\Fanatir-Ecosystem`
- Workspace file: `Fanatir-Ecosystem.code-workspace`
- Graphify output (local, uncommitted): `graphify-out/`

## Fanatir

- Remote: `https://github.com/IamShehri/Fanatir.git`
- Local path: `C:\Projects\Fanatir-Ecosystem\Fanatir`
- Branch (local planning): `docs/f0-planning-memory-bootstrap` (not pushed)
- HEAD at branch creation: `c19e3e10fe4877b6a49910126df2359a8d344ba3`
- Tip commit message: `feat: workspace invites copy-link fallback, runtime origin URLs, health probe, hardened schema`
- Spec Kit: initialized with integration `cursor-agent` (specify-cli 0.14.0)
- Constitution: template present at `.specify/memory/constitution.md` — **not yet authored**

## Fehrest

- Remote: `https://github.com/IamShehri/Fehrest.git`
- Local path: `C:\Projects\Fanatir-Ecosystem\Fehrest`
- Branch: `main`
- Commits: **none** (empty repository clone)
- Product status: **unresolved / not inspected beyond empty remote**

## DeepMed-AI

- Remote: `https://github.com/IamShehri/DeepMed-AI.git`
- Local path: `C:\Projects\Fanatir-Ecosystem\DeepMed-AI`
- Branch: `main`
- HEAD: `796168f821ff468e533234c5f05b74a1b8cc407f`
- Tip commit message: `Update project description in README.md`
- Product status beyond README tip: **unresolved / not claimed**

## Graphify local build (verified 2026-07-23)

- Command: `graphify extract C:\Projects\Fanatir-Ecosystem --code-only --out C:\Projects\Fanatir-Ecosystem` then `graphify cluster-only C:\Projects\Fanatir-Ecosystem --no-label`
- Output: `C:\Projects\Fanatir-Ecosystem\graphify-out\` (`graph.json`, `graph.html`, `GRAPH_REPORT.md`)
- Scale: 1698 nodes, 4107 edges, 199 communities
- Repository representation: **Fanatir** code nodes present; **Fehrest** has no files to index; **DeepMed-AI** has README only (skipped by `--code-only`)
- `.git` internals: **0** hits in graph node source paths
- Secret-token string scan of `graph.json`: no matches for common credential markers
- MCP config (ecosystem parent, not inside Fanatir git): `.cursor/mcp.json` server `fanatir-ecosystem-graph`

## Explicitly unresolved

- Fehrest repository contents and intended role beyond the empty remote
- Whether DeepMed-AI beyond the tip commit reflects intended product scope
- Fanatir Constitution content (pending `/speckit.constitution`)
- Cross-repo ownership boundaries for Graphify / OpenMed imports (explicitly deferred)
