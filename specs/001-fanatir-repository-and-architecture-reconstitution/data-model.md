# Data Model: Shared Primitive Boundaries (001)

**Date**: 2026-07-23  
**Scope**: Interface boundaries only — not finalized production schemas  
**Owner**: Fanatir (canonical); Fehrest/DeepMed consume versioned contracts

Schemas MUST remain language-neutral. Prefer JSON Schema documents under `contracts/` with `$id` and semver. Do not treat this document as authorization to mutate databases or rename packages.

**Language authority (ADR-15 / Decision A):** Durable mutation of Artifact, Revision, and Run — and patient/project storage authority — occurs only through the **Rust Trusted Host**. Non-Rust workers and UI may propose results via IPC; they must not bypass the kernel.

**Decision C:** Document content, extracted entities, patient material, FHIR containing PHI, and Artifact revisions are authoritative only in the local Rust-controlled Artifact Store — not Supabase.

## Classification

Every durable entity MUST declare `DataClassification` (see below). PHI-bearing entities MUST NOT default to Supabase/cloud adapters.

## Entities

### Workspace

- **Purpose**: Collaboration/tenancy boundary for optional cloud membership.
- **Core fields (boundary)**: `id`, `displayName`, `membershipRefs[]`, `createdAt`
- **Notes**: May map to current Supabase workspaces; not Artifact SoT.

### Project

- **Purpose**: Local-first unit of work opened by Trusted Host.
- **Core fields**: `id`, `rootPath` (host-mediated), `displayName`, `workspaceId?`, `memoryRefs`
- **Rules**: Opening/creating requires host FS mediation; path never trusted from WebView alone.

### Patient

- **Purpose**: Fanatir patient-scoped clinical continuity identity (local authority).
- **Core fields**: `id`, `displayLabel`, `classification`, `longitudinalMemoryRef`
- **Rules**: Patient access authority ≠ ordinary project membership. Fehrest MUST NOT be SoT for Patient clinical records.

### Artifact

- **Purpose**: Durable content object with provenance.
- **Core fields**: `id`, `projectId`, `kind`, `uri` (local store), `revisionId`, `sourceIds[]`, `classification`, `createdBy`, `createdAt`
- **Rules**: AI summaries are not original sources. Local Artifact store is host-mediated.

### Revision

- **Purpose**: Immutable version of an Artifact.
- **Core fields**: `id`, `artifactId`, `parentRevisionId?`, `contentHash`, `createdAt`, `author`

### Run

- **Purpose**: Execution record for tools/models/workers.
- **Core fields**: `id`, `projectId`, `capabilityId`, `worker`, `modelId?`, `inputArtifactIds[]`, `outputArtifactIds[]`, `status`, `startedAt`, `endedAt`, `auditEventIds[]`
- **Rules**: DeepMed invocations MUST create Runs with model + provenance.

### Source

- **Purpose**: Original or cited evidence object.
- **Core fields**: `id`, `kind` (file|url|note|fhir…), `locator`, `contentHash?`, `classification`
- **Rules**: Fehrest quotations cite Sources; DeepMed source spans reference Source locators.

### Relationship

- **Purpose**: Typed link between entities (human | extracted | inferred).
- **Core fields**: `id`, `fromRef`, `toRef`, `type`, `origin` (`human|extracted|inferred`), `confidence?`, `runId?`
- **Rules**: Origin MUST be distinguishable (Fehrest Alpha requirement).

### Review

- **Purpose**: Human review of an AI or transformed result.
- **Core fields**: `id`, `targetRef`, `reviewerId`, `status`, `comments`, `corrections[]`, `createdAt`

### Approval

- **Purpose**: Explicit human approval for share/export/clinical-assistive handoff.
- **Core fields**: `id`, `targetRef`, `approverId`, `decision` (`approved|rejected`), `rationale`, `createdAt`
- **Rules**: No silent clinical approval. CoLab Alpha may provide bounded Approval.

### Decision

- **Purpose**: Recorded product/architecture/runtime decision with authority class.
- **Core fields**: `id`, `summary`, `authority` (`constitution|founder|adr|spec|runtime`), `refs[]`, `createdAt`

### PolicyDecision

- **Purpose**: Capability Gateway / policy engine outcome.
- **Core fields**: `id`, `capabilityId`, `principal`, `resourceRef`, `classification`, `outcome` (`allow|deny`), `reasons[]`, `createdAt`

### DataClassification

- **Purpose**: Enumerated sensitivity label.
- **Values (minimum)**: `public`, `internal`, `restricted`, `phi`, `secrets`
- **Rules**: `phi` forbids default cloud egress; host enforces.

### Capability

- **Purpose**: Named privileged action (model, tool, sidecar, export, MCP).
- **Core fields**: `id`, `name`, `riskTier`, `requiredClassificationMax`, `gatewayRequired` (bool)

### ExportManifest

- **Purpose**: Portable export inventory for project/Fehrest bundles.
- **Core fields**: `id`, `projectId`, `artifactIds[]`, `excludeClassifications[]`, `createdAt`, `signature?`
- **Rules**: Reproducibility and privacy verification consume this object.

## Relationships (summary)

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

## State transitions (selected)

### Run.status

`queued → running → succeeded | failed | cancelled`  
Crash recovery: host marks `failed` + audit; no auto-Approval.

### Review.status

`open → changes_requested | accepted` → may spawn Approval

### Approval.decision

Terminal: `approved` | `rejected`

## Validation rules (planning)

1. Entities with `classification=phi` cannot be sent to optional cloud adapters without explicit PolicyDecision allow + founder-ratified exception path (not in Alpha default).
2. DeepMed outputs without source spans are non-accept for Alpha DeepMed step.
3. Fehrest Relationships MUST set `origin`.
4. Artifact without Revision is invalid for durable store.
5. ExportManifest MUST list exclusions for PHI when packaging share bundles.
