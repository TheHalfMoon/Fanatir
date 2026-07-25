# ADR-08 — Fehrest Integration and Release Model

| Field | Value |
| --- | --- |
| **ADR** | ADR-08 |
| **Title** | Fehrest Integration and Release Model |
| **Status** | **Reviewed** (T030 R1 architecture gate; not Accepted) |
| **Task origin** | T020 |
| **Acceptance / review posture** | Tier A independent review after drafting; **T030** expects ADR-08 listed **Reviewed or Accepted** (not automatically Accepted; **not** in the mandatory Accepted set that opens R2). Fehrest implementation work such as **T044+** requires ADR-08 **Accepted**; **T050** requires ADR-08 / ADR-09 / ADR-05 **Accepted** |
| **Feature** | `001-fanatir-repository-and-architecture-reconstitution` |
| **Required gate (tasks.md)** | ADR-15; Q5 |
| **Planning dependency (plan.md)** | Q5, P3, ADR-15 |
| **Constraining ADRs** | [ADR-15](./ADR-15-rust-first-polyglot-runtime.md) (Accepted at T030); [ADR-02](./ADR-02-rust-trusted-host-boundary.md) (Accepted at T030); [ADR-04](./ADR-04-shared-primitive-ownership-and-versioning.md) (Reviewed at T030); [ADR-05](./ADR-05-artifact-revision-run-storage.md) (Reviewed at T030); [ADR-06](./ADR-06-worker-and-ipc-contracts.md) (Accepted at T030); [ADR-07](./ADR-07-supabase-adapter-and-local-first-boundary.md) (Reviewed at T030) |
| **Related ADRs** | [ADR-01](./ADR-01-platform-and-desktop-composition-tauri-2.md) (Accepted at T030); [ADR-03](./ADR-03-afia-ui-strangler-migration.md) (Proposed) |
| **Planning evidence** | [spec.md](../spec.md) Q5 / repository map / memory ownership; [plan.md](../plan.md) ADR-08 row / R3 Fehrest order; [research.md](../research.md) R10 / P3; [fehrest-integration.md](../contracts/fehrest-integration.md); [data-model.md](../data-model.md); [shared-primitives.md](../contracts/shared-primitives.md); [trusted-host-ipc.md](../contracts/trusted-host-ipc.md); [worker-runtime.md](../contracts/worker-runtime.md); [tasks.md](../tasks.md) T020 / T030 / T044–T047 / T050 |
| **T030 founder decision** | **Reviewed** — R1 architecture gate; not Accepted; Fehrest product stage remains separately gated |
| **Architecture authority of this file** | **NO** — Reviewed at T030; Accepted required before Fehrest implementation |
| **Implementation authorization** | **NO** |

```text
Status: Reviewed (T030 R1 architecture gate; not Accepted)
Architecture authority: NO (Reviewed ≠ Accepted)
Implementation authorization: NO
Fehrest implementation: NOT authorized
Independent Tier A — R1 architecture gate: Pending
```

```text
Draft ADR ≠ architecture acceptance
T020 completion ≠ T030 acceptance (T030 founder Reviewed decision now recorded)
Passing Tier A review of a draft ≠ Accepted
Founder-ratified Q5 ≠ Accepted ADR-08
Reviewed ≠ Accepted for Fehrest product work
ADR-03 remains Proposed
```

```text
This draft is not:
- an implemented Fehrest integration
- permission to initialize or modify the Fehrest repository
- permission to import or fork Graphify
- a final IPC schema
- a provider integration
- a clinical authority
- an Artifact Store
- permission to expose PHI
- permission to begin R2
```

```text
No production implementation is authorized by this draft.
```

This draft **must not** be used as justification to: initialize or modify the Fehrest repository; implement Fehrest product features; fork or import Graphify; select or call external scholarly providers; publish final IPC schemas or operation payloads; mutate Artifacts, Revisions, Sources, or Relationships; bypass Trusted Host mediation; expose PHI; change authentication or sessions; modify Supabase; begin R2 (`T031+`); or treat Proposed ADRs as Accepted.

---

## 1. Context and problem

### 1.1 Assigned architectural question

**ADR-08 proposes** how **Fehrest** would integrate with **Fanatir** as an **independent product** and an **embeddable capability**, and how its **release / pin / compatibility** boundary would operate under a **Rust-supervised process** model constrained by Proposed [ADR-15](./ADR-15-rust-first-polyglot-runtime.md), [ADR-02](./ADR-02-rust-trusted-host-boundary.md), [ADR-04](./ADR-04-shared-primitive-ownership-and-versioning.md), [ADR-05](./ADR-05-artifact-revision-run-storage.md), [ADR-06](./ADR-06-worker-and-ipc-contracts.md), and [ADR-07](./ADR-07-supabase-adapter-and-local-first-boundary.md).

It does **not** implement Fehrest; initialize the Fehrest repository; import Graphify; finalize shared schemas (**ADR-04**); finalize Artifact Store persistence (**ADR-05**); finalize IPC envelopes (**ADR-06**); define DeepMed product operations (**ADR-09**, not authored); redesign authentication/sessions (**ADR-11**, not authored); or finalize PHI egress (**ADR-14**, not authored).

### 1.2 Why an explicit Fehrest integration and release model matters

Fanatir’s Founder Alpha journey requires a bounded Fehrest step for knowledge, citations, relationships, search, graph, memory helpers, and portable export usable **standalone** and **inside Fanatir** ([spec.md](../spec.md) Q5; [plan.md](../plan.md); [research.md](../research.md) R10/P3). Sibling-repo emptiness and contract-first planning are verified program facts; product maturity must not be invented ([spec.md](../spec.md) inventory; [tasks.md](../tasks.md) R0 posture).

Without an explicit integration and release boundary, planning identifies these **risks** (prospective — not claimed as currently measured production failures unless separately evidenced):

| Risk | Why it matters |
| --- | --- |
| Fehrest confused with the Artifact Store | Vault/knowledge eclipses local content SoT |
| Fehrest redefines Fanatir `Source` / `Relationship` semantics | Product records steal ADR-04 ownership |
| Sidecar treated as trusted peer | Process isolation and host authority collapse |
| Fehrest workers mutate durable Artifacts directly | Violates worker-runtime / host durable-mutation rules |
| Graph or search indexes become accidental authoritative truth | Derived indexes redefine source data |
| Fehrest becomes a clinical authority | Patient/FHIR/PolicyDecision boundaries break |
| Provider data treated as trusted | Unvalidated external content becomes SoT |
| PHI leaks via search, logs, exports, or remote calls | Classification/egress failure |
| Graphify imported without licensing review | Rights and separate-spec gates skipped |
| Release drift between Fanatir and Fehrest | Incompatible pins; broken Alpha journey |
| Version pins confused with protocol versions | Product pins mistaken for ADR-04/06 versions |
| Embedded and standalone release models diverge silently | Dual products without compatibility evidence |

### 1.3 Accepted program and planning constraints

| Constraint | Source | Treatment |
| --- | --- | --- |
| Fehrest Alpha = independent product + embedded Fanatir capability; bounded local vault/editor/notes/sources/quotations/wikilinks/backlinks/typed relationships/search/project memory/current-state/next-action/graph/origin distinction/portable export; Graphify fork/import separate | Q5 — [spec.md](../spec.md) (**founder-ratified**) | Binding Alpha capability boundary; **not** ADR-08 acceptance |
| Process/sidecar primary for Alpha; packages for shared contracts only; semver pins; compatibility matrix; rollback = prior pin | P3 — [research.md](../research.md) (**resolved for planning**) | Planning release-channel evidence; **not** implementation |
| Fehrest owns vault content / notes / quotations / typed relationships / local search index / portable export; Fanatir owns launch/embed/gateway and Project Long Memory references to exports | [fehrest-integration.md](../contracts/fehrest-integration.md) | Planning contract evidence |
| Rust-supervised Fehrest process boundary | [plan.md](../plan.md) ADR-08 row; [ADR-15](./ADR-15-rust-first-polyglot-runtime.md) Proposed | Proposed constraint |
| Fehrest MUST NOT be SoT for FHIR, databases, model weights, or original healthcare files; not clinical patient authority | [spec.md](../spec.md) cross-repo boundaries / FR-014 | Binding prohibition under 001 |
| No OpenMed/Graphify fork/import under T020; no production code; draft ≠ accepted | [tasks.md](../tasks.md) T020 | Binding for this task |
| No R2 before T030 (ADR-15/01/02/06 Accepted as applicable) | [tasks.md](../tasks.md) | Future-gate requirement |

### 1.4 Naming

| Name | Role in this ADR |
| --- | --- |
| **Fanatir** | Product/integration authority; owns shared primitive semantics (**ADR-04**); owns launch/embed/gateway and governed references |
| **Fehrest** | Independent knowledge product and embeddable Fanatir capability — **not** Artifact Store; **not** clinical SoT |
| **Artifact Store** | Local Rust-controlled Artifact/Revision/Run content authority (**ADR-05** Proposed) |
| **DeepMed** | Clinical-assistive runtime boundary (**ADR-09**, not authored) — not Fehrest |
| **`afia-ui`** | Migration shell / UI presentation; not Fehrest architecture SoT (**ADR-03** Proposed) |
| **Source** / **Relationship** / **ExportManifest** | Shared primitives semantically owned by Fanatir under **ADR-04** Proposed |
| **Graphify** | Named separate Fehrest concern; fork/import **not** authorized here |

Never confuse Fehrest with Fanatir, Fehrest with the Artifact Store, or Fehrest with DeepMed.

### 1.5 What this draft is not

This file is **not**: an implemented Fehrest integration; Fehrest repository initialization; Graphify import; final IPC schema; provider integration; clinical authority; Artifact Store; PHI-egress authorization; or R2 authorization.

---

## 2. Decision drivers

| Driver | Source |
| --- | --- |
| Founder-ratified Q5 Alpha boundary | spec.md |
| P3 release-channel planning | research.md |
| Fehrest ↔ Fanatir ownership split | fehrest-integration.md |
| Rust-supervised sidecar + pins | plan.md ADR-08; ADR-15 Proposed |
| Shared `Source` / `Relationship` / `ExportManifest` ownership | ADR-04 Proposed; data-model.md; shared-primitives.md |
| Artifact Store persistence authority | ADR-05 Proposed; Decision C |
| IPC / worker contracts | ADR-06 Proposed; trusted-host-ipc.md; worker-runtime.md |
| Local-first / Supabase non-SoT for Fehrest product data | ADR-07 Proposed; Decision C |
| Trusted Host mediation for privileged effects | ADR-02 Proposed |
| T020 draft-only; Tier A; no production code | tasks.md T020 |

---

## 3. Decision (proposed)

### 3.1 Core proposal

**ADR-08 proposes** that Fehrest integrate with Fanatir as:

1. an **independent product** (standalone usable for a non-Fanatir user under Q5);
2. an **embeddable Fanatir capability** (contextual use inside Fanatir under Q5);
3. a **local** knowledge / vault / graph / memory / export system;
4. a **governed consumer or producer** of references and **candidate** knowledge objects;

operating behind a **Rust-supervised process/sidecar** boundary with **explicit version pins** and a documented **compatibility matrix**, while **Fanatir** retains Capability Gateway invocation, shared-primitive semantic ownership, Artifact Store authority, and clinical/patient boundaries.

This is **Proposed** planning only. No Fehrest product, sidecar, pin, or matrix is claimed to exist as implemented architecture.

### 3.2 Product role (proposed)

| Fehrest is… | Fehrest is not… |
| --- | --- |
| Independent product in a separate repository boundary (`IamShehri/Fehrest` target) | Fanatir itself |
| Embeddable capability inside Fanatir | The Artifact Store |
| Local Markdown vault / notes / quotations / typed relationships / local search / graph / memory helpers / portable export system (Q5) | The Trusted Host |
| Governed consumer/producer of references and candidate knowledge objects | A clinical runtime (DeepMed) |
| | A policy / PolicyDecision authority |
| | A patient / FHIR source of truth |

### 3.3 Q5 Alpha capability boundary (proposed alignment)

ADR-08 **proposes** that Alpha Fehrest capability claims remain bounded to founder-ratified Q5 ([spec.md](../spec.md)):

**In Alpha planning scope (required capabilities):**

- local Markdown vault;
- Markdown editor;
- pages/notes;
- sources/attachments;
- quotations with provenance;
- wikilinks/backlinks;
- typed relationships;
- local search;
- project memory;
- current-state and next-action memory;
- graph visualization;
- distinction between human, extracted, and inferred relationships;
- portable export;
- usable independently and contextually inside Fanatir.

**Not required for Founder Alpha (Q5):**

- full Notion parity;
- full Obsidian plugin ecosystem;
- large-scale multiplayer editing;
- public marketplace;
- complete Graphify feature parity.

ADR-08 does **not** add capabilities unsupported by canonical sources. UI, APIs, storage engines, and implementation details remain **unresolved**.

### 3.4 Standalone and embedded modes (proposed)

| Mode | Proposed meaning | Authority notes |
| --- | --- | --- |
| **Standalone** | Fehrest usable without Fanatir product shell for a non-Fanatir user (Q5) | Fehrest owns its vault product data; does not become Fanatir clinical/Artifact SoT |
| **Embedded** | Fehrest capability invoked in Fanatir context | Fanatir owns launch/embed entry; Capability Gateway; governed references to exports |

Shared proposals for both modes:

- **Release compatibility** — pinned Fehrest versions consumed by Fanatir must remain documented (P3 planning);
- **Process supervision** — when embedded/invoked from Fanatir, Rust Trusted Host supervises the process boundary (plan / ADR-15 / ADR-06);
- **Data ownership** — Markdown vault ownership stays in Fehrest; Fanatir projects reference exports/notes ([research.md](../research.md) R10; fehrest-integration.md);
- **Host mediation** — privileged or durable Fanatir effects require Trusted Host mediation;
- **Capability gating** — Fanatir owns gateway authorization for Fehrest workers;
- **Version pinning** — Fanatir pins exact compatible Fehrest versions (P3 planning);
- **Failure isolation** — Fehrest process failure must not compromise Trusted Host continuity (align ADR-06).

Neither mode is claimed to exist in production.

### 3.5 Repository and release ownership (proposed)

| Concern | Proposed owner (planning) |
| --- | --- |
| Fehrest repository / product code | Fehrest repository boundary (separate from Fanatir monorepo) |
| Fanatir integration layer (launch/embed/gateway/refs) | Fanatir |
| Shared contract package semantics | Fanatir (**ADR-04**) |
| Release artifacts for Fehrest product | Fehrest release process (exact packaging **unresolved**) |
| Compatibility pins consumed by Fanatir | Fanatir (pins exact versions — P3 planning) |
| Compatibility matrix documentation | Fanatir release notes planning (host ↔ Fehrest ↔ DeepMed — P3) |
| Rollback | Prior pin (P3 planning) |
| Bundled vs externally installed sidecar | **Unresolved** (see §11) |

T020 does **not** initialize, clone, fork, modify, tag, or publish the Fehrest repository. Baseline evidence may show an empty Fehrest remote/clone with no product commits; emptiness is inventory, not authorization to scaffold.

### 3.6 Rust-supervised sidecar boundary (proposed)

Preserve plan acceptance direction ([plan.md](../plan.md) ADR-08 row; [ADR-15](./ADR-15-rust-first-polyglot-runtime.md) Proposed):

- Fehrest **would** run behind a **Rust-supervised** process boundary when integrated with Fanatir;
- it **would** remain restartable and replaceable ([worker-runtime.md](../contracts/worker-runtime.md));
- it **would not** receive ambient host privileges;
- it is **not** a trusted peer of the Trusted Host;
- Fehrest failures **must not** compromise the Trusted Host;
- process lifecycle **would** align with Proposed [ADR-06](./ADR-06-worker-and-ipc-contracts.md).

Library-only embed for Alpha was considered and rejected for planning isolation reasons ([research.md](../research.md) P3). Optional later library embed remains a **future** option in R10 and is **not** the Alpha primary shape.

This draft does **not** implement a sidecar.

### 3.7 Capability Gateway (proposed)

**Fanatir owns** the Capability Gateway through which Fehrest worker operations are invoked ([fehrest-integration.md](../contracts/fehrest-integration.md); [trusted-host-ipc.md](../contracts/trusted-host-ipc.md)).

Gateway-mediated Fehrest invocation **would require** (planning):

- explicit capability checks;
- authorization / PolicyDecision outcomes (host-owned);
- validated operation selection;
- bounded inputs and outputs;
- audit correlation (authoritative audit remains host-owned);
- process supervision;
- controlled environment;
- controlled network access;
- controlled secrets.

Final gateway code and command schemas are **not** published here.

### 3.8 Integration operation categories (planning; illustrative)

Canonical Alpha integration surface ([fehrest-integration.md](../contracts/fehrest-integration.md)) lists minimum planning operation categories. Every category below is **illustrative / proposed for planning**, **not** production-approved API, and **not** a final wire schema:

| Planning operation category | Direction (planning) | Notes |
| --- | --- | --- |
| `vault.open` | Fanatir → Fehrest | Project-scoped or standalone |
| `note.upsert` | Bidirectional | Preserve provenance |
| `graph.query` | Fanatir → Fehrest | Bounded graph viz/search |
| `export.portable` | Fehrest → Fanatir | ExportManifest compatible |
| `memory.current_state` / `next_action` | Fehrest | Project memory helpers |

Host-level supervision may use generic worker commands such as `worker.spawn` / `worker.status` / `worker.stop` ([trusted-host-ipc.md](../contracts/trusted-host-ipc.md)) — those remain **ADR-06** territory and are not Fehrest-specific schema publications.

Do **not** finalize payloads, response schemas, event schemas, or wire codes in this ADR.

### 3.9 Source and Relationship semantics (proposed)

| Concern | Proposed boundary |
| --- | --- |
| Fehrest product sources, quotations, typed relationships | May exist **within Fehrest’s product model** (Q5 / fehrest-integration) |
| Shared Fanatir `Source` / `Relationship` meanings | Remain under **ADR-04** (Proposed) |
| `Relationship.origin` (`human\|extracted\|inferred`) | Governed by shared semantics; distinguishable origin is a Fehrest Alpha requirement ([data-model.md](../data-model.md)) |
| Fehrest-local records → Fanatir shared primitives | **Do not** automatically become authoritative Fanatir shared primitives |
| Publication into Fanatir | **Would require** governed validation and Trusted Host mediation |

Schemas are **not** finalized here.

### 3.10 Candidate and authoritative data (proposed)

| Class | Authority posture |
| --- | --- |
| Fehrest-local notes and graph records | Authoritative **within Fehrest’s own product vault boundary**; non-authoritative to Fanatir clinical/Artifact SoT |
| Local search index / graph projections | Derived aids; **not** authoritative Artifact content |
| Candidate `Source` / `Relationship` records | Non-authoritative to Fanatir until accepted through governed publication |
| Governed Fanatir shared primitives | Fanatir / **ADR-04** |
| Artifact Store records | Fanatir / **ADR-05** |
| Portable exports | Fehrest-produced packages; Fanatir may reference; ExportManifest semantics → **ADR-04** |

Fehrest is **not** granted direct durable-mutation authority over Fanatir Artifacts or Revisions ([worker-runtime.md](../contracts/worker-runtime.md)).

### 3.11 Artifact Store interaction (proposed)

Fehrest **may** (planning):

- read governed references where authorized;
- produce candidate references;
- produce portable exports;
- **request** publication through the Trusted Host;
- participate in later handoff flows (e.g. reviewed DeepMed results → Fehrest under later Accepted gates).

Fehrest **must not**:

- directly write the Artifact Store;
- mutate published Revisions;
- advance current Revision pointers;
- bypass Artifact publication rules;
- become the Artifact Store itself.

Exact persistence and handoff mechanisms remain **unresolved**.

### 3.12 Run and provenance boundary (proposed)

Fehrest operations **may** produce (planning discussion): operation records; provenance on quotations/notes; exports; references; candidate Sources/Relationships; audit correlations requested via host.

Authoritative **Run** semantics and Artifact/Run persistence remain under **ADR-04** / **ADR-05**. DeepMed invocations have explicit Run requirements in data-model evidence; Fehrest-specific Run creation rules remain **unresolved** and must not redefine Run ownership.

Run schemas are **not** finalized here.

### 3.13 Search and graph boundary (proposed)

Planning-level local search and graph behavior ([spec.md](../spec.md) Q5; fehrest-integration.md):

- Fehrest **may** own a **local search index** within its product boundary;
- indexes and graph projections are **not** authoritative Artifact content;
- stale or corrupt indexes **must not** redefine source vault data;
- rebuildability and validation **would be required** before implementation;
- graph edges **would** preserve origin and provenance;
- search results are **discovery aids**, not clinical conclusions.

ADR-08 does **not** choose: a database; embedding model; vector store; ranking algorithm; reranker; or graph engine.

### 3.14 Memory and next-action boundary (proposed)

Canonical current-state and next-action capabilities (Q5; fehrest-integration.md):

- memory views **would** summarize governed Fehrest data;
- next-action output is **advisory**;
- neither grants authorization;
- neither becomes clinical advice;
- neither mutates Fanatir Artifacts without host mediation;
- exact derivation and persistence remain **unresolved**.

### 3.15 External provider boundary (explicit silence)

Canonical Spec 001 planning evidence is **silent** on named scholarly providers such as OpenAlex, Crossref, PubMed, or Semantic Scholar as Fehrest integrations.

Therefore ADR-08 states explicitly:

- ADR-08 does **not** select OpenAlex, Crossref, PubMed, Semantic Scholar, or any other external provider;
- external-provider integrations require **separate authorization**;
- provider responses are **untrusted** until validated;
- provider identifiers do **not** automatically become Fanatir canonical identifiers;
- provider terms, rate limits, rights, and storage constraints **must** be reviewed before implementation;
- provider unavailability **must not** silently corrupt local vault truth.

Do **not** invent provider support.

### 3.16 Graphify boundary

Accurately record ([spec.md](../spec.md) Q5; [tasks.md](../tasks.md) T020; fehrest-integration.md exclusions):

- Graphify is named as a **separate** Fehrest-related concern;
- fork/import is **prohibited** during T020 and is **not** authorized by Spec 001 reconstitution;
- licensing review is required before any use;
- ADR-08 does **not** authorize a Rust rewrite of Graphify;
- ADR-08 does **not** authorize incorporation of Graphify code or assets;
- later Fehrest specifications may evaluate Graphify separately.

Local Graphify CLI inventory / `graphify-out` evidence used in Spec 001 discovery remains **non-authoritative** and is **not** an import authorization.

### 3.17 Discovery versus clinical authority (proposed)

Fehrest **may** (planning): organize knowledge; preserve citations; expose relationships; aid discovery; provide source references; support researcher workflows.

Fehrest **must not**: validate clinical truth autonomously; publish clinical conclusions as system authority; replace researcher review; replace DeepMed; become a FHIR or patient source of truth; grant PolicyDecision authority.

### 3.18 DeepMed handoff (proposed boundary only)

| Concern | Boundary |
| --- | --- |
| Fehrest | Knowledge / vault / graph / memory / export |
| DeepMed | Clinical-assistive runtime (**ADR-09**, not authored) |
| Handoff | Spec planning anticipates preserve/handoff of reviewed results into Fehrest; **T050** later requires ADR-08/09/05 Accepted |
| Shared references | Via governed shared primitives / host mediation |
| Direct ungoverned mutual invocation | **Not** proposed |
| DeepMed operations | Deferred to **ADR-09** |

This draft does **not** define DeepMed operations.

### 3.19 Import boundary (planning)

Discussed at planning level only: Markdown content; metadata; quotations; typed relationships; portable bundles; external identifiers; full-text files; provenance; duplicate handling; validation; rights.

ADR-08 does **not** authorize:

- third-party full-text ingestion as a settled right;
- copyrighted PDF redistribution;
- external-provider bulk import;
- Graphify import;
- migration execution.

Exact import formats remain **unresolved**.

### 3.20 Export boundary (proposed)

Portable export aligns with Q5 and fehrest-integration ownership:

- vault subset export;
- provenance and attribution preservation where applicable;
- stable references (exact format unresolved);
- **ExportManifest** compatibility (semantics under **ADR-04**);
- versioning / integrity (unresolved detail);
- licensing / rights metadata awareness;
- secure export and PHI egress remain **ADR-14** / Trusted Host (not authored / Proposed elsewhere).

Persistence of store-backed artifacts remains **ADR-05**. Export format is **not** finalized here.

### 3.21 Licensing and content rights (claim language)

Acknowledge:

- Graphify licensing requires separate review;
- external-provider terms must be reviewed before use;
- metadata rights and full-text rights are distinct;
- import permission does **not** imply redistribution permission;
- citation metadata does **not** grant rights to full text;
- exports **would** preserve attribution and rights metadata where applicable;
- unknown rights **must fail closed** for redistribution.

This draft claims **no** rights to any third-party content.

### 3.22 PHI and privacy (claim language)

Address (planning): PHI-bearing notes; patient identifiers; clinical query text; uploaded files; external-provider queries; local search index content; exports; logs; diagnostics; local paths; provenance.

Proposed posture:

- minimum-necessary handling;
- classification awareness;
- redacted diagnostics;
- no PHI in ordinary identifiers or logs;
- controlled external egress (host/policy);
- fail-closed unknown classification for privileged/egress-relevant paths;
- no Fehrest-as-clinical-SoT.

Complete PHI policy and egress enforcement defer to **ADR-14**. No regulatory certification is claimed. No PHI safeguards are claimed implemented.

### 3.23 Secrets and network access (proposed)

- provider credentials are **secrets**;
- network access **would require** Trusted Host policy;
- secrets **must not** be embedded in vault content, logs, IPC envelopes, or exports;
- ambient environment inheritance is **not** allowed for privileged credentials;
- provider access is **not** assumed for Alpha;
- rate limits, retries, and backoff remain **unresolved**.

No secrets backend is chosen here.

### 3.24 Local-first and offline posture (proposed)

- vault / search / graph / memory / export are **local** planning capabilities for Alpha;
- Fehrest product data is **not** proposed to use Supabase as source of truth (**ADR-07** / Decision C);
- cloud availability **must not** silently redefine local Fehrest vault authority;
- remote providers are optional and separately authorized;
- offline behavior and remote-cache semantics remain **unresolved**.

Offline operation is **not** claimed implemented or tested.

### 3.25 Supabase boundary (preserve ADR-07)

Preserve Proposed [ADR-07](./ADR-07-supabase-adapter-and-local-first-boundary.md) / Decision C constraints:

- Supabase may later hold authorized collaboration/identity metadata only;
- Supabase is **not** the Fehrest vault or graph source of truth;
- Supabase is **not** the Artifact-content source of truth;
- Supabase is **not** a PHI or clinical source of truth;
- remote collaboration references do **not** grant access to Fehrest content;
- Fehrest product records **must not** be synchronized remotely without separate authorization.

### 3.26 Versioning and compatibility (proposed)

Distinguish:

| Version class | Owner / concern |
| --- | --- |
| Fehrest product version | Fehrest release |
| Fanatir application version | Fanatir |
| Integration adapter version | Fanatir integration layer (unresolved detail) |
| Sidecar release / pin | Fanatir pin of Fehrest binary/build (P3 planning) |
| ADR-04 shared primitive `schemaVersion` | Fanatir shared semantics |
| ADR-06 IPC protocol version | Host/worker contracts |
| ADR-05 persistence-format version | Artifact Store |
| Export format version | Unresolved |
| Index format version | Unresolved |

Propose:

- explicit compatibility pins;
- compatibility matrix (host ↔ Fehrest ↔ DeepMed — P3 planning);
- fail-closed incompatible versions;
- independent release cadence;
- rollback to prior pin;
- side-by-side compatibility only if later authorized.

Negotiation and compatibility testing are **not** claimed to exist.

### 3.27 Release channels and pins (P3 planning; not implementation)

Recover P3-supported planning direction without overstatement ([research.md](../research.md) P3):

| Concern | Planning recommendation (P3) |
| --- | --- |
| Local dev | Path/workspace pin or `FANATIR_*_BIN` env to local builds |
| Alpha tagging | Semver tags `v0.x` on Fehrest; Fanatir pins exact versions |
| Compatibility matrix | Documented in Fanatir release notes |
| Upgrade/rollback | Pin + host capability check; rollback = prior pin |
| Integration shape | Process/sidecar primary for Alpha; packages for shared contracts only |

P3 is **resolved for planning**, not an implemented release system. This draft does **not** publish releases or tags and does **not** select a package manager or updater unless later canonically required.

### 3.28 Failure, cancellation, and recovery (proposed)

Planning requirements (align lifecycle detail with **ADR-06**):

- Fehrest process crash → host continuity preserved; worker marked failed/unavailable per ADR-06 planning;
- failed operation / timeout / cancellation → bounded, auditable outcomes;
- partial export → must not be treated as complete portable export without validation;
- corrupt index → must not redefine vault source; rebuild/validation required before trust;
- unavailable vault / incompatible version / invalid result → fail closed;
- restart / quarantine / disablement → host-supervised;
- recovery tooling is **not** claimed to exist.

### 3.29 Auditability (proposed)

Audit-relevant events **may** include (planning): process launch; capability denial; operation acceptance/rejection; export request; external egress request; provider access (if ever authorized); failure; timeout; cancellation; version incompatibility; rights-policy rejection.

Authoritative audit ownership remains with the Trusted Host / **ADR-02** and later audit policy. Audit schemas are **not** finalized here.

### 3.30 Decision summary table

| Topic | Proposed disposition |
| --- | --- |
| Product role | Independent + embeddable local knowledge system |
| Integration shape | Rust-supervised sidecar primary; contracts via Fanatir-owned shared primitives |
| Release | Pins + compatibility matrix (P3 planning) |
| Semantics | ADR-04 retains Source/Relationship/ExportManifest |
| Storage | ADR-05 retains Artifact Store; Fehrest owns vault product data |
| IPC | ADR-06 retains envelopes/protocols |
| Clinical | Fehrest discovery ≠ clinical authority; DeepMed → ADR-09 |
| Providers / Graphify | Not selected / not imported |
| Status | Reviewed (T030; not Accepted) |

---

## 4. Scope

### In scope (this Proposed draft)

- Fehrest Integration and Release Model (planning)
- Q5 Alpha capability boundary alignment
- Standalone vs embedded roles
- Rust-supervised process / pin / compatibility planning
- Source/Relationship/ExportManifest non-theft; Artifact Store non-absorption
- Provider silence; Graphify exclusion; discovery vs clinical authority
- Explicit unresolved questions

### Out of scope

- Implementing Fehrest or initializing/modifying the Fehrest repository
- Graphify fork/import; provider integrations; search/graph/ranking engines
- Final shared schemas, IPC commands, Artifact Store writes
- DeepMed operations; auth/session redesign; PHI egress engine
- Publishing packages/releases; R2 / T031+ execution

---

## 5. Alternatives considered

### Option A — Fehrest as the Artifact Store

| | |
| --- | --- |
| Summary | Treat Fehrest vault as Fanatir content SoT |
| Costs / risks | Contradicts Decision C / ADR-05; collapses product boundaries |
| Draft disposition | **Rejected** |

### Option B — Fehrest as clinical / FHIR source of truth

| | |
| --- | --- |
| Summary | Fehrest becomes patient/FHIR/clinical authority |
| Costs / risks | Contradicts Constitution/spec memory ownership and exclusions |
| Draft disposition | **Rejected** |

### Option C — Direct UI integration without Trusted Host mediation

| | |
| --- | --- |
| Summary | `afia-ui` or WebView owns Fehrest privileged effects |
| Costs / risks | Bypasses host capability/audit/durable-mutation rules |
| Draft disposition | **Rejected** as architecture shortcut |

### Option D — In-process Fehrest execution as Alpha primary

| | |
| --- | --- |
| Summary | Library embed as Alpha primary shape |
| Costs / risks | Weaker process isolation for PHI/models; rejected by P3 for Alpha primary |
| Draft disposition | **Rejected** as Alpha primary; optional later embed remains future discussion only |

### Option E — Network-first Fehrest service

| | |
| --- | --- |
| Summary | Cloud/service-first Fehrest as Alpha default |
| Costs / risks | Conflicts with local Alpha vault posture; Supabase/remote SoT risks |
| Draft disposition | **Rejected** as Alpha default |

### Option F — Tightly coupled monorepo release only

| | |
| --- | --- |
| Summary | Absorb Fehrest into Fanatir monorepo releases only |
| Costs / risks | Conflicts with independent-product Q5 / Constitution remotes |
| Draft disposition | **Rejected** as exclusive model |

### Option G — Independent product with no Fanatir embedding

| | |
| --- | --- |
| Summary | Standalone only; no Fanatir capability embed |
| Costs / risks | Breaks Q5 embedded requirement and Alpha golden journey |
| Draft disposition | **Rejected** as exclusive model |

### Option H — Rust-supervised sidecar with explicit pins and governed contracts (proposed)

| | |
| --- | --- |
| Summary | Process boundary + pins + Fanatir gateway + ADR-04/05/06/07 constraints |
| Benefits | Isolation; replaceability; clear ownership; Q5 dual-mode support |
| Costs / risks | Two-repo coordination; IPC/supervision complexity (see §6) |
| Draft disposition | **Proposed** |

Alternatives were **not** empirically tested by this draft.

---

## 6. Consequences

### Positive (prospective — if later accepted and executed under separate authorization)

- Clear product boundary between Fanatir and Fehrest
- Reusable standalone Fehrest
- Embedded Fanatir capability without absorbing Fehrest as monorepo-only
- Local knowledge workflows for Alpha
- Replaceable process boundary
- Explicit compatibility pins/matrix potential
- Governed Artifact handoff path (later)
- Preserved clinical-authority separation
- Portable exports aligned with ExportManifest planning
- Clearer licensing / Graphify separation

### Costs and risks

| Cost / risk | Note |
| --- | --- |
| Two-repository coordination | Fehrest + Fanatir |
| Release compatibility management | Pins/matrix discipline |
| Sidecar supervision | Host lifecycle complexity |
| IPC complexity | ADR-06 coexistence |
| Index consistency | Search/graph vs vault source |
| Rights review | Graphify / providers / full-text |
| Import/export governance | Formats unresolved |
| Process startup and recovery | Crash/timeout/cancel |
| Schema and protocol coexistence | Product vs schema vs IPC versions |
| Embedded versus standalone divergence | Dual-mode risk |
| Product-boundary confusion | Vault vs Artifact Store |
| Clinical-authority scope creep | Discovery mistaken for truth |

### Rollback (planning only)

If rejected before acceptance: revert this ADR file; retain Q5 / P3 / fehrest-integration planning evidence; do **not** initialize Fehrest, import Graphify, or implement sidecars from this draft. Post-acceptance withdrawal would need separately authorized migration — **not** defined here.

---

## 7. Non-goals

ADR-08 does **not**:

- implement Fehrest;
- initialize or modify the Fehrest repository;
- import Graphify;
- select external providers;
- implement search, graph, ranking, or embeddings;
- define final shared schemas;
- define final IPC commands;
- implement Artifact Store writes;
- mutate Artifacts or Revisions;
- define DeepMed operations;
- redesign authentication or sessions;
- finalize PHI egress;
- publish packages or releases;
- define final import or export formats;
- authorize third-party content redistribution;
- modify Supabase;
- begin R2 / T031+.

---

## 8. Relationship to other ADRs

| ADR | Status in repo | Relationship |
| --- | --- | --- |
| ADR-15 | Proposed | Rust-supervised language/runtime boundary; constrains Fehrest process shape |
| ADR-01 | Proposed | Desktop composition; sidecars under host composition |
| ADR-02 | Proposed | Trusted Host authority; mediation for privileged effects |
| ADR-03 | Proposed | `afia-ui` migration; not Fehrest SoT |
| ADR-04 | Proposed | `Source` / `Relationship` / `ExportManifest` semantics remain here-owned by Fanatir |
| ADR-05 | Proposed | Artifact/Revision/Run persistence authority; Fehrest not the store |
| ADR-06 | Proposed | IPC envelopes, protocols, worker contracts |
| ADR-07 | Proposed | Supabase/local-first; Fehrest product data not remote SoT |
| ADR-09 | Not authored | DeepMed integration / OpenMed boundary |
| ADR-10 | Not authored | commandF ownership |
| ADR-11 | Not authored | Auth/session preservation |
| ADR-14 | Not authored | PHI classification and egress enforcement |

ADR-08 decides only **Fehrest Integration and Release Model**.

---

## 9. Gates and authority

```text
T020 produces a Proposed ADR-08 draft only.
Tier A independent review is required (Fehrest integration ADR).
Passing Tier A review is not architecture acceptance.
tasks.md founder-acceptance field for T020 is: No — draft only.
T030 remains incomplete.
At T030, ADR-08 is expected to be listed Reviewed or Accepted — it is not automatically Accepted.
ADR-08 is NOT in the mandatory T030 Accepted set that opens R2 (ADR-15/01/02/06).
T044+ requires ADR-08 Accepted for Fehrest implementation (and related gates).
T050 requires ADR-08, ADR-09, and ADR-05 Accepted.
T031+ and R2 remain separately gated.
Provider calls, imports, Artifact mutation, PHI egress, packaging, release, and implementation require separate authorization.
```

| Gate | Meaning for ADR-08 |
| --- | --- |
| T020 | Draft authoring (this task) |
| Tier A | Mandatory independent review |
| Founder draft acceptance | Not required by tasks.md for T020 work product |
| T030 | R1 gate; ADR-08 listed Reviewed or Accepted; **not** mandatory Accepted-for-R2 |
| T044–T047 | Fehrest product/integration after ADR-08 Accepted (as applicable) |
| T050 | Persist reviewed DeepMed results into Fehrest after ADR-08/09/05 Accepted |
| T031+ / R2 | Separately gated; not authorized here |

---

## 10. Validation and acceptance plan

```text
T020 completion produces a draft for Tier A review.
Architecture acceptance is not performed by T020.
```

Planning reviews **may** include: Q5 capability fidelity; P3 non-overstatement; fehrest-integration ownership split; ADR-04 semantic non-theft; ADR-05 Artifact Store non-absorption; ADR-06 IPC non-publication; ADR-07 Supabase non-SoT; Graphify exclusion; provider silence; Tier A independent review ([tasks.md](../tasks.md) T020).

Verification method from tasks.md: doc review; link from plan roadmap (plan already contains ADR-08 row — **not** modified by T020).

---

## 11. Unresolved questions

T020 does **not** resolve these. Attempted resolution: **NO**.

| ID | Question | May remain open in draft review? | Must resolve before implementation? | Belongs elsewhere? |
| --- | --- | --- | --- | --- |
| U-ADR08-1 | Standalone versus embedded entry points | YES | YES | This ADR + later Fehrest/Fanatir specs |
| U-ADR08-2 | Repository/release ownership operational detail | YES | YES | This ADR + Fehrest release process |
| U-ADR08-3 | Bundled versus external sidecar | YES | YES | This ADR; packaging may touch ADR-13 |
| U-ADR08-4 | Release-channel model detail beyond P3 | YES | YES | This ADR |
| U-ADR08-5 | Compatibility pin format | YES | YES | This ADR |
| U-ADR08-6 | Supported-version matrix fields | YES | YES | This ADR |
| U-ADR08-7 | Update authority | YES | YES | This ADR; distribution → ADR-13/14/signing |
| U-ADR08-8 | Rollback behavior detail | YES | YES | This ADR |
| U-ADR08-9 | Sidecar discovery and launch | YES | YES | This ADR + ADR-06/02 |
| U-ADR08-10 | Vault-location policy | YES | YES | This ADR + host FS mediation |
| U-ADR08-11 | Local search-index technology | YES | YES before search impl | Later Fehrest impl specs |
| U-ADR08-12 | Graph storage model | YES | YES before graph impl | Later Fehrest impl specs |
| U-ADR08-13 | Index rebuild rules | YES | YES | This ADR + Fehrest impl |
| U-ADR08-14 | Source/Relationship publication path | YES | YES | This ADR + ADR-04 |
| U-ADR08-15 | Candidate-to-authoritative handoff | YES | YES | This ADR + ADR-04/02 |
| U-ADR08-16 | Artifact Store publication mechanism | YES | YES | ADR-05 + host; this ADR constrains |
| U-ADR08-17 | Run/provenance recording for Fehrest ops | YES | YES | ADR-04/05; DeepMed Runs → ADR-09 |
| U-ADR08-18 | Import formats | YES | YES before import impl | Later Fehrest specs |
| U-ADR08-19 | Export format version | YES | YES | This ADR + ADR-04 ExportManifest |
| U-ADR08-20 | Full-text rights rules | YES | YES before redistribution | This ADR + legal/rights review |
| U-ADR08-21 | Metadata licensing | YES | YES before provider/metadata claims | This ADR |
| U-ADR08-22 | Graphify disposition | YES | YES before any Graphify use | Separate Fehrest + license review |
| U-ADR08-23 | Provider selection | YES | YES before provider use | Separate authorization |
| U-ADR08-24 | Provider credential handling | YES | YES if providers authorized | Secrets/host; ADR-14 |
| U-ADR08-25 | Network policy for Fehrest | YES | YES | ADR-02/14 + this ADR |
| U-ADR08-26 | Retries and rate limits | YES | YES if remote calls exist | This ADR + host |
| U-ADR08-27 | PHI-bearing searches | YES | YES | ADR-14 + this ADR |
| U-ADR08-28 | Local/offline posture detail | YES | YES before offline claims | This ADR |
| U-ADR08-29 | Supabase-reference boundary for Fehrest | YES | YES before remote refs | ADR-07 + this ADR |
| U-ADR08-30 | DeepMed handoff mechanics | YES | YES before T050 | ADR-09 + T050 |
| U-ADR08-31 | Audit correlation identifiers | YES | YES | ADR-02 / later audit |
| U-ADR08-32 | Failure recovery tooling | YES | YES | ADR-06 + this ADR |
| U-ADR08-33 | Cancellation semantics | YES | YES | ADR-06 |
| U-ADR08-34 | Compatibility testing method | YES | YES before release claims | This ADR + R3/R5 gates |
| U-ADR08-35 | Packaging and distribution | YES | Before distributable builds | ADR-13; Decision D / ADR-14 |

---

## 12. Security and privacy claim language

```text
Claim language only.
No regulatory certification is asserted.
No Fehrest integration, sidecar, provider call, Graphify import, or PHI egress is asserted as implemented.
PHI posture follows classification planning; egress engine detail remains ADR-14.
Fehrest is not proposed as clinical, FHIR, patient, or PolicyDecision source of truth.
```

---

## 13. References

- [tasks.md](../tasks.md) — T020, T030, T044–T047, T050
- [plan.md](../plan.md) — ADR-08 roadmap row; R3 Fehrest sequencing
- [spec.md](../spec.md) — Q5; repository map; memory ownership; cross-repo boundaries
- [research.md](../research.md) — R10; P3
- [fehrest-integration.md](../contracts/fehrest-integration.md) — ownership and Alpha integration surface
- [data-model.md](../data-model.md) — Source / Relationship / ExportManifest / Run
- [shared-primitives.md](../contracts/shared-primitives.md) — consumers and minimum types
- [trusted-host-ipc.md](../contracts/trusted-host-ipc.md) — worker.spawn and host principles
- [worker-runtime.md](../contracts/worker-runtime.md) — non-Rust worker prohibitions
- [ADR-15](./ADR-15-rust-first-polyglot-runtime.md) (Proposed)
- [ADR-01](./ADR-01-platform-and-desktop-composition-tauri-2.md) (Proposed)
- [ADR-02](./ADR-02-rust-trusted-host-boundary.md) (Proposed)
- [ADR-03](./ADR-03-afia-ui-strangler-migration.md) (Proposed)
- [ADR-04](./ADR-04-shared-primitive-ownership-and-versioning.md) (Proposed)
- [ADR-05](./ADR-05-artifact-revision-run-storage.md) (Proposed)
- [ADR-06](./ADR-06-worker-and-ipc-contracts.md) (Proposed)
- [ADR-07](./ADR-07-supabase-adapter-and-local-first-boundary.md) (Proposed)

---

```text
While Proposed:
- no Fehrest implementation from this file;
- no Fehrest repository initialization or modification;
- no Graphify fork/import;
- no external-provider integration;
- no final IPC schema publication;
- no Artifact/Source/Relationship durable mutation;
- no PHI egress;
- no auth/session behavior change;
- no R2 / T031+ work from this file alone.
```

```text
End of ADR-08 Proposed draft.
```
