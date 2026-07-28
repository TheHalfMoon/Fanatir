# R0 Frontend Entry / Route / Shell Map (T004)

**Task**: T004 — Map active frontend entry points (Vite client App/routes/Shell)  
**Recorded**: 2026-07-24 (local)  
**Baseline / start HEAD**: `cc2345cb55318998cdd15bee4f20c5bdeae8845e` (T003 completion)  
**Branch**: `docs/f0-planning-memory-bootstrap`  
**Nature**: Static source inspection only. **No** dev server, install, or code mutation.  
**Review**: Tier C  
**T004 decision**: **PASS WITH NOTES**

Absolute paths are local facts. Activity classes are source-wiring classifications, **not** runtime proof (deps absent; server not run).

---

## 1. Bootstrap / import chain

```text
afia-ui/client/index.html
  └─ <script type="module" src="/src/main.tsx">
       └─ afia-ui/client/src/main.tsx
            ├─ side effect: localStorage theme → documentElement.dark
            ├─ import "./index.css"
            ├─ createRoot(#root).render(<App />)
            └─ afia-ui/client/src/App.tsx
                 └─ providers + Router (wouter) + AppShell
```

Vite config (`afia-ui/vite.config.ts`): `root` = `client/`; aliases `@` → `client/src`, `@shared` → `shared`, `@kernel` → repo `lib/`.

| Path | Symbol | Importer | Reachable from Vite bootstrap? | Activity | Side effects / significance |
| --- | --- | --- | --- | --- | --- |
| `afia-ui/client/index.html` | HTML shell | Vite | **verified active** | Document title “AFIA”; Google Fonts; favicon via `/manus-storage/...`; Umami script placeholders `%VITE_ANALYTICS_*%` | Branding + optional analytics + Manus storage URL |
| `afia-ui/client/src/main.tsx` | `createRoot` render | HTML | **verified active** | Theme flash prevention via `localStorage` | First paint theming |
| `afia-ui/client/src/index.css` | global CSS | `main.tsx` | **verified active** | Global styles | — |
| `afia-ui/client/src/App.tsx` | `App` default export | `main.tsx` | **verified active** | Mounts providers, palette, router; `preloadDefaultAnalysisModel()` | Auth + navigation authority surface |
| Vite Manus plugins (config) | debug collector / storage proxy / manus-runtime | Vite server | **conditionally active** (dev server) | Debug logs / forge storage | Dev-only seams (not exercised in T004) |

**Not present**: React Query / TanStack Query provider. No `QueryClient`.

---

## 2. Provider hierarchy (from `App`)

```text
ErrorBoundary
 └─ ThemeProvider (defaultTheme="dark", switchable)
     └─ TooltipProvider
         └─ AuthProvider          ← Supabase session
             └─ TeamWorkspaceProvider
                 └─ WorkspaceProvider
                     ├─ Toaster
                     ├─ CommandPalette
                     ├─ InviteReturnHandler
                     └─ Router (wouter Switch)
```

| Provider | Path | Role | Activity |
| --- | --- | --- | --- |
| `ErrorBoundary` | `components/ErrorBoundary` | Catch render errors | **verified active** |
| `ThemeContext` | `contexts/ThemeContext` | Light/dark | **verified active** |
| `AuthContext` | `contexts/AuthContext` | Session / OTP / consent sync | **verified active** (wiring); **runtime verification required** for live Supabase |
| `TeamWorkspaceContext` | `contexts/TeamWorkspaceContext` | Team/workspace state | **verified active** (wiring) |
| `WorkspaceContext` | `contexts/WorkspaceContext` | Shell chrome (e.g. secondary collapse) | **verified active** |

---

## 3. Route inventory (`App.tsx` `Router`)

Router: **wouter** `Switch` / `Route` / `Redirect`.

| URL pattern | Component | Public/private | Guard / wrapper | Product-space | Evidence |
| --- | --- | --- | --- | --- | --- |
| `/login` | `Login` | **public** | none | authentication | `pages/Login.tsx` |
| `/invite/:token` | `InviteAccept` | **public** | none | CoLab / teams (invite) | `pages/InviteAccept.tsx` |
| `/` | `Home` | private | `PrivateRoute` → `Shell` | home / dashboard | `pages/Home.tsx` |
| `/patients` | `Patients` | private | PrivateRoute + Shell | Patient / clinical | `pages/Patients.tsx` |
| `/patients/:id` | `PatientDetail` | private | PrivateRoute + Shell | Patient / clinical | `pages/PatientDetail.tsx` |
| `/schedule` | `Schedule` | private | PrivateRoute + Shell | Patient / clinical | `pages/Schedule.tsx` |
| `/inbox` | `Inbox` | private | PrivateRoute + Shell | Patient / clinical | `pages/Inbox.tsx` |
| `/assistant` | `Assistant` | private | PrivateRoute + Shell | AI / model | `pages/Assistant.tsx` |
| `/documents` | `DocumentStudio` | private | PrivateRoute + Shell | **Studio** | `pages/DocumentStudio.tsx` |
| `/analytics` | `Analytics` | private | PrivateRoute + Shell | **Fanatir Lab** (nav label “Lab”) | `pages/Analytics.tsx` |
| `/analytics/report` | `AnalyticsReport` | private | PrivateRoute **only** (no Shell) | Fanatir Lab / report | `pages/AnalyticsReport.tsx` |
| `/research` | `MyResearch` | private | PrivateRoute + Shell | **Research** | `pages/MyResearch.tsx` |
| `/workspace/:id` | `WorkspaceSettings` | private | PrivateRoute + Shell | CoLab / teams / admin | `pages/WorkspaceSettings.tsx` |
| `/batch` | `BatchProcess` | private | PrivateRoute + Shell | Studio | `pages/BatchProcess.tsx` |
| `/compare` | `ModelCompare` | private | PrivateRoute + Shell | Studio / AI | `pages/ModelCompare.tsx` |
| `/deidentify` | `Deidentify` | private | PrivateRoute + Shell | Studio / privacy tooling | `pages/Deidentify.tsx` |
| `/models` | `Models` | private | PrivateRoute + Shell | Studio / AI | `pages/Models.tsx` |
| `/insights` | `Insights` | private | PrivateRoute + Shell | Patient / clinical | `pages/Insights.tsx` |
| `/settings` | `Settings` | private | PrivateRoute + Shell | administration/settings | `pages/Settings.tsx` |
| `/settings/:section` | `Settings` | private | PrivateRoute + Shell | administration/settings | same |
| `/404` | `NotFound` | private | PrivateRoute (no Shell) | unresolved | `pages/NotFound.tsx` |
| (fallback) | `NotFound` | private | PrivateRoute (no Shell) | unresolved | catch-all Route |

**Dynamic params**: `:id` (patients), `:token` (invite), `:section` (settings), `:id` (workspace).

**Nav sources** (`data/nav.ts`): `railNav` (home/research/settings); `studioNav`; `labNav` (Analytics tabs via query `?tab=`); `clinicalNav` (command palette); `aiNav` → `/assistant`.

**Not classified as active product surfaces**: **Montada**, **Pictorial** — no symbols/routes in `afia-ui/client/src` (only “Vivaldi feel” note in `afia-ui/ideas.md`, which is **desired future / ideation**, not implementation).

**commandF / FHIR**: No dedicated `/fhir` or `/commandf` route. FHIR export appears as UI capability via OpenMed client / `FhirExportModal` (Studio-adjacent), not a top-level route.

---

## 4. Auth / access boundary (observe-only)

| Item | Current source behavior | Classification |
| --- | --- | --- |
| `AuthProvider` | `supabase.auth.getSession` + `onAuthStateChange`; sets `session`/`user`/`loading` | **verified active** wiring |
| `PrivateRoute` (inline in `App.tsx`) | If `loading` → spinner; if `!session` → `<Redirect to="/login" />`; else children | **verified active** |
| Unauthenticated access to private routes | Redirect to `/login` | source-level |
| Public routes | `/login`, `/invite/:token` | source-level |
| `ProfileGate` | **No symbol** in `afia-ui` (rg empty) | **absent** — do not invent |
| Consent / profile | `syncProfileConsent` upserts `profiles` (incl. `consent_given_at`) on auth; signup stores pending consent in `localStorage` then OTP | **verified active** wiring; **runtime verification required** |
| Sign-in | OTP `signInWithOtp` (`shouldCreateUser: false`) | source-level |
| Sign-up | OTP with `shouldCreateUser: true` + consent required | source-level |
| Sign-out | `supabase.auth.signOut()` | source-level |
| Supabase client | `lib/supabase.ts` requires `VITE_SUPABASE_URL` + `VITE_SUPABASE_ANON_KEY` or **throws at module load** | **runtime verification required**; env absent in tree |
| Security correctness | Not asserted — no tests run | observation only |

`InviteReturnHandler`: after session, may redirect to `/invite/:token` from stored return token.

---

## 5. Active shell / layout

**Actual active shell**: `components/shell/AppShell.tsx` wrapped by private routes (except `/analytics/report`, `/404`, catch-all NotFound, and public routes).

Composition:

```text
AppShell
 ├─ TopBar
 ├─ BreadcrumbBar
 ├─ flex row
 │   ├─ PrimaryRail          ← railNav
 │   ├─ SecondaryBar         ← filters only for patients/schedule/inbox; else null
 │   ├─ main content outlet  ← {children}
 │   └─ Inspector
 └─ StatusBar                ← probes OpenMed bridge health
```

| Item | Class |
| --- | --- |
| `AppShell` + chrome above | **verified active** (wired) |
| SecondaryBar filters | **conditionally active** by route |
| “Arc/Vivaldi feel” primary rail | **desired future / ideation** in `ideas.md` — **not** a separate implemented shell |
| Archived desktop UI | `_archived/apps-desktop/ui` — **archived**, not Vite root |
| Duplicate router shells | Single App router; no second top-level React app under `afia-ui/client` |

---

## 6. Reachable product surfaces (from route graph)

| Surface | Path | Route reachability | Primary deps | Activity | Migration relevance |
| --- | --- | --- | --- | --- | --- |
| Home | `/` | private + shell | — | verified active wiring | keep as UI surface |
| Document Studio | `/documents` | private + shell | `openmed-client`, charts, FHIR modal | verified active wiring | Studio → UI-zero-authority target |
| Batch / Compare / De-id / Models | `/batch` `/compare` `/deidentify` `/models` | private + shell | `openmed-client` | verified active wiring | Studio / AI |
| Assistant | `/assistant` | private + shell | `openmed-client` | verified active wiring | AI |
| My Research | `/research` | private + shell | — | verified active wiring | Research |
| Analytics / Lab tabs | `/analytics?tab=` | private + shell | analytics pages | verified active wiring | Lab (UI) |
| Analytics report | `/analytics/report` | private, **no shell** | — | verified active wiring | Lab |
| Patients / detail / schedule / inbox / insights | clinical routes | private + shell | `@/data/patients`, `@kernel/*` via `kernel-adapter` | verified active wiring | clinical; kernel under `lib/` |
| Workspace settings | `/workspace/:id` | private + shell | `team-workspaces` → Supabase | verified active wiring | CoLab |
| Settings | `/settings` | private + shell | — | verified active wiring | admin |
| Login / Invite | public | — | Auth / invites | verified active wiring | auth/CoLab |

**Runtime verification required** for all surfaces that call Supabase, OpenMed (`127.0.0.1:8765`), or `@kernel` — T004 did not execute the app.

---

## 7. Inactive / orphaned / archived / contradictory

| Item | Class | Notes |
| --- | --- | --- |
| Root workspace `apps/desktop/ui` | **contradictory / missing** | Not the Vite app; archived under `_archived/apps-desktop/ui` |
| `afia-ui/ideas.md` Vivaldi notes | **scaffold / ideation** | Not current implementation authority |
| Montada / Pictorial | **not present** in active UI source | Must not be treated as active |
| `ProfileGate` | **absent** | — |
| `settingsNav` export | **deprecated alias** to `railNav[2]` | Still in `nav.ts` |
| Global Vite Manus favicon/storage | **conditionally active** | Needs forge env at runtime |
| All `pages/*.tsx` | **imported by App** | No orphan page modules found under `pages/` |

---

## 8. Direct frontend authority / network / security seams

These are **current UI behaviors**, not ratified Trusted Host authority.

| Seam | Path / mechanism | Concern |
| --- | --- | --- |
| Supabase Auth + profiles upsert | `AuthContext`, `lib/supabase.ts` | Session, consent, PHI-adjacent profile rows |
| Supabase Edge `documents-crypto` | `lib/documents.ts` | Document crypto / storage |
| Supabase Edge `workspace-invites` | `lib/team-workspaces.ts` | Invite/team mutations |
| Supabase `audit_log` insert | `lib/audit.ts` | Audit writes from UI |
| OpenMed bridge HTTP | `services/openmed-client.ts` → `http://127.0.0.1:8765` | analyze, PII, de-id, upload, FHIR export, ask-document, models |
| `@kernel/*` imports | `data/kernel-adapter.ts` → repo `lib/` | Clinical domain logic in shared TS kernel (not Rust host) |
| Analytics Umami script | `index.html` placeholders | External analytics if env set |
| Manus storage / debug | Vite plugins + favicon URL | Dev telemetry / external storage proxy |
| File upload paths | Document Studio / batch via OpenMed | Document / possible PHI handling |

**Target architecture reminder**: UI-zero-authority under Rust Trusted Host — current code **does not** match that target; recorded as migration debt.

---

## 9. Runtime-verification gaps

- No `node_modules` → app not started (T002).
- No `.env` in tree → Supabase client would throw if module evaluated without env.
- OpenMed bridge reachability unknown.
- Auth redirect / session persistence not observed live.
- SecondaryBar filter UX and Inspector data sources not runtime-proven.

---

## 10. Migration implications

1. Preserve route inventory and shell chrome as the **current** UX map for reconstitution.
2. Extract / replace direct Supabase + OpenMed + kernel authority with Trusted Host IPC later (R2+).
3. Do not rename AFIA→Fanatir in production paths during R0.
4. Lab/Studio/Research/CoLab/clinical partitions already appear in nav + routes — useful for ADR surface ownership, not proof of backend readiness.
5. `AnalyticsReport` without shell is an intentional layout exception to preserve.

---

## 11. Acceptance

| Criterion | Result |
| --- | --- |
| Entry/route map matches disk | **Met** |
| Documentation only | **Met** |
| No PHI dumps | **Met** |

## 12. Rollback

Delete this file; revert program-memory pointer edits.

## 13. Notes (PASS WITH NOTES)

- Static wiring map only; runtime unverified.
- Frontend currently exercises network/auth/document/AI seams incompatible with eventual UI-zero-authority — observed, not fixed.
- Naming remains “AFIA” in UI title/strings.
