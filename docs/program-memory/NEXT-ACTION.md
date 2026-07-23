# NEXT-ACTION

## Authorized next action

Plan 001 is founder-accepted. After the local plan-acceptance commit is clean, run:

```text
/speckit-tasks
```

Do **not** run `/speckit-implement` until tasks exist and stage entry criteria are met.

**Hard gates (from accepted plan):**

- **ADR-15** accepted before R2 implementation
- **ADR-05, ADR-07, ADR-08, ADR-09, ADR-10, ADR-14** before relevant R3 work
- **Signing custody controls (Decision D)** before R5 distribution

Active feature directory:

```text
specs/001-fanatir-repository-and-architecture-reconstitution
```

Future authorized (not started): `002-supabase-local-first-and-migration-canonicalization`

## Not authorized yet

- Product feature implementation
- `/speckit-implement`
- OpenMed or Graphify **fork/import** execution
- Creating or executing specification 002
- Fehrest or DeepMed-AI product initialization beyond planning/contracts
- Package rename, file moves/deletions for migration
- UI redesign or auth/session/`PrivateRoute`/profile behavior changes
- Expanding, deleting, or mutating `documents-crypto` / Supabase migrations
- Pictorial or Montada activation
- Pushing branches or opening pull requests
- Distributing unsigned builds
