# ADR-05 — Artifact/Revision/Run Storage

| Field | Value |
| --- | --- |
| **ADR** | ADR-05 |
| **Title** | Artifact/Revision/Run Storage |
| **Status** | **Proposed** (draft only) |
| **Task origin** | T018 |
| **Acceptance / review posture** | Tier A independent review after drafting; **T030** expects ADR-05 listed **Reviewed or Accepted** (not automatically Accepted; **not** in the mandatory Accepted set that opens R2). **T037** requires ADR-05 **Accepted** before Artifact Store foundation |
| **Feature** | `001-fanatir-repository-and-architecture-reconstitution` |
| **Related founder decision** | Decision C (content/PHI/Artifact revisions → local Rust Artifact Store; Supabase = optional collab/identity) — **Ratified** in accepted [plan.md](../plan.md) |
| **Constraining drafts** | [ADR-15](./ADR-15-rust-first-polyglot-runtime.md) (Proposed); [ADR-02](./ADR-02-rust-trusted-host-boundary.md) (Proposed); [ADR-04](./ADR-04-shared-primitive-ownership-and-versioning.md) (Proposed) |
| **Related drafts** | [ADR-01](./ADR-01-platform-and-desktop-composition-tauri-2.md) (Proposed); [ADR-03](./ADR-03-afia-ui-strangler-migration.md) (Proposed); [ADR-06](./ADR-06-worker-and-ipc-contracts.md) (Proposed) |
| **Planning evidence** | [data-model.md](../data-model.md); [shared-primitives.md](../contracts/shared-primitives.md); [trusted-host-ipc.md](../contracts/trusted-host-ipc.md); [worker-runtime.md](../contracts/worker-runtime.md); [supabase-adapter.md](../contracts/supabase-adapter.md); [research.md](../research.md) R14/P1; [plan.md](../plan.md) ADR-05 row / Decision C; [tasks.md](../tasks.md) T018/T030/T037 |
| **Architecture authority of this file** | **NO** — until a valid stage-gate acceptance action records Accepted |
| **Implementation authorization** | **NO** |

```text
This document is a planning draft and is not accepted architecture authority.
```

```text
Status: Proposed
Draft ADR ≠ architecture acceptance
T018 completion ≠ T030 acceptance
Passing Tier A review ≠ Accepted
tasks.md does not require founder acceptance of the T018 draft work product
Founder acceptance of a draft work product ≠ architecture acceptance
ADR-15 / ADR-01 / ADR-02 / ADR-03 / ADR-04 / ADR-06 remain Proposed and non-authoritative
```

```text
This draft is not:
- an implemented Artifact Store
- a production storage format
- a database schema or DDL
- a filesystem migration
- permission to mutate existing artifacts
- permission to alter Supabase
- permission to begin R2
```

```text
No production implementation is authorized by this draft.
```

This draft **must not** be used as justification to: implement the Artifact Store; create database or filesystem migrations; publish production storage formats or DDL; mutate existing Artifacts/Revisions/Runs; alter Supabase or `documents-crypto`; change `afia-ui` persistence behavior; wire Fehrest or DeepMed storage; begin R2 (`T031+` / `T037`); or treat Proposed ADRs as Accepted.

---

## 1. Context and problem

### 1.1 Assigned architectural question

**ADR-05 proposes** how **Fanatir** would store and make authoritative **Artifact**, **Revision**, and **Run** data under the local-first, Rust-mediated direction recorded by ratified **Decision C** and constrained by Proposed [ADR-15](./ADR-15-rust-first-polyglot-runtime.md), [ADR-02](./ADR-02-rust-trusted-host-boundary.md), and [ADR-04](./ADR-04-shared-primitive-ownership-and-versioning.md).

It does **not** implement the Artifact Store; finalize shared primitive schemas (**ADR-04**); define IPC commands/envelopes (**ADR-06**); define Fehrest integration (**ADR-08**); define DeepMed product operations (**ADR-09**); redesign authentication/sessions (**ADR-11**); or finalize PHI egress / classification enforcement (**ADR-14**).

### 1.2 Why explicit Artifact/Revision/Run Storage matters

Fanatir reconstitutes a polyglot system: React/`afia-ui` presentation; proposed Rust Trusted Host; supervised workers; and sibling products such as **Fehrest** and DeepMed that may produce or consume durable content through governed boundaries ([plan.md](../plan.md); [data-model.md](../data-model.md)).

Planning evidence requires durable results with provenance ([plan.md](../plan.md); [data-model.md](../data-model.md)). Without an explicit storage architecture decision, planning identifies these **risks** (prospective — not claimed as currently measured production failures unless separately evidenced):

| Risk | Why it matters |
| --- | --- |
| UI / browser state as accidental truth | Presentation state becomes de-facto Artifact SoT |
| Worker-local files as authority | Candidate outputs become durable without host validation |
| Filesystem paths as contract identity | Paths drift, escape, or collide with stable Artifact identity |
| Database rows as semantic SoT | Persistence shapes replace Fanatir language-neutral meanings ([ADR-04](./ADR-04-shared-primitive-ownership-and-versioning.md) Proposed) |
| Partial or failed writes | Incomplete bytes become “published” Artifacts |
| Revision mutation | History and provenance become untrustworthy |
| Content-integrity drift | Bytes and digests diverge silently |
| Incompatible persistence formats | Host/UI/workers cannot agree on stored shapes |
| PHI leakage | Sensitive content leaves local control via adapter, logs, or paths |
| Unauthorized Supabase content SoT | Cloud adapter silently becomes Artifact/PHI authority contrary to Decision C |

R0 and planning documents record a verified legacy `documents-crypto` PHI-egress seam in Supabase and a ratified direction to move content/PHI/Artifact revision authority to a local Rust-controlled Artifact Store ([research.md](../research.md) R14; [supabase-adapter.md](../contracts/supabase-adapter.md)). This ADR proposes the **local store planning model**; it does **not** claim the Artifact Store already exists.

### 1.3 Accepted program and planning constraints

| Constraint | Source | Treatment |
| --- | --- | --- |
| Content/PHI/Artifact revisions → local Rust Artifact Store; Supabase = optional collab/identity; freeze `documents-crypto`; future **002** authorized (not created) | Decision C — [plan.md](../plan.md) founder decisions (**Ratified**); [research.md](../research.md) R14 | Ratified planning direction; **not** ADR-05 acceptance |
| Durable Artifact/Revision/Run mutation only through Rust Trusted Host | Decision A / [ADR-15](./ADR-15-rust-first-polyglot-runtime.md) (Proposed); [data-model.md](../data-model.md) | Proposed language/host direction |
| Host authorizes Artifact/Revision/Run mutations; store design deferred here | [ADR-02](./ADR-02-rust-trusted-host-boundary.md) (Proposed) | Proposed host gateway; this ADR proposes store behind it |
| Shared meanings / `schemaVersion` owned by Fanatir; persistence-format version deferred to ADR-05 | [ADR-04](./ADR-04-shared-primitive-ownership-and-versioning.md) (Proposed) | Sequencing + ownership split |
| Large binaries via Artifact URIs, not ordinary IPC bodies; `artifact.put`/`artifact.get` Rust-only mutation path | [trusted-host-ipc.md](../contracts/trusted-host-ipc.md); [ADR-06](./ADR-06-worker-and-ipc-contracts.md) (Proposed) | IPC ≠ store |
| ADR-04 before ADR-05; T018 draft only; Tier A; no production code | [tasks.md](../tasks.md) | Binding for this task |
| No R2 before T030 (ADR-15/01/02/06 Accepted as applicable); T037 needs ADR-05 Accepted | [tasks.md](../tasks.md) | Future-gate requirements |

### 1.4 Naming

| Name | Role in this ADR |
| --- | --- |
| **Fanatir** | Owns shared primitive semantics (**ADR-04**) and the proposed Artifact Store as local content authority under Decision C |
| **Artifact Store** | Proposed local Rust-controlled persistence for Artifact/Revision/Run content under Trusted Host mediation |
| **Artifact/Revision/Run Storage** | Exact ADR-05 title / planning subject |
| **Fehrest** | Future consumer/producer via governed integration (**ADR-08**); **not** the Artifact Store; **not** Patient clinical SoT |
| **`afia-ui`** | Migration shell / UI presentation; **not** Artifact SoT ([ADR-03](./ADR-03-afia-ui-strangler-migration.md) Proposed) |

Never confuse Fehrest with Fanatir, or `afia-ui` with Artifact Store authority.

### 1.5 What this draft is not

This file is **not**: an implemented Artifact Store; a production storage format; a database schema; a filesystem migration; permission to mutate existing artifacts; permission to alter Supabase; authentication/session change authority; or R2 authorization.

---

## 2. Decision drivers

| Driver | Source |
| --- | --- |
| Ratified Decision C local content/PHI SoT | plan.md; research R14; supabase-adapter.md |
| Rust-first durable mutation | ADR-15 Proposed; data-model.md |
| Trusted Host mutation gateway | ADR-02 Proposed |
| Semantic vs persistence ownership split | ADR-04 Proposed |
| IPC Artifact URI / non-body binaries | ADR-06 Proposed; trusted-host-ipc.md |
| Provenance and immutable Revision planning | data-model.md |
| Encrypted local storage seam (future T037) | tasks.md T037; plan R2 |
| Draft ≠ accepted; Tier A Artifact Store ADR | tasks.md T018 |

---

## 3. Proposed decision

### 3.1 Decision statement

**ADR-05 proposes** that:

1. Fanatir’s **authoritative** Artifact content, Artifact revisions, PHI-bearing material covered by Decision C, extracted entities destined for durable authority, and related Run persistence for durable results **would** reside in a **local Rust-controlled Artifact Store** mediated exclusively by the **Rust Trusted Host**.
2. **Publication** into that store — not UI state, worker-local files, arbitrary filesystem paths, or unvalidated adapter writes — is what would make content **authoritative**.
3. A **published Revision** would be **immutable**; replacement would occur by creating a **new** Revision and advancing the Artifact’s current-revision reference under host-mediated rules.
4. ADR-05 would own **persistence behavior**, **storage authority**, **persistence-format versioning**, **integrity/publication semantics**, and **URI/locator resolution** for store-backed content.
5. [ADR-04](./ADR-04-shared-primitive-ownership-and-versioning.md) (Proposed) would retain **semantic ownership** and shared `schemaVersion` for Artifact/Revision/Run meanings.
6. [ADR-06](./ADR-06-worker-and-ipc-contracts.md) (Proposed) would retain **IPC commands, envelopes, and protocol versions** (including planning `artifact.put` / `artifact.get` / `run.*` command names).
7. Supabase would remain **non-authoritative** for Artifact content / PHI-bearing revisions under ratified Decision C; adapter detail → **ADR-07** (not authored).
8. Fehrest / DeepMed / auth / PHI-egress detail remain deferred to **ADR-08 / ADR-09 / ADR-11 / ADR-14** (not authored unless later evidence shows otherwise).

```text
Proposed Artifact Store ≠ implemented store
Proposed persistence model ≠ production schema
Proposed immutability rules ≠ enforced revision engine
Ratified Decision C direction ≠ Accepted ADR-05
```

### 3.2 Decision C alignment (ratified planning direction)

Per [plan.md](../plan.md) founder Decision **C** (**Ratified**) and [research.md](../research.md) R14 / [supabase-adapter.md](../contracts/supabase-adapter.md):

| Topic | Ratified / planning direction |
| --- | --- |
| Authoritative storage | Document content, extracted entities, patient material, FHIR containing PHI, and Artifact revisions → **local Rust-controlled Artifact Store** |
| Supabase role | Optional collaboration / identity adapter — **not** content, clinical, Artifact, or patient SoT |
| Legacy `documents-crypto` | Freeze expansion; synthetic/test only until ADR-07/14; preserve for inspection; do not delete/mutate in this reconstitution phase |
| Future Spec 002 | `002-supabase-local-first-and-migration-canonicalization` authorized by name only — **not** created here |

**ADR-05 proposes** how the local store would implement the Decision C boundary. It does **not** claim the store exists, and it does **not** authorize Supabase mutation or Spec 002 creation.

### 3.3 Authority model (proposed)

| Actor | Proposed role | Must not (proposed) |
| --- | --- | --- |
| Fanatir (ADR-04) | Own shared primitive semantics / schemaVersions for Artifact, Revision, Run, Source, Relationship, ExportManifest, etc. | Own store DDL as semantic SoT |
| Rust Trusted Host (ADR-15/02) | Sole durable-mutation **gateway**; validate, classify-check (with ADR-14), authorize, audit, publish | Be bypassed by UI/workers/adapters |
| Artifact Store (this ADR) | Persist published Artifacts/Revisions/Runs; resolve store URIs/locators; enforce immutability/integrity/publication rules | Self-authorize outside host; redefine ADR-04 meanings |
| UI / `afia-ui` / browser state | Request intent; present results | Be Artifact SoT; choose durable roots as authority |
| Workers / sidecars | Produce **candidate** outputs under supervision | Directly mutate authoritative Artifacts; self-publish trusted outputs |
| Supabase adapters | Optional collab/identity / authorized metadata or references | Become Artifact-content or PHI-revision SoT |
| Fehrest | Future governed consumer/producer (**ADR-08**) | Be the Artifact Store or Patient clinical SoT |
| DeepMed | Future governed producer of candidate outputs (**ADR-09**) | Own durable Artifact mutation or storage authority |

Only **Trusted Host-mediated, validated publication** into the Artifact Store would make content authoritative.

### 3.4 Artifact, Revision, and Run planning model

Planning inventory from [data-model.md](../data-model.md) and [shared-primitives.md](../contracts/shared-primitives.md). These are **boundary sketches**, not finalized fields, schemas, cardinalities, or wire formats.

#### Artifact (planning)

A durable content object with provenance. Planning concepts include: identity; project association; kind; URI/locator into the local store; current revision reference; source references; classification; creation provenance (`createdBy` / `createdAt` sketches).

**Rules (planning):** AI summaries are not original sources; local Artifact store is host-mediated; Artifact without Revision is invalid for durable store ([data-model.md](../data-model.md)).

| Concern | Owner |
| --- | --- |
| Meaning of Artifact | **ADR-04** (Proposed) |
| How Artifact bytes/refs are stored, published, and resolved | **ADR-05** (this draft) |

#### Revision (planning)

An immutable version of an Artifact. Planning concepts include: identity; Artifact identity; optional parent revision; content digest; creation time; author/provenance.

| Concern | Owner |
| --- | --- |
| Meaning of Revision / immutability as shared concept | **ADR-04** (Proposed) |
| Persistence of immutable bytes, digests, parent links, publication | **ADR-05** (this draft) |

#### Run (planning)

An execution record for tools/models/workers. Planning concepts include: identity; project; capability; worker; optional model identity; input/output Artifact ids; status; timing; audit correlation sketches. DeepMed invocations **must** create Runs with model + provenance at the planning level ([data-model.md](../data-model.md)).

| Concern | Owner |
| --- | --- |
| Meaning of Run / status vocabulary sketches | **ADR-04** (Proposed) |
| Persisting Run records and linking published inputs/outputs | **ADR-05** (this draft) |
| DeepMed-specific pipeline behavior | **ADR-09** (not authored) |

Related planning entities (**Source**, **Relationship**, **ExportManifest**) remain semantically owned under **ADR-04**. ADR-05 may persist references and store-backed content those entities point to; it does not redefine their meanings.

### 3.5 Source of truth (proposed)

| Candidate | Proposed disposition |
| --- | --- |
| UI / React state | **Not authoritative** |
| Browser-local storage | **Not authoritative** |
| Worker-local files / temp outputs | **Not authoritative** until host-validated publication |
| Arbitrary filesystem paths | **Not** stable contract identity; may be implementation details behind host-validated locators |
| Database rows / indexes | May **implement** storage; must **not** replace Fanatir language-neutral semantic ownership (**ADR-04**) |
| Supabase documents / adapter tables | **Not** authoritative Artifact-content SoT under Decision C |
| Published Artifact Store content after host validation | **Proposed authoritative** local SoT for Decision C content classes |

**Proposed rule:** failed, partial, or unverified writes do **not** become authoritative. Exact atomic-write and crash-recovery protocols remain **unresolved** (§11).

### 3.6 Content and metadata authority (proposed; partially unresolved)

| Category | Proposed planning posture | Open? |
| --- | --- | --- |
| Published Revision **content bytes** | Immutable after publication | No (principle); mechanism open |
| Content digest on Revision | Required planning concept (`contentHash`) | Algorithm/upgrade open |
| Artifact **current-revision pointer** | Mutable under host-mediated advancement only | Exact concurrency rules open |
| Classification on durable entities | Required; unknown/fail-closed interaction with ADR-14 | Enforcement detail → ADR-14 |
| Provenance / source refs / Run links | Persistable metadata associated with published objects | Field finalization open |
| Indexes / catalogs | Implementation aids; not semantic SoT | Layout open |
| Supabase collab metadata | Non-authoritative for local content; adapter → ADR-07 | Boundary detail open |
| Which Artifact header fields are mutable after first publication | **Unresolved** — do not assume all Artifact metadata is immutable or mutable | YES |

### 3.7 Revision immutability (proposed)

**ADR-05 proposes** that a **published Revision** is immutable.

| Topic | Proposed planning rule |
| --- | --- |
| Parent revision | Optional `parentRevisionId` forms a lineage chain; does not rewrite parents |
| Content digest | Stored with the Revision; verification would be required before treating content as valid |
| Replacement | Create a **new** Revision; do not mutate prior Revision bytes or digests |
| Current revision | Artifact current-revision reference advances under host rules after successful publication |
| Digest mismatch / missing content | Fail closed for authoritative reads/writes; do not silently “repair” into a different digest |
| Mechanism existence | **Not claimed** — future implementation after acceptance + separate authorization |

### 3.8 Content identity and digests (proposed; CAS unresolved)

Planning evidence includes `contentHash` on Revision and optional Source ([data-model.md](../data-model.md)). **Content-addressed storage (CAS) is not canonically mandated.**

| Topic | Proposed posture |
| --- | --- |
| Digests | Would support integrity verification and corruption detection |
| Content identity vs Artifact identity | Digests identify **bytes**; Artifact/Revision ids identify **durable objects** |
| Identical bytes ⇒ identical Revisions? | **Unresolved** — may or may not dedupe; must not silently merge distinct provenance |
| Metadata vs revision identity | Whether metadata participates in revision identity is **unresolved** |
| Algorithm / collision / upgrade | **Unresolved** — must be decided before implementation |
| CAS vs path-based vs hybrid layout | **Unresolved** — record as open; do not accept CAS as architecture by this draft alone |

### 3.9 Storage layout (planning requirements only)

**ADR-05 proposes** planning-level requirements:

- Local storage under **Trusted Host** control.
- Stable **internal identifiers** distinct from physical paths.
- **URI / locator abstraction** for IPC and contracts (see §3.11).
- Separation of **public contract identity** from physical filesystem paths.
- Distinct **temporary** vs **published** locations (exact names unresolved).
- Metadata/index storage as an implementation concern subordinate to host publication rules.
- Staged publication with validation before authoritative promotion (protocol unresolved).

**ADR-05 does not finalize:** exact directory layout; database engine; object-storage provider; filenames; table DDL; indexes; platform-specific roots.

### 3.10 Atomic publication and partial writes (proposed requirements; protocol unresolved)

Future implementation **would require** (planning):

- staged writes of candidate content;
- validation (identity, classification posture, digest, capability/policy checks as applicable) **before** publication;
- digest verification before authoritative promotion;
- atomic promotion **or equivalent** fail-closed publication;
- cleanup of abandoned temporary data;
- failure before authoritative publication leaves no authoritative Artifact/Revision;
- crash recovery that does not promote partial data;
- handling of duplicate or replayed write requests;
- unambiguous completion signaling to callers (via host/IPC — **ADR-06** owns envelopes).

```text
Exact atomic-write protocol = UNRESOLVED
Exact replay / idempotency semantics = UNRESOLVED
Transactional behavior is NOT claimed to exist
```

### 3.11 Artifact URIs and IPC boundary

Preserve [ADR-06](./ADR-06-worker-and-ipc-contracts.md) (Proposed) and [trusted-host-ipc.md](../contracts/trusted-host-ipc.md):

| Concern | Owner |
| --- | --- |
| Commands / envelopes / protocol versions (`artifact.put`/`get`, `run.*`, errors) | **ADR-06** |
| What valid Artifact URIs/refs resolve to; how authoritative content is persisted | **ADR-05** |
| Large binaries | Prefer Artifact URIs / store references — not ordinary IPC bodies |
| IPC success alone | Does **not** make referenced storage authoritative without store publication rules |

This draft does **not** redefine ADR-06 envelopes or commands.

### 3.12 Worker outputs (proposed)

Worker output remains **provisional** until it is:

1. returned through the host-mediated contract;
2. validated by the Trusted Host;
3. classified appropriately (with ADR-14 enforcement detail deferred);
4. linked to provenance / Run evidence as required;
5. committed through the Artifact Store publication boundary.

Workers **must not**: directly mutate authoritative Artifacts; bypass Revision creation for durable content; choose durable storage locations as authority; self-publish trusted outputs ([worker-runtime.md](../contracts/worker-runtime.md); ADR-15/02).

DeepMed-specific behavior → **ADR-09** (not authored).

### 3.13 Run persistence (proposed planning)

Runs **would** be persisted as host-mediated records linking:

- operation / worker / optional model identity;
- inputs and outputs (Artifact/Revision refs);
- status (`queued → running → succeeded | failed | cancelled` sketches in [data-model.md](../data-model.md));
- provenance and audit correlation sketches;
- failure / cancellation evidence.

**Clarification (planning):** a **failed** (or cancelled) Run **may** exist **without** a published output Artifact. Crash recovery planning marks failed Run + audit and forbids auto-approval ([data-model.md](../data-model.md)). Exact schemas remain open.

### 3.14 Provenance and lineage (proposed)

| Requirement | Notes |
| --- | --- |
| Origin / producer / author | Persistable with published objects |
| Source references | Artifact/Source links per data-model sketches |
| Run relationships | Inputs/outputs; DeepMed model provenance at planning level |
| Parent Revision | Lineage without mutation |
| Import/export lineage | May reference ExportManifest semantics (**ADR-04**); secure export → ADR-14/host |
| Evidence / audit correlation | Store may hold refs; **authoritative audit ownership** remains Trusted Host (**ADR-02**) |

Shared semantic definitions remain under **ADR-04**.

### 3.15 PHI and data classification (claim language)

| Topic | Proposed ADR-05 posture | Defer |
| --- | --- | --- |
| PHI-bearing Artifacts | Local store SoT under Decision C; no default cloud content SoT | ADR-14 egress detail |
| DataClassification | Required on durable entities ([data-model.md](../data-model.md)) | Enforcement engine → ADR-14 |
| Minimum-necessary access | Host-mediated reads/writes only | Capability taxonomy → ADR-02/06/14 |
| Path / diagnostic redaction | Planning requirement: avoid leaking sensitive paths/secrets/PHI in ordinary diagnostics | Exact redaction rules open |
| Encryption at rest | Required planning posture for sensitive local storage where program evidence anticipates encrypted local storage (tasks/plan/ADR-15) | Mechanism → §3.16 / UQs |
| Temporary sensitive data | Must not become authoritative; cleanup required | Protocol open |
| Unauthorized read/write | Deny by default via host | Implementation after gates |
| Export | Store supplies content for host-mediated export | Secure export / PHI → ADR-14 |
| Unknown classification | Fail closed for privileged publication/egress-relevant paths | ADR-14 |

Do **not** claim regulatory certification or implemented encryption.

### 3.16 Encryption and key custody (proposed posture; mechanism unresolved)

**ADR-05 proposes** that sensitive local Artifact Store content **would require** encryption at rest, consistent with Rust-owned encrypted local storage direction ([tasks.md](../tasks.md) Rust-first enforcement; plan R2; T037 security note that encryption keys would use secrets API).

**Unresolved (must not be chosen here):** encryption algorithm; file vs database encryption; key storage; rotation; recovery; platform keychain use; backup-key handling.

Key custody and secrets enforcement remain under **Trusted Host** security boundaries (**ADR-02** / secrets APIs). Overlap with secrets-backend choice is noted in ADR-02 unresolved questions — still open.

### 3.17 Retention, deletion, and recovery

Canonical evidence is **incomplete**. ADR-05 records these as **unresolved** and does **not** invent retention periods or deletion guarantees:

retention policy; deletion authority; secure erasure; tombstones; revision-history preservation; legal/audit holds; backup; restoration; disaster recovery; orphan cleanup.

### 3.18 Offline and remote posture

Preserve local-first planning direction ([plan.md](../plan.md); [spec.md](../spec.md) FR-015 posture):

| Topic | Proposed posture |
| --- | --- |
| Local authoritative operation | Should not depend on continuous remote availability where supported |
| Remote synchronization | **Not** accepted by ADR-05 |
| Supabase adapter behavior | **ADR-07** (not authored) |
| Remote / object storage | **Unresolved** — not authorized |
| Offline conflict resolution | **Unresolved** |

### 3.19 Fehrest, DeepMed, and export boundaries

| Boundary | Owner |
| --- | --- |
| Fehrest is not the Artifact Store; integration/release | **ADR-08** (not authored) |
| DeepMed-specific output operations | **ADR-09** (not authored) |
| ExportManifest semantics | **ADR-04** (Proposed) |
| Secure export / PHI egress | **ADR-14** / Trusted Host (ADR-14 not authored) |
| Persist referenced Artifacts / related metadata | **ADR-05** may propose store behavior only |

### 3.20 Persistence-format versioning (proposed)

**ADR-05 proposes** explicit **persistence-format version** governance for Artifact Store on-disk / catalog formats.

Keep it **distinct** from:

| Version domain | Owner / notes |
| --- | --- |
| ADR-04 `schemaVersion` | Shared primitive semantics |
| ADR-06 protocol version | IPC envelopes / commands |
| Package version | Distribution units (if later authorized) |
| Application version | Product releases |
| Database-engine version | Implementation detail |
| Worker runtime version | Worker pins (**ADR-06**/08/09 as applicable) |

Planning requirements for a future format policy **would include**: read compatibility rules; write compatibility rules; format upgrades; old/new coexistence windows; fail-closed unsupported-version behavior; corruption/unsupported handling; rollback planning for formats.

This draft does **not** implement or execute migrations.

### 3.21 Migration boundary

| Layer | ADR-05 may… | T018 / this draft must not… |
| --- | --- | --- |
| 1. Planning migration responsibility | Propose principles for moving authority to the local store under Decision C | Alter existing data |
| 2. Persistence-format version governance | Own format version rules (proposed) | Publish production formats |
| 3. Production migration execution | Describe need for separately authorized tasks | Create migration scripts; change Supabase schemas; mutate `documents-crypto` or dual migration trees; create Spec 002; run upgrades |

Legacy Supabase/`documents-crypto` freeze and Spec 002 naming remain per Decision C / [supabase-adapter.md](../contracts/supabase-adapter.md). Adapter ADR → **ADR-07**.

### 3.22 Integrity and corruption (proposed requirements)

Future store behavior **would require** planning coverage for:

- digest verification;
- missing-content detection;
- invalid Revision references;
- broken parent chains;
- inconsistent Artifact/current-revision links;
- unsupported persistence formats;
- partial data;
- corrupt metadata;
- fail-closed reads and writes;
- safe diagnostics (no secret/PHI/path leakage by default).

Corruption-recovery tooling is **not claimed**. Recovery and repair **authority** remain **unresolved**.

### 3.23 Proposed invariants (architecture authority only if later Accepted)

1. Authoritative Decision C content classes are published only through the host-mediated Artifact Store.
2. Published Revisions are immutable; replacement creates new Revisions.
3. UI, browser state, and worker-local files are never Artifact SoT.
4. Persistence-format versions are explicit and distinct from ADR-04/ADR-06 versions.
5. Supabase is not authoritative Artifact-content SoT under Decision C.

Until Accepted, these are **proposed planning invariants** only.

---

## 4. Scope

### In scope for this draft

- Artifact/Revision/Run **storage authority** and publication semantics (planning)
- Local-first SoT alignment with Decision C
- Immutability, integrity, URI/locator, and persistence-format versioning principles
- Boundaries with host, IPC, shared primitives, adapters, Fehrest, DeepMed, auth, and PHI ADRs
- Explicit unresolved questions for implementation gates

### Out of scope

- Implementing the Artifact Store or Trusted Host
- Final schemas, DDL, directory trees, engines, or cryptography choices
- Supabase mutation / Spec 002 creation
- Fehrest/DeepMed product behavior
- Auth/session changes
- R2 / T031+ / T037 execution

---

## 5. Alternatives considered

### Option A — UI / browser state as source of truth

| | |
| --- | --- |
| Summary | Treat `afia-ui` or browser storage as Artifact authority |
| Costs / risks | Accidental SoT; PHI/path leakage; no host mediation; contradicts ADR-03/15/02 planning |
| Draft disposition | **Rejected** |

### Option B — Worker-local files as source of truth

| | |
| --- | --- |
| Summary | Treat sidecar outputs on disk as durable authority |
| Costs / risks | Bypasses host validation; contradicts worker-runtime hard prohibitions |
| Draft disposition | **Rejected** |

### Option C — Supabase as primary Artifact-content SoT

| | |
| --- | --- |
| Summary | Keep or expand cloud documents as content authority |
| Costs / risks | Contradicts ratified Decision C; PHI default cloud posture |
| Draft disposition | **Rejected** as Artifact-content SoT (adapter role remains for ADR-07) |

### Option D — Database rows as canonical domain semantics

| | |
| --- | --- |
| Summary | Treat ORM/SQL shapes as the shared contract |
| Costs / risks | Breaks language-neutral ADR-04 ownership; couples storage to meaning |
| Draft disposition | **Rejected** as semantic SoT (rows may still implement persistence) |

### Option E — Mutable Revisions

| | |
| --- | --- |
| Summary | Allow in-place Revision edits |
| Costs / risks | Breaks provenance and integrity planning in data-model |
| Draft disposition | **Rejected** |

### Option F — Direct filesystem-path identity

| | |
| --- | --- |
| Summary | Use raw paths as stable Artifact identity |
| Costs / risks | Escape/traversal; unstable identity; WebView-supplied path confusion (ADR-02) |
| Draft disposition | **Rejected** as contract identity (paths may exist behind locators) |

### Option G — Unversioned persistence formats

| | |
| --- | --- |
| Summary | Store without persistence-format version |
| Costs / risks | Silent incompatibility; unsafe upgrades |
| Draft disposition | **Rejected** |

### Option H — Mandatory content-addressed storage

| | |
| --- | --- |
| Summary | Require CAS as the accepted layout |
| Costs / risks | Not canonically mandated; may be valid later, but accepting it here overclaims |
| Draft disposition | **Not accepted** by this draft; remains an **unresolved layout option** |

### Option I — Local Rust-controlled Artifact/Revision/Run storage behind the Trusted Host (proposed)

| | |
| --- | --- |
| Summary | Host-mediated local Artifact Store implementing Decision C |
| Benefits | Clear SoT; immutability; provenance; local-first PHI posture; replaceable UI/workers/adapters |
| Costs / risks | Governance, encryption, atomicity, recovery, format coexistence (see §6) |
| Draft disposition | **Proposed** |

---

## 6. Consequences

### Positive (prospective — if later accepted and executed under separate authorization)

- Clear authoritative storage boundary under Decision C
- Immutable revision history planning
- Reliable provenance hooks
- Local-first PHI posture for durable content
- Replaceable UI, workers, and adapters
- Explicit persistence-format compatibility surface
- Improved integrity checking potential
- Clearer Artifact URI / locator semantics relative to IPC

### Costs and risks

| Cost / risk | Note |
| --- | --- |
| Storage governance overhead | Tier A; stage gates; format policy |
| Migration complexity | Decision C move + Spec 002 later |
| Encryption / key-management burden | Mechanism unresolved |
| Atomic-write complexity | Protocol unresolved |
| Metadata / index consistency | Catalog vs content drift risk |
| Recovery tooling | Authority unresolved |
| Format coexistence | Read/write matrices needed |
| Backup and restore responsibility | Unresolved policy |
| Local disk management | Growth, orphan temp data |
| Corruption handling | Fail-closed vs repair tension |
| Oversized Artifact Store | Must not absorb Fehrest/DeepMed/ADR-14 concerns |
| Coupling layout to domain semantics | Must keep ADR-04 split |

### Rollback (planning only)

If rejected before acceptance: revert this ADR file; retain planning contracts and Decision C as non-authoritative/ratified planning evidence respectively; do **not** implement the Artifact Store from this draft. Post-acceptance withdrawal would need separately authorized migration — **not** defined here.

---

## 7. Non-goals

ADR-05 does **not**:

- implement the Artifact Store;
- define final shared primitive schemas (**ADR-04**);
- define IPC commands or envelopes (**ADR-06**);
- implement Trusted Host enforcement (**ADR-02**);
- define Fehrest integration (**ADR-08**);
- define DeepMed operations (**ADR-09**);
- redesign authentication or sessions (**ADR-11**);
- finalize PHI egress policy (**ADR-14**);
- create database DDL;
- create filesystem migrations;
- mutate Supabase or `documents-crypto`;
- choose final encryption or key-management technology;
- define remote synchronization;
- publish production storage formats;
- begin R2 / T031+ / T037.

---

## 8. Relationship to other ADRs

| ADR | Status in repo | Relationship |
| --- | --- | --- |
| ADR-15 | Proposed | Rust Artifact/Run kernel direction; workers cannot mutate Artifacts directly |
| ADR-01 | Proposed | Composition; WebView not Artifact SoT |
| ADR-02 | Proposed | Durable mutation **gateway**; defers store design here |
| ADR-03 | Proposed | `afia-ui` is not Artifact SoT |
| ADR-04 | Proposed | Semantic/schema ownership; persistence-format version deferred here; ADR-04 before ADR-05 |
| ADR-06 | Proposed | IPC URIs/commands; not store persistence |
| ADR-07 | Not authored | Supabase adapter / local-first boundary; Decision C adapter detail |
| ADR-08 | Not authored | Fehrest integration / release |
| ADR-09 | Not authored | DeepMed / OpenMed operations |
| ADR-11 | Not authored | Auth/session preservation |
| ADR-14 | Not authored | PHI egress / classification enforcement detail |

---

## 9. Gates and authority

```text
T018 produces a Proposed ADR-05 draft only.
Tier A independent review is required (Artifact Store ADR).
Passing Tier A review is not architecture acceptance.
tasks.md founder-acceptance field for T018 is: No — draft only.
T030 remains the R1 stage gate and remains incomplete.
At T030, ADR-05 is expected to be listed Reviewed or Accepted — it is not automatically Accepted.
ADR-05 is NOT in the mandatory T030 Accepted set that opens R2 (ADR-15/01/02/06).
T037 requires ADR-05 Accepted before Artifact Store foundation implementation.
T031+ and R2 remain separately gated.
No Artifact Store implementation, durable mutation, migration, or production format publication is authorized by this file.
```

| Gate | Meaning for ADR-05 |
| --- | --- |
| T018 | Draft authoring (this task) |
| Tier A | Mandatory independent review before treating as stage-complete evidence |
| Founder draft acceptance | Not required by tasks.md for T018 work product |
| T030 | R1 architecture gate; ADR-05 listed Reviewed or Accepted; **not** mandatory Accepted-for-R2 set |
| T037 | Artifact Store foundation **only after ADR-05 Accepted** (+ T032/T030 deps) |
| T043 | R3 Artifact/Run contracts; requires ADR-05/04/15 Accepted (per tasks.md) |
| T031+ / R2 | Separately gated; not authorized here |

---

## 10. Validation and acceptance plan

```text
T018 completion produces a draft for Tier A review.
Architecture acceptance is not performed by T018.
```

Planning reviews **may** include:

- consistency with Decision C, data-model.md, shared-primitives.md, supabase-adapter.md, trusted-host-ipc.md, worker-runtime.md, and plan ADR-05 roadmap row;
- SoT / immutability / host-gateway review;
- ADR-04 semantic vs ADR-05 persistence split;
- ADR-06 URI/IPC non-ownership;
- security/privacy claim-language review (PHI, encryption posture, no compliance overclaim);
- Tier A independent review ([tasks.md](../tasks.md) T018).

---

## 11. Unresolved questions

T018 does **not** resolve these. Attempted resolution: **NO**.

| ID | Question | May remain open in draft review? | Must resolve before implementation? | Belongs elsewhere? |
| --- | --- | --- | --- | --- |
| U-ADR05-1 | Exact storage layout (directories, catalogs) | YES | YES | Later authorized store tasks (e.g. T037+) |
| U-ADR05-2 | Database vs filesystem responsibilities | YES | YES | Later store design tasks |
| U-ADR05-3 | CAS vs path-based vs hybrid layout | YES | YES before layout lock | This ADR (open); not mandated |
| U-ADR05-4 | Which Artifact metadata fields are mutable after publication | YES | YES | This ADR + ADR-04 field finalization |
| U-ADR05-5 | Revision identity vs hash semantics / dedupe rules | YES | YES | This ADR |
| U-ADR05-6 | Hash algorithm and upgrade path | YES | YES | This ADR; crypto review |
| U-ADR05-7 | Atomic publication protocol | YES | YES | This ADR + host impl |
| U-ADR05-8 | Temporary-file cleanup rules | YES | YES | Host / this ADR (ADR-06 notes overlap) |
| U-ADR05-9 | Partial-write recovery behavior | YES | YES | This ADR |
| U-ADR05-10 | Duplicate / replayed write handling | YES | YES | This ADR + ADR-06 idempotency |
| U-ADR05-11 | Persistence-format version syntax | YES | YES before format publish | This ADR |
| U-ADR05-12 | Old/new format coexistence policy | YES | YES before dual-format runtime | This ADR |
| U-ADR05-13 | Read/write compatibility matrices | YES | YES | This ADR |
| U-ADR05-14 | Corruption repair authority | YES | YES before repair tools | This ADR + stage policy |
| U-ADR05-15 | Encryption mechanism | YES | YES before storing real PHI | This ADR; secrets/host |
| U-ADR05-16 | Key custody / storage | YES | YES | ADR-02 secrets; this ADR |
| U-ADR05-17 | Key rotation and recovery | YES | YES before production PHI | Host security tasks |
| U-ADR05-18 | Retention policy | YES | Before claiming retention | Possibly ADR-14 / policy specs |
| U-ADR05-19 | Deletion authority | YES | Before destructive APIs | This ADR + policy |
| U-ADR05-20 | Secure erasure | YES | Before PHI deletion claims | This ADR + ADR-14 |
| U-ADR05-21 | Backups | YES | Before Alpha durability claims | Later ops tasks |
| U-ADR05-22 | Restore testing | YES | Before relying on backups | Later ops/test tasks |
| U-ADR05-23 | Offline behavior detail | YES | Before offline guarantees | This ADR; local-first plan |
| U-ADR05-24 | Remote / object storage posture | YES | Before any remote store | Not authorized here; maybe later ADR |
| U-ADR05-25 | Supabase metadata vs reference boundary | YES | Before adapter content paths | **ADR-07**; Spec 002 |
| U-ADR05-26 | Audit-record relationship to store objects | YES | Before audit schema lock | **ADR-02** / later audit policy; ADR-14 |
| U-ADR05-27 | Export relationship to store objects | YES | Before secure export impl | **ADR-14** / ExportManifest (**ADR-04**) |
| U-ADR05-28 | Run failure without output Artifact edge cases | YES | Before Run contract tests | This ADR; T043 later |
| U-ADR05-29 | Fehrest portable export ↔ store refs | YES | Before Fehrest persistence paths | **ADR-08** |
| U-ADR05-30 | DeepMed output publication profile | YES | Before DeepMed persist slice | **ADR-09**; T050 later |

---

## 12. Security and privacy claim language

```text
Claim language only.
No regulatory certification is asserted.
No encryption, backup, migration, or Artifact Store implementation is asserted.
PHI posture follows Decision C + classification planning; egress engine detail remains ADR-14.
```

---

## 13. References

- [tasks.md](../tasks.md) — T018, blocking matrix, T030, T037, T043
- [plan.md](../plan.md) — Decision C; ADR-05 roadmap row
- [spec.md](../spec.md) — local-first / Q4 posture
- [research.md](../research.md) — R14 / P1 Decision C
- [data-model.md](../data-model.md) — Artifact/Revision/Run boundary sketches
- [shared-primitives.md](../contracts/shared-primitives.md) — minimum exported types
- [trusted-host-ipc.md](../contracts/trusted-host-ipc.md) — artifact/run commands; URI rule
- [worker-runtime.md](../contracts/worker-runtime.md) — no durable Artifact mutation
- [supabase-adapter.md](../contracts/supabase-adapter.md) — Decision C adapter boundary
- [ADR-15](./ADR-15-rust-first-polyglot-runtime.md) (Proposed)
- [ADR-01](./ADR-01-platform-and-desktop-composition-tauri-2.md) (Proposed)
- [ADR-02](./ADR-02-rust-trusted-host-boundary.md) (Proposed)
- [ADR-03](./ADR-03-afia-ui-strangler-migration.md) (Proposed)
- [ADR-04](./ADR-04-shared-primitive-ownership-and-versioning.md) (Proposed)
- [ADR-06](./ADR-06-worker-and-ipc-contracts.md) (Proposed)

---

```text
While Proposed:
- no Artifact Store implementation from this file;
- no durable artifact mutation;
- no database/filesystem migrations;
- no Supabase mutation;
- no production storage-format publication;
- no Fehrest/DeepMed storage wiring from this draft;
- no R2 / T031+ / T037 work from this file alone.
```

```text
End of ADR-05 Proposed draft.
```
