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

## Commands (minimum)

| Command | Purpose | Notes |
| --- | --- | --- |
| `project.open` | Open existing project root | Host validates path scope |
| `project.create` | Create project scaffold | Local-first |
| `fs.read_text` / `fs.write_text` | Mediated file IO within project | Deny escape |
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
