# Contract: DeepMed-AI ↔ Fanatir

**Version**: 0.2.0-draft  
**Boundary**: Contract-first process/sidecar; Capability Gateway mandatory; Python worker under **ADR-15** (Rust-supervised)  
**OpenMed fork/import**: **Not** authorized by Spec 001 — separate DeepMed specification only  
**R3 substrate (founder Decision B)**: Bounded worker may temporarily use existing **OpenMed PyPI** runtime dependency (package use ≠ fork/import). Details via **ADR-09**.

## DeepMed owns

- Medical intelligence runtime packaging
- Model/runtime versions reported in Run provenance
- Task-first workflow execution internals

## Fanatir owns

- Studio UX integration
- Gateway authorization
- Artifact/Run persistence (Rust kernel)
- Handoff to Fehrest and commandF

## R3 temporary OpenMed PyPI substrate (founder-ratified)

Allowed as a **bounded, temporary** DeepMed worker substrate when ADR-09 records:

| Requirement | Rule |
| --- | --- |
| Dependency pin | Exact version pin |
| Execution | Local by default |
| Process | Rust-supervised isolated worker |
| Contract | Versioned request/response |
| I/O | Bounded input and output |
| Isolation | No unrestricted filesystem, secret, network, or patient-store access |
| Quality | Source spans; confidence/uncertainty; review/correction support |
| Provenance | Model and Run provenance |
| License | Apache-2.0 + NOTICE handling; per-model license manifest |
| Replacement | Explicit replacement path documented |
| Representation | Must not claim prototype is final DeepMed architecture |

Rust-first does **not** require rewriting OpenMed in Rust.

## Alpha result contract (minimum)

| Field | Requirement |
| --- | --- |
| `taskId` | Task-first workflow identity |
| `entities[]` | Selected clinical entity extraction |
| `sourceSpans[]` | Locators into Source/Artifact |
| `confidence` / uncertainty | Explicit, not hidden |
| `phiFindings` or de-id status | Detection or de-identification workflow evidence |
| `reviewHooks` | Structure for human correction |
| `modelId` + `runtimeVersion` | Provenance |
| `runId` | Links to Fanatir Run |

## Forbidden

- Autonomous clinical decision-making presented as approved care
- Silent approval
- Fabricated mappings without source spans
- Unrestricted cloud-model PHI access
- Treating OpenMed PyPI use as governed fork/import authorization
