# NEXT-ACTION

## Authorized next action

The task program for specification 001 is founder-accepted.

Execute **only**:

```text
T001 — Record R0 repository baseline
```

Evidence path (per tasks.md):

```text
docs/program-memory/baseline/R0-repo-baseline.md
```

Do **not** execute T002 or later until T001 acceptance criteria pass.

Do **not** run `/speckit-implement` as a blanket auto-runner.

**Hard gates (unchanged):**

- T030 (ADR-15/01/02/06 Accepted) before R2 implementation
- ADR-05/07/08/09/10/14 before relevant R3 work
- T060 (Decision D custody) before R5 distribution
- Vite-only cannot satisfy T042 / T055 / T066
- Implementing agent may not be the sole independent reviewer of its own Tier A work

## Not authorized yet

- T002+ until T001 complete
- OpenMed or Graphify fork/import
- Creating/executing specification 002
- Supabase migration mutation
- Auth/session behavior changes
- Push / PR
- Distributing unsigned builds
