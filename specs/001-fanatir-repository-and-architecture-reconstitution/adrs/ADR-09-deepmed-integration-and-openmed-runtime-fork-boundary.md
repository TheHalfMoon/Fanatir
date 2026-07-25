# ADR-09 — DeepMed Integration and OpenMed Runtime/Fork Boundary

| Field | Value |
| --- | --- |
| **ADR** | ADR-09 |
| **Title** | DeepMed Integration and OpenMed Runtime/Fork Boundary |
| **Status** | **Proposed** (draft only) |
| **Task origin** | T021 |
| **Acceptance / review posture** | Tier A independent review after drafting; **T030** expects ADR-09 listed **Reviewed or Accepted** (not automatically Accepted; **not** in the mandatory Accepted set that opens R2). DeepMed implementation such as **T048** / **T049** requires ADR-09 **Accepted**; **T050** requires ADR-08 / ADR-09 / ADR-05 **Accepted** |
| **Feature** | `001-fanatir-repository-and-architecture-reconstitution` |
| **Required gate (tasks.md)** | Decision B; no fork/import task under 001 |
| **Planning dependency (plan.md)** | Q6, P3, ADR-15, Decision B |
| **Related founder decision** | Decision B (R3 DeepMed may temporarily use OpenMed **PyPI** runtime — package use ≠ fork/import; requirements via ADR-09; not final architecture) — **Ratified** in accepted [plan.md](../plan.md) |
| **Constraining drafts** | [ADR-15](./ADR-15-rust-first-polyglot-runtime.md) (Proposed); [ADR-02](./ADR-02-rust-trusted-host-boundary.md) (Proposed); [ADR-04](./ADR-04-shared-primitive-ownership-and-versioning.md) (Proposed); [ADR-05](./ADR-05-artifact-revision-run-storage.md) (Proposed); [ADR-06](./ADR-06-worker-and-ipc-contracts.md) (Proposed); [ADR-07](./ADR-07-supabase-adapter-and-local-first-boundary.md) (Proposed); [ADR-08](./ADR-08-fehrest-integration-and-release-model.md) (Proposed) |
| **Related drafts** | [ADR-01](./ADR-01-platform-and-desktop-composition-tauri-2.md) (Proposed); [ADR-03](./ADR-03-afia-ui-strangler-migration.md) (Proposed) |
| **Planning evidence** | [spec.md](../spec.md) Q6; [plan.md](../plan.md) Decision B / ADR-09 row; [research.md](../research.md) R7 / R10 / R13 / P3; [deepmed-integration.md](../contracts/deepmed-integration.md); [R0-python-services.md](../../../docs/program-memory/baseline/R0-python-services.md); [data-model.md](../data-model.md); [shared-primitives.md](../contracts/shared-primitives.md); [trusted-host-ipc.md](../contracts/trusted-host-ipc.md); [worker-runtime.md](../contracts/worker-runtime.md); [tasks.md](../tasks.md) T021 / T007 / T030 / T048–T050 |
| **Architecture authority of this file** | **NO** — until a valid stage-gate acceptance action records Accepted |
| **Implementation authorization** | **NO** |

```text
This document is a planning draft and is not accepted architecture authority.
```

```text
Status: Proposed
Draft ADR ≠ architecture acceptance
T021 completion ≠ T030 acceptance
Passing Tier A review ≠ Accepted
tasks.md founder-acceptance field for T021: No — draft only; Decision B already ratified
Founder acceptance of a draft work product ≠ architecture acceptance
Ratified Decision B ≠ Accepted ADR-09
Founder-ratified Q6 ≠ Accepted ADR-09
P3 is planning research resolution ≠ Accepted ADR-09
ADR-15 / ADR-01 / ADR-02 / ADR-03 / ADR-04 / ADR-05 / ADR-06 / ADR-07 / ADR-08 remain Proposed and non-authoritative
```

```text
This draft is not:
- an implemented DeepMed runtime
- permission to initialize or modify DeepMed-AI
- permission to install or import OpenMed
- permission to fork OpenMed
- permission to download or execute models
- permission to call remote inference providers
- permission to transmit PHI
- permission to publish clinical conclusions
- a final model or provider selection
- a final IPC schema
- permission to begin R2 or R3
```

```text
No production implementation is authorized by this draft.
```

This draft **must not** be used as justification to: implement DeepMed; initialize or modify DeepMed-AI; install, import, fork, clone, or call OpenMed; download or run models; perform inference, training, fine-tuning, RAG, retrieval, or evaluation; transmit PHI; publish clinical conclusions; mutate Artifacts, Revisions, or Runs; publish final IPC schemas; change authentication or sessions; begin R2 (`T031+`) or R3 DeepMed work; or treat Proposed ADRs as Accepted.

---

## 1. Context and problem

### 1.1 Assigned architectural question

**ADR-09 proposes** how **DeepMed** would integrate with **Fanatir** as an independent medical-intelligence runtime under a **Rust-supervised** Python worker/sidecar model, and how a **temporary OpenMed PyPI package substrate** (ratified **Decision B**) would be bounded and distinguished from **prohibited OpenMed fork/import**, constrained by Proposed [ADR-15](./ADR-15-rust-first-polyglot-runtime.md), [ADR-02](./ADR-02-rust-trusted-host-boundary.md), [ADR-04](./ADR-04-shared-primitive-ownership-and-versioning.md), [ADR-05](./ADR-05-artifact-revision-run-storage.md), [ADR-06](./ADR-06-worker-and-ipc-contracts.md), [ADR-07](./ADR-07-supabase-adapter-and-local-first-boundary.md), and [ADR-08](./ADR-08-fehrest-integration-and-release-model.md).

It does **not** implement DeepMed; install or import OpenMed; finalize shared schemas (**ADR-04**); finalize Artifact Store persistence (**ADR-05**); finalize IPC envelopes (**ADR-06**); define Fehrest product behavior (**ADR-08**); define commandF (**ADR-10**, not authored); redesign authentication/sessions (**ADR-11**, not authored); or finalize PHI egress (**ADR-14**, not authored).

### 1.2 Why an explicit DeepMed / OpenMed boundary matters

Fanatir’s Founder Alpha journey requires a functioning **bounded DeepMed** pipeline in Studio without treating DeepMed as placeholder-only ([spec.md](../spec.md) Q6). Spec 001 simultaneously forbids OpenMed **fork/import**, while ratified **Decision B** allows temporary **OpenMed PyPI package** use as an R3 substrate under ADR-09 rules ([plan.md](../plan.md); [deepmed-integration.md](../contracts/deepmed-integration.md); [research.md](../research.md) R13).

Without an explicit boundary, planning identifies these **risks** (prospective — not claimed as currently measured production failures unless separately evidenced):

| Risk | Why it matters |
| --- | --- |
| Temporary OpenMed package use becomes permanent architecture | Exit/replacement path lost |
| Package use confused with fork or import | Spec 001 prohibition bypassed |
| DeepMed becomes autonomous clinical authority | Assistive-only rule violated |
| Model outputs become clinical truth without review | Silent approval / unsafe care |
| UI or workers bypass Capability Gateway | Policy/audit collapse |
| Workers directly mutate Artifacts or Runs | ADR-05 / worker-runtime violation |
| Prompts, responses, logs, or provider requests leak PHI | Classification/egress failure |
| Unsupported clinical claims lack source spans | Fabricated mappings |
| Uncertainty hidden | Overconfident assistive output |
| Model/provider identity not recorded | Provenance failure |
| Unpinned package/model/runtime versions | Non-reproducible Alpha |
| Provider or model licensing ignored | Rights / NOTICE failure |
| Local vs remote inference authority ambiguous | Unrestricted cloud PHI risk |
| DeepMed and Fehrest responsibilities conflated | Discovery mistaken for clinical assist |

### 1.3 Accepted program and planning constraints

| Constraint | Source | Treatment |
| --- | --- | --- |
| DeepMed required in first integrated release; define contracts/process/security/spans/review/Fanatir expectations in 001 **without** importing OpenMed; separate DeepMed spec governs fork/import; Alpha pipeline minimums listed | Q6 — [spec.md](../spec.md) (**founder-ratified**) | Binding Alpha boundary; **not** ADR-09 acceptance |
| R3 DeepMed may temporarily use OpenMed **PyPI** runtime; package use ≠ fork/import; not final architecture; ADR-09 records requirements | Decision B — [plan.md](../plan.md) (**Ratified**); [research.md](../research.md) R13; [deepmed-integration.md](../contracts/deepmed-integration.md) | Ratified planning direction; **not** ADR-09 acceptance |
| Process/sidecar primary; pins; compatibility matrix | P3 — [research.md](../research.md) (**resolved for planning**) | Planning release-channel evidence |
| Capability Gateway mandatory; Rust-supervised Python worker; Fanatir owns Artifact/Run persistence | [deepmed-integration.md](../contracts/deepmed-integration.md); ADR-15 Proposed | Planning contract + Proposed constraints |
| No OpenMed/Graphify fork/import under T021; no production code; draft ≠ accepted | [tasks.md](../tasks.md) T021 | Binding for this task |
| No R2 before T030 (ADR-15/01/02/06 Accepted as applicable) | [tasks.md](../tasks.md) | Future-gate requirement |

### 1.4 Naming

| Name | Role in this ADR |
| --- | --- |
| **Fanatir** | Product/integration authority; owns Capability Gateway; owns Artifact/Run persistence direction |
| **DeepMed** | Independent medical-intelligence runtime — clinical-assistive; **not** clinical SoT |
| **OpenMed** | Named PyPI package candidate for temporary Decision B substrate; fork/import **prohibited** under Spec 001 |
| **Artifact Store** | Local Rust-controlled Artifact/Revision/Run content authority (**ADR-05** Proposed) |
| **Fehrest** | Knowledge/vault product (**ADR-08** Proposed) — not DeepMed |
| **Capability Gateway** | Fanatir-owned authorization path for DeepMed invocation |
| **Source** / **Relationship** / **Run** | Shared primitives under **ADR-04** Proposed |
| **`afia-ui`** | Migration shell; not DeepMed architecture SoT (**ADR-03** Proposed) |

Never confuse DeepMed with Fanatir, DeepMed with Fehrest, OpenMed package use with OpenMed fork/import, or assistive output with approved care.

### 1.5 What this draft is not

This file is **not**: an implemented DeepMed runtime; DeepMed-AI initialization; OpenMed install/import/fork; model download or inference; clinical publication authority; final model/provider selection; final IPC schema; PHI-egress authorization; or R2/R3 authorization.

---

## 2. Decision drivers

| Driver | Source |
| --- | --- |
| Founder-ratified Q6 Alpha / OpenMed sequencing | spec.md |
| Ratified Decision B temporary PyPI substrate | plan.md; research R13; deepmed-integration.md |
| P3 release-channel planning | research.md |
| Rust-supervised Python workers | ADR-15 Proposed; worker-runtime.md |
| Capability Gateway + host mediation | deepmed-integration.md; ADR-02 Proposed |
| Shared Source/Run/Approval semantics | ADR-04 Proposed; data-model.md |
| Artifact Store persistence | ADR-05 Proposed; Decision C |
| IPC / worker contracts | ADR-06 Proposed |
| Local-first / no unrestricted cloud PHI | ADR-07 Proposed; deepmed-integration Forbidden |
| Fehrest handoff sibling | ADR-08 Proposed |
| T021 draft-only; Tier A; no production code | tasks.md T021 |
| Prototype bridge inventory (transitional) | R0-python-services.md; T007 |

---

## 3. Decision (proposed)

### 3.1 Core proposal

**ADR-09 proposes** that DeepMed integrate with Fanatir as:

1. an **independent medical-intelligence runtime** (DeepMed-AI boundary);
2. a **first-integrated-release** clinical-assistive capability (Q6);
3. a **Rust-supervised Python** sidecar/worker under **Capability Gateway** mediation;
4. a producer of **provisional, review-required** outputs with source spans, uncertainty, and model/Run provenance;

and that any later **temporary OpenMed PyPI package substrate** (Decision B) would be:

- pinned, local-by-default, isolated, NOTICE/license-aware, replacement-pathed;
- **not** OpenMed fork/import;
- **not** final DeepMed architecture;
- **not** authorized for install/invoke by T021 itself.

This is **Proposed** planning only. No DeepMed product, OpenMed install, model, or inference is claimed to exist as implemented architecture from this draft.

### 3.2 Decision B alignment (ratified posture; ADR-09 Proposed)

Accurately record ratified Decision B ([plan.md](../plan.md); [deepmed-integration.md](../contracts/deepmed-integration.md); [research.md](../research.md) R13):

| Decision B element | ADR-09 treatment |
| --- | --- |
| R3 DeepMed may temporarily use OpenMed **PyPI** package as runtime substrate | Record as planning direction |
| Local by default | Preserve |
| Exact version pin + lockfile evidence (later) | Require as pre-implementation rule; **exact pin value unresolved** here |
| Rust-supervised isolated worker | Preserve |
| Versioned request/response; bounded I/O | Preserve (IPC detail → ADR-06) |
| No unrestricted FS/secret/network/patient-store access | Preserve |
| Source spans; confidence/uncertainty; review/correction | Preserve |
| Model and Run provenance | Preserve |
| Apache-2.0 + NOTICE; per-model license manifest | Preserve (package license evidence from inventory; model licenses separate) |
| Explicit replacement path | Required |
| Must not claim prototype is final DeepMed architecture | Required |
| Package use ≠ fork/import | Required distinction |
| T021 installs or invokes OpenMed | **Does not** |

Ratified Decision B ≠ Accepted ADR-09.

### 3.3 DeepMed product role (proposed)

| DeepMed is… | DeepMed is not… |
| --- | --- |
| Independent medical-intelligence runtime | Fanatir itself |
| First-integrated-release capability (Q6) | The Trusted Host |
| Clinical-assistive worker/runtime family | The Artifact Store |
| Governed processor of medical documents and context | Fehrest |
| Producer of provisional, review-required outputs | A patient / FHIR source of truth |
| | A clinical decision / approved-care authority |
| | A PolicyDecision / authorization authority |

### 3.4 Q6 Alpha pipeline boundary (proposed alignment)

ADR-09 **proposes** that Alpha DeepMed capability claims remain bounded to founder-ratified Q6 ([spec.md](../spec.md)):

**In Alpha planning scope (required pipeline capabilities):**

- supported medical document input;
- task-first workflow;
- PHI detection or de-identification workflow;
- selected clinical entity extraction;
- source spans;
- confidence / uncertainty;
- review / correction;
- output handoff to Fehrest and commandF;
- recorded model and Run provenance;
- integrated into Studio as a functioning bounded pipeline (later R3 — **not** by T021).

ADR-09 does **not** add capabilities unsupported by canonical sources. The pipeline is **not** claimed implemented.

### 3.5 Clinical-assistive authority (proposed)

Propose explicit separation:

| Layer | Authority posture |
| --- | --- |
| Model-generated candidate output | Provisional; non-authoritative |
| Reviewed / corrected output | Still not automatic clinical truth |
| Authoritative Artifact publication | Fanatir / host / ADR-05 path after governed validation |
| Clinical judgment | Human clinician / researcher — not DeepMed |
| Approved care | Explicit Approval / policy — not DeepMed |
| PolicyDecision | Host / gateway — not DeepMed |

DeepMed **must not**: autonomously diagnose; autonomously prescribe treatment; issue approved-care decisions; silently approve mappings; replace clinician or researcher review; become a patient-record authority; treat a confidence score as correctness.

### 3.6 Human review and approval boundary (proposed)

Planning requirements:

- review hooks and correction structures (deepmed-integration Alpha result contract);
- rejection and non-publication paths;
- inability to silently promote model output to approved care;
- shared **Approval** semantics remain under **ADR-04** / [data-model.md](../data-model.md) (“No silent clinical approval”).

Exact review UX, reviewer identity capture, and patient-facing vs clinician-facing presentation remain **unresolved**. This draft does **not** claim every DeepMed output must be patient-facing or clinician-facing.

### 3.7 Input and context boundary (proposed)

At planning level, possible inputs include (not a final schema): user task/query; supported medical documents; selected Artifacts/Revisions; Sources; Relationships; Fehrest references; FHIR resources; patient context; clinical notes; retrieval results; model configuration; AuthContext; DataClassification.

Future authorized invocation **would require**: input validation; explicit provenance; minimum-necessary context; classification awareness; authorization via Capability Gateway; bounded payloads; explicit local-versus-remote handling rules before any remote path.

Final input schemas are **not** published here.

### 3.8 PHI detection and de-identification (proposed)

Q6 requires PHI detection or de-identification workflow evidence in the Alpha pipeline.

Clarifications:

- PHI detection does **not** by itself authorize egress;
- de-identification does **not** automatically guarantee irreversible anonymity;
- unknown classification **fails closed** for privileged/egress-relevant paths;
- failed or uncertain de-identification **must block** unauthorized external transmission;
- full enforcement and egress policy remain under **ADR-14** and Trusted Host controls;
- no de-identification implementation is claimed.

No unsupported clinical or regulatory guarantees are asserted.

### 3.9 Output boundary (proposed)

Illustrative provisional result categories aligned with [deepmed-integration.md](../contracts/deepmed-integration.md) Alpha result contract (planning fields — **not** production-approved schemas):

| Planning field | Requirement (planning) |
| --- | --- |
| `taskId` | Task-first workflow identity |
| `entities[]` | Selected clinical entity extraction |
| `sourceSpans[]` | Locators into Source/Artifact |
| `confidence` / uncertainty | Explicit, not hidden |
| `phiFindings` or de-id status | Detection or de-identification workflow evidence |
| `reviewHooks` | Structure for human correction |
| `modelId` + `runtimeVersion` | Provenance |
| `runId` | Links to Fanatir Run |

Also discussable as planning outcomes: warnings; refusal; failure evidence.

Clarifications: exact schemas unresolved; outputs are not clinical truth automatically; invalid/incomplete outputs fail closed; review required before authoritative handoff or publication.

### 3.10 Source spans and evidence grounding (proposed)

DeepMed outputs **would** preserve evidence links where applicable: Source references; Artifact/Revision references; source spans; provenance; coverage awareness; unsupported-statement flags.

State that:

- fabricated mappings without evidence spans are **forbidden**;
- citations do **not** guarantee clinical correctness;
- generated reasoning **must** remain distinguishable from source evidence;
- exact evidence-scoring methods remain **unresolved**.

### 3.11 Uncertainty, abstention, and refusal (proposed)

ADR-09 **proposes** explicit architectural support for: confidence/uncertainty representation; abstention; refusal; insufficient evidence; conflicting evidence; unsupported request; missing provenance; malformed input; unsafe or prohibited egress; unavailable model/runtime; provider failure; unsupported language or modality where applicable.

Exact thresholds, calibration scores, and numerical acceptance gates are **not** invented here.

### 3.12 Artifact Store boundary (proposed)

Preserve Proposed [ADR-05](./ADR-05-artifact-revision-run-storage.md) authority.

DeepMed **may** (later, where authorized): consume governed Artifact references; read governed Revision content; produce candidate output; **request** Artifact publication; associate output with a proposed Run; preserve evidence references.

DeepMed **must not**: directly write the Artifact Store; mutate published Revisions; advance current Revision pointers; bypass publication validation; publish authoritative results without review; become the Artifact Store.

### 3.13 Run and provenance boundary (proposed)

Preserve **ADR-04** / **ADR-05** ownership. Planning rule: DeepMed invocations **MUST** create Runs with model + provenance ([data-model.md](../data-model.md)).

Future invocations **would require** provenance including, where canonically supported: task identity; model identity; model revision; runtime identity/version; provider identity where applicable; input/output references; source spans; review status; failure or cancellation evidence.

Run schemas are **not** finalized. Workers do **not** directly create authoritative Runs by bypassing Fanatir persistence — host-mediated publication paths apply.

### 3.14 Trusted Host and Capability Gateway (proposed)

**Fanatir’s Rust Trusted Host** (Proposed ADR-02 / ADR-15) **would own**: DeepMed process launch; capability checks; authorization; policy decisions; model-path mediation; network mediation; secret injection; resource bounds; audit correlation; cancellation; timeout handling; result validation; Artifact-publication requests.

DeepMed **must not** receive ambient access to: arbitrary filesystem locations; patient stores; secrets; unrestricted network; Artifact Store mutation; ambient host environment variables for privileged credentials.

Gateway behavior is **not** implemented here.

### 3.15 Worker and IPC boundary (proposed)

Align with Proposed [ADR-06](./ADR-06-worker-and-ipc-contracts.md) and [worker-runtime.md](../contracts/worker-runtime.md): process isolation; typed/versioned messages; bounded payloads; Artifact URIs for large content; progress; timeout; cancellation; normalized failures; crash/restart; resource limits.

Final command names, envelopes, payloads, event schemas, and wire codes are **not** published here (illustrative host commands such as `worker.spawn` remain ADR-06 territory per [trusted-host-ipc.md](../contracts/trusted-host-ipc.md)).

### 3.16 OpenMed package boundary (proposed)

Distinguish:

| Mode | Status under Spec 001 / T021 |
| --- | --- |
| 1. OpenMed **PyPI package** as temporary Decision B substrate | Planning-allowed for later R3 under ADR-09 rules — **not** installed by T021 |
| 2. OpenMed source-code **fork** | **Prohibited** under Spec 001 |
| 3. OpenMed source **import** | **Prohibited** under Spec 001 |
| 4. OpenMed **Rust rewrite** | **Prohibited** |
| 5. Final DeepMed architecture | **Not** the temporary PyPI substrate |

Later temporary package use **would require** (planning): exact package/version pin; lockfile evidence; license and NOTICE preservation; runtime isolation; model-license manifest; compatibility validation; replacement/exit plan; fail-closed unsupported versions.

T021 does **not** install the package. Exact pin value remains **unresolved** (plan open question).

### 3.17 OpenMed fork/import boundary

Explicitly state:

- Spec 001 contains **no** OpenMed fork/import task;
- fork/import requires a **separate DeepMed specification**;
- source import requires licensing and provenance review;
- no OpenMed code is incorporated by ADR-09;
- no Rust rewrite is authorized;
- treating PyPI use as governed fork/import authorization is **forbidden**.

No future-spec files or scaffolds are created here.

### 3.18 Prototype bridge boundary

Canonical evidence records `services/openmed_bridge.py` / related UI localhost wiring as **prototype / investigate** inventory ([R0-python-services.md](../../../docs/program-memory/baseline/R0-python-services.md); [research.md](../research.md); Proposed ADR-02/03).

Clarifications:

- it is **not** DeepMed;
- it is **not** target architecture;
- UI-to-localhost direct access is **transitional**;
- it does **not** grant production OpenMed authority;
- future migration belongs to later implementation and strangler tasks;
- T021 does **not** modify or delete the prototype.

### 3.19 Model runtime modes (proposed)

At planning level, evaluate: local inference; remote inference; hybrid inference.

Preserve: **local-by-default** Decision B posture; **no unrestricted cloud-model PHI**; host-mediated provider access if ever authorized; explicit classification and authorization; provider/model provenance; separate evaluation and licensing.

Final exclusive runtime mode remains **unresolved** beyond local-default + cloud-PHI prohibition. Remote inference is **not** authorized by T021.

### 3.20 Model and provider selection (proposed)

State explicitly:

- ADR-09 does **not** choose a final model;
- ADR-09 does **not** create a model allowlist unless canonically defined (Spec journey language “Approved DeepMed models only” implies later governance — not a settled list here);
- ADR-09 does **not** select a remote provider;
- model IDs and revisions **must** be pinned before implementation;
- provider endpoints and terms require **separate authorization**.

Do not name illustrative models as selected architecture.

### 3.21 Model pinning and reproducibility (proposed)

Planning requirements for later implementation: model ID; model revision or immutable digest where supported; tokenizer identity; runtime version; package version; prompt-template version; generation configuration; provider/model compatibility; model-license manifest.

Pins are **not** invented here. Reproducibility is **not** claimed implemented.

### 3.22 Prompt and generation boundary (proposed)

Planning concerns: prompt-template ownership; task-first prompts; evidence insertion; PHI minimization; system/policy instructions; maximum context; deterministic versus stochastic generation; sampling configuration; prompt injection; untrusted document content.

Exact prompt templates and generation parameters are later implementation or evaluation details. Production prompts are **not** published here.

### 3.23 Tool use and retrieval boundary (proposed)

Planning posture: tool calls / RAG / retrieval / Source lookup / Artifact access / FHIR lookup / provider access — if ever used — **would require** Capability Gateway authorization; retrieval results are untrusted until validated; RAG is **not** automatically part of Alpha; tool results must preserve provenance; no direct worker access to patient stores; Fehrest discovery remains separate from DeepMed clinical-assistive reasoning.

Tools and retrieval are **not** implemented here.

### 3.24 Fehrest handoff (proposed)

Preserve Proposed [ADR-08](./ADR-08-fehrest-integration-and-release-model.md):

- Fehrest may provide governed Sources, notes, or references;
- DeepMed may later return **reviewed** results;
- handoff uses governed references or Artifacts;
- Trusted Host mediation is required;
- direct ungoverned mutual invocation is **prohibited**;
- candidate Source/Relationship publication remains governed;
- **T050** requires ADR-08, ADR-09, and ADR-05 **Accepted**.

Fehrest operations are **not** defined here.

### 3.25 commandF handoff (proposed boundary only)

Where Q6 / deepmed-integration mention handoff to commandF:

- **ADR-10** (not authored) owns commandF;
- ADR-09 does **not** define commandF behavior;
- DeepMed handoff must remain governed;
- no direct authority is granted;
- exact UX and IPC remain unresolved.

### 3.26 Supabase and local-first boundary (preserve ADR-07)

Preserve Decision C and Proposed [ADR-07](./ADR-07-supabase-adapter-and-local-first-boundary.md):

- Artifact, Revision, Run, PHI, prompt, and clinical-output authority remains local;
- Supabase is **not** DeepMed’s clinical or Artifact-content SoT;
- T021 does **not** synchronize prompts or outputs;
- remote collaboration metadata cannot authorize inference;
- remote rows cannot publish DeepMed output;
- local authoritative behavior must not depend silently on Supabase.

### 3.27 Remote inference and PHI egress (proposed)

- Remote inference is **not** authorized by T021;
- unrestricted cloud-model PHI is **forbidden**;
- encrypted transport alone does **not** authorize egress;
- de-identification alone does **not** automatically authorize egress;
- classification, policy, minimum-necessary context, provider terms, and explicit authorization are required before any future remote path;
- **ADR-14** owns full PHI classification and egress policy.

Remote provider use is **not** implied as Alpha.

### 3.28 Secrets and credentials (proposed)

Address: provider API keys; model-hub credentials; package indexes; license tokens; local model paths; secret injection; logs; traces; IPC envelopes; environment variables; UI exposure.

Require: Trusted Host-controlled injection; no ambient inheritance; no secrets in prompts, outputs, ordinary logs, or persisted Artifacts; redacted diagnostics.

No secrets backend is chosen here.

### 3.29 Licensing and model rights (claim language)

Require future rights-aware selection for: OpenMed package; model weights; tokenizer assets; provider terms; gated-model access; commercial restrictions; acceptable-use policies; redistribution; attribution; output rights; dataset rights.

Preserve: OpenMed package Apache-2.0 + NOTICE obligations where canonically recorded for the PyPI package; per-model license manifest requirement; no rights claimed for unnamed models; fork/import licensing deferred to a separate DeepMed specification.

Unknown rights **must fail closed** for redistribution or bundling.

### 3.30 Evaluation and safety gates (planning)

Future validation **may** include (planning discussion): source-span accuracy; unsupported-claim rate; citation correctness; uncertainty/abstention; PHI leakage; de-identification failure; unsafe recommendation behavior; malformed output; model/provider compatibility; latency; memory/resource use; reviewer correction; clinical benchmark suitability; red-team testing; bias/fairness where applicable.

Results, thresholds, and passed benchmarks are **not** invented. Detailed protocols belong to later DeepMed or evaluation specifications / R3 work.

### 3.31 Failure and degraded operation (proposed)

Address: model unavailable; OpenMed package unavailable; provider unavailable; model download failure; license rejection; timeout; cancellation; malformed output; missing source spans; unsupported output; insufficient evidence; network failure; resource exhaustion; process crash; partial output; retry; fallback; quarantine; safe shutdown.

Do **not** select fallback models. Do **not** imply automatic retries for non-idempotent or sensitive operations. Partial or failed output **must not** become authoritative. Lifecycle detail aligns with **ADR-06**.

### 3.32 Version-domain separation (proposed)

Distinguish: DeepMed product/runtime version; OpenMed package version; model ID; model revision; tokenizer version; provider-adapter version; prompt-template version; evaluation-profile version; ADR-04 shared schema version; ADR-06 IPC protocol version; ADR-05 persistence-format version; Fanatir application version.

Do **not** collapse these into one version. Unsupported combinations **fail closed**.

### 3.33 Release and compatibility model (P3 planning; not implementation)

Use P3 only as planning evidence ([research.md](../research.md) P3): DeepMed-AI semantic versioning; Fanatir compatibility pins; sidecar/runtime compatibility; model/runtime matrix discussion; update authority; rollback; unsupported versions; temporary OpenMed replacement path.

Releases, tags, pins, and compatibility tests are **not** published or claimed by T021.

### 3.34 Auditability (proposed)

Audit-relevant events **may** include: DeepMed process launch; capability denial; model selection; provider-access request; PHI-egress denial; operation acceptance/rejection; output validation failure; review state change; Artifact publication request; timeout; cancellation; process crash; license-policy rejection; incompatible runtime or model.

Authoritative audit ownership remains with Trusted Host / **ADR-02** and later audit policy. Audit schemas are **not** finalized here.

### 3.35 Decision summary table

| Topic | Proposed disposition |
| --- | --- |
| Product role | Independent clinical-assistive runtime |
| Integration shape | Rust-supervised Python sidecar + Capability Gateway |
| Decision B | Temporary pinned OpenMed PyPI substrate; local default; exit path |
| OpenMed fork/import | Prohibited under Spec 001 |
| Clinical authority | Assistive only; review-required; no silent approval |
| Persistence | ADR-04/05 retain semantics/persistence; candidates only |
| Fehrest / commandF | Governed handoff; ADR-08 / ADR-10 |
| Status | Proposed only |

---

## 4. Scope

### In scope (this Proposed draft)

- DeepMed Integration and OpenMed Runtime/Fork Boundary (planning)
- Decision B alignment without ADR acceptance
- Q6 Alpha pipeline boundary
- Clinical-assistive vs autonomous separation
- OpenMed package vs fork/import distinction
- Host/gateway/worker/Artifact/Run deferrals
- Explicit unresolved questions

### Out of scope

- Implementing DeepMed; initializing DeepMed-AI; installing/forking OpenMed
- Model download, inference, training, RAG, retrieval, evaluation execution
- Final schemas, IPC publication, Artifact mutation, PHI egress engine
- Fehrest/commandF implementation; R2 / R3 execution

---

## 5. Alternatives considered

### Option A — DeepMed as autonomous clinical authority

| | |
| --- | --- |
| Summary | Model output as approved care / diagnosis/treatment authority |
| Costs / risks | Contradicts assistive-only / no silent approval / plan excluded autonomous clinical decision-making |
| Draft disposition | **Rejected** |

### Option B — Direct UI-to-OpenMed localhost access as architecture

| | |
| --- | --- |
| Summary | Preserve UI→`127.0.0.1` bridge as target |
| Costs / risks | Bypasses Trusted Host / Capability Gateway; transitional anti-pattern |
| Draft disposition | **Rejected** as target architecture |

### Option C — OpenMed fork/import during Spec 001

| | |
| --- | --- |
| Summary | Governed source import now |
| Costs / risks | Forbidden under Spec 001; needs separate DeepMed specification |
| Draft disposition | **Rejected** under 001 |

### Option D — Rust rewrite of OpenMed

| | |
| --- | --- |
| Summary | Reimplement OpenMed in Rust for Alpha |
| Costs / risks | Unauthorized; breaks 60-day objective; ADR-15 non-goal |
| Draft disposition | **Rejected** |

### Option E — Cloud-first model runtime as Alpha default

| | |
| --- | --- |
| Summary | Remote inference default with PHI in cloud models |
| Costs / risks | Unrestricted cloud-model PHI forbidden; Decision B local-default |
| Draft disposition | **Rejected** as Alpha default |

### Option F — Permanent OpenMed PyPI substrate

| | |
| --- | --- |
| Summary | Treat PyPI OpenMed as final DeepMed architecture |
| Costs / risks | Contradicts Decision B “not final architecture” + replacement path |
| Draft disposition | **Rejected** |

### Option G — DeepMed embedded in-process as Alpha primary

| | |
| --- | --- |
| Summary | In-process library embed without process isolation |
| Costs / risks | Weaker isolation for PHI/models; P3 prefers process/sidecar |
| Draft disposition | **Rejected** as Alpha primary |

### Option H — Rust-supervised DeepMed sidecar with temporary pinned OpenMed PyPI substrate and exit path (proposed)

| | |
| --- | --- |
| Summary | Decision B-aligned temporary substrate + gateway + review + provenance |
| Benefits | Bounded assistive Alpha path without fork/import |
| Costs / risks | Dependency debt; licensing; evaluation burden (see §6) |
| Draft disposition | **Proposed** |

Alternatives were **not** empirically tested by this draft.

---

## 6. Consequences

### Positive (prospective — if later accepted and executed under separate authorization)

- Bounded clinical-assistive role
- Explicit human review
- Source-span requirements
- Explicit uncertainty
- Local-default PHI posture
- Replaceable runtime
- Temporary reuse without fork/import
- Reproducible provenance potential
- Governed Fehrest/commandF handoff path
- Clear Artifact publication boundary

### Costs and risks

| Cost / risk | Note |
| --- | --- |
| Python sidecar supervision | Host lifecycle complexity |
| Temporary dependency debt | OpenMed PyPI exit required |
| OpenMed/package compatibility | Pin discipline |
| Model licensing | Per-model manifest |
| Model/runtime pinning | Multiple version domains |
| Clinical evaluation burden | Later specs / R3 |
| PHI and de-identification complexity | ADR-14 |
| GPU/CPU resource needs | Unresolved |
| Model failure handling | Unresolved detail |
| Provider ambiguity | Not selected |
| Prompt and model drift | Version domains |
| Two-product coordination | DeepMed-AI + Fanatir |
| Assistive output treated as clinical truth | Training/process risk |
| Replacement-path execution risk | Must remain real |

### Rollback (planning only)

If rejected before acceptance: revert this ADR file; retain Decision B / Q6 / deepmed-integration planning evidence; do **not** install OpenMed, download models, or implement DeepMed from this draft. Post-acceptance withdrawal would need separately authorized migration — **not** defined here.

---

## 7. Non-goals

ADR-09 does **not**:

- implement DeepMed;
- initialize or modify DeepMed-AI;
- install or import OpenMed;
- fork OpenMed;
- rewrite OpenMed in Rust;
- download or run models;
- select final models or providers;
- perform inference;
- implement prompts, RAG, tools, retrieval, or training;
- define final schemas;
- publish IPC commands;
- mutate Artifacts, Revisions, or Runs;
- publish clinical conclusions;
- transmit PHI;
- define full auth/session behavior;
- define full PHI egress policy;
- implement Fehrest or commandF handoff;
- publish packages or releases;
- begin R2 or R3.

---

## 8. Relationship to other ADRs

| ADR | Status in repo | Relationship |
| --- | --- | --- |
| ADR-15 | Proposed | Rust-first; supervised Python DeepMed worker; no OpenMed Rust rewrite |
| ADR-01 | Proposed | Platform composition; sidecars under host |
| ADR-02 | Proposed | Trusted Host / audit / mediation |
| ADR-03 | Proposed | `afia-ui` strangler; prototype OpenMed localhost seam retirement later |
| ADR-04 | Proposed | Source / Relationship / Run / Approval semantics |
| ADR-05 | Proposed | Artifact/Revision/Run persistence; DeepMed candidates only |
| ADR-06 | Proposed | IPC and worker contracts |
| ADR-07 | Proposed | Supabase/local-first; no remote clinical SoT |
| ADR-08 | Proposed | Fehrest integration; governed handoff |
| ADR-10 | Not authored | commandF ownership |
| ADR-11 | Not authored | Auth/session preservation |
| ADR-12 | Not authored | Rename (if referenced only peripherally) |
| ADR-13 | Not authored | Packaging / distribution |
| ADR-14 | Not authored | PHI classification and egress |

ADR-09 decides only **DeepMed Integration and OpenMed Runtime/Fork Boundary**.

---

## 9. Gates and authority

```text
T021 produces a Proposed ADR-09 draft only.
Tier A independent review is required (DeepMed/OpenMed boundary ADR).
Passing Tier A review is not architecture acceptance.
tasks.md founder-acceptance field for T021 is: No — draft only; Decision B already ratified.
Decision B is ratified planning direction; ADR-09 is not Accepted by Decision B alone.
T030 remains incomplete.
At T030, ADR-09 is expected to be listed Reviewed or Accepted — it is not automatically Accepted.
ADR-09 is NOT in the mandatory T030 Accepted set that opens R2 (ADR-15/01/02/06).
T048 and T049 require ADR-09 Accepted.
T050 requires ADR-08, ADR-09, and ADR-05 Accepted.
T031+ and R2 remain separately gated.
R3 DeepMed implementation remains separately gated.
OpenMed install/use, model execution, provider calls, inference, PHI egress, Artifact publication, and implementation require separate authorization.
```

| Gate | Meaning for ADR-09 |
| --- | --- |
| T021 | Draft authoring (this task) |
| Tier A | Mandatory independent review |
| Founder draft acceptance | Not required by tasks.md for T021 work product |
| T030 | R1 gate; ADR-09 listed Reviewed or Accepted; not mandatory Accepted-for-R2 |
| T048 / T049 | DeepMed pin/pipeline after ADR-09 Accepted |
| T050 | Persist reviewed DeepMed→Fehrest after ADR-08/09/05 Accepted |
| T031+ / R2 / R3 | Separately gated; not authorized here |

---

## 10. Validation and acceptance plan

```text
T021 completion produces a draft for Tier A review.
Architecture acceptance is not performed by T021.
```

Planning reviews **may** include: Decision B / Q6 fidelity; package≠fork distinction; clinical-assistive non-authority; ADR-04/05/06 non-theft; ADR-07 local-first; ADR-08 handoff; prototype non-target claim; Tier A independent review ([tasks.md](../tasks.md) T021).

Verification method from tasks.md: doc review; link from plan roadmap (plan already contains ADR-09 row — **not** modified by T021).

---

## 11. Unresolved questions

T021 does **not** resolve these. Attempted resolution: **NO**.

| ID | Question | May remain open in draft review? | Must resolve before implementation? | Belongs elsewhere? |
| --- | --- | --- | --- | --- |
| U-ADR09-1 | DeepMed-AI repository and release ownership detail | YES | YES | This ADR + DeepMed release process |
| U-ADR09-2 | Standalone versus embedded runtime packaging | YES | YES | This ADR; P3 |
| U-ADR09-3 | Exact OpenMed PyPI package/version pin | YES | YES before R3 substrate use | This ADR + T048 |
| U-ADR09-4 | OpenMed replacement trigger | YES | YES | This ADR |
| U-ADR09-5 | Replacement timeline or authority | YES | YES | This ADR + founder/program gates |
| U-ADR09-6 | Model allowlist governance | YES | YES before “approved models” claims | Later DeepMed / policy specs |
| U-ADR09-7 | Model and tokenizer revision pinning format | YES | YES | This ADR |
| U-ADR09-8 | Local versus remote inference beyond local-default | YES | YES before remote paths | This ADR + ADR-14 |
| U-ADR09-9 | Remote-provider authorization | YES | YES if remote ever used | Separate authorization; ADR-14 |
| U-ADR09-10 | PHI-bearing remote inference | YES | YES | ADR-14 — default deny |
| U-ADR09-11 | Provider and model licensing inventory | YES | YES before bundling/redistribution | This ADR + legal review |
| U-ADR09-12 | Model download authority | YES | YES | This ADR + host |
| U-ADR09-13 | Runtime isolation profile detail | YES | YES | ADR-06 + this ADR |
| U-ADR09-14 | Model path custody | YES | YES | ADR-02 + this ADR |
| U-ADR09-15 | GPU/CPU policy | YES | YES before resource claims | Later impl |
| U-ADR09-16 | Resource limits | YES | YES | ADR-06 + this ADR |
| U-ADR09-17 | Prompt-template ownership | YES | YES | This ADR |
| U-ADR09-18 | Prompt versioning | YES | YES | This ADR |
| U-ADR09-19 | Prompt injection handling | YES | YES | This ADR + security review |
| U-ADR09-20 | Evidence insertion rules | YES | YES | This ADR + ADR-04 |
| U-ADR09-21 | Source-span validation method | YES | YES | This ADR + evaluation specs |
| U-ADR09-22 | Uncertainty representation format | YES | YES | This ADR + ADR-04 |
| U-ADR09-23 | Abstention behavior | YES | YES | This ADR |
| U-ADR09-24 | Refusal and emergency handling | YES | YES | This ADR + ADR-14 / safety specs |
| U-ADR09-25 | Human-review state machine | YES | YES | This ADR + ADR-04 Review/Approval |
| U-ADR09-26 | Clinician versus researcher review | YES | YES | Later product UX specs |
| U-ADR09-27 | Patient-facing output prohibition or boundary | YES | YES | Later product + safety specs |
| U-ADR09-28 | Artifact publication flow | YES | YES | ADR-05 + host |
| U-ADR09-29 | Run publication flow | YES | YES | ADR-04/05 + host |
| U-ADR09-30 | Fehrest handoff mechanics | YES | YES before T050 | ADR-08 + T050 |
| U-ADR09-31 | commandF handoff mechanics | YES | YES | ADR-10 |
| U-ADR09-32 | Retry and fallback | YES | YES | ADR-06 + this ADR — no silent sensitive retry |
| U-ADR09-33 | Timeout/cancellation | YES | YES | ADR-06 |
| U-ADR09-34 | Malformed-output handling | YES | YES | This ADR + ADR-06 |
| U-ADR09-35 | Evaluation protocols | YES | YES before release claims | Later DeepMed/eval specs / R3 |
| U-ADR09-36 | Release compatibility matrix fields | YES | YES | This ADR + P3 |
| U-ADR09-37 | Audit correlation identifiers | YES | YES | ADR-02 / later audit |
| U-ADR09-38 | Temporary-file handling | YES | YES | ADR-02/14 + this ADR |
| U-ADR09-39 | Output retention | YES | YES | ADR-05/14 |
| U-ADR09-40 | Model/provider failure quarantine | YES | YES | ADR-06 + this ADR |

---

## 12. Security and privacy claim language

```text
Claim language only.
No regulatory certification is asserted.
No DeepMed runtime, OpenMed install, model download, inference, or PHI egress is asserted as implemented.
PHI posture follows classification planning; egress engine detail remains ADR-14.
DeepMed is not proposed as clinical, FHIR, patient, approved-care, or PolicyDecision source of truth.
Unrestricted cloud-model PHI access remains forbidden.
```

---

## 13. References

- [tasks.md](../tasks.md) — T021, T007, T030, T048–T050
- [plan.md](../plan.md) — Decision B; ADR-09 roadmap row; R3 DeepMed sequencing
- [spec.md](../spec.md) — Q6; DeepMed rule; cross-repo boundaries
- [research.md](../research.md) — R7 / R10 / R13 / P3
- [deepmed-integration.md](../contracts/deepmed-integration.md) — ownership, Decision B checklist, Alpha result contract
- [R0-python-services.md](../../../docs/program-memory/baseline/R0-python-services.md) — T007 OpenMed prototype inventory
- [data-model.md](../data-model.md) — Run / Source / Review / Approval
- [shared-primitives.md](../contracts/shared-primitives.md) — consumers and minimum types
- [trusted-host-ipc.md](../contracts/trusted-host-ipc.md) — worker.spawn and host principles
- [worker-runtime.md](../contracts/worker-runtime.md) — non-Rust worker prohibitions
- [ADR-15](./ADR-15-rust-first-polyglot-runtime.md) (Proposed)
- [ADR-01](./ADR-01-platform-and-desktop-composition-tauri-2.md) (Proposed)
- [ADR-02](./ADR-02-rust-trusted-host-boundary.md) (Proposed)
- [ADR-03](./ADR-03-afia-ui-strangler-migration.md) (Proposed)
- [ADR-04](./ADR-04-shared-primitive-ownership-and-versioning.md) (Proposed)
- [ADR-05](./ADR-05-artifact-revision-run-storage.md) (Proposed)
- [ADR-06](./ADR-06-worker-and-ipc-contracts.md) (Proposed)
- [ADR-07](./ADR-07-supabase-adapter-and-local-first-boundary.md) (Proposed)
- [ADR-08](./ADR-08-fehrest-integration-and-release-model.md) (Proposed)

---

```text
While Proposed:
- no DeepMed implementation from this file;
- no DeepMed-AI initialization or modification;
- no OpenMed install, import, fork, or Rust rewrite;
- no model download, inference, training, RAG, or retrieval;
- no final IPC schema publication;
- no Artifact/Revision/Run durable mutation;
- no PHI egress;
- no clinical conclusion publication;
- no auth/session behavior change;
- no R2 / R3 work from this file alone.
```

```text
End of ADR-09 Proposed draft.
```
