# R0 Auth / Session / PrivateRoute / Profile Behavior (T005)

**Task**: T005 — Verify auth/session/PrivateRoute/profile behavior (observe-only)  
**Recorded**: 2026-07-24 (local)  
**Baseline / start HEAD**: `00fc4bf0f365d451c4bbbd94081468023f69f57b` (T004 completion)  
**Branch**: `docs/f0-planning-memory-bootstrap`  
**Nature**: Source observation only. **No** login, OTP, env creation, dev server, or install.  
**Review**: **Tier A** (auth/session compatibility observation) — implementing agent is **not** the sole independent reviewer of this Tier A work.  
**Independent Tier A review**: completed 2026-07-24 by a separate reviewing agent; founder accepted as **PASS WITH NON-BLOCKING NOTES**.  
**T005 decision**: **PASS WITH NON-BLOCKING NOTES**

Absolute paths are local facts. Classifications are source-based, not runtime proof.

---

## 1. Source-path inventory

| Path | Role |
| --- | --- |
| `afia-ui/client/src/lib/supabase.ts` | Client init; env gate |
| `afia-ui/client/src/contexts/AuthContext.tsx` | AuthProvider / session / OTP / consent sync |
| `afia-ui/client/src/App.tsx` | `PrivateRoute`, `InviteReturnHandler`, router |
| `afia-ui/client/src/pages/Login.tsx` | Sign-in / sign-up UI |
| `afia-ui/client/src/pages/InviteAccept.tsx` | Invite accept + return-token helpers |
| `afia-ui/client/src/contexts/TeamWorkspaceContext.tsx` | Workspace membership/role UI state |
| `afia-ui/client/src/contexts/WorkspaceContext.tsx` | Shell chrome (palette/inspector/pins) — **not** team auth |
| `afia-ui/client/src/lib/team-workspaces.ts` | Supabase tables + `workspace-invites` Edge Function |
| `afia-ui/client/src/components/shell/AccountMenu.tsx` | Sign-out → `/login` |
| `afia-ui/client/src/pages/Settings.tsx` | Sign-out → `/login` |
| `afia-ui/client/src/lib/audit.ts` | Optional `audit_log` inserts |
| `afia-ui/client/src/const.ts` | Unused-looking OAuth portal helper (`getLoginUrl`) |
| `afia-ui/client/src/lib/database.types.ts` | Typed `profiles.consent_given_at` |

---

## 2. Auth bootstrap sequence (source-verified)

```text
Module load: lib/supabase.ts
  ├─ read import.meta.env.VITE_SUPABASE_URL
  ├─ read import.meta.env.VITE_SUPABASE_ANON_KEY
  └─ if either missing → throw Error (app cannot mount AuthProvider path)
       else createClient({ auth: persistSession, autoRefreshToken, detectSessionInUrl })

App mount: AuthProvider
  ├─ state: session=null, user=null, loading=true
  ├─ effect:
  │    ├─ supabase.auth.getSession().then → set session/user; loading=false
  │    │     (no .catch observed — hang/error path: runtime verification required)
  │    └─ onAuthStateChange → set session/user; loading=false;
  │         if user → void syncProfileConsent(user)
  └─ cleanup: subscription.unsubscribe(); active=false
```

**Classifications**: source-verified wiring; **dependent on Supabase runtime**; **runtime verification required** for live session restore.

---

## 3. Session state machine (source)

| State | Meaning | Transitions |
| --- | --- | --- |
| `loading=true` | Initial / undecided | → false after getSession resolve or auth-state event |
| `session=null`, `loading=false` | Unauthenticated | PrivateRoute → `/login` |
| `session` set | Authenticated | PrivateRoute renders children |
| Auth event (sign-in/out/refresh) | `onAuthStateChange` | Updates session/user; may sync profile |

**Representations**: Supabase `Session | null`, `User | null` from `@supabase/supabase-js`.

**Context API shape** (`AuthContextType`): `user`, `session`, `loading`, `signIn`, `signOut`, `signUp`.

---

## 4. Login / logout sequence

### Sign-in (`signIn` / Login mode `signin`)

1. UI validates non-empty email.
2. `supabase.auth.signInWithOtp({ email, options: { shouldCreateUser: false, emailRedirectTo: window.location.origin } })`.
3. UI shows “Check your email” on success; does **not** set session locally until magic-link callback restores session via Supabase client (`detectSessionInUrl: true`).

### Sign-up (`signUp` / Login mode `signup`)

1. Requires checkbox consent in UI **and** `consent===true` in `signUp`.
2. Writes `localStorage["afia:consent-pending:"+email]` timestamp.
3. OTP with `shouldCreateUser: true`, `emailRedirectTo: window.location.origin`.
4. On OTP error, removes pending consent key.

### Logout

- `signOut` → `supabase.auth.signOut()` only (no explicit localStorage wipe of consent/pins).
- `AccountMenu` / `Settings` then `setLocation("/login")`.

**Login `?next=`**: Only special-cased when `next` starts with `/invite/` → stores invite return token. **General deep-link return after login is not preserved by PrivateRoute.**

**OAuth portal helper** (`getLoginUrl` in `const.ts` using `VITE_OAUTH_PORTAL_URL`, `VITE_APP_ID`): **defined but no importers found anywhere in `client/src`** (T005 scan + Tier A review grep) — likely legacy relative to the OTP Login page. **NOT OBSERVED** as reachable from any active auth flow; only becomes a concern if a non-static reference exists at runtime.

---

## 5. PrivateRoute behavior table

Defined **inline** in `App.tsx` (not a separate module).

| Aspect | Behavior | Classification |
| --- | --- | --- |
| Inputs | `useAuth().session`, `loading`; `children` | source-verified |
| Loading | Renders full-screen `AuthLoading` spinner | source-verified |
| Missing session | `<Redirect to="/login" />` | source-verified |
| Original destination preserved? | **No** — fixed `/login` without `?next=` for general private routes | source-verified |
| Checks beyond authentication? | **No** — does **not** check roles, workspace membership, consent, profile completeness, or patient access | source-verified **absent** |
| Protected content before decision? | **No** — waits on `loading` | source-verified |
| Auth-state transitions | Re-evaluates on render when context updates | source-verified |
| Supabase init errors | If client module threw, app fails earlier; PrivateRoute itself has no error branch | conditionally executed / **runtime verification required** |
| ProfileGate / ConsentGate equivalents | **Absent** (no symbols; PrivateRoute is session-only) | **absent** |

---

## 6. Profile and consent flow

`syncProfileConsent(user)` on each authenticated `onAuthStateChange` with user:

1. Read `localStorage` pending consent for email.
2. `profiles.select("consent_given_at").eq("id", user.id).maybeSingle()`.
3. On fetch error: `console.error` and **return** (does **not** block session / PrivateRoute).
4. `consent_given_at` = existing value **or** now ISO if pending consent **or** `null`.
5. `profiles.upsert({ id, email, consent_given_at }, { onConflict: "id" })`.
6. On upsert error: log and return (does **not** block auth).
7. Clear pending consent key on success.

| Question | Answer | Class |
| --- | --- | --- |
| Automatic profile creation? | Yes via upsert when auth event fires | source-verified; Supabase-dependent |
| Table | `profiles` | source-verified |
| Fields written | `id`, `email`, `consent_given_at` | source-verified |
| Idempotent? | Upsert on `id` | source-verified |
| Errors block login? | **No** — errors swallowed to console | source-verified |
| Profile completion required for routes? | **No** | absent |
| Consent enforced for protected routes? | **Recorded** on signup path; **not** enforced by PrivateRoute | source-verified |
| ProfileGate-equivalent elsewhere? | **Not observed** under other names for route gating | absent |

---

## 7. Invite and workspace flow

### Invite

| Step | Behavior |
| --- | --- |
| Unauthenticated `/invite/:token` | Store token in `sessionStorage["afia:invite-return"]`; redirect `/login?next=/invite/:token` |
| Login reads `next` | If `/invite/...`, `storeInviteReturnToken` |
| After session | `InviteReturnHandler` consumes token → navigate `/invite/:token` |
| Authenticated accept | `acceptWorkspaceInvite(token)` → Edge `workspace-invites` action `accept` → refresh workspaces → set active → toast → `/research` |

### TeamWorkspaceProvider (membership)

- Depends on `session`; lists via `workspace_members` join `workspaces`.
- `activeWorkspaceId === null` means “Personal library”.
- `myRole` from membership row; `canEditInActiveWorkspace` if personal **or** role `owner`/`editor`.
- Used to disable some Studio edits (UI-only).
- Load failures: empty workspaces (catch).

### WorkspaceProvider (shell)

- Palette / assistant / secondary / inspector / pinned / recents.
- Persists `afia:pinned`, `afia:recents` in localStorage (default pin includes demo id `MRN-04261`).
- **Not** authentication or membership.

---

## 8. Supabase auth dependency map

| Dependency | Detail |
| --- | --- |
| Env vars (names only) | `VITE_SUPABASE_URL`, `VITE_SUPABASE_ANON_KEY`; also referenced elsewhere: `VITE_OAUTH_PORTAL_URL`, `VITE_APP_ID`, analytics placeholders |
| Auth methods | `getSession`, `onAuthStateChange`, `signInWithOtp`, `signOut`, `getUser` |
| Persistence | `persistSession: true`, `autoRefreshToken: true`, `detectSessionInUrl: true` (browser storage via Supabase client — exact storage key **runtime verification required**) |
| Tables | `profiles`, `workspace_members`, `workspaces`, `workspace_invites`, `audit_log` |
| Edge Functions | `workspace-invites` (create_workspace / create / accept); documents-crypto is document path (adjacent, not core auth) |
| RPCs | **Not observed** in AuthContext path |
| Realtime | **Not observed** for auth |

**No secrets or key values recorded.**

---

## 9. Auth vs workspace vs patient authorization

| Layer | Mechanism | Enforced by PrivateRoute? |
| --- | --- | --- |
| **Authentication** | Supabase session presence | **Yes** (only check) |
| **Project membership** | Not modeled as separate gate | N/A / absent |
| **Workspace membership / role** | `TeamWorkspaceContext` + table/Edge; UI `canEditInActiveWorkspace` | **No** |
| **Patient authorization** | No patient ACL gate in PrivateRoute; clinical pages use local/`@kernel` data patterns | **No** / **NOT OBSERVED** as auth gate |
| **Consent / profile completeness** | Upsert side effect | **No** |

Do not conflate these layers.

---

## 10. Error and loading behavior

| Case | Behavior |
| --- | --- |
| Auth loading | Spinner; no private children |
| OTP / form errors | Returned as string to Login UI |
| Profile sync errors | Logged; session continues |
| Workspace list errors | Empty list |
| Invite accept errors | Error UI + message |
| Missing Supabase env | Throw at client module import |

---

## 11. Security observations

Categories: **VERIFIED BEHAVIOR** · **VERIFIED TRUST SEAM** · **VERIFIED SECURITY DEFECT** · **POTENTIAL RISK — runtime or RLS verification required** · **NOT OBSERVED** · **OUT OF SCOPE**.

| Observation | Classification |
| --- | --- |
| Session persistence config (`persistSession: true`, `autoRefreshToken: true`, `detectSessionInUrl: true` in `lib/supabase.ts`) | Configuration is **VERIFIED BEHAVIOR**; the exact browser storage mechanism/key is an SDK implementation detail not established by repo source — **POTENTIAL RISK — runtime verification required** (token storage details not dumped) |
| `emailRedirectTo: window.location.origin` (`AuthContext.signIn` / `AuthContext.signUp`) | Same-origin, static destination in source — **VERIFIED BEHAVIOR**. No arbitrary-destination redirect observed; open redirect **NOT OBSERVED** in source. Effective callback safety still depends on the Supabase redirect allow-list — **POTENTIAL RISK — runtime verification required** |
| Invite token transits URL path and `sessionStorage["afia:invite-return"]` (`InviteAccept.storeInviteReturnToken` / `consumeInviteReturnToken`; `Login` `?next=` effect) | Transit path is **VERIFIED BEHAVIOR**; exposure impact (referrer/history/lifetime, server-side token handling) — **POTENTIAL RISK — runtime or RLS verification required** |
| Consent defaults to `null` if no pending and no existing (`syncProfileConsent`) | **VERIFIED BEHAVIOR**; forced-true consent **NOT OBSERVED** |
| Profile upsert races (multiple auth events) | **POTENTIAL RISK — runtime verification required** |
| Profile/consent synchronization failure (`syncProfileConsent`: fetch or upsert error → `console.error` + return) | **VERIFIED BEHAVIOR — fail-open synchronization**: authentication continues when profile/consent synchronization fails; consent recording may silently not persist. **Not an authentication bypass** — no auth decision depends on this sync in source. Whether server policy expects consent/profile rows is deferred to T006 |
| Role checks only in UI (`canEditInActiveWorkspace` in `TeamWorkspaceContext`; `isOwner` in `WorkspaceSettings`) | **VERIFIED TRUST SEAM** (frontend-only gate). Authorization bypass is **not** claimed: RLS / Edge Function / server enforcement is unverified until T006 |
| Workspace edit checks only in UI (`DocumentStudio` disabled state, `MoveToWorkspaceMenu` early return) | **VERIFIED TRUST SEAM** — same T006 dependency |
| Direct DB writes from UI | **VERIFIED TRUST SEAM** — exact operations: `profiles` upsert (`AuthContext.syncProfileConsent`); `workspace_members` delete (`team-workspaces.leaveWorkspace`); `workspace_invites` delete (`team-workspaces.revokeWorkspaceInvite`); `audit_log` insert (`lib/audit.logAction`). Direct client writes are not automatically vulnerabilities when protected by RLS; server-side enforcement is **unknown until T006**. Divergence from the UI-zero-authority target is a migration obligation, not a proven defect |
| Patient access ≠ workspace membership | **VERIFIED BEHAVIOR** distinction: PrivateRoute does not bridge them; clinical pages read local/`@kernel` data (`pages/PatientDetail.tsx` → `data/kernel-adapter`) |
| Auth event audit | `audit_log` helper exists for resource actions (`lib/audit.ts` `logAction`); mandatory login/logout audit in AuthContext **NOT OBSERVED** |
| Debug/telemetry of auth/session values | Dev-only: `afia-ui/vite.config.ts` `vitePluginManusDebugCollector.transformIndexHtml` returns unmodified HTML when `NODE_ENV === "production"`; otherwise injects `client/public/__manus__/debug-collector.js`, which intercepts fetch/XHR and posts console/network/session logs to `.manus-logs/*.log`. Source proves: `sanitizeValue` redacts token/session-named keys in request/response **bodies**, but captured request headers (`entry.request.headers`) and response headers (`entry.response.headers`) are **not** passed through `sanitizeValue`, and UI events capture `location.href`. Whether Supabase `Authorization`/`apikey` headers or auth-callback URLs actually enter these logs depends on SDK request shape and auth flow type — **POTENTIAL RISK — runtime verification required** (dev-only scope is source-verified) |
| Login `?next=` only for invite prefix (`Login` effect; token = `next.split("/").pop()`, single path segment) | **VERIFIED BEHAVIOR** — non-invite `next` ignored; mitigates redirect abuse via `next` for that path |

**No VERIFIED SECURITY DEFECT was established by T005 source review.** No vulnerability claim beyond source-supported classifications above.

---

## 12. Compatibility-preservation boundary

Until a dedicated auth migration specification + accepted ADR authorize changes, preserve **externally observable** behavior:

| Preserve (compatibility) | Treat as internal detail (may change later with ADR) | Security concern → later ADR / spec 002 / auth migration |
| --- | --- | --- |
| AuthContext public API: `user`, `session`, `loading`, `signIn`, `signOut`, `signUp` | Exact consent localStorage key strings | UI-direct `profiles` upsert |
| Session loading semantics (`loading` gate) | Console error strings | UI-only role gates |
| PrivateRoute: loading spinner; unauthenticated → `/login` | Inline PrivateRoute location in `App.tsx` vs extracted module | Missing destination restore for general deep links |
| Login OTP email flows (sign-in vs sign-up + consent checkbox) | Theme/pin localStorage | Invite token in URL/sessionStorage |
| Logout clears Supabase session; UI navigates to `/login` | WorkspaceProvider chrome state | Swallowing profile sync errors |
| Sign-up consent submission side effect: consent durably recorded (currently `profiles.consent_given_at`) | Upsert mechanism + localStorage pending-consent handoff | UI-direct `profiles` write (mechanism) |
| Invite return token → `/invite/:token` after auth | Edge function action names (unless clients depend) | — |
| Team workspace list/active id expectations for CoLab UI | — | — |

Explicitly **not** frozen by this boundary (replaceable via later accepted ADR/spec): Supabase as implementation provider; direct browser database writes; internal hook/component structure; unsafe UI-only authorization checks; existing telemetry/debug mechanisms (Manus collector). Do **not** change any of the above behaviors in R0 tasks.

---

## 13. Runtime-verification gaps

- No deps / no `.env` / no live Supabase.
- Token storage key contents unknown.
- Magic-link callback and `detectSessionInUrl` not exercised.
- getSession rejection/hang behavior unproven.
- Explicitly deferred to **T006** (not guessed in T005): RLS policies; migration truth; database constraints; service-role usage; Edge Function authorization (`workspace-invites`, `documents-crypto`); audit-table enforcement; whether client role restrictions are backed by server policy; exact Supabase document and PHI storage behavior.

---

## 14. Migration implications

- Strangler must keep PrivateRoute session semantics until ADR-authorized replacement.
- Move profile/consent/workspace authority behind Trusted Host / future auth migration spec — not ad hoc UI edits.
- Separate patient authorization design from workspace roles (currently conflation risk at product level, not PrivateRoute).

---

## 15. Acceptance

| Criterion | Result |
| --- | --- |
| Behavioral contract recorded | **Met** |
| ProfileGate absent confirmed | **Met** (no symbol; no equivalent route gate) |
| Documentation only; no credentials | **Met** |
| Tier A independent review | **Complete** — founder accepted PASS WITH NON-BLOCKING NOTES |

## 16. Rollback

Delete this file; revert program-memory pointer edits.

## 17. Notes (PASS WITH NON-BLOCKING NOTES)

- Source-only; runtime/RLS unverified until T006+.
- Session-only PrivateRoute is intentional current behavior, not claimed secure.
- Direct Supabase writes and UI-only role checks are **VERIFIED TRUST SEAM**s, not proven vulnerabilities.
- No **VERIFIED SECURITY DEFECT** established by T005 source inspection.
- `getLoginUrl` is defined in `const.ts` with **no importers** — not observed as active.
