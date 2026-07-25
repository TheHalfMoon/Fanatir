# ADR-07 — Supabase Adapter and Local-First Boundary

| Field | Value |
| --- | --- |
| **ADR** | ADR-07 |
| **Title** | Supabase Adapter and Local-First Boundary |
| **Status** | **Reviewed** (T030 R1 architecture gate; not Accepted) |
| **Task origin** | T019 |
| **Acceptance / review posture** | Tier A independent review after drafting; **T030** expects ADR-07 listed **Reviewed or Accepted** (not automatically Accepted; **not** in the mandatory Accepted set that opens R2). Later work such as **T057** requires ADR-07 and ADR-14 **Accepted** |
| **Feature** | `001-fanatir-repository-and-architecture-reconstitution` |
| **Related founder decision** | Decision C (content/PHI/Artifact revisions → local Rust Artifact Store; Supabase = optional collab/identity; freeze `documents-crypto`; future Spec 002 authorized by name) — **Ratified** in accepted [plan.md](../plan.md) |
| **Constraining ADRs** | [ADR-15](./ADR-15-rust-first-polyglot-runtime.md) (Accepted at T030); [ADR-05](./ADR-05-artifact-revision-run-storage.md) (Reviewed at T030); [ADR-04](./ADR-04-shared-primitive-ownership-and-versioning.md) (Reviewed at T030); [ADR-02](./ADR-02-rust-trusted-host-boundary.md) (Accepted at T030) |
| **Related ADRs** | [ADR-01](./ADR-01-platform-and-desktop-composition-tauri-2.md) (Accepted at T030); [ADR-03](./ADR-03-afia-ui-strangler-migration.md) (Proposed); [ADR-06](./ADR-06-worker-and-ipc-contracts.md) (Accepted at T030) |
| **Planning evidence** | [supabase-adapter.md](../contracts/supabase-adapter.md); [research.md](../research.md) R8/R14/P1; [plan.md](../plan.md) Decision C / ADR-07 row; [spec.md](../spec.md) Q4 / FR-015; [R0-supabase-inventory.md](../../../docs/program-memory/baseline/R0-supabase-inventory.md); [data-model.md](../data-model.md); [shared-primitives.md](../contracts/shared-primitives.md); [tasks.md](../tasks.md) T019/T028/T029/T030/T057 |
| **T030 founder decision** | **Reviewed** — R1 architecture gate; not Accepted; separate later Accepted required before governed Supabase adapter or migration work (e.g. T057) |
| **Architecture authority of this file** | **NO** — Reviewed at T030; Accepted required before Supabase adapter / migration implementation |
| **Implementation authorization** | **NO** |

```text
Status: Reviewed (T030 R1 architecture gate; not Accepted)
Architecture authority: NO (Reviewed ≠ Accepted)
Implementation authorization: NO
Supabase adapter / migration / Spec 002 execution: NOT authorized
Independent Tier A — R1 architecture gate: Pending
```

```text
Draft ADR ≠ architecture acceptance
T019 completion ≠ T030 acceptance (T030 founder Reviewed decision now recorded)
Passing Tier A review of a draft ≠ Accepted
Ratified Decision C ≠ Accepted ADR-07
Reviewed ≠ Accepted for Supabase adapter or migration work
ADR-03 remains Proposed
```

```text
This draft is not:
- an implemented Supabase adapter
- a Supabase schema or SQL/RLS policy pack
- a migration plan execution
- permission to modify documents-crypto
- permission to create Spec 002
- permission to synchronize Artifact content or PHI
- permission to change authentication or sessions
- permission to begin R2
```

```text
No production implementation is authorized by this draft.
```

This draft **must not** be used as justification to: implement a Supabase adapter; modify Supabase schemas, policies, or migrations; mutate `documents-crypto`; create Spec `002-supabase-local-first-and-migration-canonicalization`; synchronize Artifact content or PHI; change `afia-ui` auth/session/`PrivateRoute`/OTP behavior; redefine Artifact Store authority; begin R2 (`T031+`); or treat Proposed ADRs as Accepted.

---

## 1. Context and problem

### 1.1 Assigned architectural question

**ADR-07 proposes** how **Fanatir** may use **Supabase** as an **optional** adapter for identity, collaboration, and selected **non-authoritative** metadata while preserving the **local-first** source-of-truth boundary recorded by ratified **Decision C** and constrained by Proposed [ADR-05](./ADR-05-artifact-revision-run-storage.md), [ADR-15](./ADR-15-rust-first-polyglot-runtime.md), [ADR-02](./ADR-02-rust-trusted-host-boundary.md), and [ADR-04](./ADR-04-shared-primitive-ownership-and-versioning.md).

It does **not** implement the adapter; finalize remote schemas; execute migrations; mutate `documents-crypto`; create Spec 002; redefine Artifact Store persistence (**ADR-05**); redefine IPC (**ADR-06**); redesign authentication/sessions (**ADR-11**); or finalize PHI egress (**ADR-14**).

### 1.2 Why an explicit Supabase boundary matters

Fanatir reconstitutes a desktop-first, local-first system with optional cloud collaboration ([plan.md](../plan.md); [spec.md](../spec.md) Q4 / FR-015). Current baseline evidence shows Supabase used for auth/collaboration and a legacy encrypted-document path (`documents-crypto`) inventoried under T006 ([R0-supabase-inventory.md](../../../docs/program-memory/baseline/R0-supabase-inventory.md); [research.md](../research.md) R8).

Without an explicit adapter boundary, planning identifies these **risks** (prospective — not claimed as currently measured production failures unless separately evidenced):

| Risk | Why it matters |
| --- | --- |
| Supabase as accidental Artifact-content authority | Cloud rows replace local Artifact Store SoT |
| Unauthorized PHI / clinical sync | Decision C classes leave local control |
| Collaboration metadata confused with durable local truth | Membership/invite rows treated as Artifact/Revision authority |
| UI or cached remote rows as authority | Presentation/cache becomes SoT |
| Adapter schemas redefine Fanatir semantics | Remote shapes eclipse ADR-04 ownership |
| Cloud unavailability blocking local work | Local-first broken |
| Retry / dual-write duplicating mutations | Divergent authorities and integrity loss |
| Legacy `documents-crypto` expansion | PHI-egress seam grows contrary to Decision C |
| Indirect auth/session change via adapter design | Violates ADR-11 / dedicated-spec rule |
| Migration planning mistaken for migration authorization | Spec 002 / production mutation leapfrogged |

### 1.3 Accepted program and planning constraints

| Constraint | Source | Treatment |
| --- | --- | --- |
| Content/PHI/Artifact revisions → local Rust Artifact Store; Supabase = optional collab/identity; freeze `documents-crypto`; future Spec 002 authorized (not created) | Decision C — [plan.md](../plan.md) (**Ratified**); [research.md](../research.md) R14; [supabase-adapter.md](../contracts/supabase-adapter.md) | Ratified planning direction; **not** ADR-07 acceptance |
| Adapter must degrade: local-first features work when cloud unavailable | [supabase-adapter.md](../contracts/supabase-adapter.md) | Planning contract evidence |
| Auth/session preserved until dedicated migration spec | [spec.md](../spec.md) Q4; [supabase-adapter.md](../contracts/supabase-adapter.md); ADR-11 (not authored) | Binding prohibition on behavior change under 001 |
| Dual migration trees frozen; Spec 002 owns canonicalization | Decision C; research P1; tasks T029 | No migration mutation under T019 |
| T019 = draft only; Tier A; no production code | [tasks.md](../tasks.md) T019 | Binding for this task |
| No R2 before T030 (ADR-15/01/02/06 Accepted as applicable) | [tasks.md](../tasks.md) | Future-gate requirement |

### 1.4 Naming

| Name | Role in this ADR |
| --- | --- |
| **Fanatir** | Owns shared primitive semantics (**ADR-04**) and local Artifact authority direction under Decision C |
| **Supabase** | Optional remote collaboration/identity adapter candidate — **not** Artifact/clinical SoT |
| **Artifact Store** | Local Rust-controlled content authority (**ADR-05** Proposed) |
| **`documents-crypto`** | Legacy PHI-egress seam — freeze posture under Decision C |
| **`002-supabase-local-first-and-migration-canonicalization`** | Future specification name only — **not** created here |
| **Fehrest** | Separate product boundary (**ADR-08**, not authored) |
| **`afia-ui`** | Migration shell / UI presentation; not adapter architecture SoT (**ADR-03** Proposed) |

Never confuse Fehrest with Fanatir, or Supabase with the Artifact Store.

### 1.5 What this draft is not

This file is **not**: an implemented adapter; a Supabase schema; migration execution; permission to modify `documents-crypto`; permission to create Spec 002; permission to synchronize Artifact content or PHI; authentication/session change authority; or R2 authorization.

---

## 2. Decision drivers

| Driver | Source |
| --- | --- |
| Ratified Decision C authority boundary | plan.md; research R14; supabase-adapter.md |
| Local Artifact SoT (Proposed ADR-05) | ADR-05; Decision C |
| Shared semantics ownership | ADR-04 Proposed |
| Trusted Host mediation / network-egress direction | ADR-15 / ADR-02 Proposed |
| Local-first ≠ forbidding optional collaboration | spec.md FR-015 |
| Freeze `documents-crypto`; Spec 002 later | Decision C; tasks T028/T029 |
| Draft ≠ accepted; Tier A Supabase/local-first ADR | tasks.md T019 |

---

## 3. Proposed decision

### 3.1 Decision statement

**ADR-07 proposes** that:

1. **Supabase** would serve as an **optional** Fanatir **adapter** for identity, collaboration, and selected **non-authoritative** metadata/references — **not** as Artifact-content, clinical, Patient, Revision, or PolicyDecision source of truth.
2. **Authoritative** Decision C content classes (document/Artifact content, extracted entities, patient material, FHIR containing PHI, Artifact revisions) **would remain** in the **local Rust-controlled Artifact Store** under Trusted Host mediation ([ADR-05](./ADR-05-artifact-revision-run-storage.md) Proposed; Decision C **Ratified**).
3. Local authoritative workflows **would degrade safely** when Supabase is unavailable; cloud acknowledgment would **not** be required for local Artifact truth.
4. The adapter **would not** authorize itself, redefine ADR-04 semantics, bypass Trusted Host policy, or treat remote-row existence as authorization or local truth.
5. Legacy **`documents-crypto`** would remain under the ratified **freeze** posture; T019 does **not** execute T028 freeze documentation or mutate the path.
6. Migration canonicalization would be deferred to future Spec **`002-supabase-local-first-and-migration-canonicalization`** (charter via **T029**; not created here).
7. Authentication/session **behavior** remains preserved under 001; redesign → **ADR-11** + dedicated specification.
8. Full PHI classification/egress enforcement → **ADR-14** (not authored).

```text
Proposed optional adapter ≠ implemented adapter
Ratified Decision C ≠ Accepted ADR-07
Freeze posture ≠ T028 executed
Spec 002 name ≠ Spec 002 created
```

### 3.2 Decision C alignment (ratified planning direction)

Per [plan.md](../plan.md) Decision **C** (**Ratified**) and [supabase-adapter.md](../contracts/supabase-adapter.md) / [research.md](../research.md) R14:

| Topic | Ratified / planning direction |
| --- | --- |
| Local authority | Document content, extracted entities, patient material, FHIR containing PHI, Artifact revisions → local Rust Artifact Store |
| Supabase role | Optional collaboration / identity adapter — **not** content, clinical, Artifact, or patient SoT |
| Decision C nature | **Authority boundaries**, not adapter implementation details |
| `documents-crypto` | Freeze expansion; synthetic/test only; preserve; do not delete/mutate in this reconstitution phase (until ADR-07/14 acceptance gates as stated in planning evidence) |
| Spec 002 | Authorized by name only — **not** created in 001 |

**ADR-07 proposes** how the optional adapter boundary would align with Decision C. It does **not** claim a production adapter exists.

### 3.3 Adapter authority model (proposed)

| Actor | Proposed role | Must not (proposed) |
| --- | --- | --- |
| Fanatir / **ADR-04** | Own shared primitive semantics / `schemaVersion` | Let remote rows become semantic SoT |
| **ADR-05** Artifact Store | Local Artifact/Revision/Run content authority (Proposed) | Be replaced by Supabase content SoT |
| Rust Trusted Host (**ADR-15/02**) | Privileged local durable-mutation gateway; mediate elevated remote/network effects under policy | Be bypassed by UI or adapter self-authorization |
| Supabase adapter (this ADR) | Optional remote integration boundary for allowed metadata/identity/collab concerns | Become Artifact/clinical/PolicyDecision SoT; self-authorize |
| UI / `afia-ui` | Request intent; present results | Own authorization; treat remote/cache as Artifact SoT |
| Local cache | Non-authoritative performance aid | Become sole copy of truth or authorize access |
| Workers | Non-authoritative producers of candidates | Own remote or local durable Artifact mutation |
| Supabase rows | Remote **representations** of allowed metadata | Automatic Fanatir semantic authority |

Only **Trusted Host-mediated / governed** validation and authorization would make remote metadata writes or privileged remote effects acceptable — not adapter self-service and not “row exists ⇒ authorized.”

### 3.4 Allowed Supabase data classes (planning; not schemas)

At planning level, Supabase **may** be considered for **non-authoritative** categories such as:

- user identity **references** (not session redesign);
- collaboration metadata;
- workspace or project membership metadata;
- invitations;
- role/visibility metadata where auth boundaries allow and classification permits;
- non-sensitive remote references to locally authoritative Artifacts;
- synchronization **records/references** (not clinical content SoT);
- approved non-content cloud metadata only ([supabase-adapter.md](../contracts/supabase-adapter.md)).

Every remotely allowed category **would remain** subject to: minimum-necessary data; classification; authorization; validation; explicit ownership; local-versus-remote authority labeling.

**Do not** finalize fields, tables, or wire formats here.

### 3.5 Forbidden Supabase source-of-truth classes

Supabase **must not** become the authoritative source of truth for:

- Artifact content;
- Artifact Revisions;
- PHI-bearing patient material;
- extracted clinical entities;
- FHIR-with-PHI;
- local Run outputs where governed by ADR-05 as durable store authority;
- clinical evidence content;
- durable PolicyDecision authority;
- worker-local candidate output;
- secrets;
- encryption keys.

Encrypted remote PHI is **not** automatically safe or authorized. Legacy `documents-crypto` encryption does **not** authorize expansion or a new remote PHI content role.

### 3.6 Identity and collaboration boundary

| Concern | ADR-07 may… | ADR-07 must not… |
| --- | --- | --- |
| Identity references | Propose adapter boundaries for references/metadata | Redefine user identity as architecture SoT |
| Collaboration metadata | Propose optional collab metadata classes | Treat membership rows as Artifact authority |
| Authentication / session mechanics | Acknowledge preserve-current posture | Redesign OTP/session/`AuthContext`/`PrivateRoute` |
| Authorization decisions | Require host/policy mediation for privileged effects | Grant remote rows authorization authority |

Auth/session **behavior changes** require **ADR-11** plus a dedicated specification ([tasks.md](../tasks.md) prohibitions; [spec.md](../spec.md) Q4).

### 3.7 Local-first behavior (proposed)

**ADR-07 proposes** that:

- local authoritative workflows **would** degrade safely when Supabase is unavailable;
- remote unavailability **must not** corrupt or invalidate local Artifact truth;
- local authoritative writes **would not** depend on cloud acknowledgment;
- remote collaboration features **may** become unavailable or stale without changing local truth;
- reconnect and synchronization behavior **must** be explicit before implementation (currently unresolved — §11).

Offline/local-first behavior is **not claimed implemented**.

### 3.8 Remote references (proposed)

References from Supabase metadata to locally authoritative Artifacts **would**:

- not be the Artifact itself;
- not expose unsafe local filesystem paths;
- use stable, governed identifiers (exact format **unresolved**);
- not mutate local content when stale;
- not delete local Artifacts when a remote reference is deleted;
- not advance a local Revision pointer without Trusted Host authorization.

### 3.9 Adapter interface and mapping (planning seam only)

**ADR-07 proposes** a future adapter seam for:

- validated reads of allowed remote metadata;
- authorized metadata writes under host/policy mediation;
- mapping Fanatir primitives ↔ remote representations **without** transferring semantic ownership;
- mapping remote rows into **non-authoritative** adapter models;
- explicit adapter-contract version handling;
- error translation suitable for UI without leaking secrets/PHI/paths;
- audit correlation hooks (authoritative audit remains host — **ADR-02**).

**Do not** publish final interfaces, schemas, SQL, generated types, or client code here.

### 3.10 Version separation (proposed)

Keep distinct:

| Version domain | Owner / notes |
| --- | --- |
| ADR-04 `schemaVersion` | Shared primitive semantics |
| ADR-05 persistence-format version | Local Artifact Store formats |
| ADR-06 IPC / protocol version | Host IPC envelopes |
| Supabase **adapter contract version** | This ADR’s future adapter boundary |
| Remote database schema version | Remote provider schema (Spec 002 / later) |
| Package / application versions | Distribution / product releases |

Incompatible or unsupported adapter/remote versions **would fail closed**. Compatibility negotiation is **not claimed implemented**.

### 3.11 Validation and trust (proposed)

Remote inputs and remote reads would be treated as **untrusted until validated**. Future checks **would** address: unknown fields; unsupported schema/adapter versions; malformed identifiers; invalid references; invalid classification; unexpected tenant/workspace association; stale metadata; remote rows modified outside Fanatir; missing authorization context; overbroad result sets.

Remote-row existence **does not** prove authorization or local truth.

### 3.12 Remote mutation authority (proposed)

Remote metadata writes **would require**: Trusted Host or governed service mediation; explicit allowed operation; validation; authorization; minimum-necessary payload; classification check; audit correlation; explicit local-versus-remote ownership.

Direct UI→Supabase writes are **not** proposed as an architecture shortcut. Supabase policies alone are **not** complete authorization.

### 3.13 Synchronization boundary (proposed; not implemented)

Synchronization, if ever authorized later, would be limited to **allowed metadata and references**.

Clarifications:

- Artifact content synchronization is **not** authorized by this draft;
- PHI synchronization is **not** authorized by this draft;
- synchronization does **not** change local Artifact SoT;
- remote updates may become **candidate metadata**, not automatic durable truth;
- every sync direction would need explicit ownership and conflict rules before impl;
- dual-write / dual-read / background sync are **not assumed**.

Synchronization is **not claimed to exist**.

### 3.14 Offline queues and conflict handling

Canonical behavior is largely **unresolved**. Recorded as open (§11): whether offline remote-write queues exist; queue ownership/encryption; retry limits; conflict detection; last-write-wins prohibition or allowance; merge behavior; stale-read behavior; user-visible conflicts; cancellation; duplicate suppression; ambiguous completion.

**Do not** silently choose last-write-wins.

### 3.15 Retry and idempotency

No settled retry/idempotency policy is asserted. Open (§11): retryable operations; idempotency keys; duplicate suppression; ownership; limits; timeouts; ambiguous remote completion; replay after restart; durable local/remote ordering.

Automatic retry of sensitive or non-idempotent writes **must not** be implied.

### 3.16 Consistency model

Consistency model remains **unresolved** as accepted architecture. Planning concerns include: eventual consistency for collaboration metadata (candidate discussion only); local authoritative consistency for Artifact content; stale remote metadata; read-your-writes; monotonic updates; ordering; conflict visibility.

Eventual consistency is **not** described as Accepted architecture.

### 3.17 Cache boundary (proposed)

Caches **would** be: non-authoritative; possibly stale; invalidatable; classification-aware; free of prohibited PHI/Artifact content; unable to silently authorize access; not the only copy of collaboration metadata relied upon as durable truth.

Exact cache technology/retention remain **unresolved**.

### 3.18 `documents-crypto` freeze (ratified posture; T028 not executed)

Accurately record Decision C / [supabase-adapter.md](../contracts/supabase-adapter.md) freeze posture:

- do not expand the legacy `documents-crypto` path;
- preserve existing evidence for inspection;
- synthetic/test usage only where canonically stated;
- do not delete or mutate it during T019;
- ADR-07 **aligns with / records** the freeze boundary;
- **T028** later operationalizes freeze documentation (depends on T019; **not executed** by this draft).

T019 does **not** modify code, schemas, migrations, or legacy data.

### 3.19 Spec 002 boundary

Future specification name only:

`002-supabase-local-first-and-migration-canonicalization`

Clarifications:

- T019 does **not** create Spec 002;
- **T029** is the future charter task (depends on T019/T028);
- Spec 002 would govern migration canonicalization detail;
- ADR-07 may identify questions Spec 002 must resolve;
- no migration execution is authorized.

No Spec 002 directory, file, checklist, charter, or scaffold is created here.

### 3.20 Migration boundary

| Layer | May… | Must not (T019)… |
| --- | --- | --- |
| 1. ADR-07 architecture planning | Propose adapter/local-first boundaries | Execute migrations |
| 2. T028 freeze documentation | Later task | Claim executed here |
| 3. T029 Spec 002 charter | Later task | Create Spec 002 here |
| 4. Future Spec 002 | Canonicalization planning | Run now |
| 5. Production migration execution | Separately authorized later | Modify Supabase migrations/schemas/policies/RLS; mutate `documents-crypto`; backfill; dual-write/dual-read; rename tables; deploy remote changes |

### 3.21 PHI and classification (claim language)

Planning constraints:

- no PHI-bearing Artifact content in Supabase as authorized SoT;
- no clinical content SoT role for Supabase;
- minimum-necessary remote metadata;
- classification-aware adapter behavior;
- fail-closed unknown classification for privileged remote publication/egress-relevant paths;
- redacted diagnostics; no unsafe identifiers/local paths; no PHI in ordinary logs; no secrets/keys in rows or telemetry.

Full classification/egress enforcement → **ADR-14**. No regulatory certification claimed.

### 3.22 Secrets and credentials

Supabase credentials and tokens are **secrets**. They would remain under Trusted Host or approved secret-management boundaries. UI-visible configuration must not expose privileged service credentials. Adapter logs must not expose tokens. Legacy `ENCRYPTION_KEY` evidence does **not** authorize a new crypto system.

Secrets backend choice remains **unresolved** (see ADR-02 open questions).

### 3.23 Network failure and degraded operation (proposed posture)

Planning coverage for: unavailable network; timeout; authentication failure; authorization failure; rate limiting; partial remote completion; stale reads; remote schema incompatibility; service outage.

Local Artifact truth **must remain** safe and usable where local-first requirements permit. Automatic recovery is **not claimed**.

### 3.24 Remote deletion, retention, and audit

Treat as **unresolved** unless later evidence settles: deletion propagation; tombstones; remote retention; account/workspace removal; reference cleanup; orphaned rows; audit-event ownership; remote audit retention; legal/policy holds.

Remote metadata deletion **must not** automatically delete locally authoritative Artifact content. Authoritative audit ownership remains Trusted Host (**ADR-02**).

### 3.25 Row-level security and database policies

ADR-07 **may** state future defense-in-depth requirements (planning). It **must not**: publish RLS policies; claim RLS is implemented; treat RLS as the only authorization boundary; modify database policies; assume tenant/workspace isolation is proven.

Final SQL/policy design → later specifications/implementation tasks / Spec 002 as applicable.

### 3.26 Export and egress

Secure export and PHI egress belong to **ADR-14** / Trusted Host. Supabase collaboration metadata does **not** authorize content export. ExportManifest semantics belong to **ADR-04**. Remote references do **not** grant access to local content. No egress implementation is authorized.

### 3.27 Fehrest and DeepMed boundaries

| Boundary | Owner |
| --- | --- |
| Fehrest integration / release | **ADR-08** (not authored) |
| DeepMed output operations | **ADR-09** (not authored) |
| Supabase holding Fehrest/DeepMed product data as SoT | **Not proposed** |
| Collaboration references | Must not redefine Fehrest/DeepMed product boundaries |

### 3.28 Proposed invariants (architecture authority only if later Accepted)

1. Supabase is optional and non-authoritative for Decision C content classes.
2. Local Artifact Store remains Artifact-content/revision SoT under host mediation.
3. Local authoritative work does not require Supabase availability.
4. Adapter versions are explicit and fail closed when incompatible.
5. `documents-crypto` remains frozen under Decision C posture until separately authorized change.

Until Accepted, these are **proposed planning invariants** only.

---

## 4. Scope

### In scope for this draft

- Supabase Adapter and Local-First Boundary (planning)
- Decision C alignment for optional collab/identity vs local Artifact SoT
- Freeze / Spec 002 / migration authority separations
- Explicit unresolved questions for sync/retry/conflict/cache/RLS/deletion

### Out of scope

- Implementing adapters, schemas, RLS, migrations, or sync
- Mutating `documents-crypto` or creating Spec 002
- Auth/session redesign; Fehrest/DeepMed product behavior; PHI egress engine
- R2 / T031+ execution

---

## 5. Alternatives considered

### Option A — Supabase as primary Artifact-content SoT

| | |
| --- | --- |
| Summary | Keep/expand cloud documents as content authority |
| Costs / risks | Contradicts ratified Decision C |
| Draft disposition | **Rejected** |

### Option B — Supabase as PHI/clinical store

| | |
| --- | --- |
| Summary | Remote clinical/PHI content SoT |
| Costs / risks | Forbidden by Decision C / supabase-adapter forbidden list |
| Draft disposition | **Rejected** |

### Option C — Direct UI-to-Supabase writes as architecture shortcut

| | |
| --- | --- |
| Summary | Browser/client owns remote writes |
| Costs / risks | Bypasses Trusted Host mediation; authz/audit/PHI risk |
| Draft disposition | **Rejected** as architecture shortcut |

### Option D — Dual-write local and remote Artifact content

| | |
| --- | --- |
| Summary | Write content to Artifact Store and Supabase |
| Costs / risks | Dual authority; integrity/PHI risk; contradicts Decision C |
| Draft disposition | **Rejected** |

### Option E — Offline-first remote queue with automatic conflict merging

| | |
| --- | --- |
| Summary | Assume queues + automatic merges |
| Costs / risks | Not evidenced; last-write-wins/merge hazards; overclaim |
| Draft disposition | **Not accepted**; remains unresolved option space |

### Option F — No Supabase integration

| | |
| --- | --- |
| Summary | Remove optional collab/identity adapter entirely |
| Costs / risks | Conflicts with FR-015 optional collaboration and Q4 reuse posture |
| Draft disposition | **Rejected** as exclusive posture (optional adapter retained as Proposed) |

### Option G — Unrestricted generic database adapter

| | |
| --- | --- |
| Summary | Treat Supabase as general-purpose SoT store |
| Costs / risks | Scope expansion; Decision C violation |
| Draft disposition | **Rejected** |

### Option H — Optional collaboration/identity adapter with local Artifact truth preserved (proposed)

| | |
| --- | --- |
| Summary | Decision C-aligned optional adapter; local SoT preserved |
| Benefits | Local authority; optional collab; outage tolerance; PHI containment |
| Costs / risks | Mapping/sync/conflict/versioning complexity (see §6) |
| Draft disposition | **Proposed** |

---

## 6. Consequences

### Positive (prospective — if later accepted and executed under separate authorization)

- Preserved local Artifact authority under Decision C
- Optional collaboration without mandatory cloud
- Cloud-outage tolerance for local authoritative work
- Clearer PHI / content boundary
- Replaceable remote provider potential
- Reduced coupling of UI to remote SoT
- Explicit adapter validation surface
- Migration containment via Spec 002 separation

### Costs and risks

| Cost / risk | Note |
| --- | --- |
| Dual authority confusion | Metadata vs content labeling discipline |
| Schema mapping complexity | Must not steal ADR-04 ownership |
| Stale metadata | Cache/sync unresolved |
| Conflict handling | Unresolved; no silent LWW |
| Retry/idempotency complexity | Unresolved |
| Remote policy maintenance | RLS not sole authz |
| Offline queue risks | If ever introduced |
| Adapter-version compatibility | Fail closed required |
| Deletion/retention complexity | Unresolved |
| Migration burden | Spec 002 / later |
| Observability and audit | Host remains audit SoT |
| Adapter scope expansion | Must stay non-content |
| Auth/session coupling | Forbidden redesign under 001 |

### Rollback (planning only)

If rejected before acceptance: revert this ADR file; retain Decision C / supabase-adapter planning evidence; do **not** implement adapter/migrations from this draft. Post-acceptance withdrawal would need separately authorized migration — **not** defined here.

---

## 7. Non-goals

ADR-07 does **not**:

- implement a Supabase adapter;
- modify Supabase schemas;
- modify RLS or database policies;
- migrate data;
- mutate `documents-crypto`;
- create Spec 002;
- synchronize Artifact content;
- synchronize PHI;
- redefine Artifact Store authority (**ADR-05**);
- define IPC commands (**ADR-06**);
- define Fehrest integration (**ADR-08**);
- define DeepMed operations (**ADR-09**);
- redesign authentication or sessions (**ADR-11**);
- finalize PHI egress policy (**ADR-14**);
- choose final retry/conflict/cache mechanisms;
- authorize remote storage as Artifact SoT;
- begin R2 / T031+.

---

## 8. Relationship to other ADRs

| ADR | Status in repo | Relationship |
| --- | --- | --- |
| ADR-15 | Proposed | Rust-first durable mutation; adapter cannot become content SoT |
| ADR-01 | Proposed | Composition; WebView not SoT |
| ADR-02 | Proposed | Host/network/egress mediation; cloud adapter deferred here |
| ADR-03 | Proposed | `afia-ui` strangler; not adapter SoT |
| ADR-04 | Proposed | Shared semantics; adapter mappings are non-authoritative |
| ADR-05 | Proposed | Local Artifact/Revision/Run SoT; defers adapter detail here |
| ADR-06 | Proposed | IPC/worker contracts ≠ remote adapter protocol |
| ADR-08 | Not authored | Fehrest integration |
| ADR-09 | Not authored | DeepMed operations |
| ADR-11 | Not authored | Auth/session preservation / change path |
| ADR-14 | Not authored | PHI classification and egress enforcement |

---

## 9. Gates and authority

```text
T019 produces a Proposed ADR-07 draft only.
Tier A independent review is required (Supabase/local-first ADR).
Passing Tier A review is not architecture acceptance.
tasks.md founder-acceptance field for T019 is: No — draft only; Decision C already ratified.
Decision C is ratified planning direction; ADR-07 is not Accepted by Decision C alone.
T030 remains incomplete.
At T030, ADR-07 is expected to be listed Reviewed or Accepted — it is not automatically Accepted.
ADR-07 is NOT in the mandatory T030 Accepted set that opens R2 (ADR-15/01/02/06).
Later work such as T057 requires ADR-07 and ADR-14 Accepted.
T031+ and R2 remain separately gated.
Adapter implementation, migration, schema changes, synchronization, and PHI egress require separate authorization.
T028 (freeze docs) and T029 (002 charter) are downstream tasks — not executed by T019.
```

| Gate | Meaning for ADR-07 |
| --- | --- |
| T019 | Draft authoring (this task) |
| Tier A | Mandatory independent review |
| Founder draft acceptance | Not required by tasks.md for T019 work product |
| T030 | R1 gate; ADR-07 listed Reviewed or Accepted; not mandatory Accepted-for-R2 |
| T028 / T029 | Downstream freeze docs / Spec 002 charter |
| T057 | R4 adapter non-authority tests after ADR-07/14 Accepted |
| T031+ / R2 | Separately gated; not authorized here |

---

## 10. Validation and acceptance plan

```text
T019 completion produces a draft for Tier A review.
Architecture acceptance is not performed by T019.
```

Planning reviews **may** include: Decision C / supabase-adapter / research R14 consistency; ADR-05 SoT non-absorption; ADR-04 semantic non-theft; auth/session non-redesign; freeze/Spec 002 separation; Tier A independent review ([tasks.md](../tasks.md) T019).

---

## 11. Unresolved questions

T019 does **not** resolve these. Attempted resolution: **NO**.

| ID | Question | May remain open in draft review? | Must resolve before implementation? | Belongs elsewhere? |
| --- | --- | --- | --- | --- |
| U-ADR07-1 | Allowed collaboration metadata fields | YES | YES | This ADR + later schemas |
| U-ADR07-2 | Identity-reference boundaries | YES | YES | This ADR; auth details → ADR-11 |
| U-ADR07-3 | Workspace/project membership mapping | YES | YES | This ADR + ADR-04 Workspace/Project |
| U-ADR07-4 | Remote reference format | YES | YES | This ADR + ADR-05 identifiers |
| U-ADR07-5 | Adapter contract version syntax | YES | YES | This ADR |
| U-ADR07-6 | Remote schema version governance | YES | YES | Spec 002 / later |
| U-ADR07-7 | Unknown-field handling | YES | YES | This ADR |
| U-ADR07-8 | Remote-read validation rules | YES | YES | This ADR + host |
| U-ADR07-9 | Remote-write authorization model detail | YES | YES | This ADR + ADR-02/14 |
| U-ADR07-10 | Direct UI access prohibition exceptions | YES | YES before any UI exception | This ADR; prefer none |
| U-ADR07-11 | Local cache scope | YES | YES | This ADR |
| U-ADR07-12 | Cache invalidation | YES | YES | This ADR |
| U-ADR07-13 | Offline queue existence | YES | YES before queues | This ADR |
| U-ADR07-14 | Queue encryption | YES | YES if queues exist | Secrets/host |
| U-ADR07-15 | Retry eligibility | YES | YES | This ADR |
| U-ADR07-16 | Idempotency keys | YES | YES | This ADR |
| U-ADR07-17 | Duplicate suppression | YES | YES | This ADR |
| U-ADR07-18 | Conflict detection | YES | YES | This ADR |
| U-ADR07-19 | Merge policy | YES | YES | This ADR — no silent LWW |
| U-ADR07-20 | Consistency model | YES | YES before sync claims | This ADR |
| U-ADR07-21 | Stale-read behavior | YES | YES | This ADR |
| U-ADR07-22 | Reconnect behavior | YES | YES | This ADR |
| U-ADR07-23 | Dual-read / dual-write prohibition confirmation | YES | YES | This ADR — default reject dual content write |
| U-ADR07-24 | Deletion propagation | YES | YES | This ADR + ADR-05 |
| U-ADR07-25 | Remote retention | YES | Before retention claims | Possibly ADR-14 / policy |
| U-ADR07-26 | Row-level security requirements | YES | YES before remote write of sensitive metadata | Spec 002 / later SQL tasks |
| U-ADR07-27 | Audit correlation | YES | YES | ADR-02 / later audit |
| U-ADR07-28 | Remote outage handling detail | YES | YES | This ADR |
| U-ADR07-29 | Migration ownership handoff | YES | Before migrations | Spec 002; T029 charter |
| U-ADR07-30 | Spec 002 handoff question list | YES | Before Spec 002 drafting | T029 / Spec 002 |
| U-ADR07-31 | `documents-crypto` preservation ops detail | YES | Before any change | T028; ADR-14 |
| U-ADR07-32 | Adapter replacement / provider portability | YES | Before multi-provider claims | This ADR |
| U-ADR07-33 | PHI exclusion verification method | YES | YES before remote metadata paths | ADR-14 + tests (e.g. T057) |

---

## 12. Security and privacy claim language

```text
Claim language only.
No regulatory certification is asserted.
No adapter, sync, RLS, migration, or documents-crypto mutation is asserted.
PHI posture follows Decision C + classification planning; egress engine detail remains ADR-14.
```

---

## 13. References

- [tasks.md](../tasks.md) — T019, T028, T029, T030, T057
- [plan.md](../plan.md) — Decision C; ADR-07 roadmap row
- [spec.md](../spec.md) — Q4; FR-015
- [research.md](../research.md) — R8 / R14 / P1
- [supabase-adapter.md](../contracts/supabase-adapter.md) — optional adapter contract
- [R0-supabase-inventory.md](../../../docs/program-memory/baseline/R0-supabase-inventory.md) — T006 inventory evidence
- [data-model.md](../data-model.md) — shared primitive boundaries
- [shared-primitives.md](../contracts/shared-primitives.md) — minimum exported types
- [ADR-15](./ADR-15-rust-first-polyglot-runtime.md) (Proposed)
- [ADR-01](./ADR-01-platform-and-desktop-composition-tauri-2.md) (Proposed)
- [ADR-02](./ADR-02-rust-trusted-host-boundary.md) (Proposed)
- [ADR-03](./ADR-03-afia-ui-strangler-migration.md) (Proposed)
- [ADR-04](./ADR-04-shared-primitive-ownership-and-versioning.md) (Proposed)
- [ADR-05](./ADR-05-artifact-revision-run-storage.md) (Proposed)
- [ADR-06](./ADR-06-worker-and-ipc-contracts.md) (Proposed)

---

```text
While Proposed:
- no Supabase adapter implementation from this file;
- no schema/RLS/migration mutation;
- no documents-crypto mutation;
- no Spec 002 creation;
- no Artifact content or PHI synchronization;
- no auth/session behavior change;
- no R2 / T031+ work from this file alone.
```

```text
End of ADR-07 Proposed draft.
```
