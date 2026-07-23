# AFIA / Fanatir

> **Authority notice:** The founder-ratified Fanatir Constitution at
> `.specify/memory/constitution.md` is the governing authority. Legacy AFIA
> README claims below are historical evidence only and MUST NOT override the
> Constitution. Package name `afia` remains an unresolved migration detail.
> Desktop host statements below are not proof of current implementation.

> Skeleton / mixed-era repository. Reconstitution is required before treating
> legacy blueprint text as binding architecture.

Legacy description retained for audit evidence: AFIA was described as a macOS
Apple Silicon desktop application with Rust as an authoritative host layer, a
bounded Go operations service, and a managed Python AI runtime. Those claims are
targets or historical intent unless re-accepted through Spec Kit and ADRs under
the Fanatir Constitution.

## Monorepo layout

| Path | Ownership |
| --- | --- |
| `apps/desktop/ui` | React / TypeScript / Tailwind / shadcn/ui frontend |
| `apps/desktop/src-tauri` | Tauri v2 desktop shell (composition root, no domain rules) |
| `crates/` | Rust modular monolith (domain, application, adapters) |
| `services/operations-go` | Bounded Go operations service |
| `services/ai-python` | Managed Python AI runtime |
| `contracts/` | Cross-language contract source and generated output |
| `docs/` | Product, architecture, ADRs, security, quality, runbooks, releases |
| `tests/` | Fixtures, contract, integration, e2e, performance, security suites |
| `packaging/` | macOS packaging, model packs, Python bundle |
| `tools/` | Repository tooling (contract generation, etc.) |
| `.github/` | CI workflows |

## Repository rules (enforced)

- One source of cross-language contracts: schemas live under `contracts/source`; generated drift fails CI.
- No business logic in adapters (Tauri, gRPC, HTTP, SQLite, UI translate and delegate).
- No raw workspace access outside Rust.
- No unpinned runtime dependency.
- No content fixture without provenance.
- No release without archived evidence under `docs/releases`.
- No broad shared utility package.

## Status

<!-- TODO(S03-T01): Maintain the program control files and task ledger under docs/product. -->
Scaffolding in progress. See `docs/product/v1-engineering-program.md` and `docs/product/v1-task-ledger.md`.
