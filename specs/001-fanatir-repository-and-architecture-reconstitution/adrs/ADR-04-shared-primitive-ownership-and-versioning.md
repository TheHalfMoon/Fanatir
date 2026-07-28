# ADR-04 — Shared Primitive Ownership and Versioning

| Field | Value |
| --- | --- |
| **ADR** | ADR-04 |
| **Title** | Shared Primitive Ownership and Versioning |
| **Status** | **Reviewed** (T030 R1 architecture gate; not Accepted) |
| **Task origin** | T016 |
| **Acceptance / review posture** | Tier A independent review after drafting; **T030** expects ADR-04 listed **Reviewed or Accepted** (not automatically Accepted; not in the mandatory Accepted set that opens R2) |
| **Feature** | `001-fanatir-repository-and-architecture-reconstitution` |
| **Constraining ADRs** | [ADR-15](./ADR-15-rust-first-polyglot-runtime.md) (Accepted at T030) |
| **Planning evidence** | [shared-primitives.md](../contracts/shared-primitives.md); [data-model.md](../data-model.md); [research.md](../research.md) P2; [plan.md](../plan.md) ADR roadmap |
| **T030 founder decision** | **Reviewed** — R1 architecture gate; not Accepted; schema publication / shared-package implementation remain later-gated |
| **Architecture authority of this file** | **NO** — Reviewed at T030; Accepted required before implementation domains that gate on this ADR |
| **Implementation authorization** | **NO** |

```text
Status: Reviewed (T030 R1 architecture gate; not Accepted)
Architecture authority: NO (Reviewed ≠ Accepted)
Implementation authorization: NO
Independent Tier A — R1 architecture gate: Pending
```

```text
Draft ADR ≠ architecture acceptance
T016 completion ≠ T030 acceptance (T030 founder Reviewed decision now recorded)
Passing Tier A review of a draft ≠ Accepted
Reviewed ≠ Accepted for schema publication or shared-package implementation
ADR-03 remains Proposed
```

```text
This draft is not:
- production schema publication
- shared-package implementation
- runtime-contract migration
- permission to change IPC
- permission to change persistence
- permission to change authentication or sessions
- permission to begin R2
```

```text
No production implementation is authorized by this draft.
```

This draft **must not** be used as justification to: create or publish `packages/contracts`; finalize or mutate production schemas; generate or ship bindings; change IPC envelopes; change Artifact Store / persistence formats; modify Fehrest or DeepMed runtime contracts; alter `afia-ui` types as architecture authority; change authentication/session behavior; begin R2 (`T031+`); or treat Proposed ADRs as Accepted.

---

## 1. Context and problem

### 1.1 Assigned architectural question

**ADR-04 proposes** that **Fanatir** owns canonical, **language-neutral** shared primitive definitions and their **schema versions**, so that UI, Trusted Host, workers/services, Fehrest, and DeepMed (where program evidence anticipates shared consumption) can exchange stable meanings without each boundary inventing incompatible semantics.

It does **not** finalize production fields, choose code-generation tooling as a settled platform standard, define IPC command schemas (**ADR-06**), define Artifact Store persistence (**ADR-05**), define Fehrest integration behavior (**ADR-08**), or define authentication/session architecture (**ADR-11**).

### 1.2 Why cross-boundary ownership matters

Fanatir reconstitutes a polyglot system: React/`afia-ui` presentation; proposed Rust Trusted Host; supervised workers; and separate products/subsystems such as **Fehrest** and DeepMed that consume shared meanings across language and repository boundaries ([plan.md](../plan.md); [shared-primitives.md](../contracts/shared-primitives.md)).

Without explicit ownership and version governance, planning evidence identifies these **risks** (prospective — not claimed as currently measured production failures unless separately evidenced):

| Risk | Why it matters |
| --- | --- |
| Duplicated semantic authority | Multiple repos redefine “Artifact”, “Patient”, “Run”, etc. |
| Incompatible serialization | JSON shapes diverge silently across TS/Rust/Python consumers |
| Silent contract drift | Adapters “fix” payloads without a governed version bump |
| Consumer-specific redefinition | Fehrest or UI types become accidental SoT |
| Uncoordinated breaking changes | One consumer ships removals/renames others cannot parse |
| Implementation types as architecture | Rust structs, TS interfaces, ORM rows, or DB DDL become de-facto contracts |

R0 and current planning docs record that shared primitive schemas were **deferred** to later ADRs/specifications ([spec.md](../spec.md) unresolved remainder #2). This ADR proposes ownership/versioning rules for that later work; it does **not** claim schemas, bindings, or compatibility suites already exist as production artifacts.

### 1.3 Accepted program and planning constraints

| Constraint | Source | Treatment |
| --- | --- | --- |
| Language-neutral shared primitives; Fanatir owns canonical definitions (planning research) | [research.md](../research.md) P2 | Planning research — constrains this draft’s proposed direction; **not** ADR acceptance |
| Semver + `$id`; additive OK; removals/renames require major + ADR | [shared-primitives.md](../contracts/shared-primitives.md); research P2 | Draft planning contract evidence |
| Interface boundaries only — not finalized production schemas | [data-model.md](../data-model.md) | Binding planning scope limit |
| Durable Artifact/Revision/Run mutation via Rust Trusted Host direction | Decision A / [ADR-15](./ADR-15-rust-first-polyglot-runtime.md) (Proposed) | Proposed language direction; not Accepted |
| ADR-04 before ADR-05 sequencing | [tasks.md](../tasks.md) blocking matrix; [plan.md](../plan.md) | Drafting/sequencing constraint |
| T016 = draft only; Tier A review; no production code | [tasks.md](../tasks.md) T016 | Binding for this task |
| No R2 before T030 (ADR-15/01/02/06 Accepted as applicable) | Tasks Phase R1/R2 | Future-gate requirement |

### 1.4 Naming

| Name | Role in this ADR |
| --- | --- |
| **Fanatir** | Canonical owner of shared primitive **definitions** and **schema versions** |
| **Fehrest** | Consumer of versioned contracts where applicable; **not** canonical shared-contract owner; **not** Patient clinical SoT ([data-model.md](../data-model.md)) |
| **`afia-ui`** | Existing migration shell / UI presentation surface; **not** canonical shared-contract owner ([ADR-03](./ADR-03-afia-ui-strangler-migration.md) Proposed) |

Never confuse Fehrest with Fanatir, or `afia-ui` with architecture SoT.

### 1.5 What this draft is not

This file is **not**: production schema publication; shared-package implementation; runtime-contract migration; IPC change authority; persistence change authority; authentication/session change authority; or R2 authorization.

---

## 2. Decision drivers

| Driver | Source |
| --- | --- |
| Cross-repo contract drift risk | research P2 alternatives rejected |
| Polyglot consumers (TS/Rust/Python) | ADR-15 Proposed; plan packages/contracts target |
| PHI / classification on durable entities | data-model.md; Constitution posture via Spec/Plan |
| Keep IPC, persistence, Fehrest, auth as separate ADRs | plan ADR roadmap; ADR-15 deferrals |
| Draft ≠ accepted; Tier A for shared primitives | tasks.md T016 |

---

## 3. Proposed decision

### 3.1 Decision statement

**ADR-04 proposes** that:

1. **Fanatir owns** canonical, language-neutral **shared primitive** semantic definitions and their **schema identities / schema versions**.
2. **Consumers** (including Fanatir UI/`afia-ui` adapters, Trusted Host bindings, workers, **Fehrest**, and DeepMed where integration uses shared primitives) **may** create local bindings and adapters, but **must not** independently redefine canonical semantics.
3. **Schema versions are independent** of package versions, application versions, IPC/protocol versions, and persistence-format versions.
4. **Serialization planning baseline** (from existing planning contracts): UTF-8 JSON; stable `$id`; explicit `schemaVersion` (semver); language-specific bindings are **consumers** of schemas ([shared-primitives.md](../contracts/shared-primitives.md); research P2).
5. **Breaking changes** require a major `schemaVersion` bump and a governed architecture decision (ADR or ADR amendment) ([shared-primitives.md](../contracts/shared-primitives.md); research P2).
6. Shared-contract ownership **does not** grant authority over privileged host actions, IPC transport, persistence architecture, authentication/sessions, or product-specific integrations.

```text
Proposed ownership ≠ implemented packages/contracts
Proposed versioning ≠ live version negotiation
Planning inventory ≠ approved production schemas
```

### 3.2 Distinguishing version and ownership domains

| Domain | Owner / meaning (proposed) | Must not be collapsed into |
| --- | --- | --- |
| **Semantic ownership** | Fanatir defines meaning of each shared primitive | Consumer local nicknames |
| **Schema ownership** | Fanatir owns canonical schema documents / `$id` | Generated binding files |
| **Schema / contract version** | Fanatir-assigned `schemaVersion` (semver) | npm/crate app semver |
| **Package version** | Distribution unit version (if later authorized) | Schema meaning |
| **Application version** | Product/release version (Fanatir, Fehrest, etc.) | Schema meaning |
| **IPC / protocol version** | Deferred to **ADR-06** | Shared primitive schemaVersion |
| **Persistence-format version** | Deferred to **ADR-05** / store design | Shared primitive schemaVersion |
| **Generated bindings** | Consumer artifacts derived from schemas | Canonical semantics |
| **Consumer adapters** | Local mapping layers | Canonical semantics |

### 3.3 Shared primitive definition

A **shared primitive** (proposed) is a **language-neutral, cross-boundary contract** representing a stable **domain**, **evidence**, **authorization-description**, or **policy** concept intended for reuse across Fanatir surfaces and sibling consumers.

It is **not**:

| Not a shared primitive | Why |
| --- | --- |
| UI view models | Presentation-shaped; `afia-ui`-local |
| Database rows / ORM entities | Persistence-shaped; not language-neutral contracts |
| Rust structs as SoT | Implementation binding; durable mutation authority is separate (ADR-15/02) |
| TypeScript-only interfaces as SoT | Language-local; cannot be canonical for Python/Rust consumers |
| IPC commands / events | Transport/protocol → **ADR-06** |
| Persistence schemas / DDL | Store format → **ADR-05** / later specs |
| Internal DTOs | Private to one module/process |
| Framework-specific objects | Coupled to React/Tauri/ORM frameworks |

### 3.4 Canonical ownership responsibilities (proposed)

Fanatir, as proposed owner, would be responsible for:

| Responsibility | Notes |
| --- | --- |
| Semantic definitions | Canonical meaning of each primitive |
| Schema identities | Stable `$id` (planning baseline) |
| Contract / schema versions | Assign and publish `schemaVersion` under later authorized tasks |
| Compatibility policy | Additive vs breaking classification |
| Breaking-change approval | Major bump + governed ADR/amendment |
| Deprecation policy | Explicit notice; no silent removal |
| Documentation | Human-readable contract docs |
| Conformance expectations | What consumers must validate/reject |
| Consumer-impact assessment | Inventory before breaks |
| Migration planning | Separately authorized tasks execute migrations |

Ownership of shared contracts **does not** authorize privileged Trusted Host actions, IPC schemas, Artifact Store design, Fehrest product behavior, or auth/session changes.

### 3.5 Minimum planning inventory

The following is a **planning inventory** only, taken from [shared-primitives.md](../contracts/shared-primitives.md) and [data-model.md](../data-model.md). It is **not** a finalized production schema set. Core fields listed in data-model.md are **boundary sketches**, not approved cardinalities/validation rules.

| Primitive | Planning purpose (summary) |
| --- | --- |
| Workspace | Optional collaboration/tenancy boundary |
| Project | Local-first unit of work (host-mediated open/create) |
| Patient | Fanatir patient-scoped clinical continuity identity |
| Artifact | Durable content object with provenance |
| Revision | Immutable Artifact version |
| Run | Execution record for tools/models/workers |
| Source | Original or cited evidence object |
| Relationship | Typed link with distinguishable origin |
| Review | Human review of AI/transformed results |
| Approval | Explicit human approval for share/export/handoff |
| Decision | Recorded product/architecture/runtime decision |
| PolicyDecision | Policy/capability gateway outcome **as data** |
| DataClassification | Sensitivity label enumeration |
| Capability | Named privileged action **as data description** |
| ExportManifest | Portable export inventory |

**Do not** treat data-model field lists as production-approved. Unresolved schema detail remains in §11.

### 3.6 Schema representation (proposed baseline)

Where supported by existing planning evidence, this ADR **proposes**:

| Element | Proposal |
| --- | --- |
| Schema language | Language-neutral **JSON Schema** (preferred under `contracts/` per data-model.md) |
| Serialization | UTF-8 JSON |
| Identity | Stable `$id` |
| Version field | Explicit `schemaVersion` (semver) |
| Bindings | Language-specific bindings are **consumers**; hand-maintained bindings may be acceptable for Alpha **if** later authorized tasks require compatibility evidence (research P2) |

**Generated binding tooling selection** remains an unresolved question (explicit non-goal of current shared-primitives.md). This ADR does **not** select final tooling.

### 3.7 Versioning policy (proposed)

1. **Fanatir assigns** canonical schema versions.
2. **Consumers declare or pin** supported schema versions (mechanism unresolved — §11).
3. **Package versions do not redefine** contract/`schemaVersion` meaning.
4. **Application versions do not redefine** contract/`schemaVersion` meaning.
5. **Semantic changes are breaking** even when syntax remains structurally valid JSON.
6. **Breaking changes** require a **major** `schemaVersion` and a governed architecture decision (ADR or amendment).
7. **Compatibility must be demonstrated** under separately authorized verification — not assumed. This draft does **not** claim production compatibility tests already exist.
8. **Unsupported incompatible versions fail closed** at trust boundaries (planning requirement for later implementation).

Architecture-level change classes (illustrative classification — not an implemented classifier):

| Change class | Typical version impact (proposed) |
| --- | --- |
| Additive optional fields | Minor / compatible if consumers ignore unknowns under agreed policy |
| Required-field additions | Breaking (major) unless a governed compatibility window exists |
| Field removal | Breaking (major) + deprecation path |
| Field rename | Breaking (major) |
| Type narrowing | Breaking (major) |
| Enum expansion | Often compatible if consumers tolerate unknown members; must be verified |
| Default changes | Often semantic break even if syntax compatible |
| Semantic reinterpretation | Breaking regardless of syntax |

Unknown additive field handling is unresolved (§11).

### 3.8 Deprecation (proposed)

Deprecation **would require**:

- explicit deprecation notice;
- documented replacement;
- consumer inventory;
- migration evidence under separately authorized tasks;
- a governed support window (**duration not invented here** — no fixed calendar period is defined by canonical sources);
- **no silent removal**;
- an ADR or equivalent governed decision for breaking removal.

### 3.9 Consumer responsibilities (planning language only)

Consumers **should** (when later authorized to implement):

- declare supported schema versions;
- validate at trust boundaries;
- reject unsupported versions (fail closed);
- avoid redefining canonical semantics;
- keep adapters local;
- expose compatibility evidence;
- migrate only under separately authorized tasks.

This remains **planning language**, not implementation authorization.

### 3.10 Security and privacy (claim language)

| Topic | Proposed posture |
| --- | --- |
| PHI-bearing primitives | Must carry / respect `DataClassification`; PHI must not default to cloud adapters ([data-model.md](../data-model.md)) |
| Minimum necessary transfer | Cross-boundary payloads should avoid unnecessary PHI fields |
| Redaction / logging | Diagnostics must not log PHI by default |
| Trust-boundary validation | Validate schema identity/version before acting on privileged effects (effects remain host/IPC concerns) |
| Safe diagnostics | Prefer identifiers and classification labels over content |
| Schema/version identifiers | **No PHI** in `$id` or `schemaVersion` strings |
| `Capability` / `PolicyDecision` | **Data representations** of authorization description/outcomes — **not** self-executing authority |

```text
Possessing a serialized Capability or PolicyDecision must not itself grant privileged authority.
Host enforcement and IPC authorization remain outside ADR-04 (ADR-02 / ADR-06 / ADR-14).
```

### 3.11 Proposed invariants (architecture authority only if later Accepted)

1. Fanatir is the sole canonical owner of shared primitive semantics and schema versions.
2. Consumers must not silently fork semantics.
3. Schema versions remain distinct from package/app/IPC/persistence versions.
4. Breaking removals require governed decision + major version.
5. Serialized capability/policy objects never self-authorize.

Until Accepted, these are **proposed planning invariants** only.

---

## 4. Cross-ADR boundaries

| Concern | Owner ADR / vehicle | ADR-04 may… | ADR-04 must not… |
| --- | --- | --- | --- |
| Rust-first durable mutation direction | [ADR-15](./ADR-15-rust-first-polyglot-runtime.md) (Proposed) | Align schema ownership with language-neutral contracts | Redefine language authority |
| Trusted Host TCB / privileged actions | [ADR-02](./ADR-02-rust-trusted-host-boundary.md) (Proposed) | Supply shared types for host-facing data | Define host TCB composition |
| Platform composition / WebView | [ADR-01](./ADR-01-platform-and-desktop-composition-tauri-2.md) (Proposed) | Note UI consumes contracts | Define Tauri composition |
| `afia-ui` strangler migration | [ADR-03](./ADR-03-afia-ui-strangler-migration.md) (Proposed) | Clarify shell is not contract SoT | Define UI migration stages |
| IPC / worker protocols | **ADR-06** (not authored) | Defer envelopes/commands/events | Define IPC schemas |
| Artifact Store / persistence | **ADR-05** (not authored) | Defer store formats; inventory includes Artifact/Revision/Run as **shared meanings** | Define DDL or store layout |
| Fehrest integration | **ADR-08** (not authored) | Treat Fehrest as consumer | Define Fehrest release/process behavior |
| Auth / sessions | **ADR-11** (not authored) | Remain silent on session mechanics | Define auth/session architecture |
| PHI egress / plugins detail | **ADR-14** (not authored) | Classification fields as data | Finalize egress enforcement |

ADR-04 **proposes ownership and version governance only**.

---

## 5. Change governance (proposed future flow — not implemented)

A future governed change would **require** (planning sketch):

1. change proposal;
2. owner identification (Fanatir);
3. compatibility classification (additive vs breaking / semantic);
4. security/privacy review;
5. affected-consumer inventory (UI, host, workers, Fehrest, DeepMed as applicable);
6. version decision (`schemaVersion`);
7. schema update (under authorized docs/impl tasks);
8. conformance evidence;
9. separately authorized migration;
10. deprecation or release communication.

| Change kind | Proposed review posture |
| --- | --- |
| Compatible additive (no semantic reinterpretation) | Normal review under later authorized tasks |
| Breaking / semantic | **Tier A** review + ADR or ADR amendment |

```text
This process is not claimed as implemented.
```

---

## 6. Alternatives considered

### Option A — Each consumer owns its own definitions

| | |
| --- | --- |
| Benefits | Local speed |
| Costs / risks | Contract drift; incompatible JSON; uncoordinated breaks |
| Compatibility | Conflicts with research P2 and cross-repo Alpha needs |
| Draft disposition | **Rejected** |

### Option B — Rust types as canonical authority

| | |
| --- | --- |
| Benefits | Aligns with Trusted Host implementation language |
| Costs / risks | Blocks Fehrest/TS/Python as first-class consumers of language-neutral schemas |
| Compatibility | Conflicts with research P2 rejection of Rust-only types |
| Draft disposition | **Rejected** as *canonical schema authority* (Rust remains durable-mutation language direction under ADR-15) |

### Option C — TypeScript / `afia-ui` types as canonical authority

| | |
| --- | --- |
| Benefits | Matches current UI surface |
| Costs / risks | Shell becomes architecture SoT; polyglot consumers inherit UI shapes |
| Compatibility | Conflicts with ADR-03 Proposed posture and Fanatir ownership |
| Draft disposition | **Rejected** |

### Option D — Trusted Host types as universal canonical authority for all meanings

| | |
| --- | --- |
| Benefits | Single runtime owner |
| Costs / risks | Collapses schema ownership into host implementation; confuses IPC/persistence with shared contracts |
| Compatibility | Overreaches ADR-02 boundary; ignores cross-repo Fehrest/DeepMed needs |
| Draft disposition | **Rejected** as universal schema SoT |

### Option E — Database schema as the domain contract

| | |
| --- | --- |
| Benefits | Familiar for CRUD systems |
| Costs / risks | Couples domain meaning to DDL; violates language-neutral boundary; conflicts with local Artifact Store direction |
| Compatibility | Conflicts with shared-primitives non-goals (no DB DDL) |
| Draft disposition | **Rejected** |

### Option F — Unversioned JSON

| | |
| --- | --- |
| Benefits | Apparent simplicity |
| Costs / risks | Silent drift; no fail-closed version checks |
| Compatibility | Conflicts with `$id` + semver planning baseline |
| Draft disposition | **Rejected** |

### Option G — One global version for every primitive

| | |
| --- | --- |
| Benefits | Single number to communicate |
| Costs / risks | Forces unrelated breaks; collapses independent evolution |
| Compatibility | Conflicts with independently versioned contracts goal |
| Draft disposition | **Rejected** |

### Option H — Fanatir-owned, language-neutral, independently versioned contracts

| | |
| --- | --- |
| Benefits | Explicit semantic authority; polyglot consumption; auditable breaks |
| Costs / risks | Governance overhead; compatibility matrices; version coordination |
| Compatibility | Aligns with research P2, shared-primitives.md, data-model.md, plan ADR-04 row |
| Draft disposition | **Proposed** |

---

## 7. Consequences

### Positive (prospective — if later accepted and executed under separate authorization)

- Explicit semantic authority under Fanatir
- Reduced cross-boundary contract drift
- Safer cross-language consumption
- Auditable breaking changes
- Clearer compatibility responsibility for consumers

### Costs and risks

| Cost / risk | Note |
| --- | --- |
| Governance overhead | Reviews, inventories, ADR amendments |
| Compatibility matrices | Host ↔ UI ↔ Fehrest ↔ DeepMed pins |
| Version coordination | Multiple version domains must stay distinct |
| Schema-review burden | Tier A on breaks |
| Generated-binding drift | Bindings can lag schemas |
| Coexistence of versions | Temporary multi-version support cost |
| Migration costs | Separately authorized |
| Over-generalization | Risk of a monolithic shared-contract layer |
| Misuse of this draft | Treating Proposed as license to publish packages now |

### Rollback expectations (planning only)

If this draft is rejected or superseded before acceptance: revert the ADR file; keep planning contracts as non-authoritative evidence; do not implement packages/bindings from this draft. If later accepted then withdrawn under a future gate: consumers would need a separately authorized rollback/migration plan — **not** defined or authorized here.

Prospective benefits are **not** measured outcomes.

---

## 8. Non-goals

ADR-04 does **not**:

- publish final production schemas;
- finalize all primitive fields, cardinalities, or validation rules;
- define full FHIR schemas;
- define database DDL;
- choose final code-generation tooling as settled (unless a later authorized decision records it);
- define IPC envelopes, commands, events, or worker protocols;
- define persistence formats or Artifact Store layout;
- define Fehrest integration/release behavior;
- define authentication/session behavior;
- implement bindings or `packages/contracts`;
- migrate or deprecate runtime contracts;
- authorize R2 or T031+.

---

## 9. Relationship to other ADRs

| ADR | Status in repo | Relationship |
| --- | --- | --- |
| ADR-15 | Proposed | Constrains language-neutral contracts + Rust durable mutation direction |
| ADR-01 | Proposed | UI composition consumes contracts; does not own them |
| ADR-02 | Proposed | Host may use shared types; schemas deferred here |
| ADR-03 | Proposed | `afia-ui` is not shared-contract SoT |
| ADR-05 | Not authored | Artifact Store / persistence; requires ADR-04 sequencing |
| ADR-06 | Not authored | IPC/worker contracts |
| ADR-08 | Not authored | Fehrest integration consumer of pins |
| ADR-11 | Not authored | Auth/session preservation / change path |

---

## 10. Gates and authority

```text
T016 produces a Proposed draft only.
Tier A independent review is required (shared primitives ADR).
Passing Tier A review is not architecture acceptance.
Founder acceptance, if separately issued, applies only to the draft work product unless a stage gate records Accepted.
T030 remains the R1 stage gate.
At T030, ADR-04 is expected to be listed Reviewed or Accepted — it is not automatically Accepted.
ADR-04 is not in the mandatory Accepted set that opens R2 (ADR-15/01/02/06).
T031+ and R2 remain separately gated.
Implementation (schemas publish, packages, bindings, migrations) requires separate authorization.
T027 may align planning contract docs only after ADR-04 is drafted — T027 is not authorized by this file alone.
```

| Gate | Meaning for ADR-04 |
| --- | --- |
| T016 | Draft authoring (this task) |
| Tier A | Mandatory independent review before treating as stage-complete evidence |
| Founder draft acceptance | Work-product acceptance only, if issued |
| T030 | R1 architecture gate; ADR-04 Reviewed or Accepted listing |
| T027 | Planning schema boundary alignment — separate task |
| T018 / ADR-05 | Must not proceed without ADR-04 sequencing |
| T031+ | Blocked until T030 Accepted set satisfied |

---

## 11. Unresolved questions

T016 does **not** resolve these. Attempted resolution: **NO**.

| ID | Question | May remain open in draft review? | Must resolve before implementation? | Belongs elsewhere? |
| --- | --- | --- | --- | --- |
| U-ADR04-1 | Canonical schema directory layout under `contracts/` vs `packages/contracts` | YES | YES before publish | Later authorized packaging/docs tasks; plan names target path |
| U-ADR04-2 | One schema document per primitive vs grouped schemas | YES | YES before publish | Later schema authoring tasks |
| U-ADR04-3 | Code-generation tooling selection | YES | YES before generated bindings ship | Explicitly deferred by shared-primitives.md |
| U-ADR04-4 | Generated-binding ownership (Fanatir package vs consumer-owned) | YES | YES before publish | Later package ADR/tasks |
| U-ADR04-5 | Compatibility-test ownership and suite location | YES | YES before claiming compatibility | Later test tasks |
| U-ADR04-6 | Supported-version declaration mechanism (manifest, header, pin file) | YES | YES before runtime negotiation | May touch ADR-06 for transport headers |
| U-ADR04-7 | Deprecation-window governance (duration, exceptions) | YES | YES before breaking removal | Stage gates / founder policy |
| U-ADR04-8 | Registry or discovery approach for `$id` | YES | Before multi-repo automation | Later platform tasks |
| U-ADR04-9 | Extension / vendor field handling | YES | Before wide consumer adoption | Schema authoring |
| U-ADR04-10 | Future FHIR-profile relationship | YES | Before FHIR-heavy exchange | Domain specs; not full FHIR in ADR-04 |
| U-ADR04-11 | Unknown additive fields policy (ignore vs reject) | YES | YES before fail-closed validation | Schema + consumer conformance |
| U-ADR04-12 | Consumer conformance evidence format | YES | YES before T027/impl claims | T027 / later verification |
| U-ADR04-13 | Multi-version coexistence rules in one process | YES | YES before dual-version runtime | Host/IPC tasks as applicable |

---

## 12. Validation and acceptance plan

```text
T016 completion produces a draft for Tier A review.
Architecture acceptance is not performed by T016.
```

Planning reviews **may** include:

- consistency with research P2, shared-primitives.md, data-model.md, and plan ADR-04 roadmap row;
- ownership/version-domain separation review;
- security/privacy claim-language review (PHI identifiers; Capability/PolicyDecision non-authority);
- cross-ADR deferral correctness (05/06/08/11);
- Tier A independent review (tasks.md).

**Not** validation for T016: generating schemas, publishing packages, running compatibility suites, or R2 builds.

---

## 13. Prohibitions carried by this draft

While this ADR remains Proposed:

- no production schema publication;
- no `packages/contracts` implementation from this draft alone;
- no IPC/persistence/auth changes;
- no Fehrest/DeepMed runtime contract mutation;
- no treating `afia-ui` types as canonical shared primitives;
- no push/PR implied by drafting;
- no OpenMed/Graphify fork/import; no Pictorial/Montada; no Go Alpha Trusted Host authority;
- no R2 / T031+ work.

---

## 14. Document control

| Item | Value |
| --- | --- |
| Created for | T016 — Author ADR-04 Shared Primitive Ownership and Versioning (draft only) |
| Required path pattern | `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-04-*.md` |
| Plan roadmap entry | [plan.md](../plan.md) ADR-04 row (existing; not modified by T016) |
| Supersedes | Nothing |
| Superseded by | Nothing |

```text
End of ADR-04 Proposed draft.
```
