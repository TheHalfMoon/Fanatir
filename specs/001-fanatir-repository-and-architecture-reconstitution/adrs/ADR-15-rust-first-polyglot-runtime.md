# ADR-15 — Rust-First Polyglot Runtime and Language Authority

| Field | Value |
| --- | --- |
| **ADR** | ADR-15 |
| **Title** | Rust-First Polyglot Runtime and Language Authority |
| **Status** | **Proposed** (draft only) |
| **Task origin** | T012 |
| **Acceptance gate** | **T030** (with the R1 ADR package required by Plan/Tasks) |
| **Feature** | `001-fanatir-repository-and-architecture-reconstitution` |
| **Related founder decision** | Decision A (Rust-first, polyglot-at-the-edges) — **ratified direction** in accepted Plan 001 |
| **Architecture authority of this file** | **NO** — until accepted at T030 |
| **Implementation authorization** | **NO** |

```text
This document is a planning draft. It reflects the founder-ratified Rust-first
direction (Decision A) but does not become accepted architecture authority until T030.
```

```text
Founder-ratified direction ≠ accepted ADR document
Draft ADR ≠ accepted architecture authority
T012 completion ≠ T030 acceptance
```

```text
No production implementation is authorized by this draft.
```

This draft **must not** be used as justification to: modify production or archived code; introduce Rust or Tauri dependencies; change build tooling; delete existing implementations; create migrations; or commence repository restructuring.

---

## 1. Context (inspected facts + accepted program)

### 1.1 Accepted program need

Accepted Constitution v1.0.0, Specification 001, and Plan 001 require a local-first Trusted Host with durable Artifact/Revision/Run authority, capability enforcement, and supervised polyglot workers. Without explicit **language authority**, UI or workers may silently own storage, secrets, policy, PHI egress, or authoritative audit.

Plan 001 records **Decision A** as **Ratified**: Rust-first, polyglot-at-the-edges; ADR-15 is the ADR vehicle; no Rust rewrite of OpenMed/Graphify/Jupyter/R/scientific ecosystems.

### 1.2 R0 repository facts (evidence only)

R0 closed at commit `f37563d8f8499aa24e8ead68fa920167ebb8cde6` ([R0-evidence-package.md](../../../docs/program-memory/baseline/R0-evidence-package.md)). Relevant inspected facts:

| Fact | Evidence | Treatment |
| --- | --- | --- |
| Active UI surface is `afia-ui/` (Vite/React/TS) plus root `lib/` and Python bridges | T004; T007; T010; `_archived/ARCHIVED.md` | Implementation fact |
| Live UI calls localhost OpenMed bridge (`openmed-client` → `127.0.0.1:8765`) | T007; T010 C5 | Contradicts archived “UI never calls Python” doctrine; evidence for reconstitution |
| Root `Cargo.toml` / `go.work` / `pnpm-workspace.yaml` are **CONTRADICTORY** (declare missing live members) | T001; T008; T010 C1–C3 | Manifest repair not authorized by this ADR |
| `_archived/crates/**` and `_archived/apps-desktop/**` are **ARCHIVED** scaffolds (not production Trusted Host) | T008; T010 | Non-authoritative; not proof of reusability |
| Archived Go operations service disposition for Alpha: **archive** | T008 | No Go Alpha dependency |
| No substantive CI quality gate (echo-only macOS workflow) | T009 | Future honest CI; not language decision |
| Path classification: 81 families; Trusted Host / Artifact Store **absent** as implemented Rust authority | T010 | Target ≠ present |
| Fehrest empty; DeepMed-AI README-only | T001; T010 | Sibling products; not Fanatir subdirs |

R0 baseline evidence is authoritative **only as a record of inspected facts**. It does **not** independently establish product or architecture authority.

### 1.3 Legacy and archive materials

Legacy `docs/product/AFIA_*` plans and `_archived/**` may inform history. They are **non-authoritative**. Archived Rust/Tauri presence does **not** mean production-ready Trusted Host implementation (T008).

---

## 2. Decision drivers (accepted constraints)

| Driver | Source | Status |
| --- | --- | --- |
| Rust-first, polyglot-at-the-edges | Plan 001 Decision A (Ratified) | Binding **direction** |
| No Rust rewrite of OpenMed/Graphify/Jupyter/R/scientific ecosystems | Decision A; Plan/Tasks prohibitions | Binding constraint |
| Local-first Trusted Host; UI zero durable authority for reserved roles | Spec 001; Constitution; Plan R2 objective | Binding program goal |
| Preserve `afia-ui` auth/session/`PrivateRoute`/profile until dedicated migration spec | Spec/Plan | Binding during reconstitution |
| Evidence-driven reconstruction; avoid silent archive reactivation | Spec disposition; T008/T010 | Binding planning discipline |
| No Go component required for Founder Alpha | Decision A / research R12; T008 | Binding for Alpha |
| Security/trust-boundary clarity (secrets, policy, PHI egress, audit in trusted core) | Constitution; Spec; Plan ADR roadmap | Binding goal; detailed model later (ADR-14 et al.) |
| 60-day integrated delivery objective | Plan | Program constraint |

---

## 3. Proposed decision

**ADR-15 proposes** that Fanatir adopt a **Rust-first, polyglot-at-the-edges** language-authority model consistent with Decision A.

### 3.1 Proposed language roles

| Layer | Proposed authority | Notes |
| --- | --- | --- |
| Trusted systems | **Rust mandatory** | Host Core / Trusted Host responsibilities listed below |
| UI / presentation | React + TypeScript | Must not own reserved durable authorities |
| Bounded ML/science workers | **Python** required | DeepMed/OpenMed runtime, NLP, HF, Graphify-derived, scientific — under supervision rules |
| Lab governed runtimes | Python, R, SQL | Alpha **delivery tiers** remain Spec 001 (guided Python/SQL Required; R Preview) |
| Optional connectors/network services | **Go** only with a justifying ADR | **Not** required for Founder Alpha; must not own Trusted Host roles |

### 3.2 Proposed Rust-mandatory responsibility set

ADR-15 **proposes** that the following be owned only by the Rust Trusted Host (detailed boundaries deferred to ADR-02 and related ADRs):

- desktop Trusted Host Core (composition framework deferred to **ADR-01**)
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
- updater and signing **boundaries** (custody/process deferred to Decision D / ADR-13 packaging)

### 3.3 Proposed non-Rust prohibitions

Non-Rust UI and workers **must not** (under the proposed model) own or bypass:

- durable Artifact mutation
- patient/project storage authority
- unrestricted filesystem access
- secrets
- policy decisions
- PHI-egress decisions
- plugin permissions
- authoritative audit recording

### 3.4 Proposed non-Rust runtime rules

Non-Rust runtimes **would** be: process-isolated; version-pinned; capability-constrained; supervised by Rust; accessed through bounded versioned IPC; restartable; auditable; replaceable.

IPC schema, worker packaging, and concrete supervision APIs are **out of scope** for ADR-15 (ADR-06 and contracts).

### 3.5 Explicit non-authorization (binding from Decision A / Plan)

This direction **does not** authorize rewriting OpenMed, Graphify, Jupyter, R, or scientific ecosystems in Rust.

### 3.6 Founder Alpha posture (proposed sequencing)

Establish a **minimal** Rust Trusted Host early (R2, only after required ADR acceptances including ADR-15 at T030); wrap/supervise required Python workers; migrate performance- or security-critical components to Rust only when evidence justifies it; preserve the 60-day integrated delivery objective.

### 3.7 What this ADR does **not** settle

Deferred unless a later accepted ADR or task explicitly decides:

- Tauri (or other) desktop composition — **ADR-01**
- Exact Trusted Host responsibility checklist / R2 exit demo — **ADR-02**
- Shared primitive schemas — **ADR-04**
- Artifact Store design — **ADR-05**
- Worker/IPC contracts — **ADR-06**
- Supabase adapter / Decision C detail — **ADR-07** / Spec 002
- Fehrest / DeepMed / commandF integration — **ADR-08/09/10**
- Packaging and signing operations — **ADR-13** / Decision D
- PHI egress gateway detail — **ADR-14**
- Crate layout, dependency pins, CI, migrations, delete/archive plans

A Rust-first direction does **not** automatically approve Tauri, any crate layout, or any production cutover.

---

## 4. Scope

### In scope for this draft

- Language authority: which languages **may** own which classes of responsibility
- Constraints this ADR would place on later ADRs if accepted at T030
- Explicit Alpha Go non-requirement and non-rewrite rules

### Out of scope

- Implementation of Rust/Tauri/Python/Go code
- Code migration, deletion, or archive reactivation
- Dependency, lockfile, manifest, CI, or test repair
- Operational deployment
- Acceptance of this or dependent ADRs (T030 / later gates)
- Final platform composition (ADR-01)
- Detailed security model, IPC schemas, DB choice, release architecture

---

## 5. Options considered

### Option A — Rust-first, polyglot-at-the-edges (proposed)

**Description:** Rust owns trusted systems; TS UI presentation-only; Python for bounded workers; Go optional with justifying ADR; no Rust rewrite of scientific/OpenMed/Graphify stacks.

| | |
| --- | --- |
| Benefits | Aligns with Decision A; clear trust boundary; preserves polyglot ecosystems; matches Plan R2–R5 sequencing |
| Risks | Host build cost; skill demand; migration from current UI-direct OpenMed calls; incomplete R0 runtime verification |
| Compatibility | Required by Decision A / Plan |
| R0 implications | Archives remain evidence only; current UI/Python wiring is transitional fact |
| Draft disposition | **Proposed** |

### Option B — TypeScript/UI-centric durable authority

**Description:** Continue expanding `afia-ui` + bridges as durable content/policy/audit owners.

| | |
| --- | --- |
| Benefits | Faster short-term iteration on current tree |
| Risks | Contradicts Decision A and Trusted Host goals; weakens secrets/PHI/audit boundaries |
| Compatibility | **Incompatible** with ratified Decision A |
| R0 implications | Matches some current wiring (UI→OpenMed) but not target governance |
| Draft disposition | **Rejected** (incompatible with accepted direction) |

### Option C — Immediate full Rust rewrite of ML/science stacks

**Description:** Rewrite OpenMed/Graphify/Jupyter/R ecosystems in Rust before Alpha.

| | |
| --- | --- |
| Benefits | Homogeneous language (theoretical) |
| Risks | Violates Decision A non-rewrite rule; destroys 60-day objective |
| Compatibility | **Forbidden** by Decision A / Plan/Tasks |
| Draft disposition | **Rejected** |

### Option D — Reactivate archived Rust/Tauri/Go scaffolds as authority

**Description:** Treat `_archived/**` crates/desktop/Go as the current Trusted Host.

| | |
| --- | --- |
| Benefits | Apparent reuse |
| Risks | T008 found scaffolds/incomplete prototypes; contradictory root manifests; licensing/provenance gaps |
| Compatibility | Spec disposition archive/investigate/replace — not silent reactivation |
| Draft disposition | **Rejected** as authority; shapes may be **investigated** later under authorized tasks |

### Option E — Go Trusted Host or Go-required Alpha

**Description:** Make Go central to host or Alpha delivery.

| | |
| --- | --- |
| Benefits | None required by accepted program |
| Risks | Contradicts Alpha “no Go required”; T008 archive disposition |
| Compatibility | Only via future justifying ADR for bounded worker — not host |
| Draft disposition | **Rejected** for host/Alpha requirement |

---

## 6. Consequences

### Positive (prospective — not measured outcomes)

- Clear language authority for later ADRs and R2 host work **if accepted at T030**
- Reduces risk of UI/worker silent ownership of reserved authorities
- Preserves Python/Lab ecosystems under supervision rather than forced rewrite

### Costs and negative consequences

- Requires new Rust Trusted Host capability not present in R0 (T010)
- Team/tooling and packaging burden (Windows-first Alpha; current CI macOS-skew — T009)
- Migration cost from current UI-direct OpenMed calls (T007/T010 C5/C12)
- Manifest truthfulness work still required later (not authorized by this draft)

### Risks (including unresolved R0 items)

| Risk | Evidence | Note |
| --- | --- | --- |
| Over-reading archive crates as reusable Trusted Host | T008 | Investigate shapes only |
| Premature Tauri claim | ADR-01 not accepted | Composition deferred |
| Runtime unknowns (bridges, auth env, install) | T002; T005; T007; U-RT-* | Must remain unresolved here |
| Supabase vs Artifact Store migration | T006; Spec 002; Decision C | Not settled by ADR-15 |
| Treating this draft as implementation license | Authority rules | Explicitly prohibited |

---

## 7. Constraints and invariants

### Binding now (higher authority — not created by this draft)

| Constraint | Source |
| --- | --- |
| Decision A Rust-first direction | Plan 001 (Ratified) |
| No OpenMed/Graphify **fork/import** in Spec 001 program | Spec/Plan/Tasks |
| No Rust rewrite of OpenMed/Graphify/Jupyter/R/scientific ecosystems | Decision A |
| No R2 implementation before ADR-15 **acceptance** | Plan blocking matrix; Tasks T030/T031+ |
| Auth/session behavior freeze until dedicated migration spec | Spec/Plan |
| No production mutation from this ADR file | Tasks T012 Allowed/Prohibited |

### Proposed invariants (become architecture authority only if ADR-15 accepted at T030)

| Proposed invariant |
| --- |
| Durable Artifact/Revision/Run mutation only through Rust Trusted Host |
| Non-Rust runtimes supervised and capability-bounded per §3.4 |
| Go not required for Founder Alpha; host roles not owned by Go |
| UI remains presentation-only for reserved authorities |

---

## 8. Constrains (if accepted at T030)

If and when ADR-15 is accepted, it **would** constrain (per Plan roadmap):

| ADR | Proposed constraint from language authority |
| --- | --- |
| ADR-01 | Host Core implementation language is Rust (composition choice separate) |
| ADR-02 | Trusted Host implementation is Rust-only for the reserved responsibility set |
| ADR-04 | Language-neutral schemas; durable mutation enforcement in Rust |
| ADR-05 | Artifact/Revision/Run kernel is Rust; workers cannot mutate Artifacts directly |
| ADR-06 | All non-Rust runtimes supervised via versioned IPC under §3.3–3.4 |
| ADR-08 | Fehrest integration is Rust-supervised process boundary |
| ADR-09 | DeepMed/OpenMed-derived Python workers allowed; no Rust rewrite mandate; no host authority |
| ADR-10 | commandF non-Rust ⇒ supervised worker only |
| ADR-13 | Packaging uses early Rust host + supervised Python; no Go required for Alpha |
| ADR-14 | PHI egress and plugin permissions decided in Rust gateway |

Also informs (not a complete constrain-set by themselves): ADR-03 (UI-only TS), ADR-07 (adapter cannot become content SoT), ADR-11/12.

These constraints are **proposed** until T030. They do **not** authorize drafting or implementing those ADRs beyond their own tasks.

---

## 9. Unresolved questions

T012 does **not** resolve these. Attempted resolution: **NO**.

| ID | Question | Evidence source | Future task/gate |
| --- | --- | --- | --- |
| U-ADR15-1 | Exact desktop composition framework and packaging | Plan proposes Tauri 2; R0 archive Tauri stub | **ADR-01**; packaging ADR-13 |
| U-ADR15-2 | Minimal Trusted Host R2 checklist and denial tests | Plan R2 exit | **ADR-02**; R2 tasks after T030 |
| U-ADR15-3 | IPC/worker contract details | contracts/*; T007 live OpenMed calls | **ADR-06** |
| U-ADR15-4 | Artifact Store design and Supabase migration | T006; Decision C | **ADR-05/07**; Spec 002 |
| U-ADR15-5 | DeepMed substrate operationalization | Decision B; T007 | **ADR-09** |
| U-ADR15-6 | Whether any archived crate shapes are reusable | T008 | Investigate under authorized tasks; not restore |
| U-ADR15-7 | Root Cargo/go/pnpm contradiction repair sequencing | T010 C1–C3 | Manifest adapt tasks after gates |
| U-ADR15-8 | Runtime verification of bridges/auth/install | U-RT-* in R0 package | Later authorized verify tasks |
| U-ADR15-9 | PHI egress / plugin permission enforcement detail | Constitution; Plan | **ADR-14** |
| U-ADR15-10 | Signing custody operations | Decision D | Before R5; not ADR-15 |

---

## 10. Validation and acceptance plan

```text
T012 completion produces a draft for review. T030 is the acceptance gate.
```

Before T030, planning reviews **may** include:

- evidence review against R0 package and Decision A
- architectural consistency with Spec/Plan ADR roadmap
- security/trust-boundary review (claim language; PHI posture)
- migration feasibility relative to current `afia-ui` + bridges
- Tier A independent review (Tasks 001)

**Not** validation for T012: production builds, dependency installs, or runtime “green” CI as architecture acceptance.

**T030** remains the gate to mark ADR-15 (with ADR-01/02/06 per Tasks) **Accepted** before any R2 implementation.

---

## 11. Gate

**ADR-15 MUST be accepted at T030 before R2 implementation** (Plan blocking matrix; Tasks 001).

Until that acceptance, this file remains:

```text
PLANNING EVIDENCE — NON-AUTHORITATIVE ADR DRAFT
```

---

## 12. References

### Accepted authority

- Constitution v1.0.0 (`.specify/memory/constitution.md`)
- [Specification 001](../spec.md)
- [Plan 001](../plan.md) — Decision A; blocking matrix; R0–R5
- [Tasks 001](../tasks.md) — T012; T030; R2 gates

### Planning evidence (non-architecture-authority)

- [research.md](../research.md) § R12
- [contracts/worker-runtime.md](../contracts/worker-runtime.md)
- [contracts/trusted-host-ipc.md](../contracts/trusted-host-ipc.md)

### R0 factual evidence

- [R0-evidence-package.md](../../../docs/program-memory/baseline/R0-evidence-package.md) — R0 close `f37563d8f8499aa24e8ead68fa920167ebb8cde6`
- [R0-path-classification.md](../../../docs/program-memory/baseline/R0-path-classification.md) — T010 `2c9fb55244588bb35b2d08bc1e1c6bbcbadbd8ec`
- [R0-archived-assets.md](../../../docs/program-memory/baseline/R0-archived-assets.md)
- [R0-frontend-map.md](../../../docs/program-memory/baseline/R0-frontend-map.md)
- [R0-python-services.md](../../../docs/program-memory/baseline/R0-python-services.md)
- [R0-ci-tests.md](../../../docs/program-memory/baseline/R0-ci-tests.md)
- [R0-tooling.md](../../../docs/program-memory/baseline/R0-tooling.md)
- [R0-build-commands.md](../../../docs/program-memory/baseline/R0-build-commands.md)

### Explicitly non-authoritative

- `_archived/**`
- Legacy `docs/product/AFIA_*`
- Graphify output
- AI recommendations
