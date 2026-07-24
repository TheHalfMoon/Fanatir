# CURRENT-STATE

Verified local facts only. Unresolved items are marked explicitly.

## Fanatir

- Branch: `docs/f0-planning-memory-bootstrap` (local only; no upstream)
- Spec Kit: `specs/001-fanatir-repository-and-architecture-reconstitution`
- **T001–T004**: PASS WITH NOTES
- **T005–T006**: PASS WITH NON-BLOCKING NOTES (Tier A complete)
- **T007**: **PASS WITH NOTES** — `docs/program-memory/baseline/R0-python-services.md`
  - `openmed_bridge.py` / `fhir_gate.py` classified as **prototypes**
  - Bridge: FastAPI on `127.0.0.1:8765`; eager `import openmed`; UI client wired; runtime unverified
  - FHIR gate: **R4B** structural Bundle emit — not full validator / not commandF
  - OpenMed not pinned in requirements-bridge; no NOTICE/model-license manifest observed
  - services/README contradictory (describes missing dirs)
  - extract-pii TS/Python response shape mismatch

## Fehrest / DeepMed-AI

- Unchanged; clean

## Remaining

- Spec 002 migration canonicalization
- ADR-09 / ADR-10 for DeepMed OpenMed pin and commandF/FHIR version
- Authorized afia-ui install; signing custody
