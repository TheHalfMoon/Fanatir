# Contract: commandF ownership

**Version**: 0.1.0-draft  
**Owner**: Fanatir (Constitution / Spec 001)

## Responsibility

commandF is Fanatir’s validation/transform boundary for supported clinical/data formats (including FHIR-oriented gates where applicable).

## Process boundary

- Prefer host-supervised worker (may wrap existing `services/fhir_gate.py` as prototype evidence)
- Versioned I/O contract; prototype ≠ DeepMed
- Outputs become Artifacts/Revisions with Run provenance

## Alpha expectation

- At least one guided validate/transform step in the golden journey
- No claim of universal source conversion
- No live EHR writes
