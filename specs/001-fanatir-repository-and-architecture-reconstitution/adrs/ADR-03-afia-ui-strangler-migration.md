# ADR-03 — afia-ui Strangler Migration

| Field | Value |
| --- | --- |
| **ADR** | ADR-03 |
| **Title** | afia-ui Strangler Migration |
| **Status** | **Proposed** (draft only) |
| **Task origin** | T015 |
| **Required specification gate** | Spec 001 **Q1** (founder-ratified) |
| **Related planning drafts** | [ADR-01](./ADR-01-platform-and-desktop-composition-tauri-2.md) (Proposed); [ADR-15](./ADR-15-rust-first-polyglot-runtime.md) (Proposed); [ADR-02](./ADR-02-rust-trusted-host-boundary.md) (Proposed) |
| **Architecture authority of this file** | **NO** — until a separately valid acceptance action |
| **T030 note** | ADR-03 is **not** in the mandatory T030 Accepted set (ADR-15/01/02/06). T030 may review R3-blocking ADRs; R2 entry does **not** require ADR-03 Accepted. |
| **Implementation authorization** | **NO** |
| **Migration authorization** | **NO** |
| **Feature** | `001-fanatir-repository-and-architecture-reconstitution` |

```text
This document is a planning draft and is not accepted architecture authority.
```

```text
Completed ADR drafting task ≠ architecture acceptance
T015 completion ≠ T030 acceptance
T015 completion ≠ migration execution
ADR-01 / ADR-02 / ADR-15 remain Proposed and are not accepted through ADR-03
```

```text
No production implementation is authorized by this draft.
No migration mutation of afia-ui or production code is authorized by this draft.
```

This draft **must not** be used as justification to: modify `afia-ui` routes, auth, session, or UI behavior; perform file moves/deletions for migration; create adapters in production trees; begin R2; accept ADR-01/02/15; change authentication/session/`PrivateRoute`/profile behavior; fork/import OpenMed or Graphify; activate Pictorial/Montada; initialize Fehrest beyond planning; implement DeepMed product work; or otherwise execute strangler stages.

---

## 1. Context

### 1.1 Assigned architectural question

**ADR-03 proposes** how Fanatir would **incrementally adapt** the existing `afia-ui` codebase as a **migration starting shell** under a governed strangler strategy (Spec Q1 / Q7), without a full rewrite and without treating `afia-ui` as architecture source of truth.

It does **not** authorize migration execution, UI redesign, auth/session behavior change, host implementation, IPC schemas, Artifact Store design, Fehrest product initialization, or DeepMed implementation.

### 1.2 Accepted program constraints

| Constraint | Source | Treatment |
| --- | --- | --- |
| `afia-ui` is the migration starting point and reusable shell; Constitution/ADRs remain architectural authority | Spec 001 Q1 | Binding |
| Full rewrite unauthorized; incremental adaptation authorized **after planning** | Spec 001 Q1 | Binding |
| Preserve auth/session/`PrivateRoute`/profile until dedicated migration spec | Spec 001 Q1/Q4; Plan | Binding freeze |
| No `ProfileGate` invention | Spec 001 | Binding |
| Incremental vertical-slice / strangler migration; not big-bang | Spec 001 Q7 | Binding strategy |
| Shell = UI/presentation only; rewrite unauthorized | Plan ADR-03 roadmap | Binding planning direction |
| React/TypeScript = UI/presentation only (language direction) | Decision A / ADR-15 proposes | Binding **direction**; ADR-15 still **Proposed** |
| Platform composition / Trusted Host boundaries | ADR-01 / ADR-02 propose | Proposed planning context only |
| No OpenMed/Graphify fork/import; no Pictorial/Montada; no Go Alpha dependency; no Rust rewrite of OpenMed/Graphify/Jupyter/R | Spec/Plan/Tasks T015 | Binding prohibitions |
| No R2 code before T030 | Tasks Phase R1 | Future-gate requirement |

### 1.3 R0 inspected facts (evidence only)

| Fact | Evidence | Treatment |
| --- | --- | --- |
| Active frontend is Vite/React under `afia-ui/` | T004; Spec inventory | Implementation fact |
| Auth/`PrivateRoute`/`AuthContext` present as verified-active wiring | Spec inventory; T004 | Implementation fact; behavior freeze |
| Live UI calls OpenMed at `http://127.0.0.1:8765` without Trusted Host mediation | T007; T010 C5/C12 | Transitional fact; target zero-authority UI contradicts this seam |
| No live `apps/desktop` Trusted Host wrapping `afia-ui` | T008; T010 | Fact |
| Archived desktop UI under `_archived/apps-desktop/ui` is not the migration shell | T004; T008 | Non-authoritative; do not revive as authority |
| Dual AFIA/Fanatir identity remnants in UI packaging/titles | Spec disposition; T010 C10 | Fact; rename deferred (ADR-12) |

R0 baseline evidence is authoritative **only as a record of inspected facts**.

### 1.4 Name and product boundaries

| Name | Meaning in this draft |
| --- | --- |
| `afia-ui` | Existing repository path / migration-starting UI shell (package name remnants may still say `afia`) |
| Fanatir | Target product/repository architecture authority context |
| Fehrest | Separate product/subsystem (`IamShehri/Fehrest`); integration rules → **ADR-08**; not initialized by this ADR |

---

## 2. Decision drivers

| Driver | Source |
| --- | --- |
| Spec Q1 starting-shell + no full rewrite | Spec 001 |
| Spec Q7 strangler / vertical-slice stages | Spec 001 |
| Preserve verified useful journeys while adapting | Spec disposition matrix |
| UI must not own privileged authorities | Spec/Plan; ADR-01/02 propose |
| Auth/session freeze until dedicated spec + ADR-11 path | Spec/Plan |
| Avoid uncontrolled feature-by-feature replacement | Spec Q7 stage discipline |
| Avoid silent ownership transfer / duplicate live authorities | Spec Q7 prohibitions/evidence |

---

## 3. Proposed decision

**ADR-03 proposes** a **governed strangler migration** for `afia-ui`:

1. Retain `afia-ui` as the **presentation/migration shell**.
2. Progressively introduce Fanatir-owned replacements and host-mediated seams **behind explicit boundaries**.
3. Migrate **one bounded surface at a time** only after planning gates and separately authorized implementation tasks.
4. Retire legacy ownership of a surface only after evidence, acceptance, and authorization — never by silent cutover.

### 3.1 Migration intent (proposed)

| Intent | Proposed meaning |
| --- | --- |
| Starting shell | `afia-ui` is where migration begins |
| Not architecture SoT | Constitution/ADRs (when accepted) govern architecture; `afia-ui` code is implementation evidence |
| Incremental | Adapt in slices; do not rewrite wholesale |
| Behavior preservation | Preserve user-visible auth/session/`PrivateRoute`/profile and verified journeys unless a dedicated specification later authorizes change |
| Fanatir target | New ownership and presentation adaptations align toward Fanatir product architecture without mass `afia*` rename in this ADR (rename → ADR-12) |

### 3.2 Strangler boundary classes (proposed)

| Class | Meaning | Examples (planning-level) |
| --- | --- | --- |
| **Retain (behavior)** | Keep behavior unchanged during reconstitution | Auth/session/`PrivateRoute`/profile contracts until dedicated migration spec |
| **Adapt (presentation)** | Incremental UI/presentation adaptation after planning | Routed pages, shell chrome, Studio presentation — only under later authorized tasks |
| **Replace behind seam** | Introduce Fanatir-owned mediation while legacy path remains temporarily | Future host-mediated FS/secrets/workers/Artifact writes (depends on Proposed ADR-01/02 and later ADRs) |
| **Defer** | Explicitly not migrated by this ADR | Mass rename (ADR-12); Fehrest init (ADR-08); DeepMed product impl (ADR-09); packaging (ADR-13) |
| **Prohibit** | Must not occur under this program unless a later accepted spec says otherwise | Full rewrite; OpenMed/Graphify fork/import; Pictorial/Montada; auth behavior change without ADR-11 + dedicated spec; Go Alpha dependency; Rust rewrite of OpenMed/Graphify/Jupyter/R; production migration mutation from this draft |

### 3.3 UI authority (proposed)

If ADR-03 is later accepted, `afia-ui` / WebView presentation code **would** remain a **requestor/presenter**:

| May (proposed) | Must not (proposed) |
| --- | --- |
| Render UI; collect intent; display results | Own privileged host actions |
| Call host-mediated APIs when those APIs exist and are authorized | Own security policy decisions |
| Preserve frozen auth/session UX until a dedicated migration spec | Own filesystem/process/secrets/unrestricted network authority |
| | Establish authorization outcomes |

This draft does **not** accept ADR-01 or ADR-02. It references them only as **Proposed** planning dependencies for composition and Trusted Host boundary.

### 3.4 Authentication and session preservation (binding freeze)

The strangler **must not** change auth/session/`PrivateRoute`/profile behavior unless **all** of the following later occur:

1. ADR-11 exists and reaches the required acceptance state;
2. a dedicated specification authorizes the behavior change;
3. applicable review and founder gates complete;
4. implementation is **separately** authorized.

T015 / ADR-03 alone authorize **none** of those steps.

### 3.5 Product and feature exclusions (binding)

Preserved from Tasks 001 T015 / Spec / Plan:

- no OpenMed fork or import;
- no Graphify fork or import;
- no Pictorial implementation;
- no Montada implementation;
- no Go Alpha dependency;
- no production migration mutation from this draft;
- no Rust rewrite of OpenMed, Graphify, Jupyter, or R-based functionality;
- no Fehrest initialization beyond planning (Fehrest integration → ADR-08);
- no DeepMed product implementation under this draft (→ ADR-09 / later specs).

### 3.6 Migration invariants (proposed — if ADR-03 later accepted)

| Proposed invariant |
| --- |
| Preserve user-visible behavior unless a separate approved specification changes it |
| Keep changes incremental and reversible at planning/execution gates |
| No silent ownership transfers |
| Each migrated surface has an explicit authoritative owner |
| No duplicate live authorities for the same privileged responsibility |
| Compatibility boundaries remain explicit during transition |
| Legacy behavior retirement requires evidence + separate authorization |
| Fail closed when authority or ownership is ambiguous |

### 3.7 Planning-level migration stages (not execution authorization)

Conceptual sequence adapted from Spec Q7 / Plan strangler posture:

1. **Inventory and classify** existing `afia-ui` surfaces (routes, auth, Studio, collab, bridges) — evidence first.
2. **Establish compatibility seams** (planning/contracts; later authorized adapters only).
3. **Introduce Fanatir-owned replacements** behind explicit boundaries (host-mediated paths when R2+ tasks authorize).
4. **Migrate one bounded surface at a time** under task authorization.
5. **Verify** behavioral and security expectations with evidence required by those tasks.
6. **Retire** legacy ownership only after separately authorized evidence and acceptance.

These stages are **planning structure**. This ADR does **not** create tasks, timelines, or permission to execute them.

### 3.8 What this ADR does **not** settle

| Deferred matter | Owner |
| --- | --- |
| Desktop composition (Tauri 2) | **ADR-01** (Proposed) |
| Trusted Host privilege boundary | **ADR-02** (Proposed) |
| Language authority details | **ADR-15** (Proposed) |
| Auth/session behavior change design | **ADR-11** + dedicated migration spec |
| Shared primitives | **ADR-04** |
| Artifact Store | **ADR-05** |
| IPC/worker contracts | **ADR-06** |
| Supabase adapter / Decision C | **ADR-07** / Spec 002 |
| Fehrest integration | **ADR-08** |
| DeepMed / OpenMed runtime boundary | **ADR-09** |
| commandF | **ADR-10** |
| Technical rename AFIA→Fanatir | **ADR-12** |
| Packaging/signing | **ADR-13** |
| PHI / classification enforcement detail | **ADR-14** |
| Exact route-by-route cutover order | Later authorized tasks |
| How `afia-ui` is loaded in a future desktop shell | T031 / related tasks after T030 |

---

## 4. Scope

### In scope

- Strangler migration strategy for `afia-ui` as starting shell
- Surface classification (retain / adapt / replace-behind-seam / defer / prohibit)
- UI presentation-authority posture at planning level
- Auth/session freeze conditions
- Planning-level stages and invariants
- Explicit exclusions

### Out of scope

- Any code change under `afia-ui/**`, `lib/**`, `services/**`, `apps/**`
- Archive reactivation or deletion
- R2 Trusted Host / Tauri implementation
- Auth/session behavior redesign
- Production migration mutation
- Acceptance of ADR-15/01/02/06 (T030) or of this ADR

---

## 5. Options considered

### Option A — Governed strangler / incremental adaptation of `afia-ui` (proposed)

| | |
| --- | --- |
| Benefits | Matches Spec Q1/Q7; preserves journeys and auth freeze; allows progressive Fanatir ownership |
| Costs | Temporary duplication; compatibility burden; longer calendar vs rewrite fantasy |
| Compatibility | Required by Spec Q1/Q7 |
| Draft disposition | **Proposed** |

### Option B — Full immediate rewrite of the UI

| | |
| --- | --- |
| Benefits | Clean slate narrative |
| Costs / risks | Violates Spec Q1; endangers auth/session freeze; high delivery risk |
| Compatibility | **Incompatible** with Q1 |
| Draft disposition | **Rejected** |

### Option C — Continue indefinitely on unmodified `afia-ui` as architecture SoT

| | |
| --- | --- |
| Benefits | Zero near-term migration cost |
| Costs / risks | Leaves UI-direct privileged seams (e.g. OpenMed localhost) unstrangled; conflicts with host/UI-zero-authority direction |
| Compatibility | Conflicts with Q1 “not architecture authority” and Plan host direction |
| Draft disposition | **Rejected** as end-state |

### Option D — Uncontrolled feature-by-feature replacement without stages/evidence

| | |
| --- | --- |
| Benefits | Apparent speed |
| Costs / risks | Silent ownership transfer; duplicate authorities; unverifiable cutovers |
| Compatibility | Conflicts with Spec Q7 stage/evidence/rollback discipline |
| Draft disposition | **Rejected** |

### Option E — Revive archived desktop UI / fork prohibited external UIs as the shell

| | |
| --- | --- |
| Benefits | Apparent reuse |
| Costs / risks | Archived UI non-authoritative (T008); OpenMed/Graphify fork/import prohibited |
| Compatibility | Conflicts with Q1 starting-shell and T015 prohibitions |
| Draft disposition | **Rejected** |

---

## 6. Consequences

### Positive (prospective — if later accepted and executed under separate authorization)

- Clear migration posture for `afia-ui` without rewrite
- Preserves auth/session contracts during reconstitution planning
- Enables progressive strangling of privileged UI seams toward host-mediated paths
- Keeps Fanatir architecture authority distinct from shell code

### Costs and tradeoffs

- Temporary dual paths (legacy presentation vs future host-mediated seams)
- Compatibility and adapter burden (later R4 tasks; not authorized here)
- Testing/evidence obligations before legacy retirement
- Documentation/ownership tracking overhead

### Risks

| Risk | Note | Mitigation (planning constraint only) |
| --- | --- | --- |
| Draft used as license to edit `afia-ui` now | T015 prohibited | Forbidden until separate implementation authorization |
| Silent auth/session drift | Spec freeze | Require ADR-11 + dedicated spec + gates |
| Duplicate authorities (UI + host) | T007 OpenMed seam | Fail closed; explicit owner per surface |
| Treating Proposed ADR-01/02 as accepted | T013/T014 | Keep Proposed labels; T030 separate |
| Uncontrolled cutovers | Spec Q7 | One surface at a time; evidence before retire |
| Name confusion (`afia-ui` vs Fanatir vs Fehrest) | Dual identity | Keep identifiers distinct; rename → ADR-12 |

Prospective benefits are **not** measured outcomes. Feasibility, security equivalence, and auth compatibility are **not** claimed as proven by this draft.

---

## 7. Constraints and invariants

### Binding now (higher authority)

| Constraint | Source |
| --- | --- |
| Q1 starting shell; no full rewrite | Spec 001 |
| Q7 strangler strategy | Spec 001 |
| Auth/session/`PrivateRoute`/profile freeze | Spec/Plan |
| No production mutation from this ADR | Tasks T015 |
| No R2 before T030 | Tasks Phase R1 |

### Proposed invariants

See §3.6 — architecture authority only if ADR-03 is later accepted by a valid acceptance action.

---

## 8. Relationship to other ADRs

| ADR | Relationship |
| --- | --- |
| ADR-15 (Proposed) | UI remains presentation-only under proposed language model |
| ADR-01 (Proposed) | Composition would load/adapt `afia-ui` as WebView shell |
| ADR-02 (Proposed) | Privileged authorities strangle into Trusted Host, not UI |
| ADR-11 | Auth/session change path (required before behavior change) |
| ADR-07 / Spec 002 | Supabase adapter / local-first content boundary |
| ADR-08 | Fehrest integration (not Fehrest init here) |
| ADR-09 | DeepMed / OpenMed runtime boundary |
| ADR-12 | Technical rename strategy |
| ADR-04/05/06/10/13/14 | Primitives, Artifact Store, IPC, commandF, packaging, PHI detail |

---

## 9. Unresolved questions

T015 attempted resolution: **NO**.

| ID | Question | Evidence | Future owner | Must resolve before migration implementation? |
| --- | --- | --- | --- | --- |
| U-ADR03-1 | Exact surface inventory priority order for first slice | Spec routes; R0 frontend map | Later authorized tasks | YES |
| U-ADR03-2 | Compatibility seam patterns (adapter location/ownership) | Plan R4; Spec Q7 | Later tasks; not this draft | YES before adapter code |
| U-ADR03-3 | How presentation loads inside future desktop shell | ADR-01 Proposed; U-ADR01-3 | T031+ after T030 | YES before desktop cutover |
| U-ADR03-4 | When UI-direct OpenMed calls are retired | T007; ADR-02/06/09 Proposed | ADR-06/09 + tasks | YES before claiming UI-zero-authority |
| U-ADR03-5 | Auth migration spec scope if ever needed | Spec freeze | ADR-11 + dedicated spec | Only if behavior change sought |
| U-ADR03-6 | Dual AFIA/Fanatir identity UX during strangler | Spec/T010 | ADR-12 | Can remain open in draft stage |
| U-ADR03-7 | Evidence standard for “behavioral equivalence” | Spec Q7 | Later verification tasks | YES before legacy retirement |
| U-ADR03-8 | Whether Express `afia-ui/server` remains transitional indefinitely | Plan | Later ADRs/tasks | Can remain open in draft stage |

---

## 10. Validation and acceptance plan

```text
T015 completion produces a draft for Tier C review. Draft ≠ accepted.
```

Before any migration implementation:

- Tier C normal verification of this draft (Tasks 001);
- Consistency with Spec Q1/Q7 and Plan ADR-03 roadmap;
- Confirmation ADR-01/02/15 remain labeled Proposed when cited;
- Separate founder acceptance of the draft work product (task field: draft only);
- Separately valid acceptance action before ADR-03 becomes architecture authority;
- Separately authorized implementation tasks before any `afia-ui` mutation.

**Not** validation for T015: editing `afia-ui`, running migration cutovers, installing dependencies, or claiming auth compatibility tests passed.

**T030** remains the R1 gate to Accept ADR-15/01/02/06 for R2 entry. ADR-03 Accepted is **not** required for that R2 entry package. T031+ remains separately gated.

---

## 11. Gate

Until a separately valid acceptance action occurs, this file remains:

```text
PLANNING EVIDENCE — NON-AUTHORITATIVE ADR DRAFT
```

```text
ADR-03 must remain Proposed until a separately valid acceptance action.
T015 does not authorize migration execution or production implementation.
T030 completion is not claimed by this draft.
R2 is not authorized by this draft.
```

---

## 12. References

### Accepted authority

- Constitution v1.0.0 (`.specify/memory/constitution.md`)
- [Specification 001](../spec.md) — Q1; Q7; auth freeze; disposition of `afia-ui`
- [Plan 001](../plan.md) — ADR-03 roadmap; strangler UI; presentation-only shell
- [Tasks 001](../tasks.md) — T015; Phase R1; T030; review tiers

### Planning evidence (non-architecture-authority)

- [ADR-15](./ADR-15-rust-first-polyglot-runtime.md) — Proposed language authority draft
- [ADR-01](./ADR-01-platform-and-desktop-composition-tauri-2.md) — Proposed platform composition draft
- [ADR-02](./ADR-02-rust-trusted-host-boundary.md) — Proposed Trusted Host boundary draft
- [research.md](../research.md) — planning notes only

### R0 factual evidence

- [R0-evidence-package.md](../../../docs/program-memory/baseline/R0-evidence-package.md) — R0 close `f37563d8f8499aa24e8ead68fa920167ebb8cde6`
- [R0-frontend-map.md](../../../docs/program-memory/baseline/R0-frontend-map.md)
- [R0-path-classification.md](../../../docs/program-memory/baseline/R0-path-classification.md) — T010 `2c9fb55244588bb35b2d08bc1e1c6bbcbadbd8ec`
- [R0-python-services.md](../../../docs/program-memory/baseline/R0-python-services.md)
- [R0-archived-assets.md](../../../docs/program-memory/baseline/R0-archived-assets.md)

### Explicitly non-authoritative

- `_archived/apps-desktop/ui/**`
- Legacy `docs/product/AFIA_*`
- Graphify output
- AI recommendations
- Source code beyond implementation facts it demonstrates
