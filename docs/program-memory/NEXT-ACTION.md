# NEXT-ACTION

## Authorized next action

**T005 complete and founder-accepted** (evidence: `docs/program-memory/baseline/R0-auth-session.md`).

Tier A independent review for T005: **complete**.

Next eligible task (do **not** auto-execute unless requested):

```text
T006 — Inspect Supabase clients and both migration trees (freeze inventory)
```

Dependencies for T006: **T001** (per tasks.md). Review: **Tier A**.

T006 owns: RLS policies; migration truth; DB constraints; service-role use; workspace-invites / documents-crypto authorization; audit-table enforcement; server workspace-role enforcement; document/PHI storage behavior.

## Hard gates (unchanged)

- Do not change AuthContext / PrivateRoute / session / consent / profile behavior until dedicated migration specification + ADR
- T030 (ADR-15/01/02/06 Accepted) before R2 implementation
- No migration mutation under specification 001 (future 002 owns canonicalization)
- Vite-only cannot satisfy T042 / T055 / T066
- Implementing agent may not be the sole independent reviewer of its own Tier A work

## Not authorized yet

- Auth/session code changes
- Supabase migration mutation / documents-crypto expansion
- Dev server / dependency install
- Blanket `/speckit-implement`
- OpenMed or Graphify fork/import
- Push / PR
- Distributing unsigned builds
