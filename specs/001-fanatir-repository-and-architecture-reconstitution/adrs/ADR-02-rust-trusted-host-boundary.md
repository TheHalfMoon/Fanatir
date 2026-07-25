# ADR-02 — Rust Trusted Host Boundary

| Field | Value |
| --- | --- |
| **ADR** | ADR-02 |
| **Title** | Rust Trusted Host Boundary |
| **Status** | **Proposed** (draft only) |
| **Task origin** | T014 |
| **Acceptance gate** | **T030** (with ADR-15, ADR-01, and ADR-06 per Tasks 001 / Plan R1–R2 entry) |
| **Feature** | `001-fanatir-repository-and-architecture-reconstitution` |
| **Constraining drafts** | [ADR-15](./ADR-15-rust-first-polyglot-runtime.md) (Proposed); [ADR-01](./ADR-01-platform-and-desktop-composition-tauri-2.md) (Proposed) |
| **Architecture authority of this file** | **NO** — until accepted at T030 |
| **Implementation authorization** | **NO** |

```text
This document is a planning draft and is not accepted architecture authority.
```

```text
Completed ADR drafting task ≠ architecture acceptance
T014 completion ≠ T030 acceptance
ADR-15 remains Proposed and does not accept language authority by itself
ADR-01 remains Proposed and does not accept Tauri by itself
```

```text
No production implementation is authorized by this draft.
```

This draft **must not** be used as justification to: create `apps/desktop/**` or Rust crates; install Tauri/Rust toolchains for production cutover; add Tauri commands or capability files; implement IPC; mediate filesystem/process/secrets/network; wire OpenMed/sidecars; modify `afia-ui` production routes/auth; reactivate `_archived/apps-desktop` or `_archived/crates`; repair root Cargo/pnpm manifests; change CI; or otherwise implement R2.

---

## 1. Context

### 1.1 Assigned architectural question

**ADR-02 proposes** the **responsibility boundary** of Fanatir’s minimal Rust Trusted Host: which privileged authorities the host exclusively owns for Founder Alpha planning and R2 exit criteria, which components remain untrusted requestors, and how authorization/denial semantics behave at the planning level.

It does **not** finalize IPC command schemas (ADR-06), Artifact Store design (ADR-05), shared primitive schemas (ADR-04), PHI egress policy detail (ADR-14), DeepMed/OpenMed domain operations (ADR-09), packaging/signing operations (ADR-13), or crate layout.

### 1.2 Accepted program constraints

| Constraint | Source | Treatment |
| --- | --- | --- |
| Early minimal Trusted Host after plan + required ADRs, before major feature expansion | Spec 001 Q3 | Binding program requirement |
| Minimum early host capability set: local project access; filesystem mediation; secrets; capability enforcement; worker supervision; audit events; secure desktop composition | Spec 001 Minimal Trusted Host | Binding capability categories (detail via this ADR when accepted) |
| Minimal host ≠ full final kernel before first vertical slice | Spec 001 | Binding scope limit |
| Rust-first Trusted Host language | Decision A / ADR-15 proposes | Binding **direction**; ADR-15 document still **Proposed** |
| Platform composition via Tauri 2 Core + untrusted WebView loading `afia-ui` | ADR-01 proposes | Proposed composition context only |
| `afia-ui` is migration starting shell; auth/session freeze until dedicated migration spec | Spec Q1/Q4; Plan | Binding migration posture |
| No OpenMed/Graphify fork/import; no Pictorial/Montada; no Go-owned Trusted Host | Spec/Plan/Tasks | Binding prohibitions |
| No R2 implementation before ADR-15/01/02/06 Accepted (as applicable) | Plan; Tasks T030/T031 | Future-gate requirement |

### 1.3 R0 inspected facts (evidence only)

| Fact | Evidence | Treatment |
| --- | --- | --- |
| Active UI is Vite/React `afia-ui/`; not a live Trusted Host | T004; T010 | Implementation fact |
| No live `apps/desktop` Trusted Host; `apps/` contains README documenting missing desktop paths | T008; T010 C14 | CONTRADICTORY layout docs |
| Root Cargo/pnpm declare missing desktop/crate members | T008; T010 C1–C2 | CONTRADICTORY manifests |
| `_archived/apps-desktop` and `_archived/crates` are ARCHIVED empty scaffolds (not buildable hosts) | T008 | Non-authoritative historical context; disposition **replace** / investigate shapes |
| Live UI calls OpenMed bridge at `http://127.0.0.1:8765` without Trusted Host mediation | T007; T010 C5/C12 | Transitional fact; contradicts UI-zero-authority target |
| CI is macOS echo-only; no Trusted Host / packaging gates | T009 | Fact |
| Path classification: local Artifact/policy/secrets/audit **Absent** as Rust Trusted Host | T010 | Fact |

R0 baseline evidence is authoritative **only as a record of inspected facts**.

### 1.4 Planning research (non-architecture-authority)

Plan 001 R2 objectives and [contracts/trusted-host-ipc.md](../contracts/trusted-host-ipc.md) list planning-level privileged invokes and deny-by-default posture. Those contracts are **draft planning evidence**, not accepted architecture, and do **not** authorize implementation. Exact command schemas remain **ADR-06**.

---

## 2. Decision drivers

| Driver | Source |
| --- | --- |
| Spec Q3 minimum early host capability categories | Spec 001 |
| UI must not be SoT for authorization, privileged paths, secrets, process ownership, durable Artifact identity, or policy | Spec/Plan trust goals; ADR-01 proposes |
| Rust owns Trusted Host responsibilities (ADR-15 proposes) | Decision A; ADR-15 Proposed |
| Fail-closed, least-privilege, auditable decisions | Constitution/Spec security posture; Plan R2 |
| Preserve `afia-ui` journeys while strangling authority into host | Spec Q1; ADR-03 later |
| Avoid silently reactivating archived empty host scaffolds | Spec disposition; T008 |
| Keep ADR-02 boundary precise enough for R2 exit without completing IPC/storage/PHI ADRs | Plan ADR roadmap |

---

## 3. Proposed decision

**ADR-02 proposes** that Fanatir’s minimal Rust Trusted Host is the **sole privileged local authority** for the Spec Q3 early-host capability categories, operating inside the Core process that ADR-01 proposes (Tauri 2 Core) and using the language allocation that ADR-15 proposes (Rust Trusted Host).

### 3.1 Proposed exclusive host authorities (minimal / R2-oriented)

If ADR-02 is accepted at T030, the Trusted Host **exclusively** mediates and decides:

| Authority category | Proposed host role | Detail reserved to |
| --- | --- | --- |
| Project / workspace identity | Canonical local project open/create; validates and owns project root identity | Later R2 tasks; related schemas → ADR-04 |
| Filesystem mediation | Resolves and validates permitted roots; mediates reads/writes; fails closed on traversal/escape | Path roots model → unresolved; APIs → ADR-06 |
| Secrets / credentials | Stores and releases secrets under policy; UI/workers do not own secret SoT | Storage backend → later; PHI posture → ADR-14 |
| Capability / policy decisions | Evaluates allow/deny for privileged effects; deny-by-default | Exact capability files → later R2; PHI egress rules → ADR-14 |
| Worker / sidecar supervision | Spawns, monitors, stops, restarts allowlisted workers; bounds args/env; owns lifecycle | Contracts → ADR-06; DeepMed ops → ADR-09; Fehrest → ADR-08 |
| Authoritative audit events | Records host-authoritative audit of privileged decisions and effects | Event schema retention → later |
| Durable Artifact mutation gateway | Authorizes Artifact/Revision/Run mutations; UI/workers must not mutate stores directly | Store design → **ADR-05** |
| Privileged network / egress gates | Mediates elevated-trust network and PHI-relevant egress decisions | Cloud adapter → ADR-07; PHI → ADR-14 |
| IPC request validation | Validates typed privileged requests before effects | Protocol/schemas → **ADR-06** |
| Resource bounds / timeout / cancel | Enforces host-controlled limits on privileged work | Exact limits → later / ADR-06 |

### 3.2 Proposed untrusted / non-authoritative components

| Component | Proposed classification | May | Must not (proposed) |
| --- | --- | --- | --- |
| WebView / `afia-ui` React/TS | Untrusted presentation / requestor | Present UI; request intent; display results | Own authorization, privileged FS, secrets, process ownership, durable Artifact SoT, policy SoT |
| Frontend routes / local UI state | Untrusted | Navigate presentation | Establish privileged authorization |
| UI-provided paths / IDs | Requests only | Suggest intent | Become canonical roots or Artifact identity without host validation |
| Python workers / OpenMed bridge | Untrusted compute workers | Perform bounded computation under supervision | Become authority; mutate Artifacts directly; own secrets/policy |
| DeepMed / Fehrest / Graphify / notebooks / R | Untrusted or external product boundaries | Operate under host-mediated contracts when integrated | Own Trusted Host roles |
| Model outputs / imported artifacts | Untrusted content | Be reviewed/stored under host rules | Auto-grant privilege |
| External services / Supabase JS | Non-host adapters | Optional identity/collab when authorized | Own local content/PHI/Artifact SoT (Decision C direction) |
| Archived Rust/Tauri scaffolds | Non-authoritative historical context | Inform risks and intent | Serve as production Trusted Host |

“Untrusted” means **not permitted to make final privileged authorization decisions**, not that the component is assumed malicious.

### 3.3 Proposed authorization and denial semantics (planning level)

| Proposed rule |
| --- |
| Fail closed by default; privilege requires explicit capability grant |
| Privileged effects occur only after host validation of typed, bounded requests |
| Unknown/unsupported requests are denied; no silent fallback to privilege |
| Request intent (UI/worker) is distinct from host authority |
| Denials use stable categories suitable for UI explanation without leaking secrets |
| Cancellation, timeout, and resource bounds are host-enforced for privileged work |
| Human-readable decision explanations are required for privileged allow/deny outcomes at the planning level |

Exact IPC error taxonomy and capability-file formats are **not** finalized here (ADR-06 / later tasks).

### 3.4 Proposed R2 exit criteria (boundary completeness, not implementation)

If ADR-02 is accepted at T030, R2 exit planning **expects** evidence that the minimal host boundary covers Spec Q3 categories (project access, FS mediation, secrets, capability enforcement, worker supervision, audit, secure composition) **without** requiring the full final kernel. Concrete demo tasks remain **T031+** and require separate authorization after T030.

### 3.5 What this ADR does **not** settle

| Deferred matter | Owner |
| --- | --- |
| Tauri 2 composition framework | **ADR-01** (Proposed) |
| Language authority beyond Trusted Host boundary | **ADR-15** (Proposed) |
| `afia-ui` strangler sequencing | **ADR-03** |
| Shared primitive schemas | **ADR-04** |
| Artifact/Revision/Run storage design | **ADR-05** |
| IPC command/event schemas, size limits, transport details | **ADR-06** / contracts |
| Supabase / hosted adapter boundary | **ADR-07** / Spec 002 / Decision C |
| Fehrest integration | **ADR-08** |
| DeepMed / OpenMed domain operations | **ADR-09** |
| commandF ownership detail | **ADR-10** |
| Packaging / updater / signing operations | **ADR-13** / Decision D |
| PHI / data-classification enforcement detail | **ADR-14** |
| Exact crate layout, Tauri capability JSON, CI, manifest repair | Later tasks after gates |

Proposing this boundary does **not** authorize creating the Trusted Host, implementing mediation, or cutting over production.

---

## 4. Scope

### In scope

- Trusted Host responsibility boundary for Spec Q3 early-host categories
- Privileged vs untrusted classification
- Planning-level authorization/denial and fail-closed posture
- Worker/sidecar supervision boundary (not protocols)
- Filesystem/process/network/secret constraint statements at planning level
- R2 exit criteria framing for the **boundary** (not code)

### Out of scope

- Rust/Tauri/production code
- Archive deletion or reactivation
- Auth/session behavior changes
- CI/workflow/manifest/lockfile changes
- Acceptance of ADR-15/01/02/06 (T030)
- Final IPC schemas, Artifact Store, PHI policy engines, packaging

---

## 5. Options considered

### Option A — Rust Trusted Host as sole privileged local authority (proposed)

| | |
| --- | --- |
| Benefits | Matches Spec Q3; aligns with Decision A / ADR-15 proposes and ADR-01 Core proposal; enables consistent policy, audit, and worker supervision |
| Costs | Larger trusted computing base; Rust host design/test burden; migration from UI-direct seams |
| Security | Single privileged decision point; fail-closed possible; confused-deputy risk if boundary poorly drawn |
| Compatibility | Required direction of Spec/Plan; constrained by ADR-15/01 Proposed drafts |
| Draft disposition | **Proposed** |

### Option B — Distributed authority across UI and workers

| | |
| --- | --- |
| Benefits | Faster short-term feature work on current seams |
| Costs | Continues R0 UI-direct OpenMed pattern; no single policy/audit SoT |
| Security | High privilege leakage risk |
| Compatibility | **Incompatible** with Spec Q3 / UI-zero-authority target |
| Draft disposition | **Rejected** |

### Option C — Tauri capability configuration as primary authority (host logic secondary)

| | |
| --- | --- |
| Benefits | Leverages framework ACL files |
| Costs | Capability files alone do not provide typed validation, audit SoT, Artifact authority, or worker supervision semantics |
| Security | Insufficient as sole authority for Spec Q3 categories |
| Compatibility | Weak vs Plan R2 Trusted Host objectives; ACL remains a complementary control |
| Draft disposition | **Rejected** as primary authority (may complement host decisions later) |

### Option D — Privileged Python supervisor instead of Rust Trusted Host

| | |
| --- | --- |
| Benefits | Reuses existing Python bridge familiarity |
| Costs | Conflicts with Decision A / ADR-15 Rust-mandatory Trusted Host direction; expands untrusted surface into authority |
| Security | Weaker fit to Rust-first trusted systems goal |
| Compatibility | **Incompatible** with Decision A binding direction |
| Draft disposition | **Rejected** |

### Option E — Separate OS-native privileged daemon / helper outside Tauri Core

| | |
| --- | --- |
| Benefits | Isolation from WebView process |
| Costs | Extra process topology; weaker fit to ADR-01 proposed Core composition; higher delivery risk for Alpha |
| Security | Possible if well designed; not the Plan/ADR-01 proposed model |
| Compatibility | Weaker fit to Proposed ADR-01 composition |
| Draft disposition | **Rejected** for Alpha planning (re-open only via new accepted decision) |

### Option F — Minimal host shell with broad worker autonomy

| | |
| --- | --- |
| Benefits | Smaller host code initially |
| Costs | Leaves FS/secrets/policy/audit effectively in workers/UI; fails Spec Q3 minimum categories |
| Security | Privilege escapes likely |
| Compatibility | **Incompatible** with Spec Q3 |
| Draft disposition | **Rejected** |

### Option G — Reactivate archived Rust/Tauri scaffolds as Trusted Host

| | |
| --- | --- |
| Benefits | Apparent reuse |
| Costs / risks | T008: empty mains/capabilities; unbuildable; non-authoritative |
| Compatibility | Spec disposition replace/investigate; not production authority |
| Draft disposition | **Rejected** as authority (intent may be investigated, not restored as-is) |

---

## 6. Consequences

### Positive (prospective — if accepted at T030)

- Clear privileged authority boundary for R2 planning
- Reduced UI/worker authority over secrets, FS, policy, and Artifact mutation
- Consistent supervision and auditability for workers
- Fail-closed handling path for unknown privilege requests
- Enables strangling away from UI-direct localhost worker calls

### Costs and negative consequences

- Requires building a Trusted Host absent in R0
- Policy/capability design and maintenance burden
- IPC and testing complexity (shared with ADR-06 / R2)
- Migration cost from current UI→OpenMed direct seam
- Operational debugging complexity across host + workers
- Risk of host becoming an overly broad monolith if later ADRs are absorbed here

### Risks

| Risk | Evidence / note | Mitigation (proposed planning constraint only) |
| --- | --- | --- |
| Draft used as license to implement R2 now | Tasks T014/T031 | Forbidden until T030 + explicit R2 tasks |
| Capability creep / policy drift | Future growth | Keep ADR-02 minimal; detail in ADR-06/14 |
| Confused deputy via UI-supplied paths | T007/T010 path requests | Host must independently validate roots |
| Worker escape / uncontrolled subprocesses | Python bridge exists today | Supervised allowlist; no shell to WebView |
| Secret leakage to UI/workers | Transitional UI seams | Host-owned secrets; deny direct exposure |
| Denial messages leak sensitive data | UX pressure | Stable categories; redact secrets |
| Archive scaffold over-read as reusable host | T008 | Successor only; do not restore as-is |
| ADR-15/01 still Proposed | T012/T013 | Boundary constrained by direction, not accepted ADR text until T030 |

Prospective benefits are **not** measured outcomes.

---

## 7. Constraints and invariants

### Binding now (higher authority)

| Constraint | Source |
| --- | --- |
| Early minimal Trusted Host after ADRs | Spec Q3 |
| Minimum early capability categories | Spec Minimal Trusted Host |
| Decision A Rust-first direction | Plan (Ratified) |
| No production mutation from this ADR file | Tasks T014 |
| No R2 impl before required ADR acceptances | Plan; Tasks T030/T031 |

### Proposed invariants (architecture authority only if ADR-02 accepted at T030)

| Proposed invariant |
| --- |
| UI and workers may request; host decides privileged effects |
| No silent privilege fallback |
| Archived scaffolds are not the Trusted Host |
| Exact IPC/Artifact/PHI protocols remain owned by their ADRs |

---

## 8. Relationship to other ADRs

| ADR | Relationship |
| --- | --- |
| ADR-15 (Proposed) | Proposes Rust for Trusted Host language; does **not** finalize this responsibility checklist |
| ADR-01 (Proposed) | Proposes Tauri 2 Core/WebView composition that would host this boundary; does **not** accept Tauri via ADR-02 |
| ADR-03 | Strangler migration of `afia-ui` authority into host-mediated paths |
| ADR-04 | Shared primitive ownership/versioning |
| ADR-05 | Artifact/Revision/Run storage |
| ADR-06 | Worker/IPC contracts and schemas |
| ADR-07 / Spec 002 | Supabase / hosted adapter |
| ADR-08 / ADR-09 / ADR-10 | Fehrest / DeepMed / commandF integration boundaries |
| ADR-13 / Decision D | Packaging/signing operations |
| ADR-14 | Security / data-classification enforcement detail |
| T031+ | Implementation of minimal shell/host — **only after T030** and explicit task authorization |

---

## 9. Unresolved questions

T014 attempted resolution: **NO**.

| ID | Question | Evidence | Future gate |
| --- | --- | --- | --- |
| U-ADR02-1 | Exact Trusted Host module/crate split inside Core | Plan target tree; R0 missing live host | R2 tasks after T030 |
| U-ADR02-2 | Capability representation and persistence format | T008 empty capabilities scaffold | Later R2; ADR-14 overlap |
| U-ADR02-3 | Authorization-decision / denial schema fields | Planning IPC draft only | ADR-06 |
| U-ADR02-4 | Host↔worker contract versioning and sandbox expectations | T007 prototype bridge | ADR-06/09 |
| U-ADR02-5 | Filesystem permitted-root model (multi-root, aliases) | Spec project access; no live host | R2; ADR-06 |
| U-ADR02-6 | Secrets backend (OS keychain vs encrypted store) | Spec secrets category | R2; ADR-05 overlap |
| U-ADR02-7 | Network mediation scope for non-PHI vs PHI egress | T007 localhost; Decision C | ADR-07/14 |
| U-ADR02-8 | Audit retention and export rules | Spec audit events | Later; ADR-14 |
| U-ADR02-9 | User-consent surfaces for privileged grants | Auth freeze; UX unknown | ADR-03/11 later |
| U-ADR02-10 | Platform differences (Windows-first vs macOS/Linux mediation) | Spec Windows-first; T008 skew | ADR-13 / R2 |
| U-ADR02-11 | Testing/evidence requirements for R2 boundary exit | Plan R2; CI echo-only today | T031+ / CI tasks after gates |

---

## 10. Validation and acceptance plan

```text
T014 completion produces a draft for review. T030 is the acceptance gate.
```

Before T030, planning reviews **may** include:

- Tier A independent Trusted Host ADR review (Tasks 001)
- Consistency with Spec Q3, Plan R2, ADR-15 Proposed, ADR-01 Proposed
- R0 evidence that no Trusted Host is currently implemented
- Security claim-language review (fail-closed; UI untrusted)

**Not** validation for T014: creating desktop/Rust projects, implementing commands, installing dependencies, mediating FS/process/network, or CI greens.

**T030** remains the gate to mark ADR-02 (with ADR-15/01/06 as required) **Accepted** before R2 shell/host implementation (T031+).

---

## 11. Gate

**ADR-02 MUST be accepted at T030 (with the required R1 ADR package) before R2 Trusted Host implementation.**

Until that acceptance, this file remains:

```text
PLANNING EVIDENCE — NON-AUTHORITATIVE ADR DRAFT
```

```text
ADR-02 must remain Proposed until its designated architecture-acceptance gate.
```

T031+ remains separately gated and requires T030 plus explicit task authorization. Passing Tier A review alone does **not** make ADR-02 architecture authority.

---

## 12. References

### Accepted authority

- Constitution v1.0.0 (`.specify/memory/constitution.md`)
- [Specification 001](../spec.md) — Q3; Minimal Trusted Host
- [Plan 001](../plan.md) — ADR roadmap; R2 Trusted Host objective
- [Tasks 001](../tasks.md) — T014; T030; T031

### Planning evidence (non-architecture-authority)

- [ADR-15](./ADR-15-rust-first-polyglot-runtime.md) — Proposed language authority draft
- [ADR-01](./ADR-01-platform-and-desktop-composition-tauri-2.md) — Proposed platform composition draft
- [research.md](../research.md) — desktop/host notes
- [contracts/trusted-host-ipc.md](../contracts/trusted-host-ipc.md) — planning transport assumptions (not accepted schemas)

### R0 factual evidence

- [R0-evidence-package.md](../../../docs/program-memory/baseline/R0-evidence-package.md) — R0 close `f37563d8f8499aa24e8ead68fa920167ebb8cde6`
- [R0-archived-assets.md](../../../docs/program-memory/baseline/R0-archived-assets.md) — T008 archived host/crates
- [R0-path-classification.md](../../../docs/program-memory/baseline/R0-path-classification.md) — T010 `2c9fb55244588bb35b2d08bc1e1c6bbcbadbd8ec`
- [R0-frontend-map.md](../../../docs/program-memory/baseline/R0-frontend-map.md)
- [R0-python-services.md](../../../docs/program-memory/baseline/R0-python-services.md)
- [R0-ci-tests.md](../../../docs/program-memory/baseline/R0-ci-tests.md)
- [R0-build-commands.md](../../../docs/program-memory/baseline/R0-build-commands.md)

### Explicitly non-authoritative

- `_archived/apps-desktop/**`
- `_archived/crates/**`
- Legacy `docs/product/AFIA_*`
- Graphify output
- AI recommendations
- Source code beyond implementation facts it demonstrates
