# Research: 001 Fanatir Repository and Architecture Reconstitution

**Date**: 2026-07-23  
**Spec**: [spec.md](./spec.md)  
**Status**: Planning research only — no production mutation authorized

## Method

- Governing order: Constitution → accepted Spec 001 → checklists → program memory → ADRs (roadmap) → Graphify (non-authoritative) → repository inspection
- External sources: official Tauri 2 docs (process model, IPC, updater/signing)
- Graphify MCP was not required for this plan; prior Graphify CLI inventory remains non-authoritative evidence

---

## R1 — Current repository baseline

**Decision**: Treat the following as the verified reconstitution baseline.

| Path / asset | Role | Notes |
| --- | --- | --- |
| `afia-ui/` | Verified-active product UI (Vite/React client + Express server + shared) | Package name still `afia`; migration shell per Q1 |
| `afia-ui/client/` | React SPA entry (`App.tsx`, `PrivateRoute`, AuthContext, Supabase client) | Preserve auth/session/PrivateRoute/profile until dedicated migration spec |
| `afia-ui/server/` | Node/Express companion | Investigate/adapt; not Trusted Host |
| `lib/` | Clinical/shared TypeScript helpers | Adapt under contracts; not architecture authority |
| `services/openmed_bridge.py`, `services/fhir_gate.py` | Prototype Python bridges | Process-isolated workers; not DeepMed product; no OpenMed import |
| `afia-ui/supabase/` | Active Supabase app config + migrations (1 file observed) | Provisional adapter evidence |
| `supabase/migrations/` | Duplicate migration tree (1 file observed) | **Freeze**; canonicalize via ADR-P1 / dedicated migration spec |
| `_archived/` | Historical monorepo/desktop/crates/Go evidence | Archive disposition; not active authority |
| Root `Cargo.toml`, `go.work`, `pnpm-workspace.yaml` | Stale/contradictory manifests | Investigate/adapt for truthfulness later; do not “revive” missing paths |
| Fehrest remote | Empty clone | Requires Fehrest init specs |
| DeepMed-AI remote | README-only tip | Requires DeepMed product specs; OpenMed import separate |

**Rationale**: Matches Spec 001 inventory and disposition matrix; prevents elevating archived scaffolds.

**Alternatives considered**: Treating archived Tauri crates as current host (rejected — not on active path); treating root Supabase as SoT without review (rejected — duplicate, Q4).

---

## R2 — Target repository and package topology

**Decision**: Three-repo ecosystem with Fanatir as integration authority.

```text
Fanatir (product + desktop host + Studio/CoLab/Lab/commandF + gateway)
  ├── apps/desktop/          # NEW (R2): Tauri 2 shell + Rust Trusted Host
  ├── afia-ui/               # EXISTING: strangler UI until rename/migration specs
  ├── packages/contracts/    # NEW: language-neutral shared primitives + IPC schemas
  ├── services/              # EXISTING Python prototypes → workers behind host
  └── docs/, specs/, .specify/

Fehrest (independent knowledge product; embedded via release/sidecar)
DeepMed-AI (independent medical intelligence runtime; contract-first)
```

Exact folder names are planning recommendations pending ADR-01/04; technical `afia*` identifiers remain until rename ADR.

**Rationale**: Constitution ownership + Spec target repository map + Q5/Q6 independence.

**Alternatives considered**: Single mega-monorepo including Fehrest/DeepMed (rejected — remotes already separate); immediate mass package rename (rejected — Q8).

---

## R3 — Migration path (AFIA → Fanatir)

**Decision**: Strangler / vertical-slice sequence **R0→R5** (see plan.md). Preserve baseline → ADRs → minimal Trusted Host → first integrated journey → adapters → Alpha packaging. No big-bang cutover.

**Rationale**: Founder Q7.

**Alternatives considered**: Rewrite UI first (rejected — Q1); revive full archived kernel first (rejected — Q3 minimal host; archived ≠ active).

---

## R4 — Minimum Tauri/Rust Trusted Host before major expansion

**Decision**: Early **minimal** Trusted Host after plan + required ADRs (R2), before major feature expansion. Responsibilities:

| Capability | Minimal host requirement |
| --- | --- |
| Process model | Tauri 2 Core (Rust) + WebView UI; Core mediates OS access ([Tauri process model](https://v2.tauri.app/concept/process-model/)) |
| IPC | Commands (request/response) for privileged ops; Events/Channels for progress ([IPC](https://v2.tauri.app/concept/inter-process-communication/)) |
| Filesystem mediation | Open/create project roots via host-scoped APIs only |
| Local secrets | OS keychain / host-owned secret store; never in WebView storage as SoT |
| Worker supervision | Spawn/supervise sidecars (Python/DeepMed/Fehrest) via host; crash restart policy |
| Capability enforcement | Capability Gateway checks before model/tool/sidecar invocation |
| Audit events | Host-emitted audit stream for privileged actions |
| Local Artifact/Run storage | Host-mediated local artifact store with provenance hooks |
| Secure IPC | Reject untrusted invoke; least-privilege Tauri capabilities |
| Failure recovery | Worker crash → user-visible failure + audit; no silent clinical success |
| Packaging | Windows-first installers; architecture not Windows-only |
| Updater | Tauri updater plugin requires signing keys; plan key custody before Alpha ([updater](https://v2.tauri.app/plugin/updater/)) |

Not required before first vertical slice: full historical kernel feature set, every worker type, production updater fleet.

**Rationale**: Founder Q3 + Tauri 2 official model.

**Alternatives considered**: Browser-only Alpha then host later (rejected — Q3); full archived crate reactivation (rejected — risk, unverified).

---

## R5 — `afia-ui` compatibility strategy

**Decision**: `afia-ui` is the **migration starting shell**, not architecture authority. Preserve `AuthContext`, session handling, `PrivateRoute`, and profile flows until a dedicated auth/session migration spec. Incremental adaptation after ADRs: route Studio/CoLab/Lab surfaces through host IPC adapters; do not invent `ProfileGate`.

**Rationale**: Founder Q1; verified `PrivateRoute` in `afia-ui/client/src/App.tsx`; no `ProfileGate` symbol.

**Alternatives considered**: Greenfield UI rewrite (rejected); treating `afia-ui` as constitutional authority (rejected).

---

## R6 — Auth / session / PrivateRoute / profile preservation boundary

**Decision**: Freeze behavior contracts during reconstitution. Allowed: observation, documentation, adapter seams that call existing APIs. Forbidden: changing OTP/session semantics, route guards, profile upsert rules, or inventing gates.

**Rationale**: Spec hard exclusions + Q1/Q4.

---

## R7 — Current Python services

**Decision**: Treat `services/*.py` as **prototype workers**, not DeepMed product and not OpenMed import. Isolate via host-supervised processes; define versioned stdin/stdout or localhost contracts; mark prototype vs reusable in inventory.

**Verified (2026-07-23):** `services/openmed_bridge.py` has a hard `import openmed` runtime dependency (`openmed` is Apache-2.0; https://pypi.org/project/openmed/). **Decision B:** bounded R3 DeepMed may temporarily use this OpenMed **PyPI** runtime (package use ≠ fork/import) under ADR-09 requirements (pin, local, isolation, NOTICE, model licenses, replacement path).

**Rationale**: Spec disposition + Q6 sequencing + Decision B.

**Alternatives considered**: Import into DeepMed-AI repo now (rejected — needs DeepMed specs); call Python directly from Vite without host (rejected — Q3); treat PyPI use as final DeepMed architecture (rejected — Decision B).

---

## R8 — Provisional Supabase role

**Decision**: Supabase is an **optional collaboration and identity adapter** (**Decision C** / Adapt). Not SoT for document content, extracted entities, patient material, FHIR containing PHI, Artifact revisions, Runs, or policy. Disposition = **Adapt** (collab/identity only). Freeze duplicate migrations; future **002** authorized.

**Current-baseline caveat (verified 2026-07-23):** `documents-crypto` stores field-encrypted document content/metadata plus an `audit_log` in Supabase (server-held `ENCRYPTION_KEY`). **Decision C:** this is a legacy PHI-egress seam — freeze expansion; synthetic/test only; preserve for inspection; do not delete/mutate; authoritative content/PHI/Artifact revisions move to the local Rust Artifact Store. ADR-07/14 implement the boundary; future **002** handles canonicalization/migration detail.

**Rationale**: Founder Q4 + Decision C.

**Alternatives considered**: Replace auth immediately (rejected); retain as local Artifact/content SoT (rejected — Decision C).

---

## R9 — Shared ecosystem contracts

**Decision**: Introduce language-neutral contract package owned by Fanatir (`packages/contracts` recommendation) with semver; Fehrest/DeepMed consume published versions. Prefer JSON Schema / OpenAPI-style schemas + generated bindings where practical; hand-maintained TypeScript/Rust types allowed initially if generation is deferred by ADR-04.

Minimum primitives: Workspace, Project, Patient, Artifact, Revision, Run, Source, Relationship, Review, Approval, Decision, PolicyDecision, DataClassification, Capability, ExportManifest — **interface boundaries only** in this plan (see data-model.md).

**Rationale**: Spec cross-repo contract requirements + P2 resolution below.

---

## R10 — Fehrest / DeepMed integration boundaries

**Fehrest**: Independent product + embedded capability. Alpha needs bounded vault/graph/memory features usable standalone and inside Fanatir. Integration: **process boundary first** (sidecar/service), optional later library embed. Markdown vault ownership stays in Fehrest; Fanatir projects reference Fehrest exports/notes. Graphify fork/import = separate Fehrest spec.

**DeepMed**: Required in first integrated release as contract-first runtime. Fanatir invokes via Capability Gateway; results must include source spans, confidence/uncertainty, review/correction hooks, model+Run provenance. OpenMed import = separate DeepMed spec only.

**Rationale**: Q5, Q6, Constitution.

---

## R11 — First end-to-end vertical slice

**Decision**: One golden-journey slice (not a single isolated feature):

Create/open project → import supported document → open in Studio → bounded DeepMed workflow → preserve result/notes in bounded Fehrest → validate/transform via commandF → one guided Lab analysis → preserve Run/provenance → CoLab bounded review/comment/approval → secure share of approved output.

CoLab initially: comments/review/approval — not advanced realtime multiplayer.

**Rationale**: Founder Q2 + user planning brief.

---

## P1 — Supabase migrations (resolved for planning; no filesystem change)

**Decision** (aligned with **Decision C**):

| Question | Answer |
| --- | --- |
| Content/PHI SoT | Local **Rust Artifact Store** — not Supabase |
| Canonical migration candidate | `afia-ui/supabase/` provisional for adapter app location |
| Duplicates | Parallel `supabase/migrations/` frozen |
| Consolidation now? | **No** — future spec **`002-supabase-local-first-and-migration-canonicalization`** (authorized; not created here) |
| `documents-crypto` | Freeze; synthetic/test only; preserve; no delete/mutate until ADR-07/14 |

**Alternatives considered**: Keep Supabase as document SoT (rejected — Decision C); delete `documents-crypto` now (rejected — preserve for inspection).
---

## P2 — Shared primitive ownership (resolved)

**Decision**: Fanatir owns canonical definitions; language-neutral schemas are mandatory for cross-repo primitives; serialization = JSON with schema `$id` + semver; prefer generated bindings when tooling exists, hand-maintained OK for Alpha with compatibility tests; evolution via additive changes + deprecation windows; breaking changes require ADR.

**Alternatives considered**: Each repo defines independently (rejected — contract drift); Rust-only types (rejected — Fehrest/TS/Python consumers).

---

## P3 — Fehrest and DeepMed release channels (resolved for planning)

**Decision**:

| Concern | Recommendation |
| --- | --- |
| Local dev | Path/workspace pin or `FANATIR_*_BIN` env to local builds |
| Alpha tagging | Semver tags `v0.x` on Fehrest and DeepMed-AI; Fanatir pins exact versions |
| Compatibility matrix | Documented in Fanatir release notes (host ↔ Fehrest ↔ DeepMed) |
| Upgrade/rollback | Pin + host capability check; rollback = prior pin |
| Integration shape | **Combination**: process/sidecar primary for Alpha; packages for shared contracts only |

**Alternatives considered**: Library-only embed for Alpha (rejected — process isolation for PHI/models); npm-only without process boundary (rejected for DeepMed).

---

## P4 — First vertical-slice packaging (resolved)

**Decision**: **Staged hybrid** (recommended path).

1. **R2**: Stand up minimal Tauri shell that loads the existing Vite/`afia-ui` client (dev: localhost URL; prod: bundled dist) and implements Trusted Host commands for project FS, secrets, worker spawn, audit.
2. **R3**: Vertical slice features run through host IPC where privileged; keep browser/dev mode as **compatibility fallback** for non-privileged UI work only, with explicit labeling that browser mode is not Alpha packaging evidence.
3. **R5**: Windows packaged build is Alpha acceptance vehicle.

**Evidence**: Q3 requires early host; Q1 preserves shell; strangler needs dual-run; updater signing is Alpha packaging concern not R2 blocker.

**Rollback**: Disable Tauri app entry; continue Vite-only; revert host feature branch.

**Alternatives considered**: Browser-only until late (rejected — Q3); immediate full desktop rewrite without Vite shell (rejected — risk to auth/session).

---

## R12 — Rust-first polyglot language authority (founder Decision A / ADR-15)

**Decision**: Fanatir is **Rust-first, polyglot-at-the-edges** per [ADR-15](./adrs/ADR-15-rust-first-polyglot-runtime.md).

| Layer | Language authority |
| --- | --- |
| Trusted systems (host, Artifact/Revision/Run kernel, encrypted local storage, FS, secrets, policy/capabilities, audit, secure IPC, worker supervision, plugin/MCP gateway, secure export/share, updater/signing) | **Rust mandatory** |
| UI / presentation | React + TypeScript only |
| Bounded workers (DeepMed/OpenMed runtime, medical NLP, HF inference, Graphify-derived, scientific) | **Python** required |
| Lab governed runtimes | Python, R, SQL |
| Optional workers/connectors/network services | **Go** only with justifying ADR; **not** required for Founder Alpha |

**Non-Rust workers** must be process-isolated, version-pinned, capability-constrained, Rust-supervised, versioned-IPC-accessed, restartable, auditable, replaceable — and must not own durable Artifact mutation, storage authority, unrestricted FS, secrets, policy, PHI egress, plugin permissions, or authoritative audit.

**Alpha posture**: early minimal Rust Trusted Host; wrap/supervise required Python workers; migrate to Rust only with evidence; preserve 60-day objective. Does **not** authorize rewriting OpenMed, Graphify, Jupyter, R, or scientific ecosystems in Rust.

**Gate**: ADR-15 constrains ADR-01/02/04/05/06/08/09/10/13/14 and **must be accepted before R2 implementation**.

**Lab Alpha tiers**: Spec 001 stands (Python/SQL Required; R Preview).

---

## R13 — DeepMed R3 substrate (founder Decision B)

**Decision**: Bounded R3 DeepMed worker may temporarily use existing OpenMed **PyPI** runtime dependency. This is package runtime use, **not** governed fork/import. Implementation details via **ADR-09**. Requirements: exact pin; local by default; Rust-supervised isolation; versioned request/response; bounded I/O; no unrestricted FS/secret/network/patient-store access; source spans; confidence/uncertainty; review/correction; model+Run provenance; Apache-2.0 NOTICE; per-model license manifest; explicit replacement path; must not claim prototype is final DeepMed architecture.

---

## R14 — Supabase document disposition (founder Decision C)

**Decision**: Authoritative storage for document content, extracted entities, patient material, FHIR containing PHI, and Artifact revisions → **local Rust-controlled Artifact Store**. Supabase = optional collaboration/identity adapter. `documents-crypto` = legacy PHI-egress seam: until ADR-07/14 accepted — freeze expansion; no new real patient data; synthetic/test only; preserve for inspection; do not delete/mutate. Later Supabase may hold only explicitly authorized classified collab metadata, references, reviews, invitations, sync records. Future spec **`002-supabase-local-first-and-migration-canonicalization`** authorized (not created in this task).

---

## R15 — Signing custody (founder Decision D)

**Decision**: Founder is signing authority owner. Before **R5 distribution**: named operational release custodian; hardware-backed key or approved secure signing service; separation of application and updater signing where applicable; documented rotation, recovery, revocation, emergency release; no private keys in repository or ordinary CI variables. Early unsigned builds = internal/non-distributable only.

---

## Desktop architecture research notes (Tauri 2)

**Primary-source verification (retrieved 2026-07-23; Tauri v2, updater plugin requires Rust ≥ 1.77.2):** Process model, IPC, updater, and sidecar claims below verified against official docs (`v2.tauri.app` concept/process-model, concept/inter-process-communication, plugin/updater, develop/sidecar). Updater signature is **mandatory and cannot be disabled**; Windows `installMode` = `passive`|`basicUi`|`quiet`; sidecars via `bundle.externalBin` + `shell().sidecar()` with per-target-triple binaries; Windows WebView = Microsoft Edge WebView2 (runtime dependency to bundle/verify).

- Core process is sole full-OS-access component; WebViews are untrusted UI.
- Commands: typed invoke; async to avoid blocking Core.
- Events: fire-and-forget; Channels for ordered high-throughput streams.
- Sidecars: `bundle.externalBin` + shell plugin supervision patterns.
- Updater: signatures mandatory; private key custody is release-critical; Windows install modes passive/basicUi/quiet.

## Explicit non-research (excluded)

Pictorial, Montada, public plugin marketplace, live EHR writes, hospital certification, autonomous clinical decision-making, universal source conversion, unrestricted cloud-model PHI access.
