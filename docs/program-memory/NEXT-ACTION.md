# NEXT-ACTION

## Immediate next action

```text
Recover and verify the canonical T031 contract in strict read-only mode.
```

## T030 / R1 status (canonical after Commit 2)

```text
T030 complete — ADR-15/01/02/06 Accepted; R2 may begin
```

R1 architecture gate: **Passed and canonically closed** (authoring pending independent verification + Commit 2).

R2: **Eligible for separately authorized entry; not started.**

T031: **Not authorized.**

Pre-closeout package commit (Commit 1): `2b257ac08dc8c0d566ae8a4be78f8af9711ef9f1`.

Tier A decision (historical):

```text
APPROVE WITH NON-BLOCKING NOTES — TIER A R1 ARCHITECTURE GATE REVIEW PASSED
```

## Permitted next step

T031 **contract recovery only** — read-only recovery and verification of the canonical T031 task contract from repository evidence.

A separate founder execution authorization is required after T031 contract recovery before any T031 authoring or implementation.

## Explicitly prohibited now

- T031 authoring or implementation
- Starting R2 implementation
- Modifying source
- Creating schemas or generated bindings
- IPC or worker implementation
- Persistence implementation
- UI or authentication implementation
- Spec 002 creation / Spec Kit
- Migration editing or execution
- Supabase runtime or schema changes
- Real patient data or PHI handling
- Fehrest, DeepMed, or commandF implementation
- Staging, committing, pushing, or creating a PR as part of T031 recovery

## Hard gates

- R1 closeout ≠ T031 authorization ≠ R2 start
- Accepted ADR-15/01/02/06 ≠ R2 implementation start
- Reviewed ADR-04/05/07/08/09/10/14 ≠ Accepted for later implementation domains
- ADR-03/11/12/13 remain Proposed
- Preserve Decision C; T028 `documents-crypto` freeze; no real patient / production PHI
- Spec 002 remains charter-only; no `/speckit.specify`; no `specs/002*`
- Do not modify `tasks.md` under this closeout authoring
- Do not implement N-T030-1 / N-T030-2 / ADR body or EOL cleanup
