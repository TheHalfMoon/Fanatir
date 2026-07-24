# NEXT-ACTION

## Authorized next action

**T008 complete** (evidence: `docs/program-memory/baseline/R0-archived-assets.md`).

Next eligible task (do **not** auto-execute unless requested):

```text
T009 — Inspect current CI and tests honesty
```

Dependencies for T009: **T001** (per tasks.md). Review: Tier C.

## Hard gates

- Do not reactivate `_archived/**` as authority without ADR acceptance
- Do not repair root Cargo/go/pnpm manifests in R0 without an authorized task
- No OpenMed fork/import; no migration mutation under spec 001
- T030 before R2; T060 before signed R5

## Not authorized yet

- Compiling archived Rust/Go/Tauri
- Restoring crates into live paths
- `/speckit-implement`
- Push / PR
