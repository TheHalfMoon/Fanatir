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
- **T001–T004**: PASS WITH NOTES
- **T005**: **PASS WITH NON-BLOCKING NOTES** — `docs/program-memory/baseline/R0-auth-session.md`; **Tier A independent review complete** (founder-accepted)
  - Session-only PrivateRoute; no ProfileGate; profile/consent sync fail-open (not auth bypass)
  - UI role checks and direct DB writes = trust seams; server/RLS deferred to T006
  - Compatibility boundary preserves AuthContext API, loading, PrivateRoute, OTP, consent side effect, logout, invite-return, workspace expectations

## Fehrest / DeepMed-AI

- Fehrest: empty; clean; untouched through T005
- DeepMed-AI: `796168f` README-only; clean; untouched through T005

## Prior notes

- afia-ui active; stale root manifests; no `node_modules`; pnpm mismatch
- rustc/cargo 1.97.1 healthy at T003; Cargo members still missing

## Remaining unresolved operational details

- Named operational release custodian / signing custody
- OpenMed pin / model licenses
- Authorized afia-ui install + pnpm reconciliation
- T006: RLS / migration trees / documents-crypto / service-role
