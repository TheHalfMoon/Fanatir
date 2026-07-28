# Contract: Supabase Optional Adapter

| Field | Value |
| --- | --- |
| **Version** | 0.3.0-draft (T028 freeze / PHI-egress policy alignment) |
| **Disposition** | **Adapt** (founder Decision C, **Ratified**) — optional collaboration and identity adapter |
| **Not** | Content, clinical, Artifact, or patient source of truth |
| **Freeze decision** | [documents-crypto-freeze.md](../../../docs/program-memory/decisions/documents-crypto-freeze.md) |
| **Related ADRs** | [ADR-07](../adrs/ADR-07-supabase-adapter-and-local-first-boundary.md) (**Proposed**); [ADR-14](../adrs/ADR-14-security-and-data-classification-enforcement.md) (**Proposed**); [ADR-05](../adrs/ADR-05-artifact-revision-run-storage.md) (**Proposed**); [ADR-02](../adrs/ADR-02-rust-trusted-host-boundary.md) (**Proposed**); [ADR-11](../adrs/ADR-11-auth-and-session-preservation.md) (**Proposed**) |

```text
Planning contract only — non-runtime, non-database, non-migration
ADR-07 and ADR-14 remain Proposed (not Accepted by T028)
T028 updates policy notices only — no Supabase schema, SQL, RLS, or adapter implementation
No PHI by default; no real patient data on documents-crypto
```

---

## Authoritative storage (Decision C)

Document content, extracted entities, patient material, FHIR containing PHI, and Artifact revisions are intended to be authoritative only in the **local Rust-controlled Artifact Store** ([ADR-05](../adrs/ADR-05-artifact-revision-run-storage.md); [ADR-15](../adrs/ADR-15-rust-first-polyglot-runtime.md) — both **Proposed**). This contract does **not** claim the Artifact Store is already implemented or complete.

**Supabase** is optional identity and collaboration infrastructure only. It is **not** the Artifact or PHI source of truth. Remote rows do **not** grant local Artifact authority. Offline use must **not** weaken policy.

---

## Allowed adapter concerns (target)

- Auth session issuance (preserve current `AuthContext` / OTP flows until dedicated auth migration spec; [ADR-11](../adrs/ADR-11-auth-and-session-preservation.md) — Proposed)
- Workspace invites / membership metadata
- Collaboration sync metadata that is explicitly authorized and classified
- Reviews, invitations, and synchronization **records/references** (not clinical content SoT)
- Approved non-content cloud metadata only

Authentication is **not** authorization. `AuthContext` may inform policy; it does not issue `PolicyDecision`.

---

## Forbidden by default

- PHI storage or egress as content SoT
- Artifact / Run / Revision authoritative storage
- Patient material / FHIR-containing-PHI as SoT
- `PolicyDecision` authority (issuance remains Trusted Host / Capability Gateway planning — [ADR-02](../adrs/ADR-02-rust-trusted-host-boundary.md))
- Silent migration of local patient continuity to cloud
- Default PHI synchronization
- Compliance or certification claims based on adapter existence

---

## Legacy `documents-crypto` freeze (T028 / Decision C)

Inventory seam (read-only evidence): field-encrypted document title/content/metadata path associated with Supabase `documents` / related audit storage and the Edge Function / client invoke path described in [R0-supabase-inventory.md](../../../docs/program-memory/baseline/R0-supabase-inventory.md).

**Operational freeze** (see [documents-crypto-freeze.md](../../../docs/program-memory/decisions/documents-crypto-freeze.md)):

| Rule | Requirement |
| --- | --- |
| Real patient data | **Forbidden** |
| Production PHI | **Forbidden** |
| Synthetic / test-only | Allowed only as clearly labeled non-production fixtures under later authorized work |
| Preserve for inspection | **Required** — historical, threat, compatibility, and migration-planning evidence |
| Expansion | **Forbidden** under T028 |
| Deletion | **Forbidden** under T028 |
| Migration edits | **Forbidden** under T028 |
| Production enablement | **Forbidden** |
| New consumers / new writes | **Forbidden** without separate authorization |

Existence of `documents-crypto`, its tables, RLS, migration names, or encryption-related naming **does not**:

- authorize Supabase PHI storage;
- make Supabase the Artifact or PHI source of truth;
- prove safe PHI egress;
- replace classification, authorization, minimum necessary, consent, or policy evaluation.

**Encryption does not authorize egress.** ADR-07 and ADR-14 remain **Proposed**; T028 does not accept them.

---

## PHI-egress planning controls

Privileged egress decisions remain with Trusted Host / Capability Gateway planning ([ADR-02](../adrs/ADR-02-rust-trusted-host-boundary.md); [ADR-14](../adrs/ADR-14-security-and-data-classification-enforcement.md)):

- classify before egress (`DataClassification` structure — ADR-04; enforcement — ADR-14);
- default-deny cloud egress for `phi`;
- minimum necessary;
- destination and purpose review;
- unknown or conflicting classification fails closed for privileged/egress operations;
- `Approval` ≠ `PolicyDecision`; login ≠ authorization;
- no PHI or secrets in ordinary logs.

No final PolicyDecision, Approval, or audit schema is published in this adapter contract.

---

## Migration filesystem (inventory)

- Provisional app location: `afia-ui/supabase/`
- Duplicate root `supabase/migrations/` **frozen** as inventory
- Canonicalization + local-first migration detail → future spec **`002-supabase-local-first-and-migration-canonicalization`** (authorized by name; **not** created in Spec 001; T029 charter only)

T028 **does not** create migrations, run migrations, alter tables, alter RLS, alter seeds, or copy data.

---

## Classified collaboration metadata

Future classified collaboration metadata on Supabase requires separately accepted architecture and a dedicated specification. Existing rows or RLS do **not** imply approval.

---

## Compatibility

- Existing `PrivateRoute` / session behavior preserved until dedicated auth migration spec
- Adapter must degrade: local-first features work when cloud unavailable
- Network loss does not relax classification or egress policy

---

## T028 documentation posture

T028 updates **documentation and policy notices** only. It does not implement the adapter, mutate Supabase runtime configuration, handle real PHI, modify Fehrest / DeepMed / commandF, or begin T029 / T030 / R2.

**Verification** (`tasks.md` T028): reviewer sign-off note (Tier A — PHI-egress freeze). **Rollback**: revert docs.
