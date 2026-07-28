# ADR-14 — Security and Data-Classification Enforcement

| Field | Value |
| --- | --- |
| **ADR** | ADR-14 |
| **Title** | Security and Data-Classification Enforcement |
| **Status** | **Reviewed** (T030 R1 architecture gate; not Accepted) |
| **Task origin** | T026 |
| **Acceptance / review posture** | Tier A — security/classification ADR after drafting. **T030** mandatory Accepted set is ADR-15/01/02/06 only; ADR-14 **is** listed in T030’s explicit Reviewed-or-Accepted group (04/05/07/08/09/10/14). Downstream **T038** requires ADR-14 **Accepted** for the classification enforcement seam; **T036** may use ADR-14 Reviewed or Accepted for audit-foundation planning; plan R3 entry lists ADR-14 among blocking ADRs; **T060** depends on T026 (+T025, T055). Exact Reviewed-versus-Accepted progression remains an explicit ambiguity where sources differ. |
| **Feature** | `001-fanatir-repository-and-architecture-reconstitution` |
| **Required gate (tasks.md)** | Decision C/D |
| **Planning dependency (plan.md)** | Constitution, **ADR-15**, Decision C/D — PHI egress + plugin permissions in Rust; signing custody for distribution (security-relevant overlap with Decision D / ADR-13) |
| **Dependencies (tasks.md)** | T012, T018, T019 |
| **Constraining ADRs** | [ADR-15](./ADR-15-rust-first-polyglot-runtime.md) (Accepted at T030); [ADR-02](./ADR-02-rust-trusted-host-boundary.md) (Accepted at T030); [ADR-04](./ADR-04-shared-primitive-ownership-and-versioning.md) (Reviewed at T030); [ADR-05](./ADR-05-artifact-revision-run-storage.md) (Reviewed at T030); [ADR-06](./ADR-06-worker-and-ipc-contracts.md) (Accepted at T030); [ADR-07](./ADR-07-supabase-adapter-and-local-first-boundary.md) (Reviewed at T030); [ADR-08](./ADR-08-fehrest-integration-and-release-model.md) (Reviewed at T030); [ADR-09](./ADR-09-deepmed-integration-and-openmed-runtime-fork-boundary.md) (Reviewed at T030); [ADR-10](./ADR-10-commandf-ownership-and-process-boundary.md) (Reviewed at T030); [ADR-11](./ADR-11-auth-and-session-preservation.md) (Proposed); [ADR-12](./ADR-12-technical-afia-to-fanatir-rename-strategy.md) (Proposed); [ADR-13](./ADR-13-first-vertical-slice-packaging.md) (Proposed) |
| **Related ADRs** | [ADR-01](./ADR-01-platform-and-desktop-composition-tauri-2.md) (Accepted at T030); [ADR-03](./ADR-03-afia-ui-strangler-migration.md) (Proposed) |
| **Planning evidence** | [spec.md](../spec.md) PHI posture / Q4 / exclusions; [plan.md](../plan.md) ADR-14 row / Decision C/D / secure-share / PHI-egress; [research.md](../research.md) Decision C / documents-crypto; Constitution Security and Privacy Doctrine ([constitution.md](../../../.specify/memory/constitution.md)); [data-model.md](../data-model.md) DataClassification / PolicyDecision / ExportManifest; [shared-primitives.md](../contracts/shared-primitives.md); [trusted-host-ipc.md](../contracts/trusted-host-ipc.md); [worker-runtime.md](../contracts/worker-runtime.md); [tasks.md](../tasks.md) T026 / T012 / T018 / T019 / T030 / T036 / T038 / T054 / T060 |
| **T030 founder decision** | **Reviewed** — R1 architecture gate; not Accepted; does **not** authorize classified-data or PHI implementation; T028 freeze preserved |
| **Architecture authority of this file** | **NO** — Reviewed at T030; Accepted required before classified-data / PHI-egress implementation |
| **Implementation authorization** | **NO** |
| **PHI / classification / egress / audit authorization** | **NO** — Reviewed status does not authorize PHI handling |

```text
Status: Reviewed (T030 R1 architecture gate; not Accepted)
Architecture authority: NO (Reviewed ≠ Accepted)
Implementation authorization: NO
PHI / classified-data / egress implementation: NOT authorized
T028 documents-crypto freeze: preserved
Independent Tier A — R1 architecture gate: Pending
```

```text
Draft ADR ≠ architecture acceptance
T026 completion ≠ T030 acceptance (T030 founder Reviewed decision now recorded)
Passing Tier A review of a draft ≠ Accepted
Decision C/D ≠ permission to transmit PHI or configure signing keys
Reviewed ≠ Accepted for classified-data or PHI-egress implementation
ADR-03 / ADR-11 / ADR-12 / ADR-13 remain Proposed
```

```text
This draft is not:
- permission to classify production data or handle real PHI
- permission to export, transmit, upload, synchronize, email, print, or copy data
- permission to connect external providers or invoke models
- permission to invoke commandF or terminology services
- permission to configure Supabase for PHI
- permission to implement DataClassification, PolicyDecision, Approval, or audit
- permission to implement encryption, retention, deletion, or de-identification
- permission to implement consent or break-glass flows
- permission to modify UI, authentication, Artifact Store, workers, IPC, packaging, or releases
- permission to claim HIPAA, PDPL, GDPR, or other certification/compliance
- permission to begin R2, R3, or R5
```

```text
No production implementation is authorized by this draft.
```

This draft **must not** be used as justification to: classify production data; handle or transmit real PHI; export; connect providers; invoke models or commandF; configure Supabase for PHI; implement PolicyDecision, Approval, audit, encryption, retention, deletion, or de-identification; modify [`afia-ui`](../../../afia-ui); change authentication or sessions; modify Artifact Store, workers, Fehrest, DeepMed, or packaging; make certification claims; begin R2 (`T031+`), R3, or R5; or treat Proposed ADRs as Accepted.

---

## 1. Context and problem

### 1.1 Assigned architectural question

**ADR-14 proposes** a **security and data-classification enforcement** strategy for Spec **001**: how Fanatir would enforce `DataClassification` (including **PHI** and **secrets**), **default-deny egress**, Capability Gateway / Trusted Host authorization, minimum-necessary handling, no-PHI logging, provider gates, and claim-language discipline — constrained by Decision C (local-first Artifact SoT; Supabase not PHI SoT) and Decision D (signing custody security overlap) — defining **enforcement semantics and gates only**.

It does **not** implement enforcement; publish final schemas (**ADR-04** owns shared structures); classify real data; transmit PHI; configure providers or Supabase; implement audit/encryption/retention; redesign UI (**ADR-03**) or auth (**ADR-11**); or authorize R2/R3/R5.

### 1.2 Why classification and egress architecture before external connectivity

Constitution Security and Privacy Doctrine requires explicit classification, egress control, no-PHI logging, immutable audit events, and destination declaration ([constitution.md](../../../.specify/memory/constitution.md)). Decision C moves content/PHI authority to the local Rust Artifact Store and forbids default cloud PHI ([plan.md](../plan.md); [research.md](../research.md)). Plan and tasks require PHI-egress denial tests and deny-by-default for `phi` egress ([tasks.md](../tasks.md) T038). Without an explicit enforcement boundary, planning identifies these **risks** (prospective — not claimed as currently measured production failures unless separately evidenced):

| Risk | Why it matters |
| --- | --- |
| Login treated as authorization | Capability Gateway bypass (**ADR-11**) |
| Local Artifact access treated as cloud egress permission | Decision C / egress failure |
| UI or workers self-authorize | UI-zero-authority / ADR-15 violation |
| Encrypted transport mistaken for authorized PHI egress | False safety |
| PHI written into ordinary logs or crash reports | Constitution no-PHI logging violation |
| De-identification detection treated as safe egress | Residual identification risk |
| Default Supabase PHI sync | Decision C / Q4 violation |
| Export without destination/purpose review | Uncontrolled disclosure |
| Classification lost across IPC | Worker/provider leakage |
| PHI or credentials in packages/support bundles | **ADR-13** packaging risk |
| Overstated legal compliance | Constitution forbidden claim language |

### 1.3 Accepted program and planning constraints

| Constraint | Source | Treatment |
| --- | --- | --- |
| Content/PHI/Artifact SoT → local Rust Artifact Store; Supabase optional collab/identity; freeze `documents-crypto`; no PHI by default | Decision C — [plan.md](../plan.md); [research.md](../research.md); [spec.md](../spec.md) Q4 | Binding; **not** ADR-14 acceptance |
| Signing custody before R5 distribution; unsigned = internal only | Decision D | Security-relevant overlap; **not** key creation under T026 |
| Explicit data classification; egress control; no-PHI logging; immutable audit; destination declaration; claim-language limits | Constitution Security and Privacy Doctrine | Binding doctrine; **not** ADR-14 acceptance |
| DataClassification minimum values: `public`, `internal`, `restricted`, `phi`, `secrets`; `phi` forbids default cloud egress; host enforces | [data-model.md](../data-model.md) | Minimum planning values; **not** final schema |
| Deny-by-default for phi egress (future T038) | [tasks.md](../tasks.md) T038 | Downstream; not authorized here |
| T026: ADR draft only; Decision C/D gate; draft ≠ accepted; no production code | [tasks.md](../tasks.md) T026 | Binding for this task |

### 1.4 Naming

| Name | Role in this ADR |
| --- | --- |
| **DataClassification** | Shared sensitivity label type — structure owned by **ADR-04**; enforcement semantics proposed here |
| **PHI** | Protected or potentially identifiable health information; maps to `phi` planning value |
| **PolicyDecision** | Host-issued authorization outcome — structure **ADR-04**; issuance **ADR-02** |
| **Approval** | Human approval evidence — distinct from PolicyDecision (**ADR-04**) |
| **Trusted Host** / **Capability Gateway** | Enforcement owners (**ADR-02** / **ADR-15**) |
| **Artifact Store** | Local classified content SoT (**ADR-05**) |
| **AuthContext** | Identity context (**ADR-11**) — not authorization |
| **Fehrest** / **DeepMed** / **commandF** | Product/capability boundaries for egress |

Never confuse classification with access control, consent, de-identification, or legal compliance status.

### 1.5 What this draft is not

This file is **not**: an implemented security control; a final `DataClassification` schema; a PolicyDecision wire format; provider allowlist configuration; audit system; encryption deployment; retention schedule; legal certification; or R2/R3/R5 authorization.

---

## 2. Decision (proposed)

### 2.1 Core enforcement decision

**Propose** that:

1. Every governed data-bearing operation must carry or resolve a `DataClassification`.
2. Missing, unknown, conflicting, or invalid classification **fails closed** for privileged or egress operations.
3. `phi` **forbids default cloud egress**.
4. `secrets` are distinct from PHI but receive highly restricted treatment.
5. The **Trusted Host** and **Capability Gateway** own enforcement.
6. UI and workers may request operations but may **not** authorize them.
7. Access to an Artifact and permission to **egress** its content are **separate** decisions.
8. **Minimum necessary** applies to worker payloads, provider requests, logs, exports, and audit.
9. Unknown destination, purpose, provider, approval, classification, or policy state **defaults to denial**.
10. External-provider use requires explicit allowlisting and separate evaluation of purpose, retention, training use, destination, and audit.
11. **Supabase** is not the PHI source of truth and receives **no PHI by default**.
12. **No unrestricted cloud-model PHI** is allowed.
13. **commandF** network terminology, external validation, live-system access, export, and publication remain separately gated.
14. **DeepMed** provider/model egress remains separately gated.
15. **Fehrest** vault exports remain governed and product-separated.
16. No PHI or secrets may enter ordinary logs, crash reports, packages, support bundles, UI assets, or commit history.
17. Claim language such as “HIPAA-aligned design” may describe design intent only when safeguards are documented; the system must **not** claim certification or compliance without evidence.
18. Real enforcement implementation requires Accepted architecture and separately authorized specifications and tasks.

### 2.2 Classification semantics (proposed)

Minimum planning values from [data-model.md](../data-model.md) — **not** a final published schema:

| Value | Planning meaning |
| --- | --- |
| `public` | Information intended for unrestricted release, still subject to provenance, licensing, and publication authority |
| `internal` | Non-public operational or project information with bounded internal use |
| `restricted` | Sensitive information requiring explicit access and egress controls but not necessarily PHI or secrets |
| `phi` | Protected or potentially identifiable health information; default-deny external egress; strict minimum-necessary handling |
| `secrets` | Credentials, signing keys, tokens, encryption keys, recovery material, and comparable security-sensitive values |

**Explicitly:**

- classification ≠ access control;
- classification ≠ consent;
- classification ≠ de-identification;
- classification ≠ proof of legal status;
- inheritance/aggregation rules may be required later but remain **unresolved**.

**Ownership:** **ADR-04** owns the shared `DataClassification` type and structure; **ADR-14** proposes enforcement semantics; **T027** may align planning schemas later; this draft does **not** publish final wire, storage, or API schemas; no additional classification label is invented without evidence.

### 2.3 Classification resolution (proposed)

Future resolution order (principles only):

1. authoritative Artifact or source classification;
2. explicitly governed transformation result;
3. policy-approved derivative classification;
4. fail-closed unknown state.

Address (without final algorithms): conflicting classifications; mixed-content bundles; nested FHIR resources; derived artifacts; transformed outputs; model outputs; temporary files; exports; logs.

### 2.4 Trusted Host and Capability Gateway (proposed)

Preserve **ADR-02** / **ADR-15**:

- Trusted Host owns privileged decisions;
- Capability Gateway evaluates requested operations;
- UI intent is untrusted input;
- worker requests are bounded proposals;
- workers cannot self-authorize network, export, provider, filesystem, or secret access;
- classification enforcement occurs before privileged execution;
- enforcement must not be delegated to frontend route visibility.

Do **not** implement capabilities under T026.

### 2.5 PolicyDecision, Approval, and ADR-04 (proposed)

Preserve **ADR-04** ownership of shared structures.

ADR-14 may propose that future policy evaluation considers (semantic planning only — **no final fields published**): classification; operation; purpose; destination; provider; user or actor; workspace or scope; minimum-necessary fields; approval evidence; reviewer identity; expiration; audit correlation; reason; denial state.

**Explicitly:** `Approval` ≠ `PolicyDecision`; user confirmation ≠ automatic policy approval; login ≠ authorization.

### 2.6 Artifact Store (proposed)

Preserve **ADR-05** / Decision C:

- classified content remains in the local Rust Artifact Store;
- Artifact access and egress are distinct decisions;
- classification metadata must not silently disappear across revisions;
- derived artifacts require classification evaluation;
- export does not mutate canonical content;
- search and indexing must respect classification;
- deletion and retention remain separately specified;
- Supabase rows do not grant local Artifact authority.

Do **not** alter storage behavior.

### 2.7 Worker and IPC (proposed)

Preserve **ADR-06** / **ADR-15**:

- workers receive minimum-necessary bounded payloads;
- classification metadata accompanies relevant operations;
- workers receive no ambient PHI, filesystem, network, secret, or export authority;
- stdout/stderr, logs, temporary files, and crash dumps must not expose PHI or secrets;
- external subprocesses require separate capability review;
- unknown worker classification support fails closed;
- cleanup, timeout, cancellation, and crash behavior remain implementation concerns.

Do **not** modify IPC or workers.

### 2.8 UI boundary (proposed)

Preserve **ADR-03**. Future UI responsibilities may include: display classification; show destination and purpose; preview intended disclosure; present redaction/de-identification status; explain risk; collect user intent where required; show denial reasons; accessible warnings; keyboard and screen readers; Arabic/RTL.

**Explicitly:** UI does not issue PolicyDecision; UI confirmation does not bypass policy; **T026 modifies no UI**.

### 2.9 Authentication and session (proposed)

Preserve **ADR-11**:

- identity/session context may inform policy;
- login alone authorizes nothing;
- route visibility authorizes nothing;
- session tokens are `secrets`;
- profile data is not identity proof;
- reviewer identity and approval provenance may be distinct;
- no auth/session behavior changes under T026.

### 2.10 Supabase (proposed)

Preserve **ADR-07** and Decision C:

- Supabase is optional collaboration/identity infrastructure;
- Supabase is **not** the Artifact or PHI source of truth;
- PHI is **not** synchronized by default;
- existing tables or RLS do not imply PHI approval;
- future classified collaboration metadata would require a **dedicated specification** (including future Spec **002** where applicable);
- remote rows do not grant local authority;
- network loss does not relax policy;
- **no Supabase mutation** under T026.

**Canonical posture for PHI on Supabase:** **prohibited by default**; any future exception is **deferred / unresolved** and not authorized by this draft. Do not invent approval from inventory tables. Legacy `documents-crypto` remains a frozen PHI-egress seam (synthetic/test only) until ADR-07/14 stage gates and dedicated work authorize otherwise — **T026 does not mutate it**.

### 2.11 Fehrest (proposed)

Preserve **ADR-08**: Fehrest remains a separate product and repository; vault content may be sensitive; embedded and standalone exports require classification and destination review; metadata may itself be sensitive; Fanatir cannot silently synchronize Fehrest vault content; no Fehrest mutation or export under T026. Use exactly `Fehrest`.

### 2.12 DeepMed (proposed)

Preserve **ADR-09**: prompts, patient context, evidence payloads, model outputs, and provider metadata require classification; local and remote models have different egress implications; PHI detection or redaction does not itself authorize provider use; provider retention and training-use policies require review; reviewer approval and provenance remain separately governed; **no model or provider invocation** under T026. No unrestricted cloud-model PHI.

### 2.13 commandF (proposed)

Preserve **ADR-10** and the accepted FHIR workbench strategy (non-implementing):

- FHIR resources and bundles may contain PHI;
- local validation does not automatically require egress;
- external validator, terminology server, FHIR server, live-system, export, and publication operations require classification and policy evaluation;
- terminology requests may disclose patient or workflow context;
- validator output and transformed artifacts require classification;
- successful validation is not authorization to transmit;
- no live-system writes; no external validation; no PHI egress; no commandF implementation under T026.

Founder-accepted long-term FHIR direction does **not** alter ADR-10 and does **not** authorize PHI egress or packaging of validators/terminology.

### 2.14 Provider and external-service boundary (proposed)

Future providers (model APIs; terminology services; FHIR servers; email; cloud storage; telemetry; crash reporting; update services; authentication providers; collaboration services) would require, where applicable: explicit allowlisting; destination identity; purpose limitation; minimum necessary; retention review; training-use review; regional or cross-border review; contract/DPA review; capability decision; user or organizational approval where required; audit correlation; failure and revocation behavior.

Do **not** select providers under T026.

### 2.15 De-identification and redaction (proposed)

Distinguish: identification detection; redaction; pseudonymization; tokenization; de-identification; anonymization; synthetic data.

**Propose:** de-identification does not automatically make egress safe; transformations may be reversible; residual-identification risk remains; verification and provenance are required; classification reduction requires explicit governed evidence; **no safe-harbor guarantee is claimed**; implementation remains separate.

### 2.16 Consent and approval (proposed)

Future relationships among user intent, patient consent, clinician approval, reviewer approval, organizational policy, administrator approval, emergency override, break-glass, and dual approval remain **unresolved**.

**Propose:** no final consent workflow is selected; no break-glass behavior is authorized; approval must be scoped, attributable, expiring where appropriate, and auditable; consent does not replace capability policy; exact requirements remain legally and operationally dependent.

### 2.17 Audit and logging (proposed)

Preserve **ADR-02** authoritative audit ownership.

Future audit coverage may include: classification resolution; denied/approved egress; exports; provider requests; commandF operations; DeepMed invocations; Supabase synchronization; approvals; deletion; key use; incidents.

**Propose:** audit logs must minimize PHI; secrets must never be logged; denied operations should record bounded reasons without sensitive payloads; audit schema and retention remain unresolved; “immutable” is a **requirement**, not a current implementation claim; audit failure must not silently authorize egress.

### 2.18 Secrets (proposed)

API keys; signing keys; Supabase credentials; session and refresh tokens; encryption keys; recovery codes; provider credentials; webhook secrets:

- secrets are not necessarily PHI;
- secrets remain highly restricted (`secrets` classification);
- no secrets in source, ordinary CI, logs, Artifacts, exports, UI assets, or packages;
- no secure-storage backend or key-management provider is selected;
- Decision D signing custody remains relevant but separately implemented (**ADR-13** / **T060**).

### 2.19 Encryption (proposed)

Planning requirements may include encryption in transit and at rest; backups; temporary files; exports; local databases; provider transport; key rotation; revocation; recovery.

**Propose:** encryption does not replace authorization; encryption does not lower classification automatically; algorithms, key stores, field-level encryption, and IPC encryption remain **unresolved**; no encryption implementation under T026.

### 2.20 Retention and deletion (proposed)

Patient content; research data; derived artifacts; logs; audit; exports; backups; temporary files; caches; collaboration rows; provider records; prompts and outputs:

- exact periods remain unresolved or policy/legal dependent;
- deletion must consider backups, exports, derived artifacts, audit, and provider retention;
- no retention schedule is invented;
- no deletion implementation under T026.

### 2.21 Export, sharing, and publication (proposed)

Distinguish: export; publication; sharing; synchronization; backup; clipboard; printing; screenshot; external link.

Future requirements may include: classification; destination; recipient; purpose; minimum necessary; preview; redaction; encryption; provenance; approval; expiry; revocation; audit (Constitution secure-share; plan ExportManifest).

**Propose:** a local file export can still be egress; publication requires stronger authority; **no export under T026**. Social sharing must never appear for PHI or restricted data unless an authorized, reviewed public derivative exists (Constitution).

### 2.22 Incident and failure behavior (proposed)

Missing/ambiguous/conflicting classification; unavailable policy service; unknown destination; failed redaction; provider timeout; partial upload; audit failure; revoked approval; compromised credential; corrupted export; offline operation; worker crash:

- privileged/egress operations **default deny**;
- partial operations must not be treated as success;
- uncertain state must not widen authority;
- incident handling, revocation, cleanup, and notification require later specifications.

### 2.23 Packaging and distribution (proposed)

Preserve **ADR-13**: installers, packages, logs, crash reports, support bundles, updates, and release artifacts must not contain PHI or secrets; unsigned/internal artifacts do not relax data controls; signing does not authorize PHI handling; package telemetry or update checks are egress surfaces; **T026 changes no packaging behavior**.

### 2.24 Legal and compliance claims (proposed)

**Explicitly:**

- Fanatir is **not** claiming HIPAA certification;
- Fanatir is **not** claiming legal safe-harbor de-identification;
- Fanatir is **not** claiming GDPR, Saudi PDPL, Australian Privacy Act, or other compliance;
- “HIPAA-aligned” or “PDPL-aware” may describe **design intent only** when specific safeguards are documented (Constitution permitted claim language);
- legal analysis is required before deployment claims and may vary by jurisdiction, controller, processor, purpose, and transfer;
- **ADR-14 is not legal advice or certification**.

Do not invent legal conclusions.

### 2.25 Verification strategy (proposed)

Later verification categories may include: classification examples; mixed-data cases; missing/unknown classification denial; PHI cloud-egress denial; provider allowlist denial; minimum-necessary payload inspection; worker IPC inspection; log/secret scanning; **synthetic** PHI fixture scanning; export preview and denial; offline behavior; provider mocks; audit behavior; retention/deletion behavior; package/support-bundle scans; commandF terminology/provider denial; DeepMed provider denial; Supabase PHI denial.

Use synthetic fixtures only. **Do not claim any were executed under T026.** T026 verification method: doc review; link from plan roadmap (plan already contains ADR-14 row — **not** modified by T026).

### 2.26 Rollback (proposed)

| Layer | Status |
| --- | --- |
| Revert ADR-14 draft | Available — tasks.md T026 |
| Disable egress / revoke provider / revoke credentials / invalidate approval | Later requirements |
| Delete export / restore classification / revert policy / purge logs / restore backups | Later requirements |
| External provider already received data | May **not** be fully reversible |

Only **reverting the ADR file** is currently available under T026. Runtime rollback mechanisms are requirements, not implemented capabilities.

---

## 3. Alternatives considered

| Option | Rationale (evidence-based; not claimed as tested) |
| --- | --- |
| A. Trust UI warnings alone | Violates UI-zero-authority |
| B. Permit provider egress whenever transport is encrypted | Encryption ≠ authorization |
| C. Classify only PHI; ignore secrets/restricted | Incomplete Constitution doctrine |
| D. Permit Supabase PHI by default | Violates Decision C / Q4 |
| E. Rely on worker/provider self-enforcement | Violates ADR-15 / ADR-02 |
| F. Block all external use permanently | Over-blocks Alpha collab/optional adapters without evidence |
| **G (proposed).** Host-enforced classification; default-deny PHI egress; explicit provider gates; claim discipline | Aligns Decision C/D, Constitution, data-model, plan ADR-14 |

---

## 4. Consequences

### 4.1 Prospective benefits

- explicit PHI boundary;
- local-first authority;
- default-deny egress;
- bounded provider use;
- reduced log leakage;
- worker isolation;
- consistent Artifact and export treatment;
- clearer audit direction;
- claim discipline;
- commandF / DeepMed / Supabase boundaries;
- safer packaging posture.

### 4.2 Costs and risks

- workflow friction;
- false positives and false negatives;
- classification ambiguity;
- provider-review burden;
- audit complexity;
- de-identification uncertainty;
- consent/legal variability;
- offline constraints;
- performance overhead;
- user-understanding burden;
- unresolved schemas;
- incomplete retention rules;
- encryption/key-management debt;
- policy drift risk;
- denying legitimate work risk;
- incomplete egress enumeration risk;
- T030 Reviewed-or-Accepted vs T038 Accepted progression ambiguity.

### 4.3 Planning-only rollback

Per [tasks.md](../tasks.md) T026: **Revert ADR file**. No production enforcement surface is created by this draft.

---

## 5. Non-goals

ADR-14 does **not**:

- classify real data;
- transmit PHI;
- implement PolicyDecision;
- publish a final DataClassification schema;
- configure providers;
- configure Supabase for PHI;
- implement audit;
- implement encryption;
- implement retention/deletion;
- implement de-identification;
- implement consent/break-glass;
- modify UI;
- modify authentication;
- modify Artifact Store;
- modify IPC or workers;
- modify Fehrest;
- invoke DeepMed;
- invoke commandF;
- change packaging;
- make compliance claims;
- begin R2, R3, or R5;
- execute T027, T028, T029, or T030.

---

## 6. Relationship to other ADRs

| ADR | Status in repo | Relationship |
| --- | --- | --- |
| ADR-15 | Proposed | Rust trusted core; supervised workers; no worker PHI-egress ownership |
| ADR-01 | Proposed | Platform composition |
| ADR-02 | Proposed | Capability Gateway; PolicyDecision issuance; authoritative audit |
| ADR-03 | Proposed | UI presentation; no authorization |
| ADR-04 | Proposed | Shared DataClassification / PolicyDecision / Approval structures |
| ADR-05 | Proposed | Classified Artifact persistence |
| ADR-06 | Proposed | Worker/IPC payloads; minimum necessary |
| ADR-07 | Proposed | Supabase/local-first; documents-crypto freeze |
| ADR-08 | Proposed | Fehrest vault/export governance |
| ADR-09 | Proposed | DeepMed/provider egress |
| ADR-10 | Proposed | commandF/FHIR/terminology egress; strategy unchanged |
| ADR-11 | Proposed | Identity/session context; not auth redesign |
| ADR-12 | Proposed | Naming; no PHI transmission via rename |
| ADR-13 | Proposed | Packaging must not ship PHI/secrets; Decision D overlap |

ADR-14 decides only **Security and Data-Classification Enforcement**.

---

## 7. Gates and authority

```text
T026 produces a Proposed ADR-14 draft only.
Tier A — security/classification ADR is required.
Passing Tier A review is not architecture acceptance.
tasks.md founder-acceptance field for T026 is: No — draft only.
Founder draft acceptance is not required by the T026 contract for the draft work product.
ADR-14 alone does not authorize classification, egress, provider, PolicyDecision, audit, encryption, retention, packaging, or deployment implementation.
T030 remains incomplete.
T030 mandatory Accepted set is ADR-15/01/02/06 only — ADR-14 is not in that set.
T030 explicit Reviewed-or-Accepted group lists ADR-04/05/07/08/09/10/14 — ADR-14 is listed there.
T038 requires ADR-14 Accepted for the classification enforcement seam.
T036 may use ADR-14 Reviewed or Accepted for audit-foundation planning.
Plan R3 entry lists ADR-14 among blocking ADRs.
T060 depends on T026, T025, and T055.
Exact Reviewed-versus-Accepted progression remains an explicit ambiguity where sources differ.
T031+ and R2 remain separately gated.
R3, R5, and production security work remain separately gated.
```

| Gate | Meaning for ADR-14 |
| --- | --- |
| T026 | Draft authoring (this task) |
| Tier A | Mandatory independent review |
| Founder draft acceptance | Not required by tasks.md for T026 work product |
| T030 | R1 gate; ADR-14 in Reviewed-or-Accepted list; not mandatory Accepted set |
| T036 | Audit foundation; ADR-14 Reviewed or Accepted |
| T038 | Classification seam; ADR-14 Accepted |
| T054 | Secure share; ADR-14 Accepted (tasks.md) |
| T060 | Signing custody; depends on T026 |
| T031+ / R2 / R3 / R5 | Separately gated; not authorized here |

---

## 8. Validation and acceptance plan

```text
T026 completion produces a draft for Tier A review.
Architecture acceptance is not performed by T026.
PHI/classification/egress/audit implementation is not authorized by T026.
```

Planning reviews **may** include: Decision C/D fidelity; default-deny `phi` egress; ADR-04 structure non-theft; UI/worker non-authority; Supabase no-PHI-by-default; commandF/DeepMed/Fehrest gates; claim-language discipline; T030/T036/T038 ambiguity honesty; Tier A independent review ([tasks.md](../tasks.md) T026).

Verification method from tasks.md: doc review; link from plan roadmap (plan already contains ADR-14 row — **not** modified by T026).

---

## 9. Unresolved questions

T026 does **not** resolve these. Attempted resolution: **NO**.

| ID | Question | May remain open in draft review? | Must resolve before implementation? | Belongs elsewhere? |
| --- | --- | --- | --- | --- |
| U-ADR14-1 | Final DataClassification schema beyond minimum planning values | YES | YES | ADR-04; T027 |
| U-ADR14-2 | Classification inheritance rules | YES | YES | ADR-04/05; dedicated security spec |
| U-ADR14-3 | Mixed-content resolution algorithm | YES | YES | ADR-04/05; dedicated security spec |
| U-ADR14-4 | Who may set or lower classification | YES | YES | ADR-02; dedicated security spec |
| U-ADR14-5 | Classification provenance | YES | YES | ADR-04/05 |
| U-ADR14-6 | Final PolicyDecision fields | YES | YES | ADR-04; ADR-02 |
| U-ADR14-7 | Final Approval fields | YES | YES | ADR-04 |
| U-ADR14-8 | Provider allowlist ownership | YES | YES | ADR-02; dedicated security spec |
| U-ADR14-9 | Destination representation | YES | YES | ADR-04; dedicated security spec |
| U-ADR14-10 | Purpose vocabulary | YES | YES | Dedicated security spec |
| U-ADR14-11 | Minimum-necessary representation | YES | YES | ADR-04/06; dedicated security spec |
| U-ADR14-12 | Supabase classified metadata exceptions | YES | YES before any exception | ADR-07; Spec 002 |
| U-ADR14-13 | Absolute vs default Supabase PHI prohibition | YES | YES before any PHI sync | ADR-07; Decision C |
| U-ADR14-14 | Local-model handling | YES | YES | ADR-09; ADR-02 |
| U-ADR14-15 | Remote-model handling | YES | YES | ADR-09; this ADR |
| U-ADR14-16 | Terminology-request disclosure rules | YES | YES | ADR-10; this ADR |
| U-ADR14-17 | FHIR-server access | YES | YES | ADR-10; dedicated interoperability spec |
| U-ADR14-18 | commandF external validation | YES | YES | ADR-10; this ADR |
| U-ADR14-19 | De-identification verification standard | YES | YES | Dedicated security/privacy spec |
| U-ADR14-20 | Classification downgrade evidence | YES | YES | Dedicated security spec |
| U-ADR14-21 | Consent model | YES | YES | Legal + dedicated privacy spec |
| U-ADR14-22 | Break-glass | YES | YES before enabling | Dedicated security spec |
| U-ADR14-23 | Dual approval | YES | YES before enabling | Dedicated security spec |
| U-ADR14-24 | Audit schema | YES | YES | ADR-02; T036 |
| U-ADR14-25 | Audit retention | YES | YES | ADR-02; legal |
| U-ADR14-26 | Log redaction rules | YES | YES | ADR-02; this ADR |
| U-ADR14-27 | Encryption mechanisms | YES | YES | ADR-02/05; dedicated security spec |
| U-ADR14-28 | Key custody (non-signing) | YES | YES | ADR-02; Decision D overlap for signing |
| U-ADR14-29 | Retention periods | YES | YES | Legal/policy; dedicated privacy spec |
| U-ADR14-30 | Deletion across backups/providers | YES | YES | ADR-05; dedicated privacy spec |
| U-ADR14-31 | Export encryption | YES | YES | ADR-05; ExportManifest; T054 |
| U-ADR14-32 | Clipboard/print/screenshot controls | YES | YES | ADR-03; dedicated security/UI spec |
| U-ADR14-33 | Telemetry/crash reporting | YES | YES | ADR-13; this ADR |
| U-ADR14-34 | Packaging/support-bundle scans | YES | YES | ADR-13; R5 |
| U-ADR14-35 | Legal jurisdictions and DPA requirements | YES | YES before deployment claims | Legal; outside draft certification |
| U-ADR14-36 | Incident response playbooks | YES | YES before production | Dedicated security ops spec |
| U-ADR14-37 | T030 Reviewed-or-Accepted vs T038 Accepted progression | YES | YES before claiming those gates | tasks.md; stage gates |
| U-ADR14-38 | Signing-custody security overlap with ADR-13/T060 | YES | YES before R5 distribution | Decision D; ADR-13; T060 |

---

## 10. Security and privacy claim language

This draft **proposes** enforcement-strategy principles only. It does **not** claim HIPAA certification, HIPAA compliance, PDPL/GDPR compliance, legal safe-harbor de-identification, production encryption, implemented audit, configured allowlists, approved providers, secure Supabase PHI storage, implemented retention, complete egress enumeration, or that ADR-14 is Accepted.

Permitted design-intent language (Constitution): “designed with HIPAA-aligned safeguards”; “PDPL-aware”; “local-first privacy architecture” — only when safeguards are documented. Forbidden without evidence: “HIPAA certified”; “fully compliant”; “clinically safe”.

No real PHI examples are embedded in this draft.

---

## 11. Document control

| Item | Value |
| --- | --- |
| Created by | T026 draft authoring |
| Status | Reviewed (T030 R1 architecture gate; not Accepted) |
| Supersedes | None |
| Superseded by | None |
| Next expected actions | Tier A R1 gate review of T030 recording; T036/T038 only under their ADR-14 state requirements; no real PHI handling under Spec 001 without later authorization |

```text
End of ADR-14.
Status: Reviewed (T030 R1 architecture gate; not Accepted)
Implementation authorization: NO
PHI / classification / egress / audit authorization: NO
```
