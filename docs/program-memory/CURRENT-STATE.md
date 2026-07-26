# CURRENT-STATE

Verified local facts only. Unresolved items are marked explicitly.

## Fanatir

- Branch: `feat/r2-t031-tauri-shell` (local only; no upstream; nothing pushed; no PR). Canonical governance worktree `docs/f0-planning-memory-bootstrap` remains at `b7a01c9f5bb05b3b639880a836fa01ba7f44bb00` until this closeout package is committed on the feature branch.
- Spec Kit: `specs/001-fanatir-repository-and-architecture-reconstitution`
- Governance: Constitution v1.0.0; accepted Spec 001; Plan 001; Tasks 001
- **T001–T029**: R1 drafting/freeze/charter work products authored and locally committed through T029 (`2cc5e8d312210f30a751006dda6143d6d126ff43`). Historical `tasks.md` checklist boxes for T012–T030 remain unchecked (administrative bookkeeping only; not a reversal of T030/R1 closeout).
- **T030**: Complete.
- **T031**: Complete.

### T030 / R1 architecture-gate closeout (canonical)

```text
T030 complete — ADR-15/01/02/06 Accepted; R2 may begin
```

Qualification: “R2 may begin” meant R2 was **eligible for a separately authorized founder entry decision**. It did **not** auto-start R2; T031 required separate founder execution authorization, isolated implementation, Tier A review, integrity verification, and a normalized local implementation commit.

| Field | Value |
| --- | --- |
| **R1 architecture gate** | Passed and canonically closed |
| **T030 status** | Complete |
| **T030 pre-closeout commit (Commit 1)** | `2b257ac08dc8c0d566ae8a4be78f8af9711ef9f1` |
| **T030/R1 final-closeout commit (Commit 2)** | `781ca995383a061d0d9b8fb2e1f0aa4d6a623c52` |
| **Commit 2 subject** | `docs(governance): close T030 R1 architecture gate` |
| **Final-closeout verification** | `APPROVE — T030/R1 FINAL CLOSEOUT VERIFICATION PASSED` |
| **Founder work-product acceptance** | Yes — R1 architecture gate |
| **Tier A review** | Passed with non-blocking notes |
| **Post-sign-off integrity review** | Passed with non-blocking notes |

Commit 2 (`781ca995383a061d0d9b8fb2e1f0aa4d6a623c52`) closed T030/R1. Pre-execution program-memory reconciliation (`b7a01c9f5bb05b3b639880a836fa01ba7f44bb00`) is the parent of the T031 implementation commit. Unchecked historical T012–T030 markers do not reverse T030 completion or R1 closeout.

### Founder R1 architecture-gate decisions (canonical)

Founder acceptance:

```text
Yes — R1 architecture gate
```

Mandatory Accepted (architecture governance only):

```text
ADR-15 — Accepted
ADR-01 — Accepted
ADR-02 — Accepted
ADR-06 — Accepted
```

Reviewed (R1 review boundary passed; **not** Accepted for later-gated implementation domains):

```text
ADR-04 — Reviewed
ADR-05 — Reviewed
ADR-07 — Reviewed
ADR-08 — Reviewed
ADR-09 — Reviewed
ADR-10 — Reviewed
ADR-14 — Reviewed
```

Remain Proposed:

```text
ADR-03 — Proposed
ADR-11 — Proposed
ADR-12 — Proposed
ADR-13 — Proposed
```

ADR-06 clarification: Accepted because T030 / R2-entry contracts require it; T031 was shell-only and did not require ADR-06 for functional IPC/worker/capability work. Reviewed ≠ Accepted. ADR-05, ADR-07, and ADR-14 still require later Accepted status where downstream tasks require it. T031 completion does **not** ratify any additional ADR.

### Review evidence (historical; completed)

| Field | Value |
| --- | --- |
| **Review tier** | Tier A — R1 architecture gate |
| **Review status** | Passed with non-blocking notes |
| **Decision** | `APPROVE WITH NON-BLOCKING NOTES — TIER A R1 ARCHITECTURE GATE REVIEW PASSED` |
| **Reviewer** | Independent Tier A reviewer |
| **Review date** | 2026-07-26 |
| **Blocking findings** | None |
| **Non-blocking notes** | N-T030-1, N-T030-2 |
| **Observations** | O-T030-1, O-T030-2, O-T030-3, O-T030-4 |

```text
Blocking findings: None
Non-blocking notes: N-T030-1, N-T030-2
Observations: O-T030-1, O-T030-2, O-T030-3, O-T030-4
```

Post-sign-off integrity (completed before T030 pre-closeout commit):

```text
APPROVE WITH NON-BLOCKING NOTES — T030 POST-SIGN-OFF INTEGRITY REVIEW PASSED
```

```text
Blocking findings: None
Non-blocking notes: N-INT-1
Observations: O-INT-1, O-INT-2
```

Final-closeout verification:

```text
APPROVE — T030/R1 FINAL CLOSEOUT VERIFICATION PASSED
```

### T031 / R2 minimal desktop-shell closeout (canonical)

```text
T031 status: Complete
T031 implementation: Locally committed and independently verified
T031 administrative closeout: Complete when this closeout package is committed
R2 status: Entered through T031 only; no later R2 task started
T032 status: Not authorized
```

| Field | Value |
| --- | --- |
| **Subject** | Create minimal Tauri 2 desktop shell loading afia-ui client |
| **Stage** | R2 |
| **Dependencies** | T030 |
| **Implementation commit** | `eb6ca9cbcf081d1d2cbf0a8ac30e23104da83103` |
| **Implementation parent** | `b7a01c9f5bb05b3b639880a836fa01ba7f44bb00` |
| **Implementation tree** | `3d719a4d7a7784fb9043254c405d05dd0112c995` |
| **Implementation subject** | `feat(desktop): scaffold minimal Tauri shell` |
| **Committed path scope** | Exactly 15 paths under `apps/desktop/**` plus scoped `pnpm-lock.yaml` |
| **`afia-ui/**`** | Consumed read-only; no UI source copied into the desktop package |
| **Root `Cargo.toml`** | Remained canonical and unchanged; excluded from the implementation commit (blob `4b469dc4169cd2d2fc139943c4b252e5250960ca`) |
| **Workspace strategy** | Nested workspace under `apps/desktop/src-tauri`; no archived crate reactivated |
| **Checklist** | T031 checked in `tasks.md`; T032 remains unchecked; T030 marker unchanged |
| **Founder work-product acceptance** | No |
| **Upstream / remote / push / PR** | None |

T031 satisfies only the minimal desktop-shell task. The shell is **not** the full Trusted Host. No application IPC, workers, persistence, capability gateway, plugins, authentication, Supabase runtime, or PHI handling were added by T031.

#### T031 verification decisions

```text
APPROVE WITH NON-BLOCKING NOTES — TIER A TRUSTED HOST SHELL RE-REVIEW PASSED
```

```text
APPROVE WITH NON-BLOCKING NOTES — T031 POST-SIGN-OFF INTEGRITY VERIFICATION PASSED
```

```text
APPROVE WITH NON-BLOCKING NOTES — T031 POST-COMMIT VERIFICATION PASSED
```

Post-commit verification recorded: commit-object re-hash matched; exactly one parent; exactly 15 authorized paths; every committed blob matched Tier A and integrity-reviewed identities; `git diff-tree --check` passed; `git show --check` passed; implementation worktree clean; canonical governance worktree clean; no upstream, remote feature branch, push, or PR.

#### Capability / security posture

```text
WebView trust posture: Untrusted
Granted WebView-callable host commands: 0
```

Committed capability (`apps/desktop/src-tauri/capabilities/default.json`):

```json
{
  "$schema": "https://schema.tauri.app/config/2/capability",
  "identifier": "default",
  "description": "T031 minimal shell — window/runtime core only; no FS/shell/HTTP/dialog plugins.",
  "windows": ["main"],
  "permissions": []
}
```

```text
B-T031-CAP-1: Resolved
```

No `core:default`; no custom Rust commands; no `invoke_handler`; no `generate_handler!`; no plugins; no filesystem, process/shell, Tauri HTTP, event, menu, tray, path, image-from-path, window-management, or WebView-management command grants; no auth, database, Supabase, persistence, PHI, worker, or product-domain host authority.

Absence of Tauri host commands does **not** remove pre-existing `afia-ui` web-client network behavior. Pre-existing `afia-ui` Supabase or web API behavior remains outside the T031 implementation delta.

#### Workspace finding dispositions

```text
B-T031-WS-1 — Resolved
B-T031-WS-2 — Resolved
B-T031-WS-3 — Historical process observation only
```

The unauthorized root `Cargo.toml` edit was reverted before Tier A. Root `Cargo.toml` remained outside the implementation commit. The canonical phantom-member contradiction remains unresolved; durable root-workspace reconciliation requires a separate authorized task. T031 uses only its nested local workspace.

#### Acceptance evidence (shell-only)

- Windows-first launch smoke passed
- One `Fanatir` desktop window loaded the existing UI
- No panic or fatal Vite error
- Process cleanup passed

#### Accepted non-blocking notes (do not remediate here)

```text
N-T031-CSP-1 — CSP includes unsafe-inline / unsafe-eval.
N-T031-CLI-1 — @tauri-apps/cli 2.11.4 versus Rust Tauri 2.11.5.
N-T031-ICON-1 — Placeholder icons and duplicated 32×32 icon.png.
N-T031-VITE-1 — Root lock references Vite 7.3.6 while package-local runtime uses Vite 7.1.9.
```

#### Observations (do not elevate)

```text
O-T031-PORT-1 — strictPort:false.
O-T031-HOST-1 — Vite host:true.
O-T031-WS-HIST-1 — Historical stop-boundary deviation.
```

These notes and observations are **not** production approval and are **not** T032 scope.

#### Rollback

Rollback remains removal or reversion of the T031 implementation commit/package before publication.

### Plan R1 blocking-matrix checklist (canonical)

| Gate item | Recorded state |
| --- | --- |
| ADR-15 Accepted before any R2 implementation | **Accepted** |
| ADR-01 Accepted | **Accepted** |
| ADR-02 Accepted | **Accepted** |
| ADR-06 Accepted | **Accepted** |
| ADR-04 / 05 / 07 / 08 / 09 / 10 / 14 Reviewed or Accepted | **Reviewed** |
| ADR-03 / 11 / 12 / 13 | **Proposed** |
| R2 implementation commits | T031 shell only (`eb6ca9cbcf081d1d2cbf0a8ac30e23104da83103`); no later R2 task started |
| Future Spec 002 | Charter only (T029); Spec 002 not created by T031 |
| `documents-crypto` | Frozen (T028); no real patient / production PHI |

### Authority distinctions (current)

- T031 Complete ≠ full Trusted Host ≠ T032 authorization ≠ later R2 start
- Architecture-governance Accepted ≠ schema / codegen / migration / Supabase runtime / PHI / Spec 002 / packaging / release / production authorization
- Reviewed ≠ Accepted for Artifact Store (ADR-05), Supabase adapter/migrations (ADR-07), classified-data/PHI-egress implementation (ADR-14), or Fehrest/DeepMed/commandF product stages (ADR-08/09/10)
- Decision C remains ratified; T028 `documents-crypto` freeze remains in force; T029 Spec 002 charter remains charter-only (`002-supabase-local-first-and-migration-canonicalization`; no `specs/002*` implementation introduced by T031)
- No authentication, migration, Supabase runtime, PHI, Spec 002, Spec Kit, Fehrest, DeepMed, or commandF implementation was added by T031

## Fehrest / DeepMed / commandF

- Unchanged; no implementation authorization from T031
- Fehrest remains a separate product and repository; no migration, synchronization, schema work, identity merge, or implementation
- DeepMed remains separately gated; no model, provider, prompt, runtime, or PHI invocation
- commandF: ADR-10 remains **Reviewed**; no FHIR validation, transformation, terminology, server access, live write, or PHI egress

## Remaining

- Independently verify this three-file T031 administrative closeout authoring
- Separately authorized normalized local closeout commit
- Independent verification of that closeout commit
- Only after a passing closeout verification, recover the canonical T032 contract in strict read-only mode
- Obtain a separate founder T032 execution decision
- Do not begin T032 merely because T031 is complete
- Separate later Accepted required for ADR-05 / ADR-07 / ADR-14 (and product ADRs) before their gated implementation domains
- Spec 002 remains separately gated
- Durable root-workspace reconciliation remains a separate authorized task
