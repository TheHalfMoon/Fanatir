# Contract: Supabase Optional Adapter

**Version**: 0.2.0-draft  
**Disposition**: **Adapt** (founder Decision C, 2026-07-23) — optional collaboration and identity adapter  
**Not**: Content, clinical, Artifact, or patient source of truth

## Authoritative storage (founder-ratified)

Document content, extracted entities, patient material, FHIR containing PHI, and Artifact revisions MUST move to the **local Rust-controlled Artifact Store** (ADR-05 + ADR-15).

Supabase is an optional collaboration and identity adapter only.

## Allowed adapter concerns (target)

- Auth session issuance (preserve current `AuthContext` / OTP flows until dedicated auth migration spec)
- Workspace invites / membership metadata
- Collaboration sync metadata that is explicitly authorized and classified
- Reviews, invitations, and synchronization **records/references** (not clinical content SoT)
- Approved non-content cloud metadata only

## Forbidden by default

- PHI storage or egress as content SoT
- Artifact/Run/Revision authoritative storage
- Patient material / FHIR-containing-PHI as SoT
- PolicyDecision authority
- Silent migration of local patient continuity to cloud

## Legacy `documents-crypto` path (founder Decision C)

Verified seam: field-encrypted document title/content/metadata (incl. extracted entities) → Supabase `documents` + `audit_log` via server-held `ENCRYPTION_KEY`.

**Until ADR-07 and ADR-14 are accepted:**

- Freeze expansion of this path
- Do not use it for new real patient data
- Allow synthetic/test fixtures only
- Preserve for compatibility inspection
- Do not delete or mutate it in this reconstitution phase

## Migration filesystem

- Provisional app location: `afia-ui/supabase/`
- Duplicate root `supabase/migrations/` **frozen**
- Canonicalization + local-first migration detail → future spec **`002-supabase-local-first-and-migration-canonicalization`** (authorized; **not** created in 001 planning)

## Compatibility

- Existing `PrivateRoute` / session behavior preserved until dedicated auth migration spec
- Adapter must degrade: local-first features work when cloud unavailable
