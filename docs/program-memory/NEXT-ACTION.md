# NEXT-ACTION

## Authorized next action

**T006 complete and founder-accepted** (evidence: `docs/program-memory/baseline/R0-supabase-inventory.md`).

Tier A independent review for T006: **complete**.

Next eligible task (do **not** auto-execute unless requested):

```text
T007 — Inspect Python/FHIR/OpenMed prototype services
```

Dependencies for T007: **T001** (per tasks.md). Review: **Tier C**.

## Hard gates

- No migration mutation / documents-crypto expansion under spec 001
- No AuthContext/PrivateRoute changes until auth migration ADR/spec
- No OpenMed fork/import under spec 001
- T030 before R2; T060 before signed R5

## Not authorized yet

- Live Supabase / CLI / migration apply
- Spec 002 implementation
- `/speckit-implement`
- Push / PR
