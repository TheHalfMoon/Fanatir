# CURRENT-STATE

Verified local facts only. Unresolved items are marked explicitly.

## Fanatir

- Branch: `feat/r2-t032-trusted-host-skeleton` (local only; no upstream; nothing pushed; no PR). Canonical governance worktree `docs/f0-planning-memory-bootstrap` remains at `b7a01c9f5bb05b3b639880a836fa01ba7f44bb00`.
- Spec Kit: `specs/001-fanatir-repository-and-architecture-reconstitution`
- Governance: Constitution v1.0.0; accepted Spec 001; Plan 001; Tasks 001
- **T001–T029**: R1 drafting/freeze/charter work products authored and locally committed through T029 (`2cc5e8d312210f30a751006dda6143d6d126ff43`). Historical `tasks.md` checklist boxes for T012–T030 remain unchecked (administrative bookkeeping only; not a reversal of T030/R1 closeout).
- **T030**: Complete.
- **T031**: Complete.
- **T032**: Complete.

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

ADR-06 clarification: Accepted because T030 / R2-entry contracts require it; T031 was shell-only and did not require ADR-06 for functional IPC/worker/capability work. T032 implements no IPC contract or command surface; functional IPC remains deferred. Reviewed ≠ Accepted. ADR-05, ADR-07, and ADR-14 still require later Accepted status where downstream tasks require it. T032 completion does **not** ratify any additional ADR.

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
T031 administrative closeout: Complete
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
| **Closeout commit** | `5e03acc905f96c916b90ee028b1be56f722cee6f` |
| **Closeout subject** | `docs(governance): close T031 desktop shell` |
| **Committed path scope** | Exactly 15 paths under `apps/desktop/**` plus scoped `pnpm-lock.yaml` |
| **`afia-ui/**`** | Consumed read-only; no UI source copied into the desktop package |
| **Root `Cargo.toml`** | Remained canonical and unchanged; excluded from the implementation commit (blob `4b469dc4169cd2d2fc139943c4b252e5250960ca`) |
| **Workspace strategy** | Nested workspace under `apps/desktop/src-tauri`; no archived crate reactivated |
| **Checklist** | T031 checked in `tasks.md`; T030 marker unchanged |
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

Post-commit verification recorded: commit-object re-hash matched; exactly one parent; exactly 15 authorized paths; every committed blob matched Tier A and integrity-reviewed identities; `git diff --check` / `git diff-tree --check` / `git show --check` passed; implementation worktree clean; canonical governance worktree clean; no upstream, remote feature branch, push, or PR.

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

The unauthorized root `Cargo.toml` edit was reverted before Tier A. Root `Cargo.toml` remained outside the implementation commit. The canonical phantom-member contradiction remains unresolved; durable root-workspace reconciliation requires a separate authorized task. T031 uses only its nested local workspace. T032 did not resolve the root workspace contradiction.

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

These notes and observations are **not** production approval.

#### Rollback

Rollback remains removal or reversion of the T031 implementation commit/package before publication.

### T032 / R2 Trusted Host authority-skeleton closeout (canonical)

```text
T032 status: Complete
T032 implementation: Locally committed and independently verified
T032 administrative closeout: Complete
R2 status: Entered through T032 only; no later R2 task started
T033 status: Not authorized
```

| Field | Value |
| --- | --- |
| **Subject** | Implement Rust Trusted Host skeleton (project/workspace authority stubs) |
| **Stage** | R2 |
| **Dependencies / gates** | ADR-15, ADR-01, ADR-02 Accepted |
| **Implementation commit** | `d0be7345e68cc424baaa1a80b82e45f73a71ade8` |
| **Implementation parent** | `5e03acc905f96c916b90ee028b1be56f722cee6f` |
| **Implementation tree** | `f551569db451a0ae8dd7b9813791689ab9f5d303` |
| **Implementation subject** | `feat(desktop): add Trusted Host authority skeleton` |
| **Committed path scope** | Exactly five paths under `apps/desktop/src-tauri/src/**` |
| **Acceptance** | Rust module is sole privileged entry |
| **Validation** | Compile + unit stubs |
| **Review** | Tier A — Trusted Host |
| **Security** | Capability deny-by-default |
| **Rollback** | Revert host modules |
| **Founder work-product acceptance** | No |
| **Checklist** | T032 checked in `tasks.md`; T031 remains checked; T030 remains unchecked; T033 remains unchecked |
| **Upstream / remote / push / PR** | None |

Exact implementation paths:

```text
apps/desktop/src-tauri/src/lib.rs
apps/desktop/src-tauri/src/trusted_host/mod.rs
apps/desktop/src-tauri/src/trusted_host/identity.rs
apps/desktop/src-tauri/src/trusted_host/project.rs
apps/desktop/src-tauri/src/trusted_host/workspace.rs
```

T032 implements only the type-only Trusted Host authority skeleton. It is **not** a functional Trusted Host. It exposes no WebView-callable host command, grants no permission, performs no filesystem or persistence operation, has no runtime host container or application state, and implements no IPC, worker/async orchestration, Supabase, authentication, PHI, Fehrest, DeepMed, or commandF behavior.

T033 requires separate strict read-only contract recovery and a separate founder execution decision after recovery. No T033 branch, worktree, code, validation, stage, or commit is authorized by T032 closeout.

#### T032 verification decisions

```text
APPROVE WITH NON-BLOCKING NOTES — TIER A T032 TRUSTED HOST SKELETON REVIEW PASSED
```

```text
APPROVE WITH NON-BLOCKING NOTES — T032 POST-SIGN-OFF INTEGRITY VERIFICATION PASSED
```

```text
APPROVE WITH NON-BLOCKING NOTES — T032 POST-COMMIT VERIFICATION PASSED
```

Post-commit verification recorded: commit-object re-hash matched; exactly one parent; exactly five authorized paths; every committed blob matched Tier A and integrity-reviewed identities; `git diff-tree --check` passed; `git show --check` passed; T032 implementation worktree clean; T031 worktree clean; canonical governance worktree clean; no upstream, remote feature branch, push, or PR.

#### Public API

```text
Public module: fanatir_desktop::trusted_host
Public enum: AuthorityIdError (Empty, TooLong, SurroundingWhitespace, ControlCharacter)
Public structs: ProjectId, ProjectAuthority, WorkspaceId, WorkspaceAuthority
Public methods: ProjectId::{try_new, as_str}; ProjectAuthority::{new, id}; WorkspaceId::{try_new, as_str}; WorkspaceAuthority::{new, id}
Public fields: None
```

`trusted_host` is the sole package-level project/workspace authority namespace. Child modules remain private. Private validation helpers are not public API. `ProjectId` and `WorkspaceId` are nominally distinct. `ProjectAuthority` and `WorkspaceAuthority` each own one matching ID. No Project–Workspace relationship is encoded; that relationship remains intentionally undefined.

#### Identity validation

Deterministic order:

```text
1. Empty
2. TooLong
3. SurroundingWhitespace
4. ControlCharacter
```

Identifiers must be nonempty; must not exceed 128 UTF-8 bytes; leading or trailing Unicode whitespace is rejected; Unicode control characters are rejected; valid values are preserved exactly. No trimming, case conversion, Unicode normalization, path interpretation, random generation, or timestamp generation.

Exact display strings:

```text
Empty: authority identifier must not be empty
TooLong: authority identifier must not exceed 128 UTF-8 bytes
SurroundingWhitespace: authority identifier must not have leading or trailing whitespace
ControlCharacter: authority identifier must not contain control characters
```

These strings are local construction errors only; they are not an IPC or persistence protocol.

#### Type-only and runtime boundary

```text
Type-only boundary: Intact
Executable authority: None
Runtime host container: None
Application state: None
Mutable registry: None
Async or worker behavior: None
T032 direct dependency additions: 0
```

Names containing “Authority” are compile-time type boundaries only. They do not authorize operations, issue capabilities, grant filesystem access, open or create projects, switch workspaces, communicate with the WebView, persist data, call Supabase, or process PHI.

Standard-library-only: imports limited to `std` / `crate` / `super` / `self`. `apps/desktop/src-tauri/Cargo.toml` and `Cargo.lock` unchanged. `main.rs` unchanged. Capability file unchanged.

#### Capability / security posture

```text
WebView trust posture: Untrusted
Expanded permission count: 0
Granted WebView-callable host commands: 0
```

Preserved capability blob: `501b0ab6856db4ac0c9115e1c085d5f1bab0ec88`.

```json
{
  "$schema": "https://schema.tauri.app/config/2/capability",
  "identifier": "default",
  "description": "T031 minimal shell — window/runtime core only; no FS/shell/HTTP/dialog plugins.",
  "windows": ["main"],
  "permissions": []
}
```

No `core:default`; no custom Rust command; no `#[tauri::command]`; no `invoke_handler`; no `generate_handler!`; no plugin; no event transport; no application IPC; no managed state; no filesystem, process/shell, network-proxy, persistence, authentication, Supabase, or PHI host authority.

#### Root and nested workspace

```text
Root Cargo.toml: Unchanged and outside T032
Root Cargo blob: 4b469dc4169cd2d2fc139943c4b252e5250960ca
```

```text
B-T031-WS-1 — Resolved
B-T031-WS-2 — Resolved
B-T031-WS-3 — Historical process observation only
```

The canonical root phantom-member contradiction remains unresolved. T032 continues using the nested workspace under `apps/desktop/src-tauri`. No archived crate was reactivated. Root-workspace repair requires separate authorization. T032 does not claim to resolve the root workspace contradiction.

#### Validation evidence

- `cargo fmt --check`: PASS
- `cargo check`: PASS
- `cargo clippy -D warnings`: PASS
- `cargo test`: 19 passed, 0 failed, 0 ignored
- Desktop smoke: not required and not executed (no runtime integration change)

#### Accepted non-blocking notes (do not remediate here)

```text
N-T032-1 — No multibyte UTF-8 byte-boundary unit test.
N-T032-2 — No explicit 128-byte acceptance test.
N-T032-3 — Tests use expect(...) on known-valid constructors.
N-T032-4 — Unicode whitespace rejection relies on char::is_whitespace but tests cover only ASCII space.
```

#### Observations (do not elevate)

```text
O-T032-1 — Leading tab and newline classify as SurroundingWhitespace before ControlCharacter.
O-T032-2 — Library and binary dual targets arise automatically from adding lib.rs and remain manifest-free.
```

These notes and observations are **not** production approval and are **not** T033 scope.

#### Rollback

Rollback remains removal or reversion of the five T032 host modules before publication.

### Plan R1 blocking-matrix checklist (canonical)

| Gate item | Recorded state |
| --- | --- |
| ADR-15 Accepted before any R2 implementation | **Accepted** |
| ADR-01 Accepted | **Accepted** |
| ADR-02 Accepted | **Accepted** |
| ADR-06 Accepted | **Accepted** |
| ADR-04 / 05 / 07 / 08 / 09 / 10 / 14 Reviewed or Accepted | **Reviewed** |
| ADR-03 / 11 / 12 / 13 | **Proposed** |
| R2 implementation commits | T031 shell (`eb6ca9cbcf081d1d2cbf0a8ac30e23104da83103`); T032 authority skeleton (`d0be7345e68cc424baaa1a80b82e45f73a71ade8`); no later R2 task started |
| Future Spec 002 | Charter only (T029); Spec 002 not created by T031 or T032 |
| `documents-crypto` | Frozen (T028); no real patient / production PHI |

### Authority distinctions (current)

- T032 Complete ≠ functional Trusted Host ≠ T033 authorization ≠ later R2 start
- Architecture-governance Accepted ≠ schema / codegen / migration / Supabase runtime / PHI / Spec 002 / packaging / release / production authorization
- Reviewed ≠ Accepted for Artifact Store (ADR-05), Supabase adapter/migrations (ADR-07), classified-data/PHI-egress implementation (ADR-14), or Fehrest/DeepMed/commandF product stages (ADR-08/09/10)
- Decision C remains ratified; T028 `documents-crypto` freeze remains in force; T029 Spec 002 charter remains charter-only (`002-supabase-local-first-and-migration-canonicalization`; no `specs/002*` implementation introduced by T032)
- No authentication, migration, Supabase runtime, PHI, Spec 002, Spec Kit, Fehrest, DeepMed, or commandF implementation was added by T032
- `WorkspaceId` is not a Supabase identifier; `WorkspaceAuthority` contains no cloud membership state
- Explicit non-claims: no HIPAA certification; no Saudi PDPL / GDPR / Australian Privacy Act compliance assertion; no production readiness; no production-safe PHI handling; no implemented immutable audit or encryption guarantees; no legal safe harbor

## Fehrest / DeepMed / commandF

- Unchanged; no implementation authorization from T032
- Fehrest remains a separate product and repository; no migration, synchronization, schema work, identity merge, or implementation
- DeepMed remains separately gated; no model, provider, prompt, runtime, or PHI invocation
- commandF: ADR-10 remains **Reviewed**; no FHIR validation, transformation, terminology, server access, live write, or PHI egress

## Remaining

- Independently verify this three-file T032 administrative closeout authoring
- Separately authorized normalized local closeout commit
- Independent verification of that closeout commit
- Only after a passing closeout verification, authorize strict read-only recovery of the canonical T033 contract
- Obtain a separate founder T033 execution decision after recovery
- Do not begin T033 merely because T032 is complete
- Separate later Accepted required for ADR-05 / ADR-07 / ADR-14 (and product ADRs) before their gated implementation domains
- Spec 002 remains separately gated
- Durable root-workspace reconciliation remains a separate authorized task
