# NEXT-ACTION

## Authorized next action

**T010 evidence complete** (`docs/program-memory/baseline/R0-path-classification.md`), including post-Tier-B governance-authority and precision corrections.

Awaiting founder acceptance + local docs commit (do not auto-commit).

After T010 is accepted and committed, next eligible task:

```text
T011 — Produce R0 baseline evidence package and program-memory closeout
```

Dependencies for T011: **T001–T010** (per tasks.md). Review: **Tier C**.

## Hard gates

- Draft ADR ≠ accepted authority (ADR-15 waits for **T030**)
- Do not claim CI green / Trusted Host present / validated FHIR / HIPAA from current tree
- Do not reactivate `_archived/**` without ADR acceptance (`retain` ≠ reactivate)
- No OpenMed fork/import; no migration mutation under spec 001
- No Fehrest/DeepMed init in R0
- T030 before R2; T060 before signed R5

## Not authorized yet

- Manifest/CI/path repairs
- `/speckit-implement`
- Push / PR
- T011 until T010 committed
