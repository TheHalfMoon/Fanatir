# NEXT-ACTION

## Immediate next action

```text
Founder review and acceptance of T011 R0 closeout.
```

T011 documentation remains **uncommitted** until founder acceptance.

After **all** of the following:

1. T011 founder acceptance
2. Clean local T011 documentation commit
3. Any review gate required by Tasks 001 (Tier C)

…the subsequent task is:

```text
T012 — Author ADR-15 Rust-First Polyglot Runtime and Language Authority (draft only)
```

Dependencies for T012: **T011**. Review: **Tier A** (draft only; accept at **T030**).

## Hard gates

- R1 is planning and ADR drafting/review — **not** production implementation
- Draft ADR ≠ accepted architecture authority
- **T030** is the architecture-acceptance gate (including ADR-15)
- No Rust/Tauri implementation before required gates (ADR-15 acceptance before R2)
- Do not reactivate `_archived/**` without ADR acceptance
- No OpenMed/Graphify fork/import; no Spec 001 migration mutation
- No Fehrest/DeepMed init under T011/T012

## Not authorized yet

- Staging/committing T011 without founder acceptance
- Starting T012 / any R1 task
- `/speckit-implement`
- Push / PR
- Manifest/CI/path repairs
- Rust/Tauri/Go production work
