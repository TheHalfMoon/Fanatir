# Contract: Shared Primitives (boundary)

**Version**: 0.1.0-draft  
**Owner**: Fanatir  
**Consumers**: Fehrest, DeepMed-AI, Fanatir UI/host/workers  
**Detail**: See [../data-model.md](../data-model.md)

## Serialization

- Encoding: UTF-8 JSON
- Schema identity: `$id` + `schemaVersion` (semver)
- Compatibility: additive fields OK; removals/renames require major bump + ADR

## Minimum exported types

Workspace, Project, Patient, Artifact, Revision, Run, Source, Relationship, Review, Approval, Decision, PolicyDecision, DataClassification, Capability, ExportManifest

## Non-goals

- Full FHIR resource schemas
- Database DDL
- Generated binding tooling selection (ADR-04)
