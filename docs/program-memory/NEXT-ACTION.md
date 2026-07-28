# NEXT-ACTION

## Immediate next action

```text
Independently verify the T031 pre-execution program-memory consistency correction.
```

## T030 / R1 status (canonical at Commit 2)

```text
T030 complete — ADR-15/01/02/06 Accepted; R2 may begin
```

R1 architecture gate: **Passed and canonically closed** at Commit 2 `781ca995383a061d0d9b8fb2e1f0aa4d6a623c52`.

R2: **Eligible for separately authorized entry; not started.**

T031 contract recovery: **Complete.**

T031 execution: **Not authorized.**

Program-memory consistency correction: **Authored; pending independent verification and a separately authorized local correction commit.**

Pre-closeout package commit (Commit 1): `2b257ac08dc8c0d566ae8a4be78f8af9711ef9f1`.

Final-closeout commit (Commit 2): `781ca995383a061d0d9b8fb2e1f0aa4d6a623c52` — `docs(governance): close T030 R1 architecture gate`.

Final-closeout verification (historical):

```text
APPROVE — T030/R1 FINAL CLOSEOUT VERIFICATION PASSED
```

Tier A decision (historical):

```text
APPROVE WITH NON-BLOCKING NOTES — TIER A R1 ARCHITECTURE GATE REVIEW PASSED
```

Integrity decision (historical):

```text
APPROVE WITH NON-BLOCKING NOTES — T030 POST-SIGN-OFF INTEGRITY REVIEW PASSED
```

T031 recovery decision (historical):

```text
PASS — T031 CONTRACT RECOVERED AND READY FOR FOUNDER EXECUTION DECISION
```

## Remaining sequence

1. Independently verify this two-file program-memory consistency correction.
2. Create a separately authorized normalized local correction commit.
3. Only after that commit, obtain the founder’s explicit T031 execution decision.
4. If authorized later, create the separately approved branch or worktree and execute only the narrow shell contract.

## Founder choices still required after the correction commit

```text
Branch/worktree arrangement
Exact smoke commands
Commit subject
Checklist-marking policy
Explicit T031 execution authorization
```

Do not make the founder execution decision in program memory.

## Recovered T031 contract (reference only; not execution authority)

- Subject: Create minimal Tauri 2 desktop shell loading afia-ui client
- Stage: R2; Dependencies: T030
- Gate: ADR-15 + ADR-01 + ADR-02 Accepted; R1 exit (T030)
- ADR-06: Accepted for governance; not functionally required for shell-only; IPC/workers/capabilities deferred
- Paths: `apps/desktop/**`; `afia-ui/client/** (consume only)`
- Acceptance: Desktop shell launches UI in Windows-first dev
- Review: Tier A — Trusted Host shell; Founder work-product acceptance: No

## Explicitly prohibited now

- T031 execution or scaffold creation
- Starting R2 implementation
- Creating a branch, clone, or worktree for T031
- Modifying source or root manifests
- Creating schemas or generated bindings
- IPC or worker implementation
- Persistence / Artifact Store implementation
- UI or authentication implementation
- Spec 002 creation / Spec Kit
- Migration editing or execution
- Supabase runtime or schema changes
- Real patient data or PHI handling
- Fehrest, DeepMed, or commandF implementation
- Staging, committing, pushing, or creating a PR as part of this correction authoring (except under separate correction-commit authorization)

## Hard gates

- R1 closeout ≠ T031 authorization ≠ R2 start
- Accepted ADR-15/01/02/06 ≠ R2 implementation start
- Reviewed ADR-04/05/07/08/09/10/14 ≠ Accepted for later implementation domains
- ADR-03/11/12/13 remain Proposed
- Preserve Decision C; T028 `documents-crypto` freeze; no real patient / production PHI
- Spec 002 remains charter-only; no `/speckit.specify`; no `specs/002*`
- Do not modify `tasks.md` under this correction authoring
- Do not implement N-T030-1 / N-T030-2 / ADR body or EOL cleanup
