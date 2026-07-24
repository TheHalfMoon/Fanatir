# R0 Baseline Evidence Package and Closeout (T011)

**Task**: T011 — Produce R0 baseline evidence package and program-memory closeout
**Stage**: R0
**Review**: Tier C — normal verification
**T011 status**: Execution complete; **awaiting founder acceptance**; **uncommitted**
**Canonical T010 baseline SHA**: `2c9fb55244588bb35b2d08bc1e1c6bbcbadbd8ec`
**Superseded T010 SHA (do not use as boundary)**: `a24ae974770da8f4a1ddccbbd5c2d67217261b1d`

Contract source: `specs/001-fanatir-repository-and-architecture-reconstitution/tasks.md` (T011); Plan R0 exit (`plan.md` § R0 — Verified baseline); Spec 001 inventory/disposition; Constitution v1.0.0.

---

## 4.1 R0 identity

| Field | Value |
| --- | --- |
| Program | Fanatir F0 planning-memory bootstrap |
| Phase | R0 — repository-reality baseline |
| Repository | `C:\Projects\Fanatir-Ecosystem\Fanatir` |
| Branch | `docs/f0-planning-memory-bootstrap` (local only) |
| Upstream | none |
| Push / PR | none |
| R0 prerequisites (accepted before T001) | Constitution `1ee7c42ea0e06f182318522c232f598680268d2a`; Spec `46f55c4e9a6b69aecbd85007e98688141939f869`; Plan `ac5c777e91b74fc30903a364f03337a5ac8a63f6`; Tasks `797a35ac78b2599b7ef231dbd0a49b3d586ce245` |
| R0 execution start (T001 commit) | `ea2c3f66c62758e058fbe89a2781473054e2fe8e` |
| R0 closing baseline (T010; pre-T011) | `2c9fb55244588bb35b2d08bc1e1c6bbcbadbd8ec` |
| Subject at close baseline | `docs(r0): record repository path classification` |
| Parent of close baseline | `fc3069202e5f13ad80fe768a9f9b1046ef234a25` |

---

## 4.2 Purpose

### R0 establishes

- What exists in the Fanatir repository tree
- What is connected (wiring / manifests / routes)
- What is merely present
- What is contradictory
- What is archived
- What requires investigation
- Which documents currently hold **accepted** governance authority

### R0 does not

- Select or accept the final architecture
- Accept ADR-15 (or any ADR draft)
- Authorize Rust or Tauri implementation
- Repair the repository (manifests, CI, migrations, auth, code)
- Validate runtime behavior that was not executed
- Convert implementation facts into architecture authority
- Substitute for **T030** (architecture-acceptance gate)

```text
R0 closure does not constitute architecture acceptance or implementation authorization.
```

---

## 4.3 Task evidence ledger (T001–T010)

Each task appears exactly once. Paths verified present at T010 baseline. SHAs verified via `git cat-file` / `git diff-tree`.

| Task | Title | Status | Commit SHA | Parent SHA | Evidence path(s) | Evidence type | Key finding | Open issue | Authority note |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| T001 | Record exact repository baseline (HEAD, branch, remotes, sibling repos) | Completed; founder-accepted; committed | `ea2c3f66c62758e058fbe89a2781473054e2fe8e` | `797a35ac78b2599b7ef231dbd0a49b3d586ce245` | `docs/program-memory/baseline/R0-repo-baseline.md` (+ program-memory pointers) | Fact record / PLANNING EVIDENCE | Baseline SHAs; stale root manifests; Fehrest empty; DeepMed README-only | Manifest truth deferred | Factual inspection only |
| T002 | Verify actual build/dev commands for afia-ui and record evidence | Completed; founder-accepted; committed | `5f968c164b14c4afb0e14836b8f1b42ebb7675f2` | `ea2c3f66c62758e058fbe89a2781473054e2fe8e` | `docs/program-memory/baseline/R0-build-commands.md` | Fact record | Active app `afia-ui/`; pnpm skew; no `node_modules`; build/dev failed without install | Install later | Factual inspection only |
| T003 | Verify dependency managers (pnpm/node/rust/python/uv) and versions | Completed; founder-accepted; committed | `cc2345cb55318998cdd15bee4f20c5bdeae8845e` | `5f968c164b14c4afb0e14836b8f1b42ebb7675f2` | `docs/program-memory/baseline/R0-tooling.md` | Fact record | Tooling matrix; root vs afia-ui packageManager; macOS-skew toolchain notes | Tooling repair deferred | Factual inspection only |
| T004 | Map active frontend entry points (Vite client App/routes/Shell) | Completed; founder-accepted; committed | `00fc4bf0f365d451c4bbbd94081468023f69f57b` | `cc2345cb55318998cdd15bee4f20c5bdeae8845e` | `docs/program-memory/baseline/R0-frontend-map.md` | Fact record | Vite→App→routes→AppShell; UI→Supabase/OpenMed/`@kernel` seams | Runtime unverified | Implementation wiring facts |
| T005 | Verify auth/session/PrivateRoute/profile behavior (observe-only) | Completed; founder-accepted; committed | `e53c059027f51b022f4ea480ebfa82deecacfb83` | `00fc4bf0f365d451c4bbbd94081468023f69f57b` | `docs/program-memory/baseline/R0-auth-session.md` | Fact record | Session-only PrivateRoute; fail-open profile sync ≠ auth bypass | Auth behavior freeze until dedicated spec | Implementation facts; plan retain for behavior |
| T006 | Inspect Supabase clients and both migration trees (freeze inventory) | Completed; founder-accepted; committed | `21b2ceed551301e7889d3e793f56a1ee5c626675` | `e53c059027f51b022f4ea480ebfa82deecacfb83` | `docs/program-memory/baseline/R0-supabase-inventory.md` | Fact record | Dual `workspaces.sql` identical; schema vs migrations contradictory; `documents-crypto` frozen PHI-egress | Spec 002 | Factual; no migration mutation |
| T007 | Inspect Python/FHIR/OpenMed prototype services | Completed; founder-accepted; committed | `1115f181b689e136839493c480469fa4258f6768` | `21b2ceed551301e7889d3e793f56a1ee5c626675` | `docs/program-memory/baseline/R0-python-services.md` | Fact record | Bridge prototypes; `127.0.0.1:8765`; FHIR R4B ≠ full validator; extract-pii mismatch | DeepMed/OpenMed later | Factual; not production DeepMed |
| T008 | Inspect archived Rust/Tauri/Go assets without reactivation | Completed; founder-accepted; committed | `bd82b6830164e92455e738127f46923bfba66136` | `1115f181b689e136839493c480469fa4258f6768` | `docs/program-memory/baseline/R0-archived-assets.md` | Fact record | Archives non-authoritative; crates investigate; desktop replace-successor; Go archive | No reactivate | Archive ≠ active |
| T009 | Inspect current CI and tests honesty | Completed; founder-accepted; committed | `fc3069202e5f13ad80fe768a9f9b1046ef234a25` | `bd82b6830164e92455e738127f46923bfba66136` | `docs/program-memory/baseline/R0-ci-tests.md` | Fact record | Only echo CI on macOS; no substantive gates; no executable suites | Real CI later (e.g. T059+) | Placeholder ≠ quality gate |
| T010 | Classify paths active/scaffold/archived/orphaned/contradictory | Completed; founder-accepted; committed; hygiene-verified | `2c9fb55244588bb35b2d08bc1e1c6bbcbadbd8ec` | `fc3069202e5f13ad80fe768a9f9b1046ef234a25` | `docs/program-memory/baseline/R0-path-classification.md` | Fact record / classification register | 81 path-family rows; exact class totals; C1–C15 contradictions | Investigation items remain | Classification fact record; not architecture SoT |

```text
Expected tasks: 10
Ledger entries: 10
Missing tasks: 0
Duplicate tasks: 0
```

---

## 4.4 Baseline artifact index

| Title | Path | Task | Factual scope | Authority scope | Limitations |
| --- | --- | --- | --- | --- | --- |
| Repository baseline | [R0-repo-baseline.md](./R0-repo-baseline.md) | T001 | Remotes, siblings, manifests vs disk | Fact record only | Point-in-time |
| Build/dev commands | [R0-build-commands.md](./R0-build-commands.md) | T002 | afia-ui scripts/commands | Fact record only | Deps absent |
| Tooling versions | [R0-tooling.md](./R0-tooling.md) | T003 | node/pnpm/rust/python matrix | Fact record only | Host-dependent |
| Frontend map | [R0-frontend-map.md](./R0-frontend-map.md) | T004 | Bootstrap/routes/shell/seams | Wiring facts | Runtime unverified |
| Auth/session | [R0-auth-session.md](./R0-auth-session.md) | T005 | PrivateRoute/AuthContext | Wiring + behavior observe | No live auth proof |
| Supabase inventory | [R0-supabase-inventory.md](./R0-supabase-inventory.md) | T006 | Clients, migrations, edges | Fact record; freeze | Spec 002 open |
| Python/FHIR/OpenMed | [R0-python-services.md](./R0-python-services.md) | T007 | Bridges, FHIR prototype | Fact record | Not production DeepMed |
| Archived assets | [R0-archived-assets.md](./R0-archived-assets.md) | T008 | Rust/Tauri/Go archives | Non-authority evidence | Do not reactivate |
| CI/tests honesty | [R0-ci-tests.md](./R0-ci-tests.md) | T009 | Workflows/tests/scripts honesty | Fact record | Hosted Actions not inspected |
| Path classification | [R0-path-classification.md](./R0-path-classification.md) | T010 | 81-row register; C1–C15 | Fact record / classification | Fine-grained lib incomplete |
| **This package** | [R0-evidence-package.md](./R0-evidence-package.md) | T011 | R0 closeout index | Fact/closeout record; **not** architecture SoT | Uncommitted until founder accept |

Indexed R0 baseline artifacts (T001–T010): **10**. This closeout package: **1** (T011).

---

## 4.5 Repository-path classification closure

**Classification source (canonical):** [R0-path-classification.md](./R0-path-classification.md)
**Totals changed during T011:** NO

| Primary classification | Exact total |
| --- | ---: |
| VERIFIED ACTIVE | 11 |
| CONDITIONALLY ACTIVE | 2 |
| REFERENCED BUT RUNTIME-UNVERIFIED | 15 |
| SCAFFOLD ONLY | 6 |
| ARCHIVED | 6 |
| ORPHANED | 3 |
| CONTRADICTORY | 13 |
| DERIVED / REBUILDABLE | 1 |
| GOVERNANCE AUTHORITY | 4 |
| PLANNING EVIDENCE | 10 |
| EXTERNAL SIBLING PRODUCT | 2 |
| GENERATED / IGNORED | 4 |
| REQUIRES INVESTIGATION | 4 |
| **Total** | **81** |

Reconciliation: `11+2+15+6+6+3+13+1+4+10+2+4+4 = 81` — **PASS**.

---

## 4.6 Unresolved baseline register

T011 does **not** resolve any item below.

### U-INV — REQUIRES INVESTIGATION (from T010)

| ID | Item | Classification | Evidence | Why unresolved | Earliest resolution | Non-authorization |
| --- | --- | --- | --- | --- | --- | --- |
| U-INV-1 | Root `README.md` claim ledger | REQUIRES INVESTIGATION | T010 | End-to-end claim review not done in R0 | Later docs/claim tasks | Do not treat README as SoT |
| U-INV-2 | Root `tsconfig.json` scope | REQUIRES INVESTIGATION | T010; T009 | Coverage vs afia-ui unproven | Manifest/tooling adapt tasks | Do not claim typecheck green |
| U-INV-3 | `AfiaUI.zip` | REQUIRES INVESTIGATION | T010 | Purpose/safety not established | Inventory follow-up | Do not extract as authority |
| U-INV-4 | Fine-grained unwired `lib/**` | REQUIRES INVESTIGATION | T010 | Per-module importer census incomplete | Post-R0 reachability pass | Do not mark ORPHANED via Graphify |

### U-CON — Contradictions C1–C15 (from T010; unresolved)

| ID | Summary | Evidence | Earliest resolution | Non-authorization |
| --- | --- | --- | --- | --- |
| U-CON-1 | pnpm workspace → missing `apps/desktop/ui` vs `afia-ui/` | T001; T010 C1 | Manifest adapt tasks | No silent layout invent |
| U-CON-2 | Cargo members missing vs `_archived/crates` | T008; T010 C2 | After accepted ADR-15 (T030) + adapt | No archive reactivate |
| U-CON-3 | `go.work` missing live Go vs archive | T008; T010 C3 | Archive for Alpha; ADR if return | No Go Alpha dependency |
| U-CON-4 | macOS echo CI vs Windows-first | T009; T010 C4 | Honest CI later | No “CI green” product claim |
| U-CON-5 | Archive “UI≠Python” vs live OpenMed calls | T007; T008; T010 C5 | Host/ADR package | UI-zero-authority not present |
| U-CON-6 | schema.sql / dual migrations | T006; T010 C6 | **Spec 002** | No migration mutation in 001 |
| U-CON-7 | `services/README` vs flat bridges | T007; T010 C7 | Docs later | — |
| U-CON-8 | FHIR R4B prototype vs commandF canonical | T007; T010 C8 | commandF / accepted ADR | No “validated FHIR” claim |
| U-CON-9 | Legacy AFIA plans vs Constitution | T010 C9 | Docs discipline | Legacy not governing |
| U-CON-10 | Fanatir identity vs `afia*` names | T010 C10 | Rename spec | Keep AFIA working until then |
| U-CON-11 | Supabase docs storage vs Rust Artifact Store target | T006; T010 C11 | ADR-05 + Spec 002; T030 package | No PHI expand on crypto edge |
| U-CON-12 | Direct UI authority vs Trusted Host target | T004; T010 C12 | Accepted ADR package at **T030**; R2 | No host implementation in R0/R1 drafts |
| U-CON-13 | CI/tests README overclaims | T009; T010 C13 | Docs/CI honesty tasks | Placeholder ≠ gate |
| U-CON-14 | `apps/README` missing members | T010 C14 | Manifest adapt | — |
| U-CON-15 | Root pnpm 9 vs afia-ui pnpm 10.4.1 | T003; T010 C15 | Tooling adapt | — |

### U-RT — Runtime / conditional (carry-forward)

| ID | Item | Classification | Evidence | Earliest resolution | Non-authorization |
| --- | --- | --- | --- | --- | --- |
| U-RT-1 | afia-ui install/build/dev | REFERENCED BUT RUNTIME-UNVERIFIED | T002 | Authorized install/verify later | No install in T011 |
| U-RT-2 | Supabase live session/env | REFERENCED BUT RUNTIME-UNVERIFIED | T005 | Env + live verify later | No auth change |
| U-RT-3 | OpenMed/FHIR bridges execution | REFERENCED BUT RUNTIME-UNVERIFIED | T007 | Bounded DeepMed path later | No OpenMed fork/import |
| U-RT-4 | Manus/dev-only tooling | CONDITIONALLY ACTIVE | T004; T010 | Product policy later | Do not elevate to product authority |
| U-RT-5 | Demo data fixtures | CONDITIONALLY ACTIVE | T004; T010 | Fixture policy later | Not clinical SoT |
| U-RT-6 | Hosted GitHub Actions / branch protection | Unknown | T009 | Authorized `gh` inspection later | Do not invent hosted state |
| U-RT-7 | `patches/**` via `pnpm.patchedDependencies` | REFERENCED BUT RUNTIME-UNVERIFIED | T010 | Install later | Not orphaned |

T011 attempted resolution for all entries above: **NO**.

---

## 4.7 Authority and precedence

### Authoritative now

1. Constitution v1.0.0 (`.specify/memory/constitution.md`)
2. Accepted Specification 001 (`specs/001-.../spec.md`)
3. Accepted Plan 001 (`specs/001-.../plan.md`)
4. Accepted Task Program 001 (`specs/001-.../tasks.md`)
5. Explicitly accepted founder decisions (including T001–T010 acceptances; T010 classification precision)

Higher authority controls lower-level planning artifacts.

### Non-authoritative as architecture authority

- **ADR drafts**, including ADR-15
- Legacy AFIA architecture / product documents
- `_archived/**`
- Graphify output
- AI conclusions / chat memory
- Unaccepted sibling plans
- Baseline evidence **beyond** the inspected facts it records

### Clarifications

| Item | Status |
| --- | --- |
| Baseline R0 evidence | Authoritative as **records of inspected facts** only; **not** product/architecture SoT |
| Source code | Implementation facts only; subordinate to governance |
| ADR-15 | **PLANNING EVIDENCE — NON-AUTHORITATIVE ADR DRAFT**. Reflects founder-ratified Rust-first **direction**. Becomes accepted architecture authority only at **T030**. Draft ADR ≠ accepted authority |
| Archives | Do not become active by existing; `retain` ≠ reactivate |
| T030 | Architecture-acceptance gate for the R1 ADR package including ADR-15 |

---

## 4.8 R0 exit criteria

Derived from Plan R0 exit + Tasks T001–T011 checkpoint (“baseline evidence package exists; no production mutation”) + Spec inventory/disposition completion via T010.

| Exit criterion | Evidence | Result |
| --- | --- | --- |
| Spec 001 + Constitution govern entry | Prerequisite commits; plan R0 entry | **PASS** |
| Repository reality inspected (baseline, siblings) | T001 `R0-repo-baseline.md` | **PASS** |
| Build/dev commands recorded | T002 `R0-build-commands.md` | **PASS** |
| Toolchain/dependency managers recorded | T003 `R0-tooling.md` | **PASS** |
| Active frontend topology mapped | T004 `R0-frontend-map.md` | **PASS** |
| Auth/session observe-only recorded | T005 `R0-auth-session.md` | **PASS** |
| Supabase/migration trees inventoried (frozen) | T006 `R0-supabase-inventory.md` | **PASS** |
| Python/FHIR/OpenMed prototypes inspected | T007 `R0-python-services.md` | **PASS** |
| Archived Rust/Tauri/Go inspected without reactivation | T008 `R0-archived-assets.md` | **PASS** |
| CI/test honesty recorded | T009 `R0-ci-tests.md` | **PASS** |
| Path classification completed (81 rows) | T010 `R0-path-classification.md` | **PASS** |
| Stale manifests / contradictions listed | T001; T010 C1–C15; §4.6 | **PASS** |
| Fehrest empty / DeepMed README-only recorded | T001; T010 siblings | **PASS** |
| Unresolved matters preserved (not silently closed) | §4.6 | **PASS** |
| Baseline evidence package exists | This document + index §4.4 | **PASS** |
| No production mutation in R0 commits | `git diff-tree` T001–T010 = program-memory docs only | **PASS** |
| Authority hierarchy restated | §4.7 | **PASS** |
| Program memory prepared for closeout | CURRENT-STATE / NEXT-ACTION (T011 uncommitted) | **PASS** (pending founder accept + commit) |

---

## 4.9 R0 closeout decision

All mandatory accepted R0 exit criteria above have evidence.

```text
R0 CLOSED — REPOSITORY-REALITY BASELINE COMPLETE
```

```text
R0 closure does not constitute architecture acceptance or implementation authorization.
```

**Canonicalization note:** R0 closure becomes repository-canonical only after founder acceptance of T011 and the local T011 documentation commit. Until then, this package is executed evidence awaiting acceptance.

---

## Acceptance (T011)

| Criterion | Result |
| --- | --- |
| R0 exit criteria satisfied | **Met** (evidence above) |
| No production diffs in T011 work | **Met** (docs only; uncommitted) |
| Tier C verification | Normal self-check in execution report |
| Founder acceptance | **Not yet** |

## Rollback

Delete this file; revert CURRENT-STATE / NEXT-ACTION pointer edits for T011.
