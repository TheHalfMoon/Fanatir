# ADR-10 — commandF Ownership and Process Boundary

| Field | Value |
| --- | --- |
| **ADR** | ADR-10 |
| **Title** | commandF Ownership and Process Boundary |
| **Status** | **Proposed** (draft only) |
| **Task origin** | T022 |
| **Acceptance / review posture** | Tier C — normal verification after drafting; **T030** expects ADR-10 listed **Reviewed or Accepted** (not automatically Accepted; **not** in the mandatory Accepted set that opens R2). commandF implementation such as **T051** requires ADR-10 **Accepted** and **ADR-15** |
| **Feature** | `001-fanatir-repository-and-architecture-reconstitution` |
| **Required gate (tasks.md)** | ADR-15 |
| **Planning dependency (plan.md)** | Constitution, ADR-15 |
| **Constraining drafts** | [ADR-15](./ADR-15-rust-first-polyglot-runtime.md) (Proposed); [ADR-02](./ADR-02-rust-trusted-host-boundary.md) (Proposed); [ADR-04](./ADR-04-shared-primitive-ownership-and-versioning.md) (Proposed); [ADR-05](./ADR-05-artifact-revision-run-storage.md) (Proposed); [ADR-06](./ADR-06-worker-and-ipc-contracts.md) (Proposed); [ADR-07](./ADR-07-supabase-adapter-and-local-first-boundary.md) (Proposed); [ADR-08](./ADR-08-fehrest-integration-and-release-model.md) (Proposed); [ADR-09](./ADR-09-deepmed-integration-and-openmed-runtime-fork-boundary.md) (Proposed) |
| **Related drafts** | [ADR-01](./ADR-01-platform-and-desktop-composition-tauri-2.md) (Proposed); [ADR-03](./ADR-03-afia-ui-strangler-migration.md) (Proposed) |
| **Planning evidence** | [spec.md](../spec.md) FR-013 / inventory / Alpha journey; [plan.md](../plan.md) ADR-10 row / golden sequence; [research.md](../research.md) R0 bridges / golden journey; Constitution commandF Principles ([constitution.md](../../../.specify/memory/constitution.md)); [commandf.md](../contracts/commandf.md); [R0-python-services.md](../../../docs/program-memory/baseline/R0-python-services.md); [data-model.md](../data-model.md); [shared-primitives.md](../contracts/shared-primitives.md); [trusted-host-ipc.md](../contracts/trusted-host-ipc.md); [worker-runtime.md](../contracts/worker-runtime.md); [tasks.md](../tasks.md) T022 / T012 / T030 / T051 |
| **Architecture authority of this file** | **NO** — until a valid stage-gate acceptance action records Accepted |
| **Implementation authorization** | **NO** |

```text
This document is a planning draft and is not accepted architecture authority.
```

```text
Status: Proposed
Draft ADR ≠ architecture acceptance
T022 completion ≠ T030 acceptance
Passing Tier C review ≠ Accepted
tasks.md founder-acceptance field for T022: No — draft only
Founder acceptance of a draft work product ≠ architecture acceptance
Constitution commandF Principles ≠ Accepted ADR-10
commandf.md is planning contract evidence ≠ Accepted ADR-10
ADR-15 / ADR-01 / ADR-02 / ADR-03 / ADR-04 / ADR-05 / ADR-06 / ADR-07 / ADR-08 / ADR-09 remain Proposed and non-authoritative
```

```text
This draft is not:
- an implemented commandF capability
- a command palette
- a generic launcher or action registry
- permission to modify services/fhir_gate.py
- permission to implement a worker
- permission to publish IPC schemas
- permission to perform production validation or transformation
- permission to read from or write to a live EHR
- permission to mutate Artifacts or Revisions directly
- permission to transmit PHI
- permission to begin R2 or R3
```

```text
No production implementation is authorized by this draft.
```

This draft **must not** be used as justification to: implement commandF; modify [`services/fhir_gate.py`](../../../services/fhir_gate.py); modify `afia-ui`; execute validation or transformation workloads; publish final IPC schemas; mutate Artifacts, Revisions, or Runs; write to a live EHR; transmit PHI; change authentication or sessions; begin R2 (`T031+`) or R3 commandF work; or treat Proposed ADRs as Accepted.

---

## 1. Context and problem

### 1.1 Assigned architectural question

**ADR-10 proposes** how **commandF** would be owned and bounded inside **Fanatir** as the **healthcare interoperability validate–transform** capability and workbench — Fanatir-owned, Trusted Host–supervised when non-Rust, provisional in output until host-mediated publication — constrained by Proposed [ADR-15](./ADR-15-rust-first-polyglot-runtime.md), [ADR-02](./ADR-02-rust-trusted-host-boundary.md), [ADR-04](./ADR-04-shared-primitive-ownership-and-versioning.md), [ADR-05](./ADR-05-artifact-revision-run-storage.md), [ADR-06](./ADR-06-worker-and-ipc-contracts.md), [ADR-07](./ADR-07-supabase-adapter-and-local-first-boundary.md), [ADR-08](./ADR-08-fehrest-integration-and-release-model.md), and [ADR-09](./ADR-09-deepmed-integration-and-openmed-runtime-fork-boundary.md).

It does **not** implement commandF; wrap or replace [`services/fhir_gate.py`](../../../services/fhir_gate.py); finalize shared schemas (**ADR-04**); finalize Artifact Store persistence (**ADR-05**); finalize IPC envelopes (**ADR-06**); redefine Fehrest (**ADR-08**); redefine DeepMed (**ADR-09**); redesign authentication/sessions (**ADR-11**, not authored); or finalize PHI egress (**ADR-14**, not authored).

### 1.2 Why an explicit commandF ownership and process boundary matters

Fanatir’s Founder Alpha golden journey includes a **commandF validate/transform** step ([plan.md](../plan.md); [spec.md](../spec.md); [commandf.md](../contracts/commandf.md)). Spec and Constitution require Fanatir ownership of commandF even when no dedicated module exists ([spec.md](../spec.md) FR-013; Constitution commandF Principles). Prototype inventory lists [`services/fhir_gate.py`](../../../services/fhir_gate.py) as a **PROTOTYPE** FHIR transform bridge, not the product ([R0-python-services.md](../../../docs/program-memory/baseline/R0-python-services.md)).

Without an explicit boundary, planning identifies these **risks** (prospective — not claimed as currently measured production failures unless separately evidenced):

| Risk | Why it matters |
| --- | --- |
| commandF mistaken for a command palette / launcher / app search | Wrong product role; wrong UI and implementation path |
| Prototype FHIR code becomes target architecture by accident | `fhir_gate.py` frozen as product |
| UI performs clinical transformations directly | Bypass of Trusted Host / Capability Gateway |
| Transformation results become authoritative without review | Silent clinically significant change |
| Workers mutate Artifacts directly | ADR-05 / worker-runtime violation |
| commandF becomes a live EHR write path | Unauthorized clinical system mutation |
| Unsupported formats treated as successfully converted | False interoperability claims |
| Validation confused with clinical correctness | Overconfident care decisions |
| Transformation confused with semantic equivalence | Silent meaning loss |
| PHI leaks via payloads, logs, temp files, exports, or remote calls | Classification/egress failure |
| Process and protocol versions conflated | Non-reproducible, unsafe upgrades |
| Universal-conversion claimed without evidence | Alpha over-promise |

### 1.3 Accepted program and planning constraints

| Constraint | Source | Treatment |
| --- | --- | --- |
| commandF ownership MUST be Fanatir even without dedicated codebase | FR-013 — [spec.md](../spec.md) | Binding ownership; **not** ADR-10 acceptance |
| commandF = validation/transform boundary; host-supervised worker; versioned I/O; prototype ≠ DeepMed; outputs → Artifacts/Revisions + Run; Alpha guided step; no universal conversion; no live EHR writes | [commandf.md](../contracts/commandf.md) | Planning contract evidence |
| Interoperability workbench; explicit result states; no silent clinically significant transforms; live clinical writes not auto-authorized | Constitution commandF Principles | Active program direction per Constitution status; **not** ADR-10 acceptance |
| commandF non-Rust ⇒ supervised worker only | [ADR-15](./ADR-15-rust-first-polyglot-runtime.md) (Proposed); [plan.md](../plan.md) ADR-10 row | Proposed constraint |
| No production code; draft ≠ accepted; no push/PR | [tasks.md](../tasks.md) T022 | Binding for this task |
| No R2 before T030 (ADR-15/01/02/06 Accepted as applicable) | [tasks.md](../tasks.md) | Future-gate requirement |

### 1.4 Naming

| Name | Role in this ADR |
| --- | --- |
| **Fanatir** | Product/integration authority; owns commandF capability |
| **commandF** | Fanatir healthcare interoperability **validate–transform** capability / workbench — **not** a command palette |
| **FHIR** | Primary planning-oriented interoperability family for Alpha gates where applicable |
| **Artifact Store** | Local Rust-controlled Artifact/Revision/Run content authority (**ADR-05** Proposed) |
| **Capability Gateway** | Fanatir-owned authorization path for privileged commandF invocation |
| **Trusted Host** | Sole local privileged supervisor (**ADR-02** Proposed) |
| **Fehrest** | Knowledge/vault product (**ADR-08** Proposed) — not commandF |
| **DeepMed** | Clinical-assistive runtime (**ADR-09** Proposed) — not commandF |
| **`afia-ui`** | Migration shell; may present journeys; not execution authority (**ADR-03** Proposed) |
| **`services/fhir_gate.py`** | Prototype inventory evidence only — not target architecture |

Never confuse commandF with a command palette, commandF with DeepMed, commandF with Fehrest, or validation success with clinical correctness.

### 1.5 What this draft is not

This file is **not**: an implemented commandF module; a command palette; a launcher; a generic action registry; permission to modify [`services/fhir_gate.py`](../../../services/fhir_gate.py); worker implementation; final IPC schema; live EHR authority; Artifact mutation authority; PHI-egress authorization; or R2/R3 authorization.

---

## 2. Decision (proposed)

### 2.1 Core decision

**Propose** that **commandF** is Fanatir’s owned **healthcare interoperability validate–transform** capability and workbench: a governed processor for **bounded, allowlisted** healthcare and data formats (including FHIR-oriented gates where applicable), executed when non-Rust only as a **Trusted Host–supervised** worker, producing **provisional** validation/transformation results with **explicit result states**, and never silently authorizing clinically significant transformation or live clinical writes.

### 2.2 Product and capability role (proposed)

commandF **proposes** to be:

- a Fanatir-owned healthcare interoperability capability;
- a validate–transform workbench;
- a governed processor for supported healthcare and data formats;
- a supervised worker boundary where implementation is non-Rust;
- a producer of provisional validation/transformation results;
- a future producer of governed Artifact/Revision and Run evidence through the host.

commandF **is not**:

- a command palette;
- global application search;
- a launcher;
- a generic command registry;
- a favorites/recents surface;
- a plugin marketplace;
- DeepMed;
- Fehrest;
- the Trusted Host;
- the Artifact Store;
- a clinical source of truth;
- a live EHR integration authority.

### 2.3 Ownership model (proposed)

| Concern | Proposed owner |
| --- | --- |
| commandF domain semantics (validate/transform posture, result states, format allowlist rules) | **ADR-10** / Fanatir commandF capability |
| UI intent and journey presentation | `afia-ui` / Studio presentation (**ADR-03** Proposed) — convenience only |
| Privileged authorization, process launch, mediation, audit | Trusted Host / Capability Gateway (**ADR-02** Proposed) |
| Worker execution (non-Rust) | Supervised commandF worker under host (**ADR-15** / **ADR-06** Proposed) |
| Shared semantic contracts (Artifact, Revision, Run, Approval, etc.) | **ADR-04** Proposed |
| IPC and generic worker protocol | **ADR-06** Proposed |
| Artifact/Revision/Run persistence | **ADR-05** Proposed |
| PHI classification and egress detail | **ADR-14** (not authored) |
| Auth/session redesign | **ADR-11** (not authored) |

Preserve: Fanatir owns commandF; `afia-ui` does not own execution authority; Trusted Host owns privileged execution and policy; workers cannot become trusted peers.

### 2.4 Alpha scope (proposed)

Recovered Alpha boundary from [commandf.md](../contracts/commandf.md), [spec.md](../spec.md), and [plan.md](../plan.md):

**In Alpha planning scope (bounded):**

- at least one guided validate/transform step in the golden journey;
- explicitly supported input formats only (allowlisted; matrix unresolved);
- FHIR-oriented validation or transformation where canonically supported for that guided step;
- explicit validation / result states;
- explicit review requirements where risk or uncertainty demands them;
- governed output publication through the host (not worker-direct);
- interoperability processing that is distinct from DeepMed clinical-assistive inference.

**Alpha does not claim:**

- universal format conversion;
- every FHIR version or profile;
- automatic semantic equivalence;
- live EHR writes;
- unrestricted clinical transformation;
- production readiness;
- external provider integration as a default path;
- generic command-palette behavior.

### 2.5 UI → Host → Worker flow (proposed planning sequence)

1. UI collects user intent and required inputs.
2. UI performs convenience validation only (non-authoritative).
3. UI submits a governed request to the Trusted Host.
4. Trusted Host evaluates capability and policy requirements (`PolicyDecision`, `AuthContext`, `DataClassification` as applicable).
5. Trusted Host launches or invokes the commandF worker.
6. Worker performs bounded validation or transformation.
7. Worker returns typed provisional results and evidence.
8. Trusted Host validates the response.
9. User review occurs where required.
10. Trusted Host requests governed Artifact/Revision and Run publication.

Final API names and message schemas remain **unresolved** (**ADR-06** / later contracts).

### 2.6 Trusted Host boundary (proposed)

The Trusted Host **would own**:

- process launch;
- capability checks;
- `PolicyDecision`;
- `AuthContext` evaluation;
- `DataClassification` enforcement;
- filesystem mediation;
- network mediation;
- secret injection;
- resource limits;
- timeout and cancellation;
- result validation;
- audit correlation;
- Artifact publication requests;
- live-system access decisions.

commandF workers **must not** receive ambient access to: arbitrary filesystem locations; patient stores; live EHR systems; unrestricted network; secrets; host environment variables; Artifact Store mutation; Supabase clinical/content authority.

### 2.7 Supervised worker boundary (proposed)

Preserve [ADR-15](./ADR-15-rust-first-polyglot-runtime.md) and [worker-runtime.md](../contracts/worker-runtime.md) direction:

- non-Rust commandF logic runs only as a supervised worker;
- process isolated; restartable; replaceable; capability constrained; versioned; auditable;
- denied ambient host privileges;
- unable to become a trusted peer;
- MUST NOT directly own durable Artifact mutation, secrets, policy, PHI egress, plugin permissions, or authoritative audit.

This draft does **not** implement the worker and does **not** claim [`services/fhir_gate.py`](../../../services/fhir_gate.py) already satisfies the target.

### 2.8 Prototype `fhir_gate.py` boundary (proposed)

[`services/fhir_gate.py`](../../../services/fhir_gate.py) is **canonical prototype inventory evidence** only ([R0-python-services.md](../../../docs/program-memory/baseline/R0-python-services.md); [commandf.md](../contracts/commandf.md); [research.md](../research.md)):

- it is **not** commandF’s final architecture;
- it is **not** proof that ADR-10 or **T051** is complete;
- it may inform compatibility and migration planning;
- **T022 must not modify it**;
- wrapping it later does **not** bypass Trusted Host authority;
- future replacement, wrapping, or retirement belongs to separately authorized implementation work (**T051+**).

Observed prototype characteristics (inventory evidence, not target claims): FHIR R4B Bundle builder from NER entities; commandF-equivalent result states **NOT OBSERVED**; network none in fhir_gate itself ([R0-python-services.md](../../../docs/program-memory/baseline/R0-python-services.md)).

### 2.9 Validation versus transformation (proposed)

| Activity | Planning meaning |
| --- | --- |
| Syntax / structural / profile or schema validation | Checks against declared rules — not clinical correctness |
| Terminology validation (where applicable) | Unresolved whether local or remote; must not silently pass if required and missing |
| Semantic validation | Deeper meaning checks — depth unresolved; unsupported depth must be explicit |
| Transformation / mapping / normalization | Produces candidate outputs; success ≠ semantic equivalence |
| Clinical review | Human judgment where risk/uncertainty demands |
| Publication | Host-mediated durable Artifact/Revision/Run path only |

**Clarify:** validation success does **not** prove clinical correctness; transformation success does **not** prove semantic equivalence; missing terminology or profiles must **not** silently pass; clinically significant transformations **require** review.

Do not define unsupported validation engines or terminology services in this draft.

### 2.10 Supported and unsupported formats (proposed)

- Supported formats and versions **must** be explicitly allowlisted.
- Unsupported formats fail clearly (`UNSUPPORTED` / `REJECTED` as applicable).
- Partial support is represented explicitly (`PARTIAL`).
- No universal-conversion claim is permitted.
- Version/profile compatibility is recorded where applicable.
- Unknown formats fail closed.
- Conversion capability must **not** be inferred from file extension alone.

A final format matrix is **unresolved** unless later canonical sources provide one. This draft does **not** invent one.

### 2.11 Explicit result states (proposed)

Preserve Constitution-supported result states ([constitution.md](../../../.specify/memory/constitution.md)):

| State | Planning-level meaning |
| --- | --- |
| `VERIFIED` | Bounded to **declared** validation rules for the operation; **not** clinically correct by implication |
| `REVIEW REQUIRED` | Human review required before governed promotion; **cannot** be auto-promoted |
| `PARTIAL` | Some validation/transform succeeded; **must** expose what was not validated or transformed |
| `UNSUPPORTED` | Capability/format/profile not supported; **must not** silently degrade to success |
| `REJECTED` | Safe failure; preserve failure evidence **without** exposing PHI in ordinary channels |

Final state-machine schemas remain **unresolved**.

### 2.12 Input boundary (proposed, planning level)

Possible future inputs (not a final schema): files; Artifact references; Revision references; FHIR resources; bundles; documents; structured datasets; mapping instructions; selected validation profiles; transformation targets; project context; `AuthContext`; `DataClassification`.

**Would require:** input validation; provenance; minimum-necessary data; classification; authorization; bounded payloads; explicit version/profile selection; local-versus-remote handling. Exact schemas **unresolved**.

### 2.13 Output boundary (proposed, planning level)

Possible provisional outputs (not a final schema): validation status; findings; warnings; unsupported features; transformation result; mapping evidence; source-to-target traceability; provenance; worker/runtime version; profile/version identifiers; candidate Artifact or Revision references; proposed Run evidence; failure or cancellation evidence.

**Clarify:** output is provisional until host validation and publication; worker output does **not** mutate the Artifact Store directly; result-state success does **not** authorize live clinical use.

### 2.14 Provenance and Artifact Store boundary (proposed)

Future commandF operations **would** preserve, where applicable: input Artifact/Revision references; input format/version; validation profile; transformation target; mapping identity/version; worker identity/version; runtime identity; start/completion evidence; result state; warnings; review status; output references; failure/cancellation evidence.

- Shared Run and provenance semantics: **ADR-04** Proposed.
- Persistence: **ADR-05** Proposed.

commandF may, where later authorized: consume governed Artifact references; read governed Revision content; produce candidates; request publication; associate proposed Run evidence.

commandF **must not**: directly write the Artifact Store; mutate published Revisions; advance current Revision pointers; bypass validation or review; treat worker-local files as durable truth; become persistence authority.

### 2.15 Review and approval boundary (proposed)

Review **would be required** at least for: clinically significant transformations; incomplete validation; partial mappings; unsupported terminology; loss of source fields; inferred values; ambiguous mappings; PHI-sensitive outputs; export or external-write requests.

**Clarify:** user/reviewer confirmation does **not** replace authorization; UI confirmation is **not** `PolicyDecision`; review status must be preserved; silent approval is prohibited; exact Approval semantics remain under **ADR-04** or another canonical owner.

### 2.16 Live EHR and external-system boundary (proposed)

- **T022** does **not** authorize live EHR writes.
- commandF Alpha does **not** automatically write to clinical systems ([commandf.md](../contracts/commandf.md)).
- Live reads or writes **would require** separate specifications, capabilities, policies, credentials, audit, and PHI-egress controls.
- Validation or transformation success does **not** grant write authority.
- Network access is Trusted Host–mediated.
- External-system failures must **not** corrupt local truth.

Do not define live EHR APIs in this draft.

### 2.17 PHI, secrets, and network posture (proposed)

**PHI / `DataClassification`:** address PHI-bearing files, patient identifiers, FHIR resources, findings, transformed outputs, logs, diagnostics, temporary files, local paths, exports, and network calls with: minimum-necessary processing; fail-closed unknown classification; redacted diagnostics; no PHI in ordinary identifiers or logs; local-first processing where possible; controlled egress; secure temporary-file handling; explicit deletion/retention policy before implementation. Full classification and egress enforcement deferred to **ADR-14** (not authored). No compliance certification is claimed.

**Secrets:** terminology-service, EHR, external-validator, package/provider credentials, trust-store material, and signing keys — if ever needed — **would require** Trusted Host–controlled injection; no ambient inheritance; no secrets in UI state, IPC payloads, ordinary logs, Artifacts, or transformed output. Secret backend **unresolved**.

**Network:** Alpha commandF does **not** require unrestricted network access; local validation/transformation preferred where feasible; terminology or external validation calls require separate authorization; requests/responses untrusted until validated; PHI-bearing network calls require ADR-14 policy; network unavailability must **not** silently convert failure into success; remote service use does **not** make remote data authoritative. External services are **not** selected here.

### 2.18 IPC, failure, and version domains (proposed)

Align with [ADR-06](./ADR-06-worker-and-ipc-contracts.md) and [trusted-host-ipc.md](../contracts/trusted-host-ipc.md) / [worker-runtime.md](../contracts/worker-runtime.md): typed and versioned requests/responses; bounded payloads; Artifact URIs for large content; progress; cancellation; timeout; normalized failures; process crashes; unsupported protocol versions; duplicate/replay ambiguity; resource limits. **Do not** publish final command names, envelopes, payload schemas, event schemas, or wire codes.

**Failure / degraded operation would require** fail-closed behavior for: malformed input; unsupported format/version/profile; unavailable validation dependency or terminology service; mapping ambiguity; partial transformation; data-loss risk; timeout; cancellation; process crash; resource exhaustion; invalid worker output; license rejection; network failure; duplicate request; ambiguous completion; host restart. No silent success; no automatic authoritative publication; partial output clearly labeled; no invented automatic retries; failure evidence without PHI leakage.

**Version domains remain separate:** commandF capability version; worker version; worker I/O contract version; validation-engine version; transformation-engine version; mapping version; FHIR version; profile version; terminology version; ADR-04 shared schema version; ADR-06 IPC protocol version; ADR-05 persistence-format version; Fanatir application version. Unsupported combinations fail closed. Negotiation is **not** claimed.

### 2.19 Progressive Constitution vision versus Alpha scope (proposed)

Constitution describes progressive interoperability (ingestion; profiling; FHIR conversion/validation/search; explanation; comparison; research; safe draft editing; provenance; loss reporting; reproducibility packages) ([constitution.md](../../../.specify/memory/constitution.md)).

**ADR-10 distinguishes:**

- Constitution-level **progressive** workbench vision — future / progressive;
- bounded Alpha validate/transform scope per [commandf.md](../contracts/commandf.md).

Where sources mention FHIR search or broader workbench capabilities: label them **future or progressive**; do **not** claim Alpha implementation; do **not** select search engines, ranking algorithms, embeddings, or remote services; do **not** turn ADR-10 into a generic search or command-palette ADR.

### 2.20 Fehrest, DeepMed, Supabase, and auth boundaries (proposed)

| Boundary | Proposal |
| --- | --- |
| **Fehrest (ADR-08)** | Fehrest owns knowledge/vault/graph/memory/export; commandF owns interoperability validate/transform; handoff uses governed references/Artifacts; no direct ungoverned mutation; Fehrest content is not automatic transformation authority |
| **DeepMed (ADR-09)** | DeepMed owns clinical-assistive model/runtime; commandF owns bounded validate/transform; DeepMed output may later be passed for governed interoperability processing; provisional DeepMed output is not approved clinical truth; no direct ungoverned invocation; **T050** and later gates remain separate |
| **Supabase / Decision C (ADR-07)** | Supabase is not commandF’s validation, transformation, Artifact, PHI, or clinical SoT; T022 does not synchronize content; collaboration metadata cannot authorize transformation; remote rows cannot publish commandF output; local authoritative behavior must not silently depend on Supabase |
| **Auth/session (ADR-11)** | `AuthContext` may input host authorization; commandF does not redesign OTP/sessions/identities/membership; UI state is not proof of authorization; cached session cannot authorize privileged execution; auth/session changes require ADR-11 + dedicated spec |

### 2.21 Audit, accessibility, and plugin posture (proposed)

**Audit-relevant events (planning):** worker launch; capability denial; validation/transformation request; selected profile or mapping; result state; review requirement; output publication request; live-system access denial; PHI-egress denial; timeout; cancellation; failure; incompatible version; license-policy rejection. Authoritative audit ownership remains with **ADR-02** / Trusted Host. Avoid logging PHI, full input payloads, secrets, or transformed clinical content by default. Audit schemas **unresolved**.

**Accessibility / UI (architectural only):** keyboard accessibility; screen-reader-compatible journeys; clear status presentation; review-state visibility; clear unsupported/rejected states; no color-only status communication; focus/error handling for later UI specs. **ADR-03** owns `afia-ui` migration; ADR-10 does **not** implement UI; commandF is **not** a keyboard command palette.

**Plugins / extensibility:** canonical commandF plugin or macro registration is **unresolved**. ADR-10 does **not** authorize arbitrary plugins, scripts, macros, or remote commands; worker implementations require governed registration and capabilities; extensions cannot bypass Trusted Host, IPC, PHI, licensing, or Artifact rules; signing, sandboxing, revocation, and registration remain **unresolved**. Do not invent a plugin system.

---

## 3. Alternatives considered

| Option | Rationale (evidence-based; not claimed as tested) |
| --- | --- |
| A. commandF as a command palette / launcher / app search | Contradicts Constitution, [commandf.md](../contracts/commandf.md), FR-013, and inventory role as interop workbench |
| B. commandF as UI-only transformation logic | Violates Trusted Host / UI-zero-authority direction ([trusted-host-ipc.md](../contracts/trusted-host-ipc.md); ADR-02 / ADR-03 Proposed) |
| C. commandF as the Artifact Store | Conflates processing with persistence (**ADR-05**) |
| D. commandF as direct live-EHR integration | Violates Alpha “no live EHR writes” ([commandf.md](../contracts/commandf.md)) and host mediation |
| E. commandF as unrestricted network service | Violates local-first / Capability Gateway posture |
| F. commandF as in-process Python with ambient privileges | Violates ADR-15 / worker-runtime supervised, capability-constrained isolation |
| G. commandF as universal conversion engine | Explicitly forbidden for Alpha ([commandf.md](../contracts/commandf.md); [spec.md](../spec.md)) |
| **H (proposed).** Fanatir-owned, Trusted Host–supervised validate–transform worker with bounded formats and explicit result states | Aligns Constitution + [commandf.md](../contracts/commandf.md) + ADR-15 + plan ADR-10 acceptance gate |

---

## 4. Consequences

### 4.1 Prospective benefits

- clear interoperability ownership under Fanatir;
- bounded Alpha promise (guided validate/transform; no universal conversion; no live EHR writes);
- explicit result states;
- supervised process isolation for non-Rust logic;
- governed Artifact publication path;
- traceable validation and transformation (when implemented);
- separation from DeepMed and Fehrest;
- future replaceability of prototype [`services/fhir_gate.py`](../../../services/fhir_gate.py);
- local-first PHI posture;
- explicit live-EHR prohibition for Alpha.

### 4.2 Costs and risks

- worker supervision and recovery complexity;
- validation-engine and FHIR profile/version matrix growth;
- mapping maintenance and reviewer burden;
- terminology dependency complexity if introduced later;
- PHI and temporary-file handling;
- risk of `VERIFIED` being misunderstood as clinical correctness;
- risk of transformation being mistaken for semantic equivalence;
- prototype migration debt from `fhir_gate.py`;
- future external-service governance;
- process-startup overhead.

### 4.3 Planning-only rollback

Per [tasks.md](../tasks.md) T022: **Revert ADR file**. No production rollback surface is created by this draft.

---

## 5. Non-goals

ADR-10 does **not**:

- implement commandF;
- implement a command palette;
- modify [`services/fhir_gate.py`](../../../services/fhir_gate.py);
- modify `afia-ui`;
- implement validation or transformation;
- publish final schemas or IPC commands;
- select final validation, mapping, search, or terminology engines;
- provide universal conversion;
- write to live EHR systems;
- mutate Artifacts or Revisions directly;
- define final Run schemas;
- redesign auth/session;
- define complete PHI-egress policy;
- implement Fehrest or DeepMed handoff;
- create plugins or macros;
- publish packages or releases;
- begin R2 or R3.

---

## 6. Relationship to other ADRs

| ADR | Status in repo | Relationship |
| --- | --- | --- |
| ADR-15 | Proposed | Rust-first; commandF non-Rust ⇒ supervised worker only |
| ADR-01 | Proposed | Platform composition; workers/sidecars under host |
| ADR-02 | Proposed | Trusted Host / Capability Gateway / PolicyDecision / audit |
| ADR-03 | Proposed | `afia-ui` strangler; presentation of journeys only |
| ADR-04 | Proposed | Shared semantic contracts, Run, Approval |
| ADR-05 | Proposed | Artifact/Revision/Run persistence; commandF candidates only |
| ADR-06 | Proposed | IPC and worker contracts |
| ADR-07 | Proposed | Supabase/local-first; no remote clinical SoT |
| ADR-08 | Proposed | Fehrest integration; governed handoff |
| ADR-09 | Proposed | DeepMed / OpenMed; governed handoff to commandF |
| ADR-11 | Not authored | Auth/session preservation |
| ADR-12 | Not authored | Rename planning (peripheral if referenced) |
| ADR-13 | Not authored | Packaging / distribution |
| ADR-14 | Not authored | PHI classification and egress |

ADR-10 decides only **commandF Ownership and Process Boundary**.

---

## 7. Gates and authority

```text
T022 produces a Proposed ADR-10 draft only.
Tier C — normal verification is required.
Passing Tier C review is not architecture acceptance.
tasks.md founder-acceptance field for T022 is: No — draft only.
Founder draft acceptance is not required by the T022 contract for the draft work product.
T030 remains incomplete.
At T030, ADR-10 is expected to be listed Reviewed or Accepted — it is not automatically Accepted.
ADR-10 is NOT in the mandatory T030 Accepted set that opens R2 (ADR-15/01/02/06).
T051 requires ADR-10 Accepted and ADR-15.
T031+ and R2 remain separately gated.
R3 and commandF implementation remain separately gated.
Worker execution, transformation, live-system access, Artifact publication, PHI egress, package publication, and production work require separate authorization.
```

| Gate | Meaning for ADR-10 |
| --- | --- |
| T022 | Draft authoring (this task) |
| Tier C | Mandatory normal verification |
| Founder draft acceptance | Not required by tasks.md for T022 work product |
| T030 | R1 gate; ADR-10 listed Reviewed or Accepted; not mandatory Accepted-for-R2 |
| T051 | commandF validate/transform worker after ADR-10 Accepted (+ ADR-15) |
| T031+ / R2 / R3 | Separately gated; not authorized here |

---

## 8. Validation and acceptance plan

```text
T022 completion produces a draft for Tier C review.
Architecture acceptance is not performed by T022.
```

Planning reviews **may** include: Fanatir ownership; non–command-palette product role; supervised-worker boundary; prototype ≠ target; Alpha no universal conversion / no live EHR writes; explicit result states; ADR-04/05/06/07/08/09 non-theft; Tier C verification ([tasks.md](../tasks.md) T022).

Verification method from tasks.md: doc review; link from plan roadmap (plan already contains ADR-10 row — **not** modified by T022).

---

## 9. Unresolved questions

T022 does **not** resolve these. Attempted resolution: **NO**.

| ID | Question | May remain open in draft review? | Must resolve before implementation? | Belongs elsewhere? |
| --- | --- | --- | --- | --- |
| U-ADR10-1 | Exact Alpha supported formats | YES | YES | This ADR + commandF capability spec |
| U-ADR10-2 | FHIR versions and profiles for Alpha | YES | YES | This ADR + later format matrix |
| U-ADR10-3 | Validation depth (syntax/structure/profile/terminology/semantic) | YES | YES | This ADR |
| U-ADR10-4 | Terminology-service use (local vs remote) | YES | YES before networked use | This ADR; ADR-14 for PHI |
| U-ADR10-5 | Terminology versions | YES | YES if used | This ADR |
| U-ADR10-6 | Local versus remote validation | YES | YES | This ADR; ADR-02 network mediation |
| U-ADR10-7 | Transformation targets for Alpha | YES | YES | This ADR + [commandf.md](../contracts/commandf.md) |
| U-ADR10-8 | Mapping-language ownership | YES | YES | This ADR; possibly ADR-04 |
| U-ADR10-9 | Mapping versioning | YES | YES | This ADR |
| U-ADR10-10 | Mapping review workflow | YES | YES | This ADR; ADR-04 Approval |
| U-ADR10-11 | Semantic-equivalence claim rules | YES | YES | This ADR |
| U-ADR10-12 | Guided workflow UI boundaries | YES | YES for UI tasks | ADR-03 + later UI specs |
| U-ADR10-13 | Exact result-state machine | YES | YES | This ADR; ADR-04 shared enums if any |
| U-ADR10-14 | `VERIFIED` declared-rule scope | YES | YES | This ADR |
| U-ADR10-15 | `PARTIAL` publication rules | YES | YES | This ADR; ADR-05 |
| U-ADR10-16 | Artifact publication flow detail | YES | YES | ADR-05 |
| U-ADR10-17 | Revision publication flow detail | YES | YES | ADR-05 |
| U-ADR10-18 | Run recording detail | YES | YES | ADR-04 / ADR-05 |
| U-ADR10-19 | Worker implementation language (expect Python for Alpha unless decided otherwise) | YES | YES | ADR-15; T051 |
| U-ADR10-20 | Prototype wrap versus replace for `fhir_gate.py` | YES | YES before T051 cut | This ADR; T051 |
| U-ADR10-21 | Worker discovery and launch | YES | YES | ADR-06 / trusted-host-ipc |
| U-ADR10-22 | Temporary-file custody | YES | YES | ADR-02; ADR-14 |
| U-ADR10-23 | Output retention | YES | YES | ADR-05; ADR-14 |
| U-ADR10-24 | PHI handling detail | YES | YES | ADR-14 |
| U-ADR10-25 | Network authorization matrix | YES | YES before networked ops | ADR-02; ADR-14 |
| U-ADR10-26 | External-system reads and writes | YES | YES; separate specs | Later EHR/integration specs |
| U-ADR10-27 | Credential handling | YES | YES | ADR-02 secrets |
| U-ADR10-28 | Cancellation semantics | YES | YES | ADR-06 |
| U-ADR10-29 | Retry and idempotency | YES | YES | ADR-06; fail-closed default |
| U-ADR10-30 | Duplicate requests | YES | YES | ADR-06 |
| U-ADR10-31 | Resource limits | YES | YES | ADR-02 / ADR-06 |
| U-ADR10-32 | Process recovery | YES | YES | ADR-06 / worker-runtime |
| U-ADR10-33 | Audit correlation IDs | YES | YES | ADR-02 |
| U-ADR10-34 | License review for validators/mappers | YES | YES | Packaging / legal; ADR-13 if packaging |
| U-ADR10-35 | Compatibility testing matrix | YES | YES | Later verification tasks |
| U-ADR10-36 | UI presentation of result states | YES | YES for UI | ADR-03 + UI specs |
| U-ADR10-37 | Accessibility validation | YES | YES for UI | Later UI / a11y specs |
| U-ADR10-38 | Fehrest handoff mechanics | YES | YES | ADR-08 + later tasks |
| U-ADR10-39 | DeepMed handoff mechanics | YES | YES | ADR-09 + later tasks |
| U-ADR10-40 | Plugin/extensibility policy | YES | YES if plugins claimed | Unresolved; not authorized here |

---

## 10. Security and privacy claim language

This draft **proposes** planning postures only. It does **not** claim implemented PHI controls, compliance certification, operational encryption, or audited egress. Any future commandF processing of healthcare data **would require** Capability Gateway authorization, classification-aware mediation, and ADR-14 egress rules before PHI leaves the local trusted boundary.

---

## 11. Document control

| Item | Value |
| --- | --- |
| Created by | T022 draft authoring |
| Status | Proposed |
| Supersedes | None |
| Superseded by | None |
| Next expected actions | Tier C normal verification; later T030 listing; T051 only after ADR-10 Accepted |

```text
End of ADR-10 draft.
Status: Proposed
Implementation authorization: NO
```
