# ADR-06 — Worker and IPC Contracts

| Field | Value |
| --- | --- |
| **ADR** | ADR-06 |
| **Title** | Worker and IPC Contracts |
| **Status** | **Accepted** (T030 R1 architecture gate) |
| **Task origin** | T017 |
| **Acceptance / review posture** | Tier A independent review after drafting; **T030** mandatory Accepted set includes ADR-06 (with ADR-15/01/02). T017 itself grants **no** architecture acceptance |
| **Feature** | `001-fanatir-repository-and-architecture-reconstitution` |
| **Constraining ADRs** | [ADR-15](./ADR-15-rust-first-polyglot-runtime.md) (Accepted at T030); [ADR-02](./ADR-02-rust-trusted-host-boundary.md) (Accepted at T030) |
| **Related ADRs** | [ADR-01](./ADR-01-platform-and-desktop-composition-tauri-2.md) (Accepted at T030); [ADR-03](./ADR-03-afia-ui-strangler-migration.md) (Proposed); [ADR-04](./ADR-04-shared-primitive-ownership-and-versioning.md) (Reviewed at T030) |
| **Planning evidence** | [trusted-host-ipc.md](../contracts/trusted-host-ipc.md); [worker-runtime.md](../contracts/worker-runtime.md); [plan.md](../plan.md) ADR-06 row; [research.md](../research.md) |
| **T030 founder decision** | **Accepted** — R1 architecture gate (`Yes — R1 architecture gate`); Tier A review Pending. Accepted for R1/R2 governance; does **not** auto-authorize T031 or plugin/capability/runtime work. |
| **Architecture authority of this file** | **YES** — architecture governance only (T030); does **not** authorize implementation |
| **Implementation authorization** | **NO** |

```text
Status: Accepted (T030 R1 architecture gate)
Architecture authority: YES (governance only)
Implementation authorization: NO
T031 / IPC publication / worker runtime / plugin work: NOT authorized by this status alone
Independent Tier A — R1 architecture gate: Pending
```

```text
Draft ADR ≠ architecture acceptance
T017 completion ≠ T030 acceptance (T030 founder decision now recorded)
Passing Tier A review of a draft ≠ Accepted (founder R1 gate recorded separately)
Accepted ADR-06 ≠ T031 or plugin/capability/runtime authorization
ADR-03 remains Proposed; ADR-04 is Reviewed (not Accepted for implementation)
```

```text
This Accepted ADR is not:
- production IPC publication
- worker implementation
- Trusted Host implementation
- permission to execute processes
- permission to access filesystems, networks, secrets, or PHI
- permission to alter UI authority
- permission to begin R2
```

```text
No production implementation is authorized by this draft.
```

This draft **must not** be used as justification to: implement Tauri commands/events/channels; spawn workers/sidecars; mediate filesystem/process/network/secrets; publish final IPC schemas; modify `afia-ui` production routes; wire OpenMed/Fehrest/DeepMed runtimes; implement Artifact Store; begin R2 (`T031+` / `T034+`); or treat Proposed ADRs as Accepted.

---

## 1. Context and problem

### 1.1 Assigned architectural question

**ADR-06 proposes** governed **Worker and IPC Contracts**: typed, versioned, bounded, auditable planning contracts between:

- the untrusted / non-privileged UI/presentation layer (`afia-ui` / WebView);
- the proposed Rust Trusted Host;
- supervised, process-isolated workers and sidecars (Python, Lab R/SQL, Fehrest, DeepMed, optional Go — per planning evidence).

It does **not** implement IPC, workers, or the Trusted Host; finalize production schemas; define Artifact Store persistence (**ADR-05**); define Fehrest integration (**ADR-08**); define DeepMed product operations (**ADR-09**); redefine shared primitives (**ADR-04**); or change authentication/sessions (**ADR-11**).

### 1.2 Why typed, versioned, bounded contracts are needed

Fanatir’s reconstitution direction is polyglot: React/`afia-ui` presentation; Rust Trusted Host for privileged authorities; supervised non-Rust workers at the edges ([ADR-15](./ADR-15-rust-first-polyglot-runtime.md) Proposed; [plan.md](../plan.md)). Planning contracts already sketch Tauri Commands/Events/Channels and worker properties ([trusted-host-ipc.md](../contracts/trusted-host-ipc.md); [worker-runtime.md](../contracts/worker-runtime.md)).

Without explicit Worker and IPC Contracts, planning evidence identifies these **risks** (prospective — not claimed as currently measured production failures unless separately evidenced):

| Risk | Why it matters |
| --- | --- |
| Untyped / loosely typed IPC | Ambiguous privilege and silent shape drift |
| Ambiguous command ownership | UI or workers invent privileged operations |
| Unbounded payloads | Memory/DoS; accidental bulk PHI/binary transfer |
| Implicit privilege escalation | Request intent mistaken for host authority |
| Protocol-version drift | Host/UI/worker disagree silently |
| Uncontrolled process invocation | Shell strings / ambient spawn |
| Secret or PHI leakage | Logs, errors, IPC bodies, worker env |
| Silent timeout / cancellation failures | Durable work continues or aborts without audit |
| Worker crashes affecting host | Lack of isolation / supervision contracts |
| Implementation messages as architecture | Ad-hoc TS/Rust/Python types become SoT |

R0 evidence records live UI→localhost OpenMed bridge calls without Trusted Host mediation (planning/ADR-02/15 context). That is a **transitional inspected fact**, not authorization for direct UI–worker architecture.

### 1.3 Accepted program and planning constraints

| Constraint | Source | Treatment |
| --- | --- | --- |
| Non-Rust workers: process-isolated, version-pinned, capability-constrained, Rust-supervised, versioned-IPC, restartable, auditable, replaceable | Decision A / ADR-15; [worker-runtime.md](../contracts/worker-runtime.md); [plan.md](../plan.md) ADR-06 | Planning direction; ADR-15 still Proposed |
| UI/presentation only; no direct privileged FS/secret/policy/PHI-egress/plugin/audit | ADR-15; trusted-host-ipc.md | Binding planning posture |
| Trusted Host owns privileged enforcement categories | Spec Q3; [ADR-02](./ADR-02-rust-trusted-host-boundary.md) Proposed | Host TCB ≠ this ADR’s schemas |
| Tauri 2 Commands + Events/Channels as planning transport baseline | ADR-01 Proposed; trusted-host-ipc.md | Composition Proposed; not Accepted |
| ADR-02 before ADR-06 sequencing; T017 deps T012+T014 | [tasks.md](../tasks.md) | Binding for this task |
| T017 draft only; Tier A; no production code | tasks.md T017 | Binding |
| No R2 impl before T030 (ADR-15/01/02/06 Accepted) | tasks.md Phase R1/R2 | Future gate |
| Large binaries via Artifact URIs, not IPC bodies | trusted-host-ipc.md | Planning rule; store → ADR-05 |

### 1.4 Naming

| Name | Role in this ADR |
| --- | --- |
| **Fanatir** | Product/repository owning host-mediated IPC/worker contracts |
| **Fehrest** | Supervised sidecar / consumer where applicable — **not** IPC authority |
| **`afia-ui`** | Migration shell / UI requestor — **not** privileged IPC peer |

Canonical ADR title: **Worker and IPC Contracts** (not “Typed IPC and Worker Protocol”).

### 1.5 What this draft is not

Not production IPC publication; not worker/Trusted Host implementation; not process/filesystem/network/secret/PHI access permission; not UI-authority change; not R2 authorization.

---

## 2. Decision drivers

| Driver | Source |
| --- | --- |
| Fail-closed privileged mediation | ADR-02 Proposed; trusted-host-ipc Security |
| Worker mandatory properties | worker-runtime.md; plan ADR-06 row |
| Separate protocol versions from shared-primitive schemaVersions | ADR-04 Proposed |
| Deny unknown commands; JSON-serializable payloads | trusted-host-ipc.md |
| No Rust rewrite of OpenMed/Graphify/Jupyter/R | ADR-15; worker-runtime non-goals |
| Draft ≠ accepted; Tier A for IPC/worker ADR | tasks.md T017 |

---

## 3. Proposed decision

### 3.1 Decision statement

**ADR-06 proposes** that Fanatir adopt **typed, versioned, bounded, host-mediated Worker and IPC Contracts** such that:

1. All privileged UI↔host and host↔worker interactions use **explicit typed operations** with **protocol/schema versions**, **fail-closed validation**, and **audit correlation**.
2. The **Rust Trusted Host** is the sole privileged mediator and enforcement point for those operations ([ADR-02](./ADR-02-rust-trusted-host-boundary.md) Proposed).
3. **Workers** are isolated, supervised, replaceable executors — not independent authorization authorities ([worker-runtime.md](../contracts/worker-runtime.md)).
4. **Shared domain primitives** remain owned under [ADR-04](./ADR-04-shared-primitive-ownership-and-versioning.md) (Proposed); ADR-06 owns **envelopes, operations, events, error categories, and worker protocol rules**.
5. Planning transport baseline is **Tauri 2 Commands (request/response) + Events/Channels (progress)** ([trusted-host-ipc.md](../contracts/trusted-host-ipc.md); [ADR-01](./ADR-01-platform-and-desktop-composition-tauri-2.md) Proposed) — composition still Proposed.
6. Exact numerical limits, wire codes, retry/idempotency policy, and remote-worker posture remain **unresolved** where sources are silent (§11).

```text
Proposed contracts ≠ implemented IPC
Proposed worker model ≠ running supervisors
Planning command inventory ≠ production allowlist ratification
```

### 3.2 Authority model

| Actor | Proposed role | Must not |
| --- | --- | --- |
| UI / `afia-ui` / WebView | Requestor and presenter | Privileged FS; arbitrary process control; unrestricted network; secret retrieval as authority; self-authorization; bypass capability/policy; direct worker channels outside host mediation |
| Rust Trusted Host | Privileged mediator and enforcement point | Treat UI assertions as trusted; skip validation |
| Workers / sidecars | Isolated, replaceable executors | Independent authorization; durable Artifact mutation; unrestricted FS/secrets/policy/PHI egress/plugin/audit ownership |
| Shared primitives (ADR-04) | Canonical domain meanings / schemaVersions | Become IPC envelope SoT |
| IPC/worker contracts (ADR-06) | Operations, envelopes, protocol versions, worker rules | Own Artifact Store or Fehrest product behavior |

**Workers must not become independent authorization authorities.**

**UI assertions are untrusted.**

**Schema/protocol validity ≠ authorization validity.**

### 3.3 IPC contract model (architecture-level)

Typed planning contracts **would require** (not implement):

| Element | Purpose |
| --- | --- |
| Operation / command identity | Explicit allowlisted identifier |
| Request payload | Typed, bounded, JSON-serializable |
| Response payload | Typed success or typed failure |
| Event / progress payload | Advisory, bounded, typed |
| Protocol / schema version | Distinct from ADR-04 `schemaVersion` |
| Correlation / request identity | End-to-end audit and cancellation association |
| Error representation | Stable categories; safe diagnostics |
| Cancellation representation | Where applicable; host-mediated |
| Bounded metadata | Non-secret, non-PHI-by-default context |

**Domain separation:**

| Concern | Owner |
| --- | --- |
| Shared domain primitives | **ADR-04** (Proposed) |
| IPC envelopes / commands / events / worker protocol | **ADR-06** (this draft) |
| Persistence formats / Artifact Store | **ADR-05** (not authored) |
| Privileged enforcement / TCB | **ADR-02** (Proposed) |
| Policy/PHI egress detail | **ADR-14** (not authored) |

Do **not** finalize production schemas in this draft.

### 3.4 Contract categories (planning-level)

Adapted from [trusted-host-ipc.md](../contracts/trusted-host-ipc.md) and worker supervision needs:

| Category | Direction | Notes |
| --- | --- | --- |
| UI→host commands | Request | Privileged invokes; deny unknown |
| Host→UI responses | Response | Success or typed failure |
| Host→UI events | Event/Channel | Progress, run updates, audit observability |
| Host→worker requests | Supervised IPC | Versioned; capability-constrained |
| Worker→host responses | Response | Validated before host effects |
| Worker progress / status events | Event | Advisory; bounded |
| Cancellation / termination requests | Host-mediated | UI disconnect ≠ auto-cancel (§3.11) |
| Health / liveness reporting | Host↔worker | Exact model unresolved (§11) |
| Audit-relevant protocol events | Host-owned authority | Surfaces may notify UI; authoritative audit in Rust |

**Planning command inventory** (from trusted-host-ipc.md — **not** production-approved schemas):
`project.open/create`, `fs.read_text/write_text`, `secrets.get/set`, `worker.spawn/status/stop`, `artifact.put/get`, `run.start/complete`, `audit.append`, `gateway.authorize`, `export.secure`, `update.*`.
Events: `worker.progress`, `worker.exited`, `audit.emitted`, `run.updated`.

Exact allowlists, fields, and codes remain for later authorized schema tasks after acceptance gates.

### 3.5 Versioning (proposed)

1. **IPC/protocol versions** are **distinct** from ADR-04 shared-primitive `schemaVersion`s.
2. **Package versions** do not automatically define protocol compatibility.
3. **Application releases** do not silently redefine protocol compatibility.
4. **Unsupported incompatible versions fail closed**.
5. **Compatibility must be verified** under separately authorized tests — not assumed. This draft does **not** claim negotiation or tests exist.
6. If version negotiation is later allowed, it **would require** explicit, bounded, host-controlled rules (unresolved — §11).

### 3.6 Typed request/response envelopes (architecture-level sketch)

Envelopes **would require** at planning level:

- protocol version;
- operation identifier;
- correlation identifier;
- payload;
- bounded contextual metadata;
- success or typed failure result.

**Do not** embed secrets or raw authorization authority in generic envelopes.
Timestamps, user/tenant identity, and security-context fields remain **unresolved** unless later sources require them — not invented here as mandatory wire fields.
This is **not** an implementation-ready type definition.

### 3.7 Payload bounds (proposed)

Contracts **would require** explicit bounds for:

- message size;
- collection length;
- nesting depth;
- text length;
- event frequency;
- streaming chunk size (if streaming is later authorized);
- decode/validate time budgets where relevant.

**Exact numerical values remain unresolved** unless later canonical evidence defines them (§11).

Large binaries and durable content **should not** travel in ordinary IPC bodies; prefer Artifact URIs / store references ([trusted-host-ipc.md](../contracts/trusted-host-ipc.md)). Artifact Store design → **ADR-05**.

### 3.8 Validation and fail-closed behavior (proposed)

Validation **would be required** before any privileged, durable, or worker action:

| Condition | Proposed behavior |
| --- | --- |
| Unknown operation identifier | Reject / deny |
| Malformed envelope | Reject |
| Unsupported protocol version | Fail closed |
| Invalid shared primitives | Reject (per ADR-04 conformance when implemented) |
| Oversized payload | Reject |
| Invalid capability references | Reject; do not self-authorize from payload |
| Invalid classification metadata | Fail closed |
| Unexpected event types | Reject / ignore-with-audit (exact policy unresolved) |
| Worker protocol violation | Terminate/quarantine path + audit (details §11) |

Invalid input **must not** silently degrade into permissive behavior.
**Schema validity ≠ authorization validity.**

### 3.9 Capability and authorization enforcement (boundary)

- IPC **may** carry references or **data representations** of Capability / PolicyDecision ([ADR-04](./ADR-04-shared-primitive-ownership-and-versioning.md) Proposed).
- **Possession does not grant authority.**
- **Trusted Host** validates and enforces authorization ([ADR-02](./ADR-02-rust-trusted-host-boundary.md) Proposed).
- **Workers do not authorize** independently.
- **UI assertions are untrusted.**
- **Stale, forged, replayed, or merely structurally valid** authorization data **must not** automatically be trusted.

Full policy-engine design → ADR-02 / **ADR-14** (not authored) / later tasks — not this ADR.

### 3.10 Worker model (proposed)

Workers **are proposed** as:

- process-isolated where required by planning evidence;
- supervised by the Trusted Host;
- restartable and replaceable;
- capability-constrained and version-pinned;
- auditable;
- **without ambient host privileges by default**.

**Planning-level lifecycle states** (illustrative — not an implemented state machine):

`not_started` → `starting` → `ready` → `busy` → `cancelling` → `stopped` | `failed` | `quarantined` / `unavailable` (quarantine semantics unresolved — §11).

Crash marking failed Runs and restartability are planning expectations from trusted-host-ipc.md / worker-runtime.md — **not** claimed as implemented.

### 3.11 Worker invocation (architecture-level requirements)

Invocation **would require**:

- explicit operation allowlist;
- validated typed input;
- bounded resources;
- explicit working-directory policy;
- controlled environment;
- controlled secret injection (no ambient inheritance);
- controlled network policy;
- explicit timeout;
- cancellation and termination path;
- output validation before host durable effects;
- audit correlation.

**Do not** authorize arbitrary shell strings, ambient environment inheritance, or unrestricted network from this draft.

### 3.12 Isolation boundaries (contract requirements)

Contracts **would require** isolation expectations for:

| Domain | Contract requirement (proposed) | Enforcement owner |
| --- | --- | --- |
| Process execution | Isolated supervised processes | ADR-02 / R2 tasks |
| Filesystem | No unrestricted FS; host-mediated paths | ADR-02 |
| Network | No ambient unrestricted network | ADR-02 / ADR-14 |
| Environment variables | Controlled, non-ambient | ADR-06 contract + host |
| Secrets | Controlled injection; no IPC echo | ADR-02 |
| Temporary files | Bounded; cleanup expectations | Unresolved detail §11 |
| Artifact references | URI/refs; no worker-direct mutation | ADR-05 + ADR-02 |
| PHI-bearing data | Classification; min-necessary | ADR-14 + ADR-04 types |
| Worker crashes | Must not compromise host TCB | ADR-02 |
| Resource exhaustion | Host-enforced bounds | ADR-02 / ADR-06 limits |

ADR-06 defines **what the contract requires**; ADR-02 owns **privileged enforcement**.

### 3.13 Timeout and cancellation (proposed semantics)

Architecture-level semantics **would require**:

| Concern | Proposal |
| --- | --- |
| Operation timeout | Host-enforced; duration unresolved |
| Worker-start timeout | Host-enforced; duration unresolved |
| Cancellation request | Explicit, host-mediated |
| Cancellation acknowledgment | Should be defined before impl (§11) |
| Forced termination | Host capability after thresholds (§11) |
| Partial results | Not durable without host validation |
| Audit outcome | Timeout/cancel/force must be auditable |
| Cleanup | Best-effort expectations; details unresolved |

**UI navigation or disconnection does not automatically determine** whether a privileged or durable operation is cancelled — cancellation **would require** an explicit host-mediated decision path.

### 3.14 Progress, events, and streaming (proposed)

- Events **do not grant authority**.
- Progress is **advisory** and may be incomplete.
- Ordering / delivery guarantees must be **explicit** before implementation (§11).
- Event frequency must be **bounded**.
- Backpressure / overflow must **fail safely**.
- Streaming (if later authorized) **does not bypass** validation or payload limits.

This draft does **not** claim streaming is implemented. Planning preference for Channels for ordered progress comes from trusted-host-ipc.md.

### 3.15 Error taxonomy (architecture-level categories)

Proposed **categories** (wire codes unresolved):

| Category |
| --- |
| invalid_request |
| unsupported_version |
| unauthorized |
| forbidden_capability |
| payload_too_large |
| timeout |
| cancelled |
| worker_unavailable |
| worker_failed |
| protocol_violation |
| validation_failed |
| internal_host_failure |

Errors **must not** leak secrets, PHI, sensitive filesystem paths, environment variables, raw stack traces, or unsafe process details in ordinary UI-facing payloads.

### 3.16 Retries and idempotency — unsettled

Canonical evidence is **largely silent** on retries and idempotency.

```text
No settled retry or idempotency policy is claimed by this draft.
Automatic retries for privileged or durable actions are not implied.
```

Unresolved questions are recorded in §11 (U-ADR06-9…U-ADR06-14).

### 3.17 Auditability (IPC-correlated)

Audit-relevant outcomes **would include** (authoritative recording remains Trusted Host / ADR-02):

- request accepted or rejected;
- authorization outcome;
- worker start or failure;
- timeout;
- cancellation;
- forced termination;
- protocol violation;
- unsupported version;
- sensitive egress decision where applicable (policy detail → ADR-14).

ADR-06 may define **correlation requirements** and IPC-visible notification shapes; it does **not** define the complete audit store architecture.

### 3.18 PHI and secret handling (claim language)

Contracts **would require**:

- minimum-necessary payloads;
- data classification awareness;
- redacted diagnostics;
- no secrets in ordinary responses/logs;
- controlled secret injection to workers;
- bounded secret lifetime;
- no ambient inheritance;
- explicit egress policy checks before cloud/adapter calls (trusted-host-ipc.md);
- safe cleanup of temporary sensitive data;
- fail-closed for unknown classification.

Full egress / classification enforcement → **ADR-14** (not authored; not Accepted).

### 3.19 Local vs remote boundaries

**Local / process-isolated** communication is the **current planning baseline** (Tauri host + supervised sidecars; research/plan Alpha posture).

Remote workers / network-first worker meshes are **not** accepted architecture in this draft. If needed later, record as future scope requiring separate authorization (§11).

### 3.20 Python, Jupyter, R, and related workers

Preserve Rust-first boundary ([ADR-15](./ADR-15-rust-first-polyglot-runtime.md) Proposed; [worker-runtime.md](../contracts/worker-runtime.md)):

- Rust Trusted Host owns privileged orchestration and durable mutation;
- Python / Lab R / SQL workloads remain isolated workers where applicable;
- **no** Rust rewrite of OpenMed, Graphify, Jupyter, or R;
- worker IPC contracts **do not** make workers trusted authorities;
- Go optional only with justifying ADR; **not** required for Founder Alpha;
- DeepMed-specific operations → **ADR-09** (not authored).

Fehrest appears as a supervised sidecar candidate in planning command notes — product integration → **ADR-08**.

### 3.21 Proposed invariants (architecture authority only if later Accepted)

1. Privileged UI and worker effects occur only through host-mediated typed IPC.
2. Unknown operations and unsupported versions fail closed.
3. Workers cannot self-authorize or own durable Artifact mutation.
4. Protocol versions ≠ shared-primitive schemaVersions ≠ app/package versions.
5. Events and progress never confer privilege.
6. Possession of Capability/PolicyDecision data never confers privilege.

Until Accepted, these are **proposed planning invariants** only.

---

## 4. Cross-ADR boundaries

| Concern | Owner | ADR-06 may… | ADR-06 must not… |
| --- | --- | --- | --- |
| Rust-first / durable mutation | ADR-15 (Proposed) | Align worker/IPC under host | Redefine language authority |
| Platform / Tauri composition | ADR-01 (Proposed) | Use Commands/Events/Channels as planning transport | Accept Tauri alone |
| Trusted Host TCB / enforcement | ADR-02 (Proposed) | Define contract requirements | Own TCB composition |
| `afia-ui` strangler | ADR-03 (Proposed) | Treat UI as untrusted requestor | Define migration stages |
| Shared primitives | ADR-04 (Proposed) | Embed typed domain payloads by reference | Own schemaVersions |
| Artifact Store / persistence | ADR-05 (not authored) | Use Artifact URIs in IPC | Define store/DDL |
| Fehrest integration | ADR-08 (not authored) | Generic sidecar IPC rules | Fehrest release model |
| DeepMed operations | ADR-09 (not authored) | Generic worker protocol | Product pipeline design |
| Auth / sessions | ADR-11 (not authored) | Remain silent on session mechanics | Redesign auth |
| PHI egress / classification detail | ADR-14 (not authored) | Classification fail-closed in IPC | Finalize egress engine |

ADR-06 **proposes Worker and IPC Contracts only**.

---

## 5. Alternatives considered

### Option A — Direct UI access to workers

| | |
| --- | --- |
| Benefits | Apparent speed |
| Costs / risks | Bypasses host mediation; privilege and PHI risk; matches transitional OpenMed localhost anti-pattern |
| Draft disposition | **Rejected** |

### Option B — Direct UI access to privileged OS APIs

| | |
| --- | --- |
| Benefits | Fewer hops |
| Costs / risks | Violates UI-presentation-only and Trusted Host direction |
| Draft disposition | **Rejected** |

### Option C — Untyped JSON messages

| | |
| --- | --- |
| Benefits | Flexibility |
| Costs / risks | Drift; weak fail-closed validation |
| Draft disposition | **Rejected** as architecture end-state |

### Option D — Implementation-language types as protocol authority

| | |
| --- | --- |
| Benefits | Fast for one language |
| Costs / risks | Accidental architecture; polyglot workers excluded |
| Draft disposition | **Rejected** as SoT (bindings may consume contracts later) |

### Option E — One monolithic worker process

| | |
| --- | --- |
| Benefits | Simpler ops |
| Costs / risks | Blast radius; conflicts with replaceable/isolated workers |
| Draft disposition | **Rejected** as default architecture |

### Option F — In-process Python/Jupyter/R inside the host

| | |
| --- | --- |
| Benefits | Lower latency |
| Costs / risks | Breaks process isolation / crash containment |
| Draft disposition | **Rejected** as default |

### Option G — Unrestricted shell-command IPC

| | |
| --- | --- |
| Benefits | Escape hatch |
| Costs / risks | Arbitrary execution; unauditable privilege |
| Draft disposition | **Rejected** |

### Option H — Network-first remote worker architecture

| | |
| --- | --- |
| Benefits | Scale narrative |
| Costs / risks | Not Alpha planning baseline; expands trust/network surface |
| Draft disposition | **Rejected** as Accepted architecture; may be future unresolved scope |

### Option I — Typed, versioned, bounded, host-mediated worker and IPC contracts

| | |
| --- | --- |
| Benefits | Explicit authority; replaceable workers; auditable; bounded |
| Costs / risks | Governance and lifecycle complexity |
| Compatibility | Aligns with plan ADR-06, trusted-host-ipc.md, worker-runtime.md, ADR-15/02 |
| Draft disposition | **Proposed** |

---

## 6. Consequences

### Positive (prospective — if later accepted and executed under separate authorization)

- Explicit authority boundary between UI, host, and workers
- Safer privilege mediation
- Replaceable, supervised workers
- Versioned compatibility surface
- Auditable requests and denials
- Bounded payloads
- Clearer failure categories
- Reduced UI↔worker coupling

### Costs and risks

| Cost / risk | Note |
| --- | --- |
| Schema/protocol governance | Tier A on breaks; matrices |
| Worker lifecycle complexity | States, restart, quarantine |
| Compatibility matrices | Host ↔ UI ↔ workers ↔ Fehrest/DeepMed pins |
| Cancellation ambiguity | Must be explicit before impl |
| Event backpressure | Overflow fail-safe design |
| Resource controls | Limits unresolved numerically |
| Audit volume | Correlation discipline needed |
| Adapter maintenance | UI and worker adapters |
| Multi-version coexistence | Temporary protocol versions |
| Oversized host risk | Keep ADR-02 boundary tight |
| Over-generic / over-specific contracts | Balance via allowlists + UQs |

### Rollback (planning only)

If rejected before acceptance: revert this ADR file; retain planning contracts as non-authoritative evidence; do not implement IPC/workers from this draft. Post-acceptance withdrawal would need separately authorized migration — **not** defined here.

---

## 7. Non-goals

ADR-06 does **not**:

- implement IPC, workers, or the Trusted Host;
- publish final production schemas;
- define shared primitive ownership (ADR-04);
- define Artifact Store persistence (ADR-05);
- define Fehrest integration (ADR-08);
- redesign authentication or sessions (ADR-11);
- define complete PHI egress policy (ADR-14);
- implement remote workers;
- authorize arbitrary process execution;
- authorize direct UI privilege;
- settle retries/idempotency;
- begin R2 / T031+ / T034+.

---

## 8. Relationship to other ADRs

| ADR | Status in repo | Relationship |
| --- | --- | --- |
| ADR-15 | Proposed | Language + worker property constraints |
| ADR-01 | Proposed | Tauri transport planning baseline |
| ADR-02 | Proposed | Host enforcement; defers IPC schemas here |
| ADR-03 | Proposed | UI remains non-privileged shell |
| ADR-04 | Proposed | Shared primitives ≠ protocol versions |
| ADR-05 | Not authored | Artifact URIs / store |
| ADR-08 | Not authored | Fehrest sidecar integration |
| ADR-09 | Not authored | DeepMed domain ops |
| ADR-11 | Not authored | Auth/session |
| ADR-14 | Not authored | PHI/classification enforcement detail |

---

## 9. Gates and authority

```text
T017 produces a Proposed ADR-06 draft only.
Tier A independent review is required (IPC/worker ADR).
Passing Tier A review is not architecture acceptance.
Operational founder acceptance applies only to the draft work product unless a stage gate records Accepted.
ADR-06 is in the mandatory T030 Accepted set (with ADR-15/01/02).
T030 remains incomplete.
T031+ and R2 remain separately gated.
T031 shell notes ADR-06 is NOT required for shell-only work — IPC foundation (T034+) requires ADR-06 Accepted.
IPC, worker, and Trusted Host implementation requires separate authorization after gates.
```

| Gate | Meaning |
| --- | --- |
| T017 | Draft authoring (this task) |
| Tier A | Mandatory independent review |
| Founder draft acceptance | Work product only, if issued |
| T030 | Architecture acceptance of ADR-06 with 15/01/02 |
| T034 / T035 | Implementation after T030 + ADR-06 Accepted |
| T018 | Next checklist ADR draft (ADR-05) — not authorized here |

---

## 10. Validation and acceptance plan

```text
T017 completion produces a draft for Tier A review.
Architecture acceptance is not performed by T017.
```

Planning reviews **may** include: consistency with trusted-host-ipc.md / worker-runtime.md / plan ADR-06; authority-model review; fail-closed and Capability/PolicyDecision non-authority language; cross-ADR deferrals; Tier A independent review.

**Not** validation for T017: running IPC tests, spawning workers, or R2 builds.

---

## 11. Unresolved questions

T017 does **not** resolve these. Attempted resolution: **NO**.

| ID | Question | Open in draft review? | Before implementation? | Elsewhere? |
| --- | --- | --- | --- | --- |
| U-ADR06-1 | Exact protocol-version format | YES | YES | Schema authoring after gates |
| U-ADR06-2 | Final envelope fields | YES | YES | Schema authoring |
| U-ADR06-3 | Numeric payload/collection/nesting/text limits | YES | YES | Host/IPC impl tasks |
| U-ADR06-4 | Event ordering and delivery guarantees | YES | YES | Transport detail / ADR-01 ops |
| U-ADR06-5 | Progress frequency and backpressure policy | YES | YES | Impl + tests |
| U-ADR06-6 | Timeout defaults | YES | YES | Host policy |
| U-ADR06-7 | Cancellation acknowledgment protocol | YES | YES | IPC schema |
| U-ADR06-8 | Forced-termination thresholds | YES | YES | Host supervisor |
| U-ADR06-9 | Which operations may be retried | YES | YES | **Unsetled — no policy claimed** |
| U-ADR06-10 | Idempotency-key format | YES | YES | Unsettled |
| U-ADR06-11 | Duplicate suppression rules | YES | YES | Unsettled |
| U-ADR06-12 | Retry ownership and limits | YES | YES | Unsettled |
| U-ADR06-13 | Behavior after timeout / ambiguous completion | YES | YES | Unsettled + audit |
| U-ADR06-14 | Durable mutation replay safety | YES | YES | ADR-05 + host |
| U-ADR06-15 | Worker health / liveness model | YES | YES | Supervisor |
| U-ADR06-16 | Restart policy details | YES | YES | Supervisor |
| U-ADR06-17 | Quarantine policy | YES | YES | Supervisor / security |
| U-ADR06-18 | Resource limit magnitudes | YES | YES | Host |
| U-ADR06-19 | Secret-injection mechanism | YES | YES | ADR-02 / secrets |
| U-ADR06-20 | Worker network policy detail | YES | YES | ADR-14 / host |
| U-ADR06-21 | Temporary-file handling | YES | YES | Host / ADR-05 |
| U-ADR06-22 | Audit-event schema ownership detail | YES | Partial | ADR-02 / later |
| U-ADR06-23 | Remote-worker future posture | YES | Before any remote | Future spec/ADR |
| U-ADR06-24 | DeepMed operation separation boundary | YES | Before DeepMed IPC | **ADR-09** |
| U-ADR06-25 | Concrete error-code taxonomy | YES | YES | Schema authoring |
| U-ADR06-26 | Compatibility-test ownership | YES | YES | T034 tests / later |

---

## 12. Prohibitions carried by this draft

While Proposed:

- no IPC/worker/Trusted Host implementation;
- no production schema publication;
- no arbitrary process execution;
- no direct UI privilege;
- no Fehrest/DeepMed runtime wiring from this draft;
- no auth/session changes;
- no OpenMed/Graphify fork/import; no Pictorial/Montada;
- no Go Alpha Trusted Host authority;
- no R2 / T031+ / T034+ work from this file alone.

---

## 13. Document control

| Item | Value |
| --- | --- |
| Created for | T017 — Author ADR-06 Worker and IPC Contracts (draft only) |
| Required path pattern | `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-06-*.md` |
| Plan roadmap entry | [plan.md](../plan.md) ADR-06 row (existing; not modified by T017) |
| Supersedes | Nothing |
| Superseded by | Nothing |

```text
End of ADR-06 Proposed draft.
```
