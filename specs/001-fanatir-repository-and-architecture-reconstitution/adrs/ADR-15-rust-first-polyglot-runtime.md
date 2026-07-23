# ADR-15 — Rust-First Polyglot Runtime and Language Authority

**Status**: Founder-ratified (planning authority) — must be accepted before **R2 implementation**  
**Date**: 2026-07-23  
**Feature**: `001-fanatir-repository-and-architecture-reconstitution`  
**Deciders**: Founder (Decision A)

## Context

Fanatir requires a local-first Trusted Host with durable Artifact/Run authority, capability enforcement, and supervised polyglot workers. Without explicit language authority, UI or workers may silently own storage, secrets, policy, or audit.

## Decision

Fanatir uses a **Rust-first, polyglot-at-the-edges** architecture.

### Rust (mandatory trusted systems language)

- Tauri desktop host
- project/workspace authority
- Artifact, Revision, and Run kernel
- encrypted local storage
- filesystem mediation
- secrets
- policy and capability enforcement
- audit
- secure IPC
- worker supervision
- plugin/MCP gateway
- secure export and sharing
- updater and signing boundaries

### React / TypeScript

Presentation and UI only. Must not own durable Artifact mutation, patient/project storage authority, unrestricted FS, secrets, policy, PHI egress, plugin permissions, or authoritative audit.

### Python

Required for bounded workers including DeepMed/OpenMed runtime, medical NLP, Hugging Face inference, Graphify-derived processing, and scientific processing.

### Lab

R and SQL remain governed Fanatir Lab runtimes (with Python). Alpha **delivery tiers** for Lab remain as Spec 001 (guided Python/SQL Required; R Preview).

### Go

Permitted but not required. May be introduced only for a bounded worker, connector, or network service when an ADR proves material advantage over Rust or Python. Must not own Trusted Host, Artifact/Run authority, patient storage, policy, secrets, audit authority, or unrestricted filesystem access. **No Go component required for Founder Alpha.**

### Non-Rust runtime rules

Must not own or bypass: durable Artifact mutation; patient/project storage authority; unrestricted filesystem access; secrets; policy decisions; PHI-egress decisions; plugin permissions; authoritative audit recording.

Must be: process-isolated; version-pinned; capability-constrained; supervised by Rust; accessed through bounded versioned IPC; restartable; auditable; replaceable.

### Explicit non-authorization

Does **not** authorize rewriting OpenMed, Graphify, Jupyter, R, or scientific ecosystems in Rust.

### Founder Alpha posture

Establish minimal Rust Trusted Host early; wrap/supervise required Python workers; migrate performance- or security-critical components to Rust only when evidence justifies it; preserve the 60-day integrated delivery objective.

## Constrains (mandatory)

| ADR | Constraint |
| --- | --- |
| ADR-01 | Host Core composition language is Rust |
| ADR-02 | Trusted Host implementation is Rust-only; responsibility set includes encrypted storage, plugin/MCP gateway, export/share, updater/signing |
| ADR-04 | Language-neutral schemas; durable mutation enforcement in Rust |
| ADR-05 | Artifact/Revision/Run kernel is Rust; workers cannot mutate Artifacts directly |
| ADR-06 | All non-Rust runtimes supervised via versioned IPC under the rules above |
| ADR-08 | Fehrest integration is Rust-supervised process boundary |
| ADR-09 | DeepMed/OpenMed-derived Python workers allowed; no Rust rewrite mandate; no host authority |
| ADR-10 | commandF non-Rust ⇒ supervised worker only |
| ADR-13 | Packaging uses early Rust host + supervised Python; no Go required for Alpha |
| ADR-14 | PHI egress and plugin permissions decided in Rust gateway |

Also informs (not listed as formal constrain-set by founder): ADR-03 (UI-only TS), ADR-07 (adapter cannot become content SoT), ADR-11/12.

## Gate

**ADR-15 MUST be accepted before R2 implementation.**

## References

- [plan.md](../plan.md) blocking matrix
- [contracts/worker-runtime.md](../contracts/worker-runtime.md)
- [contracts/trusted-host-ipc.md](../contracts/trusted-host-ipc.md)
