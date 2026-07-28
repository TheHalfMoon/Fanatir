# Future Specification 002 Charter — Supabase Local-First and Migration Canonicalization

| Field | Value |
| --- | --- |
| **Document type** | Future-specification charter (program-memory) |
| **Task origin** | T029 |
| **Stage** | R1 |
| **Canonical future spec name** | `002-supabase-local-first-and-migration-canonicalization` |
| **Human-readable title** | Supabase Local-First and Migration Canonicalization |
| **Related founder decision** | Decision C (**Ratified** in [plan.md](../../../specs/001-fanatir-repository-and-architecture-reconstitution/plan.md)) |
| **Dependencies** | T019 (ADR-07 draft); T028 ([documents-crypto-freeze.md](./documents-crypto-freeze.md)) |
| **Task authority** | [tasks.md](../../../specs/001-fanatir-repository-and-architecture-reconstitution/tasks.md) T029 |
| **Companion planning** | [supabase-adapter.md](../../../specs/001-fanatir-repository-and-architecture-reconstitution/contracts/supabase-adapter.md); [ADR-07](../../../specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-07-supabase-adapter-and-local-first-boundary.md) (**Proposed**) |

```text
Status: Future Spec 002 charter recorded (T029 documentation)
This document is a charter only — it is not Spec 002
Spec 002 has not been created
No Spec Kit command has been run
This document does not accept ADR-07 or ADR-14
This document does not authorize migrations, Supabase schema/runtime changes, or real PHI
This document does not complete T030 and does not authorize R2
tasks.md founder-acceptance for T029: No
Tier C — normal verification is required before any commit of this work
```

---

## 1. Charter status and authority

| Field | Value |
| --- | --- |
| **Task** | T029 |
| **Stage** | R1 |
| **Artifact type** | Future-specification charter |
| **Canonical future spec name** | `002-supabase-local-first-and-migration-canonicalization` |
| **Decision gate** | Decision C |
| **Dependencies** | T019; T028 |
| **Review tier** | Tier C — normal verification |
| **Founder work-product acceptance** | No |
| **Spec 002 creation status** | **Not created** |
| **Implementation authority** | None |
| **Migration authority** | None |
| **R2 authority** | None |

This charter is non-authoritative planning evidence. It is **not** an Accepted ADR, accepted specification, implementation plan, migration plan, architecture acceptance, or PHI approval.

---

## 2. Problem statement

Repository evidence shows Supabase-related paths, dual migration trees, and legacy seams (including `documents-crypto`) that require a **single future canonical specification** before migration or adapter implementation.

Today:

- local versus remote authority boundaries are recorded across Decision C, [ADR-07](../../../specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-07-supabase-adapter-and-local-first-boundary.md) (**Proposed**), and the [supabase-adapter](../../../specs/001-fanatir-repository-and-architecture-reconstitution/contracts/supabase-adapter.md) planning contract;
- legacy paths and migrations are inventory evidence only — their existence does **not** make them accepted architecture;
- Decision C requires local-first authority and **no PHI synchronization by default**;
- [T028](./documents-crypto-freeze.md) freezes `documents-crypto` rather than resolving final disposition;
- structured planning for migration intent, disposition, compatibility, and safety is deferred to a future specification named here.

This charter does **not** claim current migrations are valid, invalid, safe, unsafe, active, or production-approved.

---

## 3. Future specification mission

When separately authorized, Spec **`002-supabase-local-first-and-migration-canonicalization`** is expected to establish a governed planning and (later) implementation boundary for:

- local-first authority;
- Supabase identity and collaboration scope;
- Artifact Store authority versus remote metadata;
- migration canonicalization;
- legacy schema and path inventory;
- compatibility;
- phased disposition;
- data-classification preservation;
- egress controls;
- rollback and verification;
- offline behavior;
- security and audit expectations.

**This charter does not settle these items.**

---

## 4. In-scope questions for future Spec 002

Future Spec 002 (not this charter) should address questions such as:

1. Which Supabase structures are canonical identity or collaboration infrastructure?
2. Which structures are legacy inventory only?
3. Which structures must remain frozen?
4. Which structures may be retained, quarantined, migrated, replaced, or deleted under later authority?
5. How are remote identifiers mapped to local entities without granting local Artifact authority?
6. What metadata may synchronize?
7. What classified metadata must never synchronize by default?
8. How is no-PHI-by-default enforced?
9. How are migrations inventoried, ordered, versioned, and validated?
10. What compatibility or rollback evidence is required?
11. How are offline and remote states reconciled without weakening policy?
12. What security review is required before classified data or any PHI exception?
13. Which Accepted ADRs and dedicated specifications must precede implementation?

These remain **future-spec questions**, not charter decisions.

---

## 5. Explicit non-goals (T029)

T029 does **not**:

- create Spec 002 or any `specs/002-*` directory;
- run `/speckit.specify` or any Spec Kit command;
- create Spec 002 `spec.md`, `plan.md`, `tasks.md`, `research.md`, `data-model.md`, or contracts;
- edit, create, or run migrations;
- alter tables, RLS, or seed data;
- copy data or inspect real patient data;
- authorize Supabase PHI storage or egress;
- modify adapters, authentication, or Artifact Store behavior;
- implement synchronization or offline reconciliation;
- define final schemas, APIs, or generate bindings;
- accept ADR-07 or ADR-14;
- execute T030;
- authorize R2 or implement source code.

---

## 6. Decision C boundary

Preserve ratified Decision C ([plan.md](../../../specs/001-fanatir-repository-and-architecture-reconstitution/plan.md)):

- authoritative patient content remains local by default;
- the local Rust-controlled **Artifact Store** is the **intended** authority ([ADR-05](../../../specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-05-artifact-revision-run-storage.md) — Proposed);
- **Supabase** is optional identity and collaboration infrastructure;
- Supabase is **not** the Artifact or PHI source of truth;
- remote rows do **not** grant local Artifact authority;
- **no PHI synchronization by default**;
- offline operation must **not** weaken policy;
- existing Supabase structures do **not** imply approval.

The Artifact Store is **intended direction**, not claimed complete implementation.

---

## 7. T028 freeze boundary

Reference the committed freeze decision: [documents-crypto-freeze.md](./documents-crypto-freeze.md).

Preserve:

- `documents-crypto` is **frozen**;
- no real patient data;
- no production PHI;
- synthetic/test-only when fixtures are later authorized;
- no expansion; no deletion under the current boundary; no migration edits;
- no new consumers; no new writes; no production enablement;
- preservation is **not** endorsement;
- future disposition requires separate authority.

This charter does **not** implement `N-T028-1` or rewrite T028 observations.

---

## 8. Supabase boundary

- Supabase remains **optional**.
- Its permissible future role is Spec 002 subject matter — not settled here.
- It is **not** currently authorized as Artifact or PHI source of truth.
- RLS does **not** replace Trusted Host / Capability Gateway policy ([ADR-02](../../../specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-02-rust-trusted-host-boundary.md) — Proposed).
- Authentication is **not** authorization ([ADR-11](../../../specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-11-auth-and-session-preservation.md) — Proposed).
- Remote membership or profile state grants **no** local Artifact authority.
- Existing table, migration, encryption, or column names establish **no** security approval.
- No Supabase schema or adapter behavior is selected by this charter.

See [supabase-adapter.md](../../../specs/001-fanatir-repository-and-architecture-reconstitution/contracts/supabase-adapter.md).

---

## 9. Data and PHI boundary

- T029 uses **no** real patient data and inspects **no** production PHI.
- T029 creates **no** data fixtures.
- Future work must preserve `DataClassification` (structure — [ADR-04](../../../specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-04-shared-primitive-ownership-and-versioning.md); enforcement — [ADR-14](../../../specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-14-security-and-data-classification-enforcement.md) — both **Proposed**).
- Missing or ambiguous classification must **not** fail open for privileged/egress operations.
- PHI egress remains **default denied**.
- Minimum necessary and no-PHI logging remain required.
- Encryption does **not** create authorization.
- Any future PHI exception requires Accepted architecture, a dedicated specification, security review, and explicit authorization.

No final enforcement schemas are published here.

**Security/privacy field (`tasks.md` T029: N/A):** T029 introduces no new security or privacy implementation and requires no separate security artifact. “N/A” does **not** permit weakening Decision C, T028, ADR-07, or ADR-14 constraints.

---

## 10. Shared primitive boundary

Reference only — do **not** redefine ([shared-primitives.md](../../../specs/001-fanatir-repository-and-architecture-reconstitution/contracts/shared-primitives.md); [data-model.md](../../../specs/001-fanatir-repository-and-architecture-reconstitution/data-model.md)):

`Artifact`, `Revision`, `Run`, `Review`, `Approval`, `Decision`, `PolicyDecision`, `AuthContext`, `DataClassification`, `Capability`, `ExportManifest`, `Workspace`, `Project`, `Patient`.

Ownership (all ADRs **Proposed**):

| Concern | Owner |
| --- | --- |
| Shared structures | ADR-04 |
| Trusted Host, Capability Gateway, runtime policy issuance, authoritative audit | ADR-02 |
| Artifact persistence | ADR-05 |
| Supabase / local-first | ADR-07 |
| AuthContext / session | ADR-11 |
| Classification and egress enforcement | [ADR-14](../../../specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-14-security-and-data-classification-enforcement.md) |
| Rust core / supervised workers | [ADR-15](../../../specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-15-rust-first-polyglot-runtime.md) |

---

## 11. Migration canonicalization questions

Future Spec 002 may investigate (without resolution here):

- migration inventory and provenance;
- ordering and dependencies;
- duplicate or conflicting migrations;
- legacy paths;
- environment assumptions;
- schema ownership;
- compatibility and rollback;
- data preservation, quarantine, deletion, replacement;
- canonical migration history and evidence;
- security validation;
- dry-run strategy;
- backup and recovery requirements.

This charter does **not** state that any migration will be retained, rewritten, squashed, deleted, or executed.

---

## 12. Architecture preconditions for future Spec 002

**Confirmed current gates (program evidence):**

- Decision C ratified;
- T019 ADR-07 drafted (**Proposed**);
- T028 freeze committed;
- Spec 002 **not** created under Spec 001 or T029.

**Possible future gates** (confirm against the future task contract; **not** satisfied by this charter):

- T030 completion where required;
- relevant ADR acceptance;
- security review;
- migration inventory;
- founder authorization;
- explicit Spec Kit invocation;
- dedicated branch/worktree discipline;
- synthetic fixtures only until real-data authority exists.

Do not invent gates beyond what later contracts establish.

---

## 13. Expected future Spec 002 deliverables (deferred)

Possible deferred categories only:

- specification; plan; research; data model;
- migration inventory; compatibility matrix; rollback plan;
- security and privacy plan; acceptance criteria; task breakdown;
- verification evidence; ADR follow-ups where required.

**None** of these deliverables is created by T029.

---

## 14. Future verification expectations

When Spec 002 is later authorized, expectations may include:

- repository and migration inventory;
- link and path verification;
- schema-difference analysis;
- synthetic-only tests;
- rollback evidence;
- no-PHI validation;
- security review;
- offline behavior validation;
- policy and RLS boundary checks;
- audit and provenance evidence.

**None** of this verification is claimed executed under T029.

---

## 15. Product boundaries

| Product | T029 rule |
| --- | --- |
| **Fehrest** | Separate product and repository; no vault migration; no synchronization; no Fehrest schema work; no identity merging |
| **DeepMed** | No model, provider, prompt, or PHI invocation; no DeepMed integration defined |
| **commandF** | No FHIR validation, transformation, terminology, or live-system action; no external validation; no FHIR-server access; no PHI egress; commandF strategy did **not** alter T029 or ADR-10 |

---

## 16. R1, T030, and R2 boundary

- T029 is an **R1** charter task.
- T029 does **not** close R1.
- T029 is a **prerequisite** for [T030](../../../specs/001-fanatir-repository-and-architecture-reconstitution/tasks.md) (dependencies `T012`–`T029`).
- T029 completion is **not** sufficient for T030 passage.
- T030 remains separately executed.
- **R2 must not begin before T030**.
- This charter grants **no** T031+ authority.

---

## 17. Future entry criteria (not currently granted)

Later Spec 002 creation will need separate confirmation of criteria such as:

- explicit founder authorization;
- exact Spec 002 path/name confirmation;
- T030/R2 authority as required by the future task contract;
- required ADR states confirmed;
- clean canonical repository state;
- dedicated Spec Kit invocation authorized;
- no migration or PHI action implied by specification creation alone;
- clearly documented stop boundaries.

These criteria are **not** claimed satisfied by T029.

---

## 18. Risks and open questions

Unresolved (do not resolve here):

- legacy migration ambiguity;
- stale schema assumptions;
- accidental PHI synchronization;
- remote/local identifier conflation;
- RLS mistaken for host authorization;
- offline reconciliation ambiguity;
- migration rollback uncertainty;
- hidden consumers;
- configuration drift;
- audit gaps;
- compatibility debt;
- retention and deletion obligations;
- jurisdictional uncertainty;
- incomplete inventory;
- future product-boundary drift.

---

## 19. Verification for T029

Per [tasks.md](../../../specs/001-fanatir-repository-and-architecture-reconstitution/tasks.md) T029:

1. this charter path `docs/program-memory/decisions/002-charter.md` exists;
2. canonical future name `002-supabase-local-first-and-migration-canonicalization` is exact;
3. charter scope (name + scope) is present;
4. no `specs/002` directory exists;
5. no migration changed;
6. no other path changed under T029;
7. no implementation occurred;
8. **Tier C — normal verification** is required before commit.

Tier C is **not** claimed passed by this authoring step.

---

## 20. Rollback

T029 rollback is **reverting or removing only this charter document**.

No runtime, schema, migration, database, or data rollback applies — none was changed.

---

## 21. Review record

| Field | Value |
| --- | --- |
| **Review tier** | Tier C — normal verification |
| **Review status** | Passed with non-blocking notes |
| **Decision** | `APPROVE WITH NON-BLOCKING NOTES — TIER C FUTURE SPEC 002 CHARTER VERIFICATION PASSED` |
| **Reviewer** | Independent Tier C reviewer |
| **Review date** | 2026-07-26 |
| **Founder work-product acceptance** | No |
| **Reviewed artifact path** | `docs/program-memory/decisions/002-charter.md` |
| **Reviewed identity** | no-filters `17454406e83d84da7a464e42ded60543cb551883` / filtered `e956dec23f4ca5bededc11c460fc577af7b0b47d` |
| **Blocking findings** | None |
| **Non-blocking notes** | `N-T029-1` (referenced only; not implemented by this status update) |
| **Observations** | `O-T029-1` (referenced only; not implemented by this status update) |
| **Authority disclaimer** | Tier C verification does **not** create Spec 002; does **not** accept Spec 002; does **not** authorize `/speckit.specify`; does **not** accept ADR-07; does **not** accept ADR-14; is **not** architecture acceptance; does **not** authorize migration editing or execution; does **not** authorize Supabase schema, RLS, configuration, adapter, or runtime changes; does **not** authorize real patient data or PHI handling; does **not** execute or authorize T030; does **not** authorize R2, R3, or R5; does **not** grant T031+ authority; is **not** founder work-product acceptance. |

---

## 22. Legal and compliance claim discipline

This charter does **not** claim HIPAA certification or compliance, Saudi PDPL compliance, GDPR compliance, Australian Privacy Act compliance, legal safe harbor, complete de-identification, secure Supabase PHI support, production readiness, or implemented encryption/audit guarantees.
