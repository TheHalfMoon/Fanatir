# Contract: Non-Rust Worker Runtime Rules

**Version**: 0.2.0-draft  
**Authority**: ADR-15 — Rust-First Polyglot Runtime and Language Authority (Decision A)  
**Supervisor**: Fanatir Rust Trusted Host

## Permitted edge languages

| Runtime | Role | Alpha |
| --- | --- | --- |
| Python | DeepMed/OpenMed runtime, NLP, HF inference, Graphify-derived, scientific/data; Lab | Required workers as needed; R3 DeepMed may use OpenMed PyPI per Decision B / ADR-09 |
| R | Lab governed runtime | Architecture supported; Alpha evidence = Spec Preview |
| SQL | Lab governed runtime | Alpha Required (guided workflows) |
| Go | Optional bounded workers/connectors/network services only with justifying ADR | **Not required** for Founder Alpha |
| TypeScript/React | UI / presentation only | Existing shell |

## Hard prohibitions (non-Rust)

Workers and UI MUST NOT directly own or bypass:

- durable Artifact mutation
- patient or project storage authority
- unrestricted filesystem access
- secrets
- policy decisions
- PHI egress decisions
- plugin permissions
- authoritative audit recording

## Mandatory worker properties

Process-isolated · version-pinned · capability-constrained · Rust-supervised · bounded versioned IPC · restartable · auditable · replaceable

## Non-goals

- Rewriting OpenMed, Graphify, Jupyter, R, or scientific ecosystems in Rust
- Requiring Go for Alpha
- Elevating Node/Express to Trusted Host
- Treating OpenMed PyPI use as governed fork/import authorization
