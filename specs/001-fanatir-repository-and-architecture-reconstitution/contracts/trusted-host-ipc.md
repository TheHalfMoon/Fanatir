# Contract: Trusted Host IPC

**Version**: 0.1.0-draft  
**Transport**: Tauri 2 Commands (request/response) + Events/Channels (progress)  
**Authority**: Fanatir Rust Trusted Host (Core process)

## Principles

- **ADR-15**: Rust Trusted Host is the sole durable authority for project/workspace, Artifact/Revision/Run, encrypted local storage, FS mediation, secrets, policy/capabilities, audit, worker supervision, plugin/MCP gateway, secure export/share, and update/signing boundaries.
- WebView / React/TypeScript MUST NOT perform privileged filesystem, secret, policy, PHI-egress, plugin-permission, or authoritative audit operations directly (UI/presentation only).
- All privileged invokes are subject to Capability Gateway / PolicyDecision **in Rust**.
- Payloads are JSON-serializable; large binaries use Artifact URIs, not IPC bodies.
- Non-Rust workers access host capabilities only through bounded versioned IPC; they MUST NOT mutate Artifacts directly.
- This document remains a **planning contract**, not a production-approved external API. ADR-06 IPC implementation is **not** complete.

## T033 Host FS semantic operations (internal Rust-only)

T033 defines the semantic Host operations `project.open` and `project.create` as **internal Rust methods** on an immutable `ProjectFilesystemAuthority`. In T033 they are:

- **not** WebView-callable Tauri commands;
- **not** registered invoke handlers;
- **not** a stable or versioned IPC wire schema;
- **not** capability-granted to the WebView.

Root model for T033:

- one immutable permitted local filesystem root per `ProjectFilesystemAuthority` instance;
- project paths are validated **relative** requests under that root;
- `project.create` creates **only** an empty project-root directory (no scaffold files);
- Workspace root and Project–Workspace relationships remain **undefined**;
- containment is bounded lexical + canonical + Windows reparse-point denial for this single-root open/create model;
- **TOCTOU limitation:** checks are not race-free; OS-handle-level hardening is required before production security claims. This is **not** a production-grade sandbox or compliance certification claim.

Deferred beyond T033 (still unauthorized here):

- `fs.read_text` / `fs.write_text`;
- delete, rename, move, copy, listing, watch, and unrestricted filesystem operations;
- Artifact Store filesystem operations;
- IPC transport, request/response envelopes, versioning, invoke registration, and capability exposure (**owned by T034**).

## Commands (minimum)

| Command | Purpose | Notes |
| --- | --- | --- |
| `project.open` | Open existing project root | T033: internal Rust semantic op; Host validates path scope. T034: transport/capability. |
| `project.create` | Create empty project-root directory | T033: internal Rust; creates one directory only (no scaffold). T034: transport/capability. |
| `fs.read_text` / `fs.write_text` | Mediated file IO within project | Deferred; not implemented by T033. Deny escape when later authorized. |
| `secrets.get` / `secrets.set` | Local secret storage | OS-backed |
| `worker.spawn` | Start sidecar (Python/R/SQL Lab, Fehrest, DeepMed, optional Go) | Rust-supervised; see `worker-runtime.md` |
| `worker.status` / `worker.stop` | Lifecycle | Crash → failed Run; restartable |
| `artifact.put` / `artifact.get` | Local Artifact/Revision store | **Rust-only mutation path**; provenance required |
| `run.start` / `run.complete` | Run lifecycle | Links Capability; Rust kernel |
| `audit.append` | Append audit event | **Authoritative audit in Rust only** |
| `gateway.authorize` | Pre-flight PolicyDecision | Required for models/tools/plugins/MCP |
| `export.secure` | Secure export / share boundary | Rust-mediated; ExportManifest |
| `update.*` | Desktop update / signing boundary | Rust-owned; Tauri updater |

## Events

| Event | Purpose |
| --- | --- |
| `worker.progress` | Ordered progress (prefer Channel) |
| `worker.exited` | Exit code / signal |
| `audit.emitted` | UI observability |
| `run.updated` | Status changes |

## Security

- Least-privilege Tauri capabilities ACL.
- Reject unknown commands; deny by default.
- Classification checks before cloud adapter calls.
- T033 does not expand WebView-callable host commands or capability grants.
