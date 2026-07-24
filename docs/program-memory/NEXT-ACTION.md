# NEXT-ACTION

## Authorized next action

**T009 complete** (evidence: `docs/program-memory/baseline/R0-ci-tests.md`).

Next eligible task (do **not** auto-execute unless requested):

```text
T010 — Classify paths active/scaffold/archived/orphaned/contradictory
```

Dependencies for T010: **T004, T006, T007, T008, T009** (per tasks.md). Review: **Tier B**.

## Hard gates

- Do not claim CI green / production-ready quality from current workflows
- Do not reactivate `_archived/**` without ADR acceptance
- No OpenMed fork/import; no migration mutation under spec 001
- T030 before R2; T060 before signed R5

## Not authorized yet

- Repairing CI / creating tests
- Running builds or installs
- `/speckit-implement`
- Push / PR
