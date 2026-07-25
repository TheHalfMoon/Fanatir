# ADR-01 — Platform and Desktop Composition (Tauri 2)

| Field | Value |
| --- | --- |
| **ADR** | ADR-01 |
| **Title** | Platform and Desktop Composition (Tauri 2) |
| **Status** | **Proposed** (draft only) |
| **Task origin** | T013 |
| **Acceptance gate** | **T030** (with ADR-15, ADR-02, and ADR-06 per Tasks 001 / Plan R1–R2 entry) |
| **Feature** | `001-fanatir-repository-and-architecture-reconstitution` |
| **Constraining draft** | [ADR-15](./ADR-15-rust-first-polyglot-runtime.md) (Proposed; Decision A direction) |
| **Architecture authority of this file** | **NO** — until accepted at T030 |
| **Implementation authorization** | **NO** |

```text
This document is a planning draft and is not accepted architecture authority.
```

```text
Completed ADR drafting task ≠ architecture acceptance
T013 completion ≠ T030 acceptance
ADR-15 remains Proposed and does not accept Tauri by itself
```

```text
No production implementation is authorized by this draft.
```

This draft **must not** be used as justification to: create `apps/desktop/**`; install Tauri/Rust toolchains for production cutover; modify `afia-ui` production routes/auth; reactivate `_archived/apps-desktop`; repair root Cargo/pnpm manifests; change CI; introduce updater signing keys; or otherwise implement R2.

---

## 1. Context

### 1.1 Assigned architectural question

**ADR-01 proposes** how Fanatir’s desktop product **composes** a local Trusted Host Core with an untrusted WebView UI surface, including the intended composition framework for Founder Alpha planning.

It does **not** define the full Trusted Host responsibility checklist (ADR-02), IPC schemas (ADR-06), Artifact Store (ADR-05), or packaging/signing operations (ADR-13 / Decision D).

### 1.2 Accepted program constraints

| Constraint | Source | Treatment |
| --- | --- | --- |
| Minimal Tauri/Rust Trusted Host is an early foundational requirement after plan + required ADRs, before major feature expansion | Spec 001 Q3 | Binding program requirement (composition detail via this ADR when accepted) |
| Windows-first Alpha packaging; architecture not Windows-only | Spec/Plan | Binding Alpha packaging direction |
| `afia-ui` is migration starting shell / presentation; not architecture SoT | Spec Q1; Plan; research R5 | Binding migration posture |
| Preserve auth/session/`PrivateRoute`/profile until dedicated migration spec | Spec/Plan | Binding freeze |
| Rust Trusted Host Core language | Decision A / ADR-15 proposes | Binding **direction**; ADR-15 document still **Proposed** |
| No OpenMed/Graphify fork/import; no Pictorial/Montada activation | Spec/Plan/Tasks | Binding prohibitions |
| No R2 implementation before ADR-15/01/02/06 Accepted (as applicable) | Plan blocking matrix; Tasks T030/T031 | Future-gate requirement |

### 1.3 R0 inspected facts (evidence only)

| Fact | Evidence | Treatment |
| --- | --- | --- |
| Active UI is Vite/React `afia-ui/`, not a live Tauri app | T004; T010 | Implementation fact |
| No live `apps/desktop` Trusted Host; root Cargo/pnpm declare missing desktop paths | T001; T008; T010 C1–C2/C14 | CONTRADICTORY manifests |
| `_archived/apps-desktop/**` is ARCHIVED Tauri v2 scaffold (empty main/capabilities; macOS-skewed `app`/`dmg`; AFIA productName; not buildable from current layout) | T008 | Non-authoritative; disposition **replace** via successor (do not delete archive in R0) |
| Live UI calls localhost OpenMed (UI-direct worker seam) | T007; T010 C5/C12 | Transitional fact; contradicts UI-zero-authority target |
| CI is macOS echo-only; not Windows packaging gate | T009 | Fact |
| Path classification: desktop host absent as VERIFIED ACTIVE Trusted Host | T010 | Fact |

R0 baseline evidence is authoritative **only as a record of inspected facts**.

### 1.4 Planning research (non-architecture-authority)

Plan 001 and research.md propose Tauri 2 Core + WebView, with Core as sole full-OS-access component and WebViews untrusted ([research R4 / desktop notes](../research.md)). Those notes cite official Tauri 2 docs as external research; they are **not** ADR acceptance.

---

## 2. Decision drivers

| Driver | Source |
| --- | --- |
| Spec Q3 early minimal secure desktop composition | Spec 001 |
| Rust-first Trusted Host Core (Decision A / ADR-15 proposes) | Plan Decision A; ADR-15 Proposed |
| WebView must not be OS/filesystem/secrets SoT | Constitution/Spec trust goals; Tauri process-model research |
| Preserve `afia-ui` journeys while strangling authority into host | Spec Q1; ADR-03 later |
| Windows-first Alpha; avoid macOS-only archive skew | Spec/Plan; T008 archive findings |
| Deny-by-default capability intent is desirable; archive empty capabilities are only scaffold evidence | T008 |
| Avoid silent archive reactivation | Spec disposition; T008/T010 |

---

## 3. Proposed decision

**ADR-01 proposes** that Fanatir’s Founder Alpha desktop composition use **Tauri 2**:

| Element | Proposed role |
| --- | --- |
| **Tauri Core (Rust)** | Sole privileged desktop process for OS-mediated operations; hosts the Trusted Host Core **language** required by ADR-15’s proposed model |
| **WebView UI** | Untrusted presentation surface; loads/adapts the existing `afia-ui` client as migration shell |
| **IPC boundary** | Privileged operations only via host-mediated invokes (detailed contracts → **ADR-06**) |
| **Capabilities** | Least-privilege / deny-by-default posture (exact capability set → later ADRs/R2 tasks) |
| **Sidecars** | Supervised workers (Python/Fehrest/DeepMed) launched/supervised by Core (rules → ADR-15/06/09) |
| **Packaging target** | Windows-first Alpha installers; retain architectural support for macOS/Linux (details → ADR-13) |

### 3.1 Proposed composition invariants (only if ADR-01 accepted at T030)

| Proposed invariant |
| --- |
| WebView storage/network/filesystem access is not the system of record for secrets, project roots, Artifact mutation, policy, PHI egress, or authoritative audit |
| UI may present and request; Core mediates reserved authorities |
| Archived `_archived/apps-desktop` is not the production host; a **successor** layout is required |
| `afia-ui` remains the UI migration shell until ADR-03 / later migration tasks authorize changes |

### 3.2 What this ADR does **not** settle

| Deferred matter | Owner |
| --- | --- |
| Exact Trusted Host responsibility checklist / R2 demo exits | **ADR-02** |
| Language authority details beyond composition | **ADR-15** (Proposed) |
| IPC command/event schemas, error taxonomy, size limits | **ADR-06** / contracts |
| Artifact/Revision/Run storage | **ADR-05** |
| `afia-ui` strangler route/adapter plan | **ADR-03** |
| Supabase vs local content SoT | **ADR-07** / Spec 002 / Decision C |
| Packaging, updater, signing custody operations | **ADR-13** / Decision D |
| Exact folder names under `apps/desktop/**` | Later R2 tasks after T030 |
| Manifest repair for root Cargo/pnpm | Later adapt tasks — **not** authorized here |
| WebView2 redistributable/install UX details | Packaging tasks / ADR-13 |

Proposing Tauri 2 does **not** authorize creating the desktop app, installing dependencies, or cutting over production.

---

## 4. Scope

### In scope

- Desktop composition framework selection for planning (Tauri 2)
- Core vs WebView trust split
- Relationship to `afia-ui` as presentation shell
- Explicit non-reactivation of archived desktop scaffold as authority
- Windows-first Alpha packaging **direction** (not installer implementation)

### Out of scope

- Rust/Tauri/production code
- Archive deletion or reactivation
- Auth/session behavior changes
- CI/workflow/manifest/lockfile changes
- Acceptance of ADR-15/01/02/06 (T030)
- Detailed security control design beyond composition trust split

---

## 5. Options considered

### Option A — Tauri 2 Core + WebView loading `afia-ui` (proposed)

| | |
| --- | --- |
| Benefits | Aligns Spec Q3 + Plan; Rust Core matches Decision A / ADR-15 proposes; official process model separates untrusted WebView; supports sidecars; Windows packaging path exists in Tauri 2 research notes |
| Risks | New host work absent in R0; WebView2 dependency on Windows; migration from UI-direct OpenMed calls; packaging/signing burden (Decision D) |
| Compatibility | Required direction of Spec/Plan research; constrained by ADR-15 Proposed language model |
| R0 implications | Successor replaces archive scaffold; keep `afia-ui` |
| Draft disposition | **Proposed** |

### Option B — Browser-only Alpha; desktop host later

| | |
| --- | --- |
| Benefits | Faster short-term UI iteration |
| Risks | Rejects Spec Q3 early host requirement; leaves FS/secrets/workers without Trusted Host |
| Compatibility | **Incompatible** with Spec Q3 |
| Draft disposition | **Rejected** |

### Option C — Reactivate `_archived/apps-desktop` as production host

| | |
| --- | --- |
| Benefits | Apparent reuse |
| Risks | T008: empty main/capabilities; no deps; macOS-skewed bundles; AFIA naming; not buildable; contradicts Windows-first |
| Compatibility | Spec disposition **replace** successor; archive non-authoritative |
| Draft disposition | **Rejected** as authority (intent may be investigated, not restored as-is) |

### Option D — Alternative native shell (e.g. Electron, custom Wry/WebView without Tauri)

| | |
| --- | --- |
| Benefits | Familiar web packaging (Electron) or lower-level control |
| Risks | Electron expands untrusted surface / weaker Rust-first fit; custom stack increases delivery risk vs Plan’s Tauri 2 assumption; no accepted founder decision selecting Electron |
| Compatibility | Weaker fit to Plan/research Tauri 2 path and ADR-15 Rust Core expectation |
| Draft disposition | **Rejected** for Alpha planning (may be re-opened only via new accepted decision) |

### Option E — Greenfield UI rewrite inside desktop shell (abandon `afia-ui`)

| | |
| --- | --- |
| Benefits | Clean composition |
| Risks | Violates Spec Q1 starting-shell constraint; endangers auth/session freeze |
| Compatibility | **Incompatible** with Q1 / ADR-03 direction |
| Draft disposition | **Rejected** |

---

## 6. Consequences

### Positive (prospective — if accepted at T030)

- Clear Core/WebView trust split for later Trusted Host and IPC ADRs
- Aligns planning with Spec Q3 and Plan R2 entry assumptions
- Enables strangling `afia-ui` presentation toward host-mediated authority without immediate UI rewrite

### Costs and negative consequences

- Requires building a successor desktop host not present in R0
- Windows WebView2 and installer complexity
- Must redesign away from archive macOS-oriented bundle assumptions (T008)
- Temporary dual paths (Vite-only vs future Tauri shell) until R2 cutover tasks

### Risks

| Risk | Evidence | Note |
| --- | --- | --- |
| Treating archive Tauri stub as reusable host | T008 | Replace via successor |
| Treating this draft as license to scaffold `apps/desktop` now | Tasks T013/T031 gates | Forbidden until T030 + R2 tasks |
| Premature updater/signing work | Decision D | Before R5 distribution controls |
| UI continuing direct worker calls after shell exists | T007; T010 C12 | ADR-02/06/03 must address strangler |
| ADR-15 still Proposed | T012 | Composition constrained by direction, not by accepted ADR-15 text until T030 |

---

## 7. Constraints and invariants

### Binding now (higher authority)

| Constraint | Source |
| --- | --- |
| Early minimal desktop Trusted Host after ADRs | Spec Q3 |
| Windows-first Alpha; not Windows-only architecture | Spec/Plan |
| `afia-ui` starting shell; auth freeze | Spec Q1/Q4; Plan |
| Decision A Rust-first direction | Plan (Ratified) |
| No production mutation from this ADR file | Tasks T013 |
| No R2 impl before required ADR acceptances | Plan; Tasks T030/T031 |

### Proposed invariants (architecture authority only if ADR-01 accepted at T030)

See §3.1.

---

## 8. Relationship to other ADRs

| ADR | Relationship |
| --- | --- |
| ADR-15 (Proposed) | Constrains Core language to Rust; does **not** by itself accept Tauri |
| ADR-02 (future draft T014) | Defines Trusted Host responsibility boundary inside the composition |
| ADR-03 | Strangler migration of `afia-ui` |
| ADR-06 | IPC/worker supervision transport details |
| ADR-13 | Packaging/updater/signing mechanics |
| T031 | First implementation of minimal Tauri shell — **only after T030** |

---

## 9. Unresolved questions

T013 attempted resolution: **NO**.

| ID | Question | Evidence | Future gate |
| --- | --- | --- | --- |
| U-ADR01-1 | Exact `apps/desktop` layout and crate split | Plan target tree; R0 missing live path | R2 tasks after T030 |
| U-ADR01-2 | Capability allowlist contents | T008 empty capabilities scaffold | ADR-02/14; R2 |
| U-ADR01-3 | How `afia-ui` is loaded (dev URL vs bundled assets) | T002 build facts | T031 / ADR-03 |
| U-ADR01-4 | Sidecar packaging for Python bridges | T007; research sidecar notes | ADR-06/09; ADR-13 |
| U-ADR01-5 | Windows WebView2 bootstrap/UX | Research notes | ADR-13 / R5 |
| U-ADR01-6 | Root manifest repair sequencing vs new desktop path | T010 C1–C2 | Adapt tasks after gates |
| U-ADR01-7 | Whether any archive config intent is reusable | T008 | Investigate only; not restore |

---

## 10. Validation and acceptance plan

```text
T013 completion produces a draft for review. T030 is the acceptance gate.
```

Before T030, planning reviews **may** include:

- Tier A independent platform ADR review (Tasks 001)
- Consistency with Spec Q3, Plan R2, and ADR-15 Proposed constraints
- R0 archive/Windows-first evidence check
- Security claim-language review (WebView untrusted)

**Not** validation for T013: creating desktop projects, `tauri build`, dependency installs, or CI greens.

**T030** remains the gate to mark ADR-01 (with ADR-15/02/06 as required) **Accepted** before R2 shell/host implementation (T031+).

---

## 11. Gate

**ADR-01 MUST be accepted at T030 (with the required R1 ADR package) before R2 desktop implementation.**

Until that acceptance, this file remains:

```text
PLANNING EVIDENCE — NON-AUTHORITATIVE ADR DRAFT
```

---

## 12. References

### Accepted authority

- Constitution v1.0.0 (`.specify/memory/constitution.md`)
- [Specification 001](../spec.md) — Q1, Q3
- [Plan 001](../plan.md) — ADR roadmap; R2 entry; Windows-first
- [Tasks 001](../tasks.md) — T013; T030; T031

### Planning evidence (non-architecture-authority)

- [ADR-15](./ADR-15-rust-first-polyglot-runtime.md) — Proposed language authority draft
- [research.md](../research.md) — R4; desktop Tauri 2 notes
- [contracts/trusted-host-ipc.md](../contracts/trusted-host-ipc.md) — planning transport assumptions

### R0 factual evidence

- [R0-evidence-package.md](../../../docs/program-memory/baseline/R0-evidence-package.md) — R0 close `f37563d8f8499aa24e8ead68fa920167ebb8cde6`
- [R0-archived-assets.md](../../../docs/program-memory/baseline/R0-archived-assets.md) — T008 desktop/Tauri archive
- [R0-path-classification.md](../../../docs/program-memory/baseline/R0-path-classification.md) — T010 `2c9fb55244588bb35b2d08bc1e1c6bbcbadbd8ec`
- [R0-frontend-map.md](../../../docs/program-memory/baseline/R0-frontend-map.md)
- [R0-python-services.md](../../../docs/program-memory/baseline/R0-python-services.md)
- [R0-ci-tests.md](../../../docs/program-memory/baseline/R0-ci-tests.md)
- [R0-build-commands.md](../../../docs/program-memory/baseline/R0-build-commands.md)

### Explicitly non-authoritative

- `_archived/apps-desktop/**`
- Legacy `docs/product/AFIA_*`
- Graphify output
- AI recommendations
