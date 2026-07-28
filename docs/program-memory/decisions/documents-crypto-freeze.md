# documents-crypto Freeze Decision

| Field | Value |
| --- | --- |
| **Document type** | Operational freeze record (program-memory decision) |
| **Task origin** | T028 |
| **Related founder decision** | Decision C (**Ratified** in [plan.md](../../../specs/001-fanatir-repository-and-architecture-reconstitution/plan.md)) |
| **Related ADR planning** | [ADR-07](../../../specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-07-supabase-adapter-and-local-first-boundary.md) (**Proposed**); [ADR-14](../../../specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-14-security-and-data-classification-enforcement.md) (**Proposed**) |
| **Companion contract** | [supabase-adapter.md](../../../specs/001-fanatir-repository-and-architecture-reconstitution/contracts/supabase-adapter.md) |
| **Inventory evidence** | [R0-supabase-inventory.md](../baseline/R0-supabase-inventory.md) |

```text
Status: Operational freeze recorded (T028 documentation)
This document is not an ADR
This document does not accept ADR-07 or ADR-14
This document does not grant migration or implementation authority
This document does not authorize handling real PHI or real patient data
tasks.md founder-acceptance for T028: No — operationalizes Decision C
Tier A — PHI-egress freeze review is required before any commit of this work
```

---

## 1. Status and authority

This document **operationalizes** ratified **Decision C** by recording an **operational freeze** of the legacy `documents-crypto` PHI-egress seam and documenting planning PHI-egress seam controls.

It is:

- a program-memory freeze decision;
- documentation and policy-notice authority only under T028;
- subject to **Tier A — PHI-egress freeze** independent review before commit.

It is **not**:

- an Accepted ADR;
- architecture acceptance of ADR-07 or ADR-14;
- a completed migration;
- a security certification;
- a compliance approval;
- permission to inspect, transmit, or store real patient data;
- permission to edit migrations, expand the seam, or delete the path.

Founder work-product acceptance is **not** required by the T028 contract (`No — operationalizes Decision C`). Passing Tier A review is not architecture acceptance.

---

## 2. Scope

### 2.1 Legacy seam (inventory terminology)

Repository planning and R0 inventory refer to the legacy **`documents-crypto`** seam: a field-encrypted remote document path associated with Supabase `documents` / related audit storage and the Edge Function / client invoke path inventoried under T006 ([R0-supabase-inventory.md](../baseline/R0-supabase-inventory.md); [R0-path-classification.md](../baseline/R0-path-classification.md)).

This freeze applies to:

- the current `documents-crypto` path and its inventoried role;
- its current migrations and duplicate migration trees as **inventory evidence**;
- any attempted use for real patient data or production PHI;
- any attempted expansion, new consumer, new write path, or production enablement.

Sensitive implementation detail is intentionally minimized. Existing code and migrations remain **inventory evidence only** and are **not** modified by T028.

### 2.2 What this freeze does not cover

- Spec `002-supabase-local-first-and-migration-canonicalization` (T029 charter only; do not create `specs/002` here);
- Artifact Store implementation ([ADR-05](../../../specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-05-artifact-revision-run-storage.md) — Proposed);
- IPC or worker-runtime changes;
- Fehrest, DeepMed, or commandF product changes;
- ADR status flips.

---

## 3. Freeze rules

1. **No real patient data** may enter `documents-crypto`.
2. **No production PHI** may enter `documents-crypto`.
3. Use is **synthetic/test-only** when any fixture is required for planning or tests under later authorized work.
4. The seam is **preserved for inspection** (see §8); preservation is not endorsement.
5. **No expansion** — no new features, consumers, write paths, or product enablement of this seam.
6. **No deletion** of the path under T028.
7. **No migration edits** under T028 — neither `afia-ui/supabase/` nor root `supabase/migrations/` trees.
8. **No production enablement** of remote PHI content storage via this seam.
9. Existence of the seam **does not authorize** Supabase PHI storage.
10. Existence of the seam **does not** make Supabase the Artifact or PHI source of truth.
11. Encryption-related naming (including inventory references to ciphertext or key env names) **does not prove** safe PHI storage or egress.
12. **Encryption does not replace** classification, authorization, minimum necessary, consent, or policy evaluation ([ADR-14](../../../specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-14-security-and-data-classification-enforcement.md) — Proposed).
13. Existing code or migrations are **inventory evidence only**.
14. Any future disposition requires separately accepted architecture, a dedicated specification, security review, migration planning, and explicit authorization.

---

## 4. Permitted inspection

Read-only inspection is permitted for:

- architecture reconstitution;
- threat analysis;
- migration planning (when separately authorized);
- compatibility analysis;
- test-fixture understanding;
- provenance of legacy decisions.

Inspection **does not** authorize execution against production systems, access to real accounts, or handling of real patient data.

---

## 5. PHI-egress seam controls (planning)

Planning controls for any governed egress (including legacy or future collaboration paths):

| Control | Planning rule |
| --- | --- |
| Classification | Governed data-bearing operations resolve `DataClassification` ([ADR-04](../../../specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-04-shared-primitive-ownership-and-versioning.md) structure; [ADR-14](../../../specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-14-security-and-data-classification-enforcement.md) enforcement) |
| Default deny | `phi` forbids default cloud egress |
| Authority | Trusted Host / Capability Gateway evaluate privileged operations ([ADR-02](../../../specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-02-rust-trusted-host-boundary.md) — Proposed) |
| Minimum necessary | Applies to payloads, logs, exports, and provider requests |
| Destination / purpose | Unknown destination or purpose defaults to denial |
| Provider / retention | External-provider use requires separate review |
| Approval vs PolicyDecision | Distinct; login and user confirmation are not authorization |
| Audit | Authoritative audit ownership remains with Trusted Host planning; schema unresolved |
| Logging | No PHI or secrets in ordinary logs, crash reports, packages, support bundles, UI assets, or commit history |
| Failure | Missing, unknown, conflicting, or invalid classification fails closed for privileged/egress operations |

No final `PolicyDecision`, `Approval`, or audit schemas are published here.

---

## 6. Supabase boundary

- Supabase remains **optional** identity and collaboration infrastructure.
- Supabase is **not** the Artifact Store or PHI source of truth.
- **No PHI by default** — including no default synchronization of patient content.
- RLS does **not** replace host Capability Gateway / policy evaluation.
- Existing tables, rows, RLS, migration names, or encryption-related names do **not** grant PHI or Artifact authority.
- Remote rows do **not** grant local Artifact authority.
- Offline use must **not** weaken policy.
- Classified collaboration metadata remains separately gated and unresolved without a dedicated specification (future Spec **002** charter via T029 — not created here).
- Authentication / `AuthContext` ([ADR-11](../../../specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-11-auth-and-session-preservation.md) — Proposed) may inform policy; authentication is **not** authorization.

See companion [supabase-adapter.md](../../../specs/001-fanatir-repository-and-architecture-reconstitution/contracts/supabase-adapter.md).

---

## 7. Synthetic / test-only rule

When fixtures are needed under later authorized work, they must be:

- synthetic;
- fabricated;
- non-identifiable;
- non-production;
- bounded;
- clearly labeled test evidence.

**Do not:**

- copy production structures containing identifiers;
- use realistic identifying combinations;
- import external patient records;
- use provider logs or real Supabase rows as fixtures.

T028 itself introduces **no** fixtures and inspects **no** real PHI.

---

## 8. Preservation-for-inspection rule

The seam is preserved (not deleted under T028) to support:

- historical evidence;
- migration discovery;
- compatibility analysis;
- threat analysis;
- rollback and provenance understanding.

**Preservation is not endorsement** of remote PHI content storage and is not permission to expand or enable the seam.

---

## 9. Future disposition gates

Separate future authority is required for:

| Disposition | Planning note |
| --- | --- |
| Continued archival preservation | Explicit ops policy beyond this freeze record |
| Quarantine | Dedicated security/ops specification |
| Migration to local Artifact Store | ADR-05/07/14 acceptance gates as applicable; Spec 002; separately authorized tasks |
| Replacement / deletion | Explicit authorization; not T028 |
| Data extraction | Separate security review; no real PHI under this freeze |
| Compatibility adapter | ADR-07 Accepted and later adapter tasks (e.g. T057) remain separately gated |
| Supabase schema changes | Prohibited under T028; later migration authority only |
| Real PHI support on Supabase | Default prohibited; any exception requires founder-ratified path and Accepted architecture — not granted here |

Downstream references (canonical, not authorization): T029 (002 charter); T030 (R1 gate); T057 (adapter tests after ADR-07/14 Accepted).

---

## 10. Verification

T028 verification method (`tasks.md`): **Reviewer sign-off note**.

Verification is documentation review against Decision C, ADR-07 draft consistency, ADR-14 planning non-weakening, and the acceptance criteria:

- freeze policy recorded;
- synthetic/test-only rule explicit;
- preserve for inspection.

No code tests, migration checks, or production system inspection are claimed executed under T028 authoring.

---

## 11. Rollback

T028 rollback is **revert the documentation changes**:

1. this freeze decision file;
2. related policy-notice updates in [supabase-adapter.md](../../../specs/001-fanatir-repository-and-architecture-reconstitution/contracts/supabase-adapter.md).

Runtime rollback, schema rollback, and data restoration are **out of scope**.

---

## 12. Reviewer sign-off record

| Field | Value |
| --- | --- |
| **Review tier** | Tier A — PHI-egress freeze |
| **Review status** | Passed with non-blocking notes |
| **Decision** | `APPROVE WITH NON-BLOCKING NOTES — TIER A DOCUMENTS-CRYPTO PHI-EGRESS FREEZE REVIEW PASSED` |
| **Reviewer** | Independent Tier A reviewer |
| **Review date** | 2026-07-26 |
| **Reviewed artifact paths** | `docs/program-memory/decisions/documents-crypto-freeze.md`; `specs/001-fanatir-repository-and-architecture-reconstitution/contracts/supabase-adapter.md` |
| **Reviewed identities** | Freeze no-filters `dd0418172376fa490a9ee760708726e7f137b5d8` / filtered `ec65085c3d79afc3a919e796b2ef396df3948965`; adapter no-filters `693576cb7bccab1e3329d102dd0bf31561625e46` / filtered `16a0975a183d20e12af0a0e5c28057284bb53b9c` |
| **Required checks** | Freeze policy; synthetic/test-only; preserve-for-inspection; no expansion/deletion/migration edits; Decision C fidelity; ADR-07/14 remain Proposed; no real PHI; no compliance claims; no migration/source mutation |
| **Blocking findings** | None |
| **Non-blocking notes** | `N-T028-1` (referenced only; not implemented by this sign-off) |
| **Observations** | `O-T028-1`, `O-T028-2`, `O-T028-3`, `O-T028-4` (referenced only; not implemented by this sign-off) |
| **Authority disclaimer** | This Tier A result does **not** accept ADR-07; does **not** accept ADR-14; does **not** grant architecture acceptance; does **not** authorize real patient data or PHI handling; does **not** authorize Supabase PHI storage; does **not** authorize migration or implementation; does **not** authorize T029, T030, R2, R3, or R5. Tier A pass ≠ founder work-product acceptance. |

---

## 13. Legal and compliance claim discipline

This freeze record does **not** claim HIPAA certification or compliance, Saudi PDPL compliance, GDPR compliance, Australian Privacy Act compliance, legal safe harbor, complete de-identification, implemented encryption guarantees, implemented immutable audit, secure Supabase PHI support, or production readiness.

Design-intent language such as local-first privacy architecture remains subject to Constitution claim-language limits and documented safeguards.

---

## 14. Product boundaries

| Product / capability | T028 rule |
| --- | --- |
| Fehrest | Separate; no vault migration or synchronization; no mutation |
| DeepMed | No prompt, provider, model, or PHI invocation; no mutation |
| commandF | Remains commandF; no external validation, terminology request, FHIR-server access, live-system write, or PHI egress; accepted FHIR strategy did not alter T028 or ADR-10 |

---

## 15. Non-goals

T028 does **not**:

- edit migrations;
- expand or delete `documents-crypto`;
- mutate Supabase schemas, RLS, seeds, or configuration;
- implement adapters, encryption, audit, or classification enforcement;
- accept ADR-07 or ADR-14;
- create Spec 002;
- begin T029, T030, T031+, R2, R3, or R5;
- handle real PHI.
