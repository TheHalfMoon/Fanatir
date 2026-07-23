# NEXT-ACTION

## Authorized next action

**T002 complete** (evidence: `docs/program-memory/baseline/R0-build-commands.md`).

Next eligible task (do **not** auto-execute unless requested):

```text
T003 — Verify dependency managers (pnpm/node/rust/python/uv) and versions
```

Dependencies for T003: **T001** (per tasks.md).

## Hard gates (unchanged)

- T030 (ADR-15/01/02/06 Accepted) before R2 implementation
- ADR-05/07/08/09/10/14 before relevant R3 work
- T060 (Decision D custody) before R5 distribution
- Vite-only cannot satisfy T042 / T055 / T066
- Implementing agent may not be the sole independent reviewer of its own Tier A work

## Not authorized yet

- Installing `afia-ui` dependencies (explicitly withheld in T002)
- Blanket `/speckit-implement`
- OpenMed or Graphify fork/import
- Specification 002 implementation
- Supabase migration mutation
- Auth/session behavior changes
- Push / PR
- Distributing unsigned builds
