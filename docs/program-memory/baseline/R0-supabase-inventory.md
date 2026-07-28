# R0 Supabase Clients & Migration Trees Inventory (T006)

**Task**: T006 — Inspect Supabase clients and both migration trees (freeze inventory)  
**Recorded**: 2026-07-24 (local)  
**Baseline / start HEAD**: `e53c059027f51b022f4ea480ebfa82deecacfb83` (T005 completion)  
**Branch**: `docs/f0-planning-memory-bootstrap`  
**Nature**: Read-only source/SQL inventory. **No** CLI, live project connection, migration run, or mutation.  
**Review**: **Tier A** — implementing agent is **not** the sole independent reviewer.  
**Independent Tier A review**: completed 2026-07-24 by a separate reviewing agent; founder accepted as **PASS WITH NON-BLOCKING NOTES**.  
**T006 decision**: **PASS WITH NON-BLOCKING NOTES**

Absolute paths are local facts. Deployed database state is **unknown** unless proven elsewhere.

Security taxonomy: **VERIFIED BEHAVIOR** · **VERIFIED TRUST SEAM** · **VERIFIED SECURITY DEFECT** · **POTENTIAL RISK — deployed-state or runtime verification required** · **NOT OBSERVED** · **OUT OF SCOPE**.

---

## 1. Founder-ratified target boundary (context)

| Current baseline (evidence) | Target (Decision C / ADR-07/14 direction) |
| --- | --- |
| Supabase used for auth, collaboration, encrypted document rows via Edge Functions | Content/PHI/Artifact authority → local **Rust Artifact Store** |
| `documents-crypto` stores field-encrypted title/content/metadata remotely | **Legacy PHI-egress seam** — freeze expansion; synthetic/test only until ADR gates |
| Dual SQL trees in repo | Spec **001** freezes mutation; future **002** owns migration canonicalization |
| Optional identity/collaboration adapter may remain | Not content SoT |

T006 does **not** change code to enforce the target.

---

## 2. Supabase path inventory

| Path | Role | Tracked |
| --- | --- | --- |
| `afia-ui/client/src/lib/supabase.ts` | Browser client (`VITE_SUPABASE_URL`, `VITE_SUPABASE_ANON_KEY`) | yes |
| `afia-ui/client/src/lib/database.types.ts` | Hand-maintained table types | yes |
| `afia-ui/client/src/contexts/AuthContext.tsx` | Auth + `profiles` | yes |
| `afia-ui/client/src/lib/team-workspaces.ts` | Workspaces + `workspace-invites` | yes |
| `afia-ui/client/src/lib/documents.ts` | `documents-crypto` invoke | yes |
| `afia-ui/client/src/lib/audit.ts` | `audit_log` insert | yes |
| `afia-ui/supabase/schema.sql` | Reference schema: profiles, documents, audit_log + own-user RLS | yes |
| `afia-ui/supabase/migrations/workspaces.sql` | Wave 3A workspaces + document policy replacement | yes |
| `supabase/migrations/workspaces.sql` | Duplicate of afia-ui workspaces migration | yes |
| `afia-ui/supabase/functions/documents-crypto/index.ts` | Document encrypt/decrypt CRUD Edge Function | yes |
| `afia-ui/supabase/functions/workspace-invites/index.ts` | Workspace create / invite / accept | yes |
| `afia-ui/supabase/.temp/linked-project.json` | Local CLI linkage metadata (keys: `ref`, `name`, `organization_id`, `organization_slug`) | yes — **values not reproduced** in this evidence |
| `afia-ui/client/src/vite-env.d.ts` | Type declarations for `VITE_SUPABASE_URL`, `VITE_SUPABASE_ANON_KEY` | yes |

**No** `.env` / env-example files inventored in-tree for Supabase secrets.  
**Env names only** (never values): `VITE_SUPABASE_URL`, `VITE_SUPABASE_ANON_KEY`, `SUPABASE_URL`, `SUPABASE_ANON_KEY`, `SUPABASE_SERVICE_ROLE_KEY`, `ENCRYPTION_KEY`, `RESEND_API_KEY`, `APP_PUBLIC_URL`, `RESEND_FROM`.

---

## 3. Migration-tree comparison

| Tree | Files | Notes |
| --- | --- | --- |
| `supabase/migrations/` | `workspaces.sql` only | Non-dated filename (not `YYYYMMDDHHMMSS_*.sql`) |
| `afia-ui/supabase/migrations/` | `workspaces.sql` only | Same |
| `afia-ui/supabase/schema.sql` | Reference (not under migrations/) | Declares base tables + **legacy own-user document policies** |

**Hash comparison (SHA-256):**

| File | SHA-256 | Bytes |
| --- | --- | --- |
| `supabase/migrations/workspaces.sql` | `D3B29F5F7BCBE2D437B4716E30D70F7C516FD2407283D8116EEB470A22FD5C9F` | 13437 |
| `afia-ui/supabase/migrations/workspaces.sql` | `D3B29F5F7BCBE2D437B4716E30D70F7C516FD2407283D8116EEB470A22FD5C9F` | 13437 |

→ **Byte-identical duplicates.**

| Question | Answer |
| --- | --- |
| Canonical migration history proven? | **No** — dual paths + undated SQL + `schema.sql` vs migration policy supersession |
| Scripts referencing either tree? | **NOT OBSERVED** as automated apply scripts in this inventory |
| Choose canonical tree in T006? | **No** — contradictory / unresolved authority; deferred to spec **002** |

**Contradiction**: `schema.sql` creates per-user document policies that `workspaces.sql` **drops and replaces** with workspace-aware policies. Applied order in a live project is **deployed-state unknown**. `schema.sql`'s own header claims destructive changes live in **dated** migration files, yet the only migration is **undated** — the convention contradicts itself.

**Migration-order risk (static analysis; no SQL executed):**

| Order | Outcome |
| --- | --- |
| `schema.sql` → `workspaces.sql` | Intended order (workspaces.sql header: "run after profiles and documents exist"): legacy own-user documents policies dropped, workspace-aware policies active |
| `workspaces.sql` first (empty DB) | **Fails** — `alter table public.documents` and policy references require `documents` to exist |
| `schema.sql` re-applied after `workspaces.sql` | `create table if not exists` no-ops, but `create policy` (no `if not exists`) **re-creates the four dropped own-user documents policies** → mixed policy state. Permissive policies OR-combine and the legacy policies are subsets of the workspace-aware ones, so effective access does **not** broaden — but declared policy state drifts from either file alone |
| `schema.sql` re-applied on schema-only DB | **Errors** on duplicate `profiles`/`audit_log` policy names → partial-application risk |

Classification: **VERIFIED BEHAVIOR** of the SQL text; which state a live project is in — **POTENTIAL RISK — deployed-state verification required**. Canonicalization deferred to spec 002.

---

## 4. Database-object inventory (declared in source)

### `profiles` — `afia-ui/supabase/schema.sql`

| Field | Notes |
| --- | --- |
| Columns | `id` → `auth.users`, `email`, `consent_given_at`, `created_at` |
| RLS | **enabled** |
| Policies | SELECT/INSERT/UPDATE own row (`auth.uid() = id`); **no DELETE policy** → client DELETE denied by default |
| Client | `AuthContext.syncProfileConsent` select + upsert |
| PHI relevance | Email / consent timestamps — identity-adjacent |

### `documents` — `schema.sql` + `workspace_id` in `workspaces.sql`

| Field | Notes |
| --- | --- |
| Columns | `id`, `bridge_document_id`, `user_id`, `title_encrypted`, `content_encrypted`, `metadata_encrypted`, `status`, timestamps; `workspace_id` added by migration |
| Unique | `(user_id, bridge_document_id)` |
| RLS | **enabled** |
| Legacy policies (schema) | own-user CRUD |
| Post-migration policies | select owner **or** workspace member; insert self + optional editor/owner workspace; update owner or workspace editor; delete creator or workspace owner |
| Triggers | `documents_set_updated_at`; `documents_preserve_user_id` (immutable creator) |
| Client | **Must** use `documents-crypto` for title/content/metadata (schema comment); frontend uses Edge Function |
| PHI relevance | **High** — title/content/entities in metadata (ciphertext at rest; plaintext via Edge decrypt) |

### `audit_log` — `schema.sql`

| Field | Notes |
| --- | --- |
| Columns | `id`, `user_id`, `action`, `resource_type`, `resource_id`, `created_at` |
| RLS | **enabled** |
| Policies | **INSERT only** own user — **no SELECT policy** in schema; **no UPDATE/DELETE policies** → rows client-immutable once written |
| Client | `lib/audit.ts` best-effort insert; Edge Functions also insert |
| Forgery bound | Actor is bound (`auth.uid() = user_id`), but `action`/`resource_type`/`resource_id` are **client-chosen strings** — content forgeable for own rows |

### `workspaces` / `workspace_members` / `workspace_invites` — `workspaces.sql`

| Object | RLS | Key rules |
| --- | --- | --- |
| `workspaces` | enabled | select member/owner; insert as owner; update/delete owner |
| `workspace_members` | enabled | select members; insert/update owner; delete owner or self-leave |
| `workspace_invites` | enabled | owner select/insert/delete; **no invitee SELECT**; **no UPDATE** via RLS (accept via service role); token = `uuid` `gen_random_uuid()` unique (~122-bit random), indexed |

### Helpers / triggers (`workspaces.sql`)

- `is_workspace_member`, `is_workspace_owner`, `has_workspace_role` — **SECURITY DEFINER**, execute granted to `authenticated` + `service_role`
- Trigger `workspaces_add_owner_member` seeds owner membership

### Storage buckets

**NOT OBSERVED** in these SQL files.

### Other tables/views

**NOT OBSERVED** beyond the above in inventoried SQL.

---

## 5. RLS / authorization matrix (SQL-declared)

| Concern | SQL-declared | Edge Function | Client-only | Deployed? |
| --- | --- | --- | --- | --- |
| Profile access | own-row policies | N/A | upsert | unknown |
| Workspace membership | policies + helpers | invite accept (service) | list/UI role | unknown |
| Workspace roles | insert/update/delete policies; doc policies use `has_workspace_role` | documents-crypto mirrors via RPC | `canEditInActiveWorkspace` | unknown |
| Invitations | owner-only RLS; invitee no SELECT | accept uses service role after token/email checks | revoke/delete via client | unknown |
| Document access | workspace-aware policies (if migration applied) | defense-in-depth checks + encrypt/decrypt | invoke only | unknown |
| Audit access | insert-own only | insert; failures logged | best-effort | unknown |
| Patient access | **NOT OBSERVED** in SQL | N/A | clinical UI uses local/`@kernel` | N/A |
| Service-role bypass | grants on helpers | **workspace-invites accept only** | N/A | unknown |

SQL comments themselves document **COMPROMISE**s (e.g. column-level move controls rely on Edge Function). That is **VERIFIED BEHAVIOR** of the declared design, not automatic proof of a live defect.

---

## 6. Edge Function inventory

### `documents-crypto`

| Item | Source fact |
| --- | --- |
| Auth | Requires `Authorization: Bearer …`; user client with anon key + user JWT — **no service role** |
| Env | `SUPABASE_URL`, `SUPABASE_ANON_KEY`, `ENCRYPTION_KEY` (32-byte raw after base64 decode) |
| Actions | `create`, `update`, `get`, `list`, `delete` |
| Crypto | AES-256-GCM; IV+ciphertext packed; server-held key |
| Fields | Encrypts **title**, **content**, **metadata** (JSON may include `entities`, `qa_history`, graph, timestamps) |
| Storage | Supabase `documents` table ciphertext columns |
| Decrypt/read | Returns **plaintext** title/content/metadata to caller over HTTPS |
| AuthZ | Mirrors RLS via `canRead/Update/Delete` + workspace RPCs; explicit move validation |
| Audit | Best-effort `audit_log` insert; failures `console.error` only |
| CORS | `Access-Control-Allow-Origin: *` |
| PHI classification enforced? | **NOT OBSERVED** — no synthetic-vs-real patient gate in function |

**Data-flow (legacy PHI-egress seam):**

```text
Browser (documents.ts)
  → Edge documents-crypto (user JWT)
      → encrypt/decrypt with ENCRYPTION_KEY
      → read/write public.documents (ciphertext)
      → respond with plaintext title/content/metadata (+ entities in metadata)
```

→ Document title/content/extracted entities **can leave the local device** to Supabase Edge and return decrypted to any authorized browser session. Plaintext travels browser↔Edge over TLS and exists in Edge Function memory before encryption — this is **server-side field encryption**, not local encryption before egress.  
Classification: **VERIFIED TRUST SEAM** / legacy PHI-egress (Decision C). Live exploitation depends on deployment — **POTENTIAL RISK — deployed-state or runtime verification required** for whether production projects still host this with real data.

**Cryptographic detail (source: `documents-crypto/index.ts` `getAesKey`/`encryptField`/`decryptField`):**

| Property | Source fact | Class |
| --- | --- | --- |
| Algorithm/mode | AES-256-GCM (WebCrypto), 12-byte IV, 16-byte tag; packed `IV‖ciphertext+tag`, base64 | **VERIFIED BEHAVIOR** |
| Key parsing | `ENCRYPTION_KEY` base64 → must decode to exactly 32 bytes, else throw; imported non-extractable; cached | **VERIFIED BEHAVIOR** |
| IV | `crypto.getRandomValues` per field encryption — randomized, non-static | **VERIFIED BEHAVIOR** |
| Integrity | GCM tag verified on decrypt; tamper → throw → HTTP 500 with error message (not silent) | **VERIFIED BEHAVIOR** |
| Key versioning / rotation | **Absent** — one global server-held key decrypts **all rows**; no key-version column or packed version byte; rotation requires re-encrypting every row | **VERIFIED TRUST SEAM** (design limitation; ADR-07/14 scope) |
| Context binding (AAD) | **Absent** — ciphertext not bound to row/field/document; ciphertext transplanted between rows/fields within an attacker's RLS-writable scope decrypts as valid plaintext. No confidentiality escalation (Edge `canReadDocument` mirrors RLS SELECT), but row-level authenticity is not provided | **VERIFIED TRUST SEAM** |
| Size bounds | **No in-function limits** on title/content/metadata; platform request limits unknown | **POTENTIAL RISK — deployed-state verification required** |
| Plaintext logging | **NOT OBSERVED** — `console.error` paths log error/RPC messages only |
| Deterministic encryption | Not used (randomized IVs) — **VERIFIED BEHAVIOR** |

**Additional authorization facts (source-verified):**

- `user_id` cannot be chosen or altered by callers: `handleCreate` sets it from the JWT; update payloads never include it; DB trigger `documents_preserve_user_id` enforces immutability.
- Unauthorized lookups return `not_found`/`null` (`resolveDocument` gates on `canReadDocument`), so existence is not disclosed for unreadable rows; 403 is only returned for readable-but-not-writable rows.
- Denied operations (401/403/404) are **not audited** — audit rows are written on success paths only.
- Edge/RLS move inconsistency (**fail-closed**): `validateWorkspaceMove` would allow a workspace **owner** to move a member's document to personal, but the documents UPDATE `WITH CHECK` denies it (new `workspace_id` NULL ∧ caller ≠ creator) → DB error, not a bypass.
- Ciphertext columns are protected by RLS only; **no column-level privilege restriction** forces the Edge-only path. The current client uses the Edge Function exclusively (no direct `.from("documents")` call sites in `client/src`), but that is convention, not enforcement.

### `workspace-invites`

| Item | Source fact |
| --- | --- |
| Auth | Bearer user JWT for all actions |
| User client | `create_workspace`, `create` invite (RLS applies) |
| Service role | **`accept` only** via `SUPABASE_SERVICE_ROLE_KEY` after token/expiry/email match |
| Email | Resend API (`RESEND_API_KEY`, `APP_PUBLIC_URL`, optional `RESEND_FROM`) |
| Returns | Invite payload **includes `token`** to caller |
| CORS | `*` |
| Denied-action audit | **NOT OBSERVED** — audit on create/join success paths; failures mostly error responses |

**Accept-flow detail (source: `handleAcceptInvite`):**

- Order: bearer JWT → `getUser` → token lookup (service) → `accepted_at` null check (409 if set) → expiry check (410) → normalized invited-email vs JWT-email comparison (403) → existing-membership check (keeps existing role) → member insert with **role from invite row only** → `accepted_at` timestamp update. Caller supplies only the token; workspace/role are not caller-controllable.
- Consumption is `accepted_at` timestamp; the invite row is **not deleted**.
- **Non-transactional check-then-act**: two simultaneous accepts can both pass the `accepted_at` check. Bounded outcome: same token → same invited email → same user; duplicate membership insert fails on PK `(workspace_id, user_id)`; role fixed by invite row → no privilege escalation path visible in source; worst case one 500 response. Classification: **POTENTIAL RISK — runtime verification required** (benign-race analysis is static).
- Denied paths (404/409/410/403) are **not audited**; only successful `join` is audited (via the user client).
- Database error messages (`inviteError.message`, `memberError.message`) are returned to callers — implementation-detail exposure, **VERIFIED BEHAVIOR**, low impact.
- `sendInviteEmail` interpolates the caller-chosen workspace name **unescaped** into invite email HTML — a workspace owner can inject HTML into an AFIA-branded email to the invitee. **VERIFIED BEHAVIOR** (no escaping in source); abuse impact depends on mail rendering — **POTENTIAL RISK — runtime verification required**.

Service-role use is bounded to invite accept with checks — **VERIFIED BEHAVIOR**. Whether sufficient in production — **POTENTIAL RISK — deployed-state verification required**.

---

## 7. Service-role boundary

| Function | Instantiates service role? | Before bypass | Propagates user identity? |
| --- | --- | --- | --- |
| `documents-crypto` | **No** | N/A | Yes (JWT user) |
| `workspace-invites` `accept` | **Yes** | token exists, not accepted, not expired, email matches user | Yes (JWT user + email compare) |
| `workspace-invites` create* | **No** | RLS | Yes |

---

## 8. Frontend Supabase dependency map

| Caller | Operation | Target | Frontend check | Server enforcement visible |
| --- | --- | --- | --- | --- |
| `AuthContext` | getSession / onAuthStateChange / OTP / signOut | auth | UI forms | Supabase Auth |
| `AuthContext.syncProfileConsent` | select/upsert | `profiles` | none beyond session | RLS own-row (if applied) |
| `team-workspaces.listMyWorkspaces` | select | `workspace_members`+workspaces | session | RLS |
| `team-workspaces` leave/revoke | delete | members/invites | UI owner checks elsewhere | RLS |
| `team-workspaces` create/invite/accept | invoke | `workspace-invites` | UI | Edge + RLS/service |
| `documents.ts` | invoke create/list/get/update/delete | `documents-crypto` | `canEditInActiveWorkspace` for some UI | Edge + RLS |
| `audit.logAction` | insert | `audit_log` | none | insert RLS; best-effort |

---

## 9. Current vs target authority

| Domain | Current baseline | Target |
| --- | --- | --- |
| Identity | Supabase Auth | May remain optional adapter |
| Collaboration | workspaces + invites | Adapter; not content SoT |
| Document/PHI content | Remote encrypted rows + server key + decrypt-to-client | Local Rust Artifact Store |
| Audit | Client/Edge best-effort inserts | Stronger host-side later |
| Policy SoT | Mixed UI + RLS + Edge | Trusted Host |

---

## 10. Security findings (taxonomy)

| Finding | Class |
| --- | --- |
| Dual identical `workspaces.sql` trees; undated; `schema.sql` vs migration policy supersession | **VERIFIED BEHAVIOR** / contradictory history — **no** canonical tree chosen |
| RLS enabled in SQL for profiles/documents/workspaces/members/invites/audit | **VERIFIED BEHAVIOR** (declared); deployment **unknown** |
| `documents-crypto` returns plaintext document fields using server-held `ENCRYPTION_KEY` | **VERIFIED TRUST SEAM** (legacy PHI-egress) |
| CORS `*` on both Edge Functions | **VERIFIED BEHAVIOR**; abuse depends on auth — **POTENTIAL RISK — deployed-state or runtime verification required** |
| Audit inserts fail-open | **VERIFIED TRUST SEAM** |
| `audit_log` has no SELECT policy in schema | **VERIFIED BEHAVIOR** (append-oriented); admin read path **NOT OBSERVED** |
| Service role only on invite accept with token/email checks | **VERIFIED BEHAVIOR**; residual bypass risk if checks fail in prod — **POTENTIAL RISK** |
| UI-only `canEdit*` remains a trust seam even where RLS exists | **VERIFIED TRUST SEAM** (defense-in-depth still required) |
| Linked project metadata in `.temp` | **VERIFIED BEHAVIOR** path present; values redacted here |
| One global `ENCRYPTION_KEY`; no key versioning or rotation path; key decrypts all rows | **VERIFIED TRUST SEAM** (cryptographic design limitation; ADR-07/14 scope) |
| No AAD: ciphertext not bound to row/field context (transplant within RLS-writable scope decrypts validly) | **VERIFIED TRUST SEAM** (no confidentiality escalation; row-level authenticity absent) |
| Invite accept is non-transactional check-then-act | **POTENTIAL RISK — runtime verification required** (bounded: PK dedupe, role fixed by invite row) |
| Unescaped workspace name in invite email HTML | **VERIFIED BEHAVIOR** (no escaping) / abuse — **POTENTIAL RISK — runtime verification required** |
| Edge move validation vs documents UPDATE `WITH CHECK` inconsistency (owner→personal move) | **VERIFIED BEHAVIOR** — fail-closed; functional inconsistency, not a bypass |
| Edge-only path for ciphertext columns is convention (no column-level grants restriction) | **VERIFIED TRUST SEAM**; current client complies (no direct `documents` call sites) |
| Denied/failed operations not audited in either Edge Function | **VERIFIED BEHAVIOR** (success-path audit only) |
| **VERIFIED SECURITY DEFECT** | **None established** — no SQL policy or implementation flaw proving unauthorized access was found in source |

---

## 10a. T005 trust-seam resolution (SQL-declared backing)

T005 deferred server-side enforcement questions to T006. Source-declared answers (deployment still unknown for all rows):

| T005 seam | SQL/Edge backing found | Where |
| --- | --- | --- |
| UI workspace-role checks (`canEditInActiveWorkspace`) | **Yes (declared)** — documents INSERT/UPDATE policies require `has_workspace_role(owner/editor)`; delete requires creator or `is_workspace_owner` | `workspaces.sql` §6 |
| Workspace edit permissions | **Yes (declared)** — same policies + Edge `canUpdateDocument` mirror | `workspaces.sql`; `documents-crypto` |
| Direct `profiles` upsert (T005 fail-open sync) | **Yes (declared)** — own-row INSERT/UPDATE policies bound to `auth.uid()` | `schema.sql` |
| Workspace membership reads | **Yes (declared)** — members-only SELECT via `is_workspace_member` | `workspaces.sql` |
| Invitation acceptance | **Yes** — Edge service-role path with token/expiry/email checks (RLS intentionally closed to invitees) | `workspace-invites` `handleAcceptInvite` |
| Invitation revocation (client delete) | **Yes (declared)** — owner-only DELETE policy | `workspaces.sql` |
| Membership deletion / leave (client delete) | **Yes (declared)** — owner-or-self DELETE policy | `workspaces.sql` |
| Client `audit_log` inserts | **Yes (declared)** — insert-own-only; content strings remain client-chosen | `schema.sql` |

No factual contradiction with `R0-auth-session.md` was found; T005 conclusions stand unmodified.

---

## 11. Frozen paths (spec 001)

Do **not** under this specification:

- mutate either migration tree
- expand `documents-crypto` for real patient data
- delete the seam without ADR/002
- choose/merge canonical history here

Future **`002-supabase-local-first-and-migration-canonicalization`** owns canonicalization.

---

## 12. Deployed-state unknowns

- Whether either SQL file was applied, and in which order
- Live RLS identical to repo
- Whether `ENCRYPTION_KEY` / service role are configured
- Whether any production/project holds real PHI in `documents`
- Exact network exposure of Edge Functions

---

## 13. Migration implications

1. Treat both `workspaces.sql` copies as frozen duplicates until 002.
2. Preserve auth/collaboration adapters; strangler document authority away from `documents-crypto`.
3. Keep synthetic/test-only discipline for document crypto until ADR-07/14.
4. Do not “fix” CORS/policies in 001 — inventory only.

---

## 14. Acceptance

| Criterion | Result |
| --- | --- |
| Dual trees inventoried | **Met** (byte-identical) |
| documents-crypto labeled legacy PHI-egress seam | **Met** |
| Documentation only; no keys/PHI values | **Met** |
| Tier A independent review | **Complete** — founder accepted PASS WITH NON-BLOCKING NOTES |

## 15. Rollback

Delete this file; revert program-memory pointer edits.

## 16. Notes (PASS WITH NON-BLOCKING NOTES)

- Static inventory only; deployment unknown.
- Migration authority: **CONTRADICTORY / UNRESOLVED — specification 002 required**.
- No **VERIFIED SECURITY DEFECT** claimed.
- Cryptography limitations (global unversioned key, no AAD, no in-function size limits) = **VERIFIED TRUST SEAM**s.
- Audit = generated but not authoritative; Trusted Host owns target audit.

- Static inventory only.
- Identical duplicate migrations reduce divergence risk but do not prove deployment.
- No **VERIFIED SECURITY DEFECT** claimed.
