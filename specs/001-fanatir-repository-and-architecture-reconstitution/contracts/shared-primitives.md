# Contract: Shared Primitives (boundary)

| Field | Value |
| --- | --- |
| **Document** | Shared primitives planning contract |
| **Version** | 0.2.0-draft (planning alignment; T027) |
| **Owner** | Fanatir (canonical semantics) |
| **Consumers (planned)** | Fanatir Trusted Host / UI / workers; Fehrest; DeepMed |
| **Task origin** | T027 |
| **Companion** | [../data-model.md](../data-model.md) |
| **Governing ADR (structures)** | [ADR-04](../adrs/ADR-04-shared-primitive-ownership-and-versioning.md) (**Proposed**) |
| **Posture** | Planning contract only — language-neutral, non-runtime, non-database, non-migration, not generated bindings, not `packages/contracts` production publication |

```text
Status: planning draft (non-authoritative)
T027 alignment ≠ architecture acceptance
ADR-04 / ADR-15 / ADR-01–ADR-14 remain Proposed
No final wire, storage, or API schema is published here
```

This document and [../data-model.md](../data-model.md) together describe one coherent **planning** model. Prefer semantic tables over code-like schemas. Illustrative field lists are **non-normative boundary sketches**.

---

## 1. Document role

| Document | Role |
| --- | --- |
| **This file** (`shared-primitives.md`) | Concise cross-runtime primitive contracts and semantic boundaries |
| **[data-model.md](../data-model.md)** | Conceptual entities, relationships, lifecycle, and aggregate planning |

Both are subject to **Tier B — contract group** review (`tasks.md` T027). Neither is Accepted architecture.

---

## 2. Cross-ADR ownership matrix (all Proposed)

| Domain | Planning owner |
| --- | --- |
| Shared primitive structures | [ADR-04](../adrs/ADR-04-shared-primitive-ownership-and-versioning.md) |
| Trusted Host / Capability Gateway | [ADR-02](../adrs/ADR-02-rust-trusted-host-boundary.md) |
| Artifact persistence | [ADR-05](../adrs/ADR-05-artifact-revision-run-storage.md) |
| IPC and envelopes | [ADR-06](../adrs/ADR-06-worker-and-ipc-contracts.md) |
| Supabase / local-first | [ADR-07](../adrs/ADR-07-supabase-adapter-and-local-first-boundary.md) |
| Fehrest integration | [ADR-08](../adrs/ADR-08-fehrest-integration-and-release-model.md) |
| DeepMed integration | [ADR-09](../adrs/ADR-09-deepmed-integration-and-openmed-runtime-fork-boundary.md) |
| commandF integration | [ADR-10](../adrs/ADR-10-commandf-ownership-and-process-boundary.md) |
| AuthContext / session | [ADR-11](../adrs/ADR-11-auth-and-session-preservation.md) |
| Technical naming | [ADR-12](../adrs/ADR-12-technical-afia-to-fanatir-rename-strategy.md) |
| Packaging | [ADR-13](../adrs/ADR-13-first-vertical-slice-packaging.md) |
| Classification / egress enforcement | [ADR-14](../adrs/ADR-14-security-and-data-classification-enforcement.md) |
| Supervised workers / Rust core | [ADR-15](../adrs/ADR-15-rust-first-polyglot-runtime.md) |

Ownership here is **planning**. Proposed ADRs are not Accepted.

---

## 3. Minimum shared primitive set

Language-neutral planning types (same set as [data-model.md](../data-model.md)):

| Primitive | Planning purpose |
| --- | --- |
| `Workspace` | Collaboration / organizational context |
| `Project` | Bounded work context |
| `Patient` | Patient-associated context where clinical continuity applies |
| `Artifact` | Durable, host-mediated logical content object |
| `Revision` | Immutable committed version of an Artifact |
| `Run` | Bounded execution intent and outcome record |
| `Source` | Provenance input / evidence object |
| `Relationship` | Typed link among governed entities |
| `Review` | Structured evaluation of a work product |
| `Approval` | Scoped, attributable authorization/acceptance record |
| `Decision` | Durable record of a governed decision and rationale |
| `PolicyDecision` | Data representation of a privileged policy evaluation outcome |
| `DataClassification` | Sensitivity label (minimum planning values) |
| `Capability` | Bounded description of permitted operation scope |
| `ExportManifest` | Planning evidence describing an export/disclosure package |

Do not silently remove primitives. Do not invent new shared primitives without canonical evidence.

---

## 4. Supporting context (not absorbed into the shared set)

| Concept | Treatment |
| --- | --- |
| `AuthContext` | Owned by [ADR-11](../adrs/ADR-11-auth-and-session-preservation.md); may inform policy; **not** a shared primitive in this export set; authentication ≠ authorization |
| Actor / user identifiers | Opaque planning identifiers |
| Provider identifiers | Opaque; provider use separately gated ([ADR-14](../adrs/ADR-14-security-and-data-classification-enforcement.md)) |
| Capability identifiers | Opaque references to `Capability` descriptions |
| Workspace / project identifiers | Opaque |
| Correlation identifiers | Opaque audit/correlation handles |
| Timestamps | Opaque temporal values; precision unresolved |
| Hashes / digests | Opaque integrity metadata; algorithm unresolved |
| URIs | Opaque locators; grammar unresolved |
| Schema versions | `schemaVersion` (semver) as planning/publication convention |

---

## 5. Primitive semantic boundaries

### 5.1 `Artifact`

Durable, host-mediated logical object with provenance and classification. **Not** a filesystem path, Supabase row, or worker-local object. May have multiple immutable `Revision` records. Export only through separately governed operations. Access ≠ egress ([ADR-14](../adrs/ADR-14-security-and-data-classification-enforcement.md)). Persistence owned by [ADR-05](../adrs/ADR-05-artifact-revision-run-storage.md).

### 5.2 `Revision`

Immutable once committed to conceptual Artifact history; associated with one Artifact; provenance-bearing; classifiable; may be derived. Distinct from mutable display/index metadata. No final revision schema published.

### 5.3 `Run`

Record of bounded execution intent and outcome (worker/tool/model/provider, inputs/outputs, status, timing, provenance, classification context, optional Approval/PolicyDecision refs, audit correlation). Planning lifecycle sketch (not final enum): `queued → running → succeeded | failed | cancelled`. DeepMed invocations must create Runs with model + provenance ([data-model.md](../data-model.md)).

### 5.4 `Source`

Provenance input (imported content, repository evidence, external reference, user material, generated/transformed content). Final source kinds unresolved.

### 5.5 `Relationship`

Typed links among Artifacts, Revisions, Sources, Runs, Reviews, or other governed entities. Origin distinguishability required for Fehrest Alpha planning. Final relationship enums unresolved.

### 5.6 `Review` / `Approval` / `Decision`

Keep distinct:

| Type | Is | Is not |
| --- | --- | --- |
| `Review` | Structured evaluation | Automatically authorization |
| `Approval` | Scoped, attributable workflow authorization/acceptance | User confirmation; consent; login; founder/architecture/stage-gate/release acceptance; `PolicyDecision` |
| `Decision` | Durable governed decision + rationale | Necessarily a runtime authorization result |

No silent clinical approval.

### 5.7 `PolicyDecision`

Data representation of a Capability Gateway / Trusted Host policy evaluation outcome ([ADR-02](../adrs/ADR-02-rust-trusted-host-boundary.md)). **Not** self-executing. Distinct from `Approval`, login, confirmation, consent, and architecture/founder acceptance. May reference actor, operation, capability, purpose, destination, provider, scope, classification, minimum necessary, Approval evidence, result, bounded reason, expiry, audit correlation — **semantic planning only; no final fields**.

### 5.8 `DataClassification`

Minimum planning values (not a final enum contract):

```text
public
internal
restricted
phi
secrets
```

- [ADR-04](../adrs/ADR-04-shared-primitive-ownership-and-versioning.md) owns shared structure;
- [ADR-14](../adrs/ADR-14-security-and-data-classification-enforcement.md) owns enforcement semantics;
- classification ≠ access control, consent, de-identification, or legal status proof;
- unknown/conflicting classification fails closed for privileged/egress operations under ADR-14 planning;
- inheritance, aggregation, and downgrade rules remain **unresolved** (no algorithm published here).

`phi` forbids default cloud egress. Host enforces. Every durable entity must declare classification.

### 5.9 `Capability`

Bounded description of permitted operation scope **as data**. Does not imply ambient, UI, or worker self-authorization; unrestricted filesystem/network/secret access; or PHI egress. Runtime enforcement: [ADR-02](../adrs/ADR-02-rust-trusted-host-boundary.md) / [ADR-15](../adrs/ADR-15-rust-first-polyglot-runtime.md).

### 5.10 `ExportManifest`

Planning evidence for an export/disclosure package. Does **not** itself authorize export. Local file creation may still be egress. Publication requires stronger authority. Enforcement: [ADR-14](../adrs/ADR-14-security-and-data-classification-enforcement.md).

### 5.11 `Workspace` / `Project` / `Patient`

| Primitive | Boundary |
| --- | --- |
| `Workspace` | Collaboration/org context; remote membership does not grant local Artifact authority |
| `Project` | Bounded work context; not equivalent to repository, directory, or Supabase row alone |
| `Patient` | Optional for non-clinical workflows; local clinical continuity authority where applicable; does not imply remote PHI storage |

---

## 6. Serialization and versioning (planning conventions)

| Convention | Planning baseline |
| --- | --- |
| Encoding | UTF-8 |
| Interchange shape | JSON-compatible where stated |
| Schema identity | `$id` + `schemaVersion` (semver) |
| Compatibility | Additive optional fields OK under later policy; removals/renames require major bump + governed ADR |
| Bindings | Language bindings are **consumers**; tooling selection unresolved ([ADR-04](../adrs/ADR-04-shared-primitive-ownership-and-versioning.md)) |

Separate version domains (do not collapse): shared-primitives document version; data-model version; Artifact/Revision/Run schema versions; IPC protocol; worker protocol; provider adapter; export format; persistence; application/package. No migration guarantees. Unknown-field handling remains an unresolved compatibility concern.

**Not published here:** final JSON objects, JSON Schema files, OpenAPI, Protobuf, generated bindings, database DDL, or language structs.

---

## 7. Persistence, IPC, Supabase, and products

| Boundary | Rule |
| --- | --- |
| Persistence | Shared primitives do not prescribe storage layout; [ADR-05](../adrs/ADR-05-artifact-revision-run-storage.md) owns Artifact Store; local Artifact Store is authoritative |
| IPC | Shared meanings may be referenced by IPC; [ADR-06](../adrs/ADR-06-worker-and-ipc-contracts.md) owns envelopes; this task does not modify [trusted-host-ipc.md](./trusted-host-ipc.md) or [worker-runtime.md](./worker-runtime.md) |
| Supabase | Optional collaboration/identity ([ADR-07](../adrs/ADR-07-supabase-adapter-and-local-first-boundary.md); Decision C); not Artifact/PHI SoT; no PHI by default; remote rows do not grant local authority |
| Fehrest | Separate product ([ADR-08](../adrs/ADR-08-fehrest-integration-and-release-model.md)); vault IDs ≠ Fanatir Artifact IDs automatically |
| DeepMed | Separate ([ADR-09](../adrs/ADR-09-deepmed-integration-and-openmed-runtime-fork-boundary.md)); may produce Runs/Artifacts/Revisions; no final DeepMed API here |
| commandF | Remains commandF ([ADR-10](../adrs/ADR-10-commandf-ownership-and-process-boundary.md)); validation ≠ transmission authority; no final commandF protocol here |

---

## 8. Security and privacy

- Classification fields present on durable entities;
- Minimum classification values only as listed;
- No real PHI, production identifiers, or secrets in this contract;
- No-PHI logging posture; minimum necessary; default-deny PHI egress ([ADR-14](../adrs/ADR-14-security-and-data-classification-enforcement.md));
- No HIPAA/PDPL/GDPR certification or compliance claims;
- Synthetic/abstract examples only.

---

## 9. Non-goals

- Final runtime or database schemas;
- Production migration;
- `packages/contracts` production publish;
- Generated binding tooling selection;
- Full FHIR resource schemas;
- Executable IPC envelopes;
- Classification aggregation algorithms;
- Auth/session behavior changes;
- Supabase, Fehrest, DeepMed, or commandF implementation.

---

## 10. Consistency with data-model.md

The primitive names, capitalization, ownership, classification values, Artifact/Revision/Run distinctions, Review/Approval/Decision/PolicyDecision separations, AuthContext boundary, persistence/Supabase/IPC deferrals, and product boundaries in this file **must** match [../data-model.md](../data-model.md). Conceptual detail lives in the data model; this file is the concise contract boundary.
