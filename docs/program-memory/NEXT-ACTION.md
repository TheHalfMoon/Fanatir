# NEXT-ACTION

## Authorized next action

**T004 complete** (evidence: `docs/program-memory/baseline/R0-frontend-map.md`).

Next eligible task (do **not** auto-execute unless requested):

```text
T005 — Verify auth/session/PrivateRoute/profile behavior (observe-only)
```

Dependencies for T005: **T004** (per tasks.md). Review: **Tier A**.

## Hard gates (unchanged)

- T030 (ADR-15/01/02/06 Accepted) before R2 implementation
- ADR-05/07/08/09/10/14 before relevant R3 work
- T060 (Decision D custody) before R5 distribution
- Vite-only cannot satisfy T042 / T055 / T066
- Implementing agent may not be the sole independent reviewer of its own Tier A work

## Not authorized yet

- Dev server / dependency install
- Auth/session/PrivateRoute/profile code changes
- Blanket `/speckit-implement`
- OpenMed or Graphify fork/import
- Specification 002 implementation
- Supabase migration mutation
- Push / PR
- Distributing unsigned builds
