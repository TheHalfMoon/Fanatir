# NEXT-ACTION

## Authorized next action

**T007 complete** (evidence: `docs/program-memory/baseline/R0-python-services.md`).

Next eligible task (do **not** auto-execute unless requested):

```text
T008 — Inspect archived Rust/Tauri/Go assets without reactivation
```

Dependencies for T008: **T001** (per tasks.md). Review: Tier C.

## Hard gates

- No OpenMed fork/import; no service rewrite under T007 closeout
- No migration mutation / documents-crypto expansion under spec 001
- No AuthContext/PrivateRoute changes until auth migration ADR/spec
- T030 before R2; T060 before signed R5

## Not authorized yet

- Starting OpenMed/FHIR services or downloading models
- Spec 002 / ADR implementation coding
- `/speckit-implement`
- Push / PR
