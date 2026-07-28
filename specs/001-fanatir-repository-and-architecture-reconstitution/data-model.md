# Data Model: Shared Primitive Boundaries (001)

| Field | Value |
| --- | --- |
| **Document** | Shared primitive conceptual data model (planning) |
| **Version** | 0.2.0-draft (planning alignment; T027) |
| **Date** | 2026-07-23 (original); aligned 2026-07-25 under T027 |
| **Owner** | Fanatir (canonical); Fehrest / DeepMed consume versioned contracts later |
| **Companion** | [contracts/shared-primitives.md](./contracts/shared-primitives.md) |
| **Governing ADR (structures)** | [ADR-04](./adrs/ADR-04-shared-primitive-ownership-and-versioning.md) (**Proposed**) |
| **Posture** | Planning contract — language-neutral, non-runtime, non-database, non-migration, not generated bindings, not `packages/contracts` production publication |

```text
Status: planning draft (non-authoritative)
Interface boundaries only — not finalized production schemas
T027 alignment ≠ architecture acceptance
ADR-04 / ADR-15 / ADR-01–ADR-14 remain Proposed
Do not treat this document as authorization to mutate databases, rename packages, or implement enforcement
```

Schemas **MUST** remain language-neutral. Prefer planning documentation under `contracts/` with `$id` and semver as **publication conventions** when later authorized. This document does **not** publish final JSON Schema, OpenAPI, Protobuf, or language structs.

**Language authority ([ADR-15](./adrs/ADR-15-rust-first-polyglot-runtime.md) / Decision A — Proposed):** Durable mutation of Artifact, Revision, and Run — and patient/project storage authority — occurs only through the **Rust Trusted Host**. Non-Rust workers and UI may propose results via IPC; they must not bypass the kernel.

**Decision C:** Document content, extracted entities, patient material, FHIR containing PHI, and Artifact revisions are authoritative only in the local Rust-controlled **Artifact Store** — not Supabase ([ADR-07](./adrs/ADR-07-supabase-adapter-and-local-first-boundary.md)).

---

## 1. Document role

| Document | Role |
| --- | --- |
| **This file** (`data-model.md`) | Conceptual entities, relationships, lifecycle, aggregate planning |
| **[shared-primitives.md](./contracts/shared-primitives.md)** | Concise cross-runtime primitive contracts and semantic boundaries |

Both are subject to **Tier B — contract group** review (`tasks.md` T027). Neither is Accepted architecture.

---

## 2. Cross-ADR ownership matrix (all Proposed)

| Domain | Planning owner |
| --- | --- |
| Shared primitive structures | [ADR-04](./adrs/ADR-04-shared-primitive-ownership-and-versioning.md) |
| Trusted Host / Capability Gateway | [ADR-02](./adrs/ADR-02-rust-trusted-host-boundary.md) |
| Artifact persistence | [ADR-05](./adrs/ADR-05-artifact-revision-run-storage.md) |
| IPC and envelopes | [ADR-06](./adrs/ADR-06-worker-and-ipc-contracts.md) |
| Supabase / local-first | [ADR-07](./adrs/ADR-07-supabase-adapter-and-local-first-boundary.md) |
| Fehrest integration | [ADR-08](./adrs/ADR-08-fehrest-integration-and-release-model.md) |
| DeepMed integration | [ADR-09](./adrs/ADR-09-deepmed-integration-and-openmed-runtime-fork-boundary.md) |
| commandF integration | [ADR-10](./adrs/ADR-10-commandf-ownership-and-process-boundary.md) |
| AuthContext / session | [ADR-11](./adrs/ADR-11-auth-and-session-preservation.md) |
| Technical naming | [ADR-12](./adrs/ADR-12-technical-afia-to-fanatir-rename-strategy.md) |
| Packaging | [ADR-13](./adrs/ADR-13-first-vertical-slice-packaging.md) |
| Classification / egress enforcement | [ADR-14](./adrs/ADR-14-security-and-data-classification-enforcement.md) |
| Supervised workers / Rust core | [ADR-15](./adrs/ADR-15-rust-first-polyglot-runtime.md) |

---

## 3. Classification

Every durable entity **MUST** declare `DataClassification`. PHI-bearing entities **MUST NOT** default to Supabase/cloud adapters.

### `DataClassification`

| Aspect | Planning rule |
| --- | --- |
| Minimum values | `public`, `internal`, `restricted`, `phi`, `secrets` |
| Final enum? | **No** — minimum planning values only |
| Structure owner | [ADR-04](./adrs/ADR-04-shared-primitive-ownership-and-versioning.md) |
| Enforcement owner | [ADR-14](./adrs/ADR-14-security-and-data-classification-enforcement.md) |
| Not equivalent to | Access control; consent; de-identification; legal status proof |
| Unknown / conflicting | Fail closed for privileged/egress operations (ADR-14 planning) |
| Inheritance / aggregation / downgrade | **Unresolved** — no algorithm published here |
| `phi` | Forbids default cloud egress; host enforces |

No additional classification labels are invented.

---

## 4. Supporting context (not shared primitives)

| Concept | Treatment |
| --- | --- |
| `AuthContext` | [ADR-11](./adrs/ADR-11-auth-and-session-preservation.md); may provide authenticated actor/session context to policy; login ≠ authorization; profile ≠ identity proof; remote identity ≠ local Artifact authority; offline/remote identity relationships unresolved |
| Actor / provider / capability / workspace / project / correlation identifiers | Opaque planning identifiers |
| Timestamps, hashes/digests, URIs, schema versions | Opaque scalars; algorithms, precision, grammar, UUID vs ULID unresolved |

---

## 5. Entities (planning inventory)

Core field lists are **non-normative boundary sketches**, not approved cardinalities, wire formats, or storage schemas.

### Workspace

- **Purpose**: Collaboration/tenancy boundary for optional cloud membership.
- **Sketch fields**: `id`, `displayName`, `membershipRefs[]`, `createdAt`
- **Rules**: May map to current Supabase workspaces; **not** Artifact source of truth; remote membership does not grant local Artifact authority.

### Project

- **Purpose**: Local-first bounded unit of work opened by the Trusted Host.
- **Sketch fields**: `id`, `rootPath` (host-mediated), `displayName`, `workspaceId?`, `memoryRefs`
- **Rules**: Opening/creating requires host filesystem mediation; path never trusted from WebView alone. Not equivalent to repository, directory, or Supabase row alone.

### Patient

- **Purpose**: Fanatir patient-scoped clinical continuity identity (local authority) where clinical workflows apply.
- **Sketch fields**: `id`, `displayLabel`, `classification`, `longitudinalMemoryRef`
- **Rules**: May be absent from non-clinical workflows. Patient access authority ≠ ordinary project membership. Fehrest **MUST NOT** be source of truth for Patient clinical records. Does not imply Fanatir stores PHI remotely.

### Artifact

- **Purpose**: Durable content object with provenance.
- **Sketch fields**: `id`, `projectId`, `kind`, `uri` (local store locator sketch), `revisionId`, `sourceIds[]`, `classification`, `createdBy`, `createdAt`
- **Rules**: Host-mediated; not a filesystem path, Supabase row, or worker-local object; may have multiple immutable Revisions; AI summaries are not original sources; access ≠ egress; export separately governed. Persistence: [ADR-05](./adrs/ADR-05-artifact-revision-run-storage.md).

### Revision

- **Purpose**: Immutable version of an Artifact.
- **Sketch fields**: `id`, `artifactId`, `parentRevisionId?`, `contentHash`, `createdAt`, `author`
- **Rules**: Immutable once committed; associated with one Artifact; provenance-bearing; classifiable; may be derived; distinct from mutable display/index metadata. Content-addressed storage is **not** mandated. No final schema published.

### Run

- **Purpose**: Record of bounded execution intent and outcome for tools/models/workers.
- **Sketch fields**: `id`, `projectId`, `capabilityId`, `worker`, `modelId?`, `inputArtifactIds[]`, `outputArtifactIds[]`, `status`, `startedAt`, `endedAt`, `auditEventIds[]`
- **Rules**: DeepMed invocations **MUST** create Runs with model + provenance. May reference Approval or PolicyDecision where relevant. Classification context applies. No Run engine is implemented by this document.

### Source

- **Purpose**: Original or cited evidence / provenance input.
- **Sketch fields**: `id`, `kind` (illustrative: file \| url \| note \| fhir …), `locator`, `contentHash?`, `classification`
- **Rules**: May represent imported content, repository evidence, external references, user material, or generated/transformed content. Final source kinds unresolved. Fehrest quotations cite Sources; DeepMed source spans reference Source locators.

### Relationship

- **Purpose**: Typed link between governed entities.
- **Sketch fields**: `id`, `fromRef`, `toRef`, `type`, `origin` (`human|extracted|inferred`), `confidence?`, `runId?`
- **Rules**: Origin **MUST** be distinguishable (Fehrest Alpha requirement). Final relationship type enums unresolved. Preserve provenance and derivation distinctions.

### Review

- **Purpose**: Structured human evaluation of an AI or transformed result.
- **Sketch fields**: `id`, `targetRef`, `reviewerId`, `status`, `comments`, `corrections[]`, `createdAt`
- **Rules**: Review is **not** necessarily authorization.

### Approval

- **Purpose**: Explicit, scoped, attributable human approval for share/export/clinical-assistive handoff.
- **Sketch fields**: `id`, `targetRef`, `approverId`, `decision` (`approved|rejected`), `rationale`, `createdAt`
- **Rules**: No silent clinical approval. CoLab Alpha may provide bounded Approval. **Not** automatically: user confirmation; patient consent; authentication; founder acceptance; architecture acceptance; stage-gate acceptance; release approval; or `PolicyDecision`.

### Decision

- **Purpose**: Recorded product/architecture/runtime decision with authority class.
- **Sketch fields**: `id`, `summary`, `authority` (`constitution|founder|adr|spec|runtime`), `refs[]`, `createdAt`
- **Rules**: Not necessarily a runtime authorization result. Distinct from Review and Approval.

### PolicyDecision

- **Purpose**: Capability Gateway / Trusted Host policy evaluation outcome **as data**.
- **Sketch fields**: `id`, `capabilityId`, `principal`, `resourceRef`, `classification`, `outcome` (`allow|deny`), `reasons[]`, `createdAt`
- **Semantic planning may also consider** (no final fields): operation, purpose, destination, provider, scope, minimum necessary, Approval evidence, expiry, audit correlation.
- **Rules**: Not self-executing; distinct from Approval, login, confirmation, and consent. Issuance authority: [ADR-02](./adrs/ADR-02-rust-trusted-host-boundary.md). Enforcement semantics: [ADR-14](./adrs/ADR-14-security-and-data-classification-enforcement.md).

### Capability

- **Purpose**: Named privileged action description (model, tool, sidecar, export, MCP) **as data**.
- **Sketch fields**: `id`, `name`, `riskTier`, `requiredClassificationMax`, `gatewayRequired` (bool)
- **Rules**: Does not imply ambient UI/worker self-authorization, unrestricted filesystem/network/secret access, or PHI egress. Runtime enforcement: [ADR-02](./adrs/ADR-02-rust-trusted-host-boundary.md) / [ADR-15](./adrs/ADR-15-rust-first-polyglot-runtime.md). Final capability identifiers unresolved.

### ExportManifest

- **Purpose**: Portable export / disclosure inventory for project or Fehrest bundles.
- **Sketch fields**: `id`, `projectId`, `artifactIds[]`, `excludeClassifications[]`, `createdAt`, `signature?`
- **Planning semantics may also consider**: Revision refs, destination/recipient, purpose, format, provenance, redaction/de-identification evidence, Approval or PolicyDecision reference, integrity metadata, expiry/revocation.
- **Rules**: Does **not** itself authorize export. Local file creation may still constitute egress. Publication requires stronger authority. Reproducibility and privacy verification consume this object. Enforcement: [ADR-14](./adrs/ADR-14-security-and-data-classification-enforcement.md). Final export format unresolved.

---

## 6. Relationships (summary)

```text
Workspace 1—* Project
Project 1—* Artifact 1—* Revision
Project 1—* Run
Run *—* Artifact (inputs/outputs)
Artifact *—* Source
Source *—* Relationship
Run → PolicyDecision / Capability
Artifact|Run → Review → Approval
ExportManifest → Artifact[]
Patient 1—* longitudinal Artifacts (Fanatir-owned)
```

---

## 7. State transitions (planning sketches — not final enums)

### Run.status

Planning lifecycle sketch:

`queued → running → succeeded | failed | cancelled`

Crash recovery planning: host marks `failed` + audit; no auto-approval. This is a **planning state model**, not a final runtime enum.

### Review.status

`open → changes_requested | accepted` → may spawn Approval

### Approval.decision

Terminal sketch: `approved` | `rejected`

### Error and status distinctions (names only)

Distinguish without inventing a final taxonomy: execution failure; policy denial; approval required; cancellation; timeout; unavailable dependency; invalid request; conflicting state; unsupported operation.

---

## 8. Validation rules (planning)

1. Entities with `classification=phi` cannot be sent to optional cloud adapters without explicit PolicyDecision allow + founder-ratified exception path (not in Alpha default).
2. DeepMed outputs without source spans are non-accept for Alpha DeepMed step.
3. Fehrest Relationships **MUST** set `origin`.
4. Artifact without Revision is invalid for durable store.
5. ExportManifest **MUST** list exclusions for PHI when packaging share bundles.
6. Classification fields remain present; unknown/conflicting classification fails closed for privileged/egress operations under ADR-14 planning.

---

## 9. Versioning and serialization

| Domain | Notes |
| --- | --- |
| Document versions | This file and shared-primitives versions may advance independently |
| Artifact / Revision / Run schema versions | Separate from IPC, persistence, and app versions ([ADR-04](./adrs/ADR-04-shared-primitive-ownership-and-versioning.md)) |
| Serialization baseline | UTF-8; JSON-compatible interchange where stated; `$id` + `schemaVersion` (semver) |
| Not published | Final JSON objects, JSON Schema artifacts, OpenAPI, Protobuf, bindings, DDL |

No migration guarantees. Unknown-field handling unresolved.

---

## 10. Persistence, IPC, Supabase, and products

| Boundary | Rule |
| --- | --- |
| Persistence | Shared primitives do not prescribe storage layout; [ADR-05](./adrs/ADR-05-artifact-revision-run-storage.md) owns Artifact Store; caches/indexes are not canonical objects by default; T027 creates no migration |
| IPC | Shared meanings may be referenced by IPC; [ADR-06](./adrs/ADR-06-worker-and-ipc-contracts.md) owns envelopes; T027 does not modify [trusted-host-ipc.md](./contracts/trusted-host-ipc.md) or [worker-runtime.md](./contracts/worker-runtime.md); no executable handshake published |
| Supabase | Optional collaboration/identity; not Artifact/PHI SoT; no PHI by default; RLS ≠ host policy; remote rows ≠ local authority ([ADR-07](./adrs/ADR-07-supabase-adapter-and-local-first-boundary.md)) |
| Fehrest | Separate ([ADR-08](./adrs/ADR-08-fehrest-integration-and-release-model.md)); vault identifiers ≠ Fanatir Artifact identifiers automatically |
| DeepMed | Separate ([ADR-09](./adrs/ADR-09-deepmed-integration-and-openmed-runtime-fork-boundary.md)); prompts/outputs classifiable; no final DeepMed API |
| commandF | Remains commandF ([ADR-10](./adrs/ADR-10-commandf-ownership-and-process-boundary.md)); FHIR I/O may map to Artifact/Revision/Run concepts; successful validation ≠ transmission; external validation/terminology/live-system writes separately gated; no final commandF protocol |

---

## 11. Security and privacy

- Classification fields present;
- Minimum classification values only as listed;
- No real PHI, production identifiers, or secrets in this document;
- No-PHI logging posture; minimum necessary; default-deny PHI egress;
- No HIPAA/PDPL/GDPR certification or compliance claims;
- Synthetic/abstract examples only;
- Do not weaken [ADR-14](./adrs/ADR-14-security-and-data-classification-enforcement.md).

---

## 12. Non-goals

- Final runtime or database schemas;
- Production migration;
- `packages/contracts` production publish;
- Generated bindings;
- Executable IPC;
- Classification aggregation/inheritance algorithms;
- Auth/session, Supabase, Fehrest, DeepMed, or commandF implementation;
- Architecture acceptance.

---

## 13. Consistency with shared-primitives.md

Primitive names, capitalization, ownership, classification values, Artifact/Revision/Run distinctions, Review/Approval/Decision/PolicyDecision separations, AuthContext boundary, persistence/Supabase/IPC deferrals, and product boundaries in this file **must** match [contracts/shared-primitives.md](./contracts/shared-primitives.md).
