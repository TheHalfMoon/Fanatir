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
- HEAD at T004 evidence: `cc2345cb55318998cdd15bee4f20c5bdeae8845e` (T003)
- **T001–T003**: PASS WITH NOTES (baseline, build commands, tooling)
- **T004**: **PASS WITH NOTES** — `docs/program-memory/baseline/R0-frontend-map.md`
  - Bootstrap: `client/index.html` → `main.tsx` → `App.tsx`
  - Shell: `AppShell` (TopBar / PrimaryRail / SecondaryBar / Inspector / StatusBar)
  - Router: wouter; `PrivateRoute` redirects to `/login`; **no ProfileGate**
  - Surfaces: Studio, clinical, Research, Lab/Analytics, CoLab workspace, AI/OpenMed
  - Direct seams: Supabase auth/profiles/edge functions; OpenMed `127.0.0.1:8765`; `@kernel` via `lib/`
  - Montada / Pictorial: not present as active UI

## Fehrest / DeepMed-AI

- Fehrest: empty; clean; untouched through T004
- DeepMed-AI: `796168f` README-only; clean; untouched through T004

## Manifest / toolchain notes (prior)

- Stale root Cargo/Go/pnpm paths; afia-ui is active UI
- PATH pnpm 9.0.0 vs afia-ui pnpm 10.4.1; no `node_modules`
- rustc/cargo 1.97.1 healthy at T003; T001 rustc failure historical; Cargo members still missing

## Remaining unresolved operational details

- Named operational release custodian
- Signing custody details
- OpenMed pin / model licenses
- Authorized afia-ui install + pnpm reconciliation
