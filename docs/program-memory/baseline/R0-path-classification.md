# R0 Path Classification Register (T010)

**Task**: T010 — Classify paths active/scaffold/archived/orphaned/contradictory
**Recorded**: 2026-07-24 (local)
**Baseline / start HEAD**: `fc3069202e5f13ad80fe768a9f9b1046ef234a25` (T009 completion)
**Branch**: `docs/f0-planning-memory-bootstrap`
**Nature**: Documentation-only classification. **No** path moves, restores, manifest repairs, installs, builds, or sibling mutation.
**Review**: **Tier B** — independent review **APPROVE WITH NOTES**; governance-authority + precision corrections applied.
**T010 decision**: **PASS WITH NOTES** (pending founder acceptance)

Governing evidence (summaries verified against paths for consequential rows): Constitution; Spec 001 disposition matrix; accepted plan/tasks; ADR-15 **draft** (non-authoritative); `R0-*.md` baselines; root manifests; filesystem.

Activity classification ≠ plan disposition. Both recorded where useful.
**Exactly one primary classification per register row.** Secondary notes and plan dispositions are not primary classes.

---

## 1. Classification definitions (primary — exactly one)

| Class | Meaning |
| --- | --- |
| **VERIFIED ACTIVE** | Reachable/invoked from a verified active entry point, or used by the accepted planning workflow |
| **CONDITIONALLY ACTIVE** | Reachable only under documented config, route, feature, or development mode |
| **REFERENCED BUT RUNTIME-UNVERIFIED** | Active caller or manifest reference exists; execution not proven in R0 |
| **SCAFFOLD ONLY** | Structure/placeholders without substantive implementation |
| **ARCHIVED** | Under archival boundaries or formally removed from active authority |
| **ORPHANED** | Source/repository search establishes **all** of: no active importer; no route; no manifest reference; no workflow or script reference; no accepted governance role; no documented conditional activation |
| **CONTRADICTORY** | Conflicts with filesystem, another declared authority, or verified behavior |
| **DERIVED / REBUILDABLE** | Generated analysis useful but non-canonical |
| **GOVERNANCE AUTHORITY** | **Only**: founder-ratified Constitution; founder-accepted Spec 001; founder-accepted plan 001; founder-accepted task program 001; already accepted governance decisions |
| **PLANNING EVIDENCE** | Baseline reports, research, contracts drafts, **ADR drafts**, program-memory, legacy plans — inform decisions; do **not** implement product and are **not** architecture gates until separately accepted |
| **EXTERNAL SIBLING PRODUCT** | Fehrest or DeepMed-AI boundaries (read-only from Fanatir T010) |
| **GENERATED / IGNORED** | Build/cache/env/editor/generated paths outside canonical source authority |
| **REQUIRES INVESTIGATION** | Evidence insufficient for a defensible primary class (including incomplete fine-grained reachability) |

**Rule — ADR drafts:** `draft ADR ≠ accepted authority`. Every future ADR draft is **PLANNING EVIDENCE** until accepted at its gate (for ADR-15 and the R1 package: **T030**). Implementation must not cite an ADR draft as an accepted gate.

**Retain** (disposition) does **not** imply reactivation of archives.
Graphify is **DERIVED / REBUILDABLE** only — never orphan proof or governance authority.

---

## 2. Path-family register

### 2.1 Root manifests and instructions

| Path / family | Primary class | Current role | Evidence | Caller/manifest | Authority now | Runtime/build | Sensitive | Migration | Contradiction / uncertainty | R0 permitted | R0 prohibited | Follow-up | Plan disposition |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `README.md` | REQUIRES INVESTIGATION | Root narrative | file present | human readers | non-governing vs Constitution | unread end-to-end for claim ledger in T010 | low | identity/docs | may mix AFIA/target language | cite only | treat as architecture SoT | claim ledger / docs tasks | investigate |
| `package.json` (root) | CONTRADICTORY | Declares workspace `apps/desktop/ui`; scripts via `npx --yes` | package.json; R0-ci-tests | npm/pnpm consumers | misleading | not executed (fetch risk) | low | manifest truth | missing live path | observe | repair / install | adapt tasks | **adapt** |
| `pnpm-workspace.yaml` | CONTRADICTORY | Only `apps/desktop/ui` | yaml; filesystem | pnpm | misleading | — | low | same | active UI is `afia-ui/` | observe | repair | adapt | **adapt** |
| `pnpm-lock.yaml` (root) | CONTRADICTORY | Lock for stale workspace | present; Cargo.lock absent | pnpm | non-canonical for afia-ui | unverified | low | lock honesty | pairs with missing package path | observe | regenerate/repair | adapt | investigate/adapt |
| `afia-ui/pnpm-lock.yaml` | REFERENCED BUT RUNTIME-UNVERIFIED | App lockfile | afia-ui package | afia-ui pnpm | implementation | install not run (T002) | low | active app | pnpm 10.4.1 vs root 9.0.0 | observe | mass update | install later | adapt |
| `Cargo.toml` | CONTRADICTORY | Members → missing live crates/Tauri | Cargo.toml; `_archived/crates` | cargo | misleading | not buildable as declared | low | host reconstitution | archive counterparts exist | observe | repair/reactivate | adapt after ADR acceptance | **adapt** |
| `Cargo.lock` | CONTRADICTORY | Expected lock for declared Cargo workspace is **missing** | filesystem; R0-archived | — | none | not locked | low | workspace honesty | pairs with unbuildable `Cargo.toml` members | observe | invent lock without repair plan | adapt | note under C2 |
| `rust-toolchain.toml` | CONTRADICTORY | Unpinned channel; `aarch64-apple-darwin` | file | rustup | skew vs Windows-first | toolchain may install | low | packaging | macOS target vs Alpha Windows | observe | “pin as Alpha truth” without accepted ADR | tooling ADR | adapt |
| `deny.toml` | SCAFFOLD ONLY | cargo-deny config without live workspace | deny.toml; R0-ci | aspirational CI | none | unused | low | security gates | CI does not run cargo-deny | observe | claim advisory gate | T059+ | adapt |
| `go.work` | CONTRADICTORY | `use ./services/operations-go` missing | go.work; archive | go | misleading | — | low | Go disposition archive | archive at `_archived/services-operations-go` | observe | repair/revive Go | ADR if Go returns | **adapt** |
| `.gitignore` | VERIFIED ACTIVE | Ignore rules | file | git | tooling | — | may hide secrets | — | — | maintain carefully | commit secrets | — | retain |
| `.node-version` / `.python-version` | REFERENCED BUT RUNTIME-UNVERIFIED | Version pins | files | tooling | advisory | not cross-checked every shell | low | tooling | — | observe | ignore for installs | T003 | retain/adapt |
| `tsconfig.json` (root) | REQUIRES INVESTIGATION | Root TS config; may not cover afia-ui | file; boundary script | `typecheck` script | weak | not executed | low | scripts | stale vs app layout | observe | fake typecheck green | T009 | investigate |
| `AGENTS.md` | PLANNING EVIDENCE | Agent operating notes; continuity subordinate to Constitution | AGENTS.md | agents | **not** Constitution/spec/plan/tasks | — | low | — | must not override Constitution | follow subordinate to governance | override Constitution | retain | retain |
| `AfiaUI.zip` | REQUIRES INVESTIGATION | Unknown binary archive | path exists; Spec matrix | no importer found; purpose/safety not established | none | not opened | unknown binary risk | inventory | incomplete reachability/safety review | leave closed | extract as authority | inventory follow-up | **investigate** |
| `apps/README.md` | CONTRADICTORY | Documents `desktop/ui` + `src-tauri` as present layout | README; only file under `apps/` | root manifests | misleading | — | low | workspace truth | directories missing; archived copies exist | observe | create fake live apps | adapt | investigate |
| `packaging/**` | SCAFFOLD ONLY | README stubs (macos/models/python) | packaging/README* | none executable | none | — | models/signing later | R5 | macOS packaging docs vs Windows-first | observe | claim packaged | R5 tasks | adapt |
| `tools/README.md` | SCAFFOLD ONLY | Placeholder tooling dir | README | none | none | — | low | generators | TODO contract generator | observe | invent tools | later | investigate |

### 2.2 CI / tests / scripts

| Path / family | Primary class | Role | Evidence | Notes | Disposition |
| --- | --- | --- | --- | --- | --- |
| `.github/workflows/ci.yml` | SCAFFOLD ONLY | Echo TODO jobs on `macos-14` | R0-ci-tests | PLACEHOLDER; not a quality gate; macOS vs Windows-first | **adapt** |
| `.github/README.md` | CONTRADICTORY | Claims format/lint/typecheck/build/unit **gates** | R0-ci-tests | overclaim vs echo jobs | investigate (docs later) |
| `tests/**` | SCAFFOLD ONLY | README dirs only | R0-ci-tests | no executables | **adapt** |
| `scripts/agent-boundary-check.ts` | CONTRADICTORY | Root `check:boundaries`; stale Next-style paths | R0-ci-tests; package.json | broken path assumptions | investigate |
| `scripts/*-test.ts`, `scripts/fault-injection/**` | ORPHANED | Local kernel smoke helpers | Search: no App route; no `package.json` test script; not in `ci.yml`; no governance role; no documented conditional activation | Files import `lib/**` (they are callers of kernel, not product entry). Not executed in T009. Graphify not used. | investigate |
| `.specify/scripts/powershell/**` | VERIFIED ACTIVE | Spec Kit planning scripts | `.specify` workflow | Accepted planning workflow tooling | **retain** |

### 2.3 Active application (`afia-ui/`)

| Path / family | Primary class | Role | Evidence | Caller | Authority | Runtime | Sensitive tags | Disposition |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `afia-ui/` (package) | REFERENCED BUT RUNTIME-UNVERIFIED | Active frontend product surface | T002/T004; ARCHIVED.md | founder-active path | implementation fact ≠ architecture SoT | deps absent; build failed historically | auth, PHI-capable UI, network | **adapt** |
| `afia-ui/client/index.html` → `main.tsx` → `App.tsx` | VERIFIED ACTIVE | Bootstrap chain | R0-frontend-map | Vite entry | implementation fact | runtime-unverified | analytics placeholders | adapt |
| `App.tsx` routes + `PrivateRoute` + `AppShell` | VERIFIED ACTIVE | Router/shell/auth gate | R0-frontend-map; R0-auth-session | main | implementation; auth behavior **retain** (plan) | session needs live Supabase | auth/session | retain (auth) / adapt (UI) |
| `pages/**` (routed) | VERIFIED ACTIVE | Product pages | App.tsx Route list | Router | implementation fact | unverified | PHI/patient/AI/sharing by page | adapt |
| `contexts/**` | VERIFIED ACTIVE | Auth/Theme/Workspace/Team | App providers | App | implementation fact | unverified | auth, collab | retain/adapt |
| `hooks/**` | VERIFIED ACTIVE | UI hooks | importers in client | pages/components | implementation fact | — | low–med | adapt |
| `client/src/lib/supabase.ts` | REFERENCED BUT RUNTIME-UNVERIFIED | Supabase client; throws if env missing | R0-auth-session | AuthContext et al. | implementation fact | env absent in tree | auth, secrets (keys), network | investigate→adapt |
| `client/src/lib/documents.ts` | REFERENCED BUT RUNTIME-UNVERIFIED | Invokes `documents-crypto` | R0-supabase; grep | Document flows | **frozen PHI-egress** evidence | unverified | PHI, durable docs, crypto, service edge | investigate; no expand |
| `client/src/lib/team-workspaces.ts` | REFERENCED BUT RUNTIME-UNVERIFIED | Invokes `workspace-invites` | grep | WorkspaceSettings | implementation fact | unverified | auth, service-role edge, sharing | investigate |
| `client/src/services/openmed-client.ts` | REFERENCED BUT RUNTIME-UNVERIFIED | localhost OpenMed HTTP | R0-python; R0-frontend | Studio/AI pages, StatusBar | contradicts archived “UI≠Python” doctrine | bridge not run in T010 | model inference, network, FHIR-ish | investigate |
| `client/src/data/kernel-adapter.ts` | VERIFIED ACTIVE | `@kernel` → root `lib/` | vite alias; imports | clinical UI paths | implementation fact | unverified | PHI/patient structures | **adapt** |
| `client/src/data/*` fixtures (patients, clinicians, …) | CONDITIONALLY ACTIVE | Demo/static data | pages import | Patients/Schedule/… | not clinical SoT | — | may look like PHI (synthetic) | investigate fixtures policy |
| Vite Manus plugins / `ManusDialog` / manus-storage | CONDITIONALLY ACTIVE | Dev-only Manus tooling | vite.config.ts | vite dev server | non-product authority | not exercised T004 | external network, logs | investigate; do not elevate |
| `afia-ui/server/index.ts` | REFERENCED BUT RUNTIME-UNVERIFIED | Express static companion in build/start | package.json build/start | production start path | hosting helper | not run | filesystem static | adapt |
| `afia-ui/shared/**` | VERIFIED ACTIVE | Shared consts | `@shared` alias | client | implementation fact | — | low | adapt |
| `afia-ui/package.json` / vite/tsconfig | VERIFIED ACTIVE | App scripts/config | T002 | developers | implementation fact | check/build unverified without node_modules | — | adapt |
| `afia-ui/template.json` | ORPHANED | Manus/template snapshot | Search: no App/route importer; no package script reference; no CI; no governance role; no conditional activation documented | none | none | — | may embed secrets patterns | investigate |
| `afia-ui/ideas.md` | PLANNING EVIDENCE | Ideation | file | none | non-governing | — | — | investigate |
| `afia-ui/gen_patients.py` | ORPHANED | Patient generator script | Search: no App/CI/package script reference; no route; no governance role; no documented conditional activation | none | none | not run | synthetic patient data | investigate |
| `afia-ui/patches/**` | REFERENCED BUT RUNTIME-UNVERIFIED | pnpm patch for `wouter@3.7.1` | `afia-ui/package.json` → `pnpm.patchedDependencies` → `patches/wouter@3.7.1.patch` | pnpm install | implementation fact | install not run in R0 | low | adapt |

### 2.4 Root `lib/` (clinical TS kernel)

| Path | Primary class | Role | Evidence | Disposition |
| --- | --- | --- | --- | --- |
| `lib/**` (family via `@kernel` / kernel-adapter) | VERIFIED ACTIVE | Clinical kernel modules imported through Vite `@kernel` alias | ARCHIVED.md; `kernel-adapter.ts`; vite.config alias | **adapt** |
| Fine-grained `lib/**` modules without proven UI/script importers | REQUIRES INVESTIGATION | Per-module orphan census incomplete | T010 did not exhaustively map every `lib/**` file to importers; Graphify not used as proof | investigate |

Secondary note (not a primary class): even wired `lib/**` execution is runtime-unverified without installs/tests.

### 2.5 Services

| Path | Primary class | Role | Evidence | Sensitive | Disposition |
| --- | --- | --- | --- | --- | --- |
| `services/openmed_bridge.py` | REFERENCED BUT RUNTIME-UNVERIFIED | FastAPI OpenMed bridge | R0-python; UI client | model, network, possible PHI in payloads | **investigate** |
| `services/fhir_gate.py` | REFERENCED BUT RUNTIME-UNVERIFIED | FHIR R4B prototype gate | R0-python | FHIR | **investigate** |
| `services/requirements-bridge.txt` | REFERENCED BUT RUNTIME-UNVERIFIED | Bridge deps pin list | R0-python | license/runtime | investigate |
| `services/README.md` | CONTRADICTORY | Describes `operations-go/` + `ai-python/` live layout | README vs filesystem | — | investigate (docs) |

### 2.6 Supabase

| Path | Primary class | Role | Evidence | Sensitive | Disposition |
| --- | --- | --- | --- | --- | --- |
| `afia-ui/supabase/schema.sql` | CONTRADICTORY | Monolithic schema vs undated migration trees | R0-supabase | auth, RLS, PHI storage shapes | **investigate** |
| `afia-ui/supabase/migrations/workspaces.sql` | REFERENCED BUT RUNTIME-UNVERIFIED | Workspace migration | R0-supabase | collab, authz | investigate→adapt |
| `supabase/migrations/workspaces.sql` | CONTRADICTORY | Byte-identical duplicate | R0-supabase (`D3B29F5F…`) | same | **investigate** |
| `afia-ui/supabase/functions/documents-crypto/**` | REFERENCED BUT RUNTIME-UNVERIFIED | Edge crypto/docs; frozen PHI-egress | R0-supabase | PHI, secrets, crypto | investigate; **no expand** |
| `afia-ui/supabase/functions/workspace-invites/**` | REFERENCED BUT RUNTIME-UNVERIFIED | Invites; service-role use | R0-supabase | auth, service-role, email | investigate |
| `afia-ui/supabase/.temp/**` | GENERATED / IGNORED | Local link temp | path | may hold project ids | ignore |

### 2.7 Archives

| Path | Primary class | Role | Evidence | Plan disposition (activity stays ARCHIVED) |
| --- | --- | --- | --- | --- |
| `_archived/ARCHIVED.md` | ARCHIVED | Archive rationale; names active path (secondary planning note only) | file 2026-07-04 | archive |
| `_archived/crates/**` | ARCHIVED | 11 scaffold crates v0.0.0 | R0-archived | **investigate** shapes only (not restore) |
| `_archived/apps-desktop/**` | ARCHIVED | Tauri v2 + old UI | R0-archived | **replace** (successor host; do not delete archive) |
| `_archived/services-operations-go/**` | ARCHIVED | Go ops skeleton | R0-archived | **archive** |
| `_archived/services-ai-python/**` | ARCHIVED | AI python skeleton | tree | archive / investigate |
| `_archived/contracts/**` | ARCHIVED | Incomplete multi-lang contracts | R0-archived | archive / investigate |

### 2.8 Governance / planning

| Path | Primary class | Role | Disposition |
| --- | --- | --- | --- |
| `.specify/memory/constitution.md` | GOVERNANCE AUTHORITY | Founder-ratified Constitution | **retain** |
| `specs/001-.../spec.md` | GOVERNANCE AUTHORITY | Founder-accepted Spec 001 | retain |
| `specs/001-.../plan.md` | GOVERNANCE AUTHORITY | Founder-accepted plan 001 | retain |
| `specs/001-.../tasks.md` | GOVERNANCE AUTHORITY | Founder-accepted task program 001 | retain |
| `.specify/**` excluding `memory/constitution.md` and `scripts/powershell/**` (already classified) | PLANNING EVIDENCE | Spec Kit templates, workflows, integration manifests | **retain** |
| `specs/001-.../contracts/**` | PLANNING EVIDENCE | Target contracts; not implementation gates until accepted via ADR/process | retain/adapt via ADR process |
| `specs/001-.../adrs/ADR-15-*.md` | PLANNING EVIDENCE | **NON-AUTHORITATIVE ADR DRAFT.** Expresses founder-ratified Rust-first **direction**. The draft document is **not** accepted architecture authority until **T030** (R1 Architecture Gate) with the relevant R1 ADR package. Implementation must not cite this draft as an accepted gate. Rule: draft ADR ≠ accepted authority. | accept at T030 |
| `specs/001-.../research.md`, `data-model.md`, `quickstart.md`, checklists | PLANNING EVIDENCE | Supporting Spec Kit materials | retain |
| `docs/program-memory/baseline/R0-*.md` | PLANNING EVIDENCE | R0 baselines — authoritative as **records of inspected facts**; **not** product/architecture authority | retain |
| `docs/program-memory/{CURRENT-STATE,NEXT-ACTION,HOME,...}.md` | PLANNING EVIDENCE | Program memory continuity | **retain** |
| `docs/product/AFIA_MASTERPLAN.md`, `AFIA_FULL_BUILD_PLAN.md` | PLANNING EVIDENCE | Obsolete AFIA plans — **not** governance authority | **investigate** |
| `docs/product/v1-engineering-program.md`, `v1-task-ledger.md`, `docs/product/README.md` | PLANNING EVIDENCE | Legacy blueprint/ledger docs — **not** governance authority | **investigate** |
| `docs/adr/**` (README only) | SCAFFOLD ONLY | Empty ADR mirror | investigate |
| `.cursor/**` | GENERATED / IGNORED | Editor config | ignore as product authority |

### 2.9 Sibling repositories

| Path | Primary class | Role | Evidence | Disposition |
| --- | --- | --- | --- | --- |
| `C:\Projects\Fanatir-Ecosystem\Fehrest` | EXTERNAL SIBLING PRODUCT | Empty working tree (`.git` only) | T001/T010 read-only | **investigate** (init later) |
| `C:\Projects\Fanatir-Ecosystem\DeepMed-AI` | EXTERNAL SIBLING PRODUCT | README-only | T001 | **investigate** |
| `C:\Projects\Fanatir-Ecosystem\graphify-out/**` (if present) | DERIVED / REBUILDABLE | Graphify output | R0-tooling; outside Fanatir tree | non-authority; do not import |

### 2.10 Generated / ignored (representative)

| Path | Primary class | Notes |
| --- | --- | --- |
| `node_modules/`, `dist/`, `.manus-logs/`, `__pycache__/`, `target/` | GENERATED / IGNORED | Absent or local-only; not canonical |
| Env files with secrets | GENERATED / IGNORED | Not inventoried as source authority |

---

## 3. Root-manifest classification summary

Root `package.json`, `pnpm-workspace.yaml`, `Cargo.toml`, and `go.work` are **CONTRADICTORY**: they declare missing live members while archive counterparts and/or `afia-ui/` exist. `Cargo.lock` absent. `rust-toolchain.toml` is macOS-target skewed. No manifest repair authorized in T010.

---

## 4. Active application classification summary

**Active product surface** = `afia-ui/` + root `lib/` (wired family) + Python bridges under `services/*.py`, per `_archived/ARCHIVED.md` and T004–T007.

Wiring is **VERIFIED ACTIVE** for bootstrap/routes/shell/providers/kernel-adapter imports. Runtime remains largely **REFERENCED BUT RUNTIME-UNVERIFIED**. Manus tooling is **CONDITIONALLY ACTIVE** (dev). Fine-grained unwired `lib/**` candidates are **REQUIRES INVESTIGATION**, not ORPHANED.

---

## 5. Service and Supabase classification summary

| Family | Class | Notes |
| --- | --- | --- |
| OpenMed bridge / FHIR gate | REFERENCED BUT RUNTIME-UNVERIFIED | Wired from UI; prototypes; investigate |
| `services/README.md` | CONTRADICTORY | Documents absent Go/AI layout |
| Dual `workspaces.sql` | CONTRADICTORY | Identical duplicates; Spec 002 |
| `schema.sql` vs migrations | CONTRADICTORY | Authority unresolved |
| Edge functions | REFERENCED BUT RUNTIME-UNVERIFIED | documents-crypto frozen; invites service-role |

---

## 6. Archive and governance classification summary

All `_archived/**` = **ARCHIVED** (non-authoritative). Crates: investigate shapes; desktop: replace via successor; Go: archive for Alpha.

**GOVERNANCE AUTHORITY (exact set in this register):** Constitution; Spec 001 `spec.md`; plan 001 `plan.md`; tasks 001 `tasks.md`.

**ADR-15** = **PLANNING EVIDENCE — NON-AUTHORITATIVE ADR DRAFT** (Rust-first direction expressed; acceptance at **T030**). Legacy `docs/product/**` and baseline R0 files = planning evidence only.

---

## 7. Contradiction register

| # | Side A | Side B | Sources | Factual conclusion | Authority resolution | Implementation impact | Owner |
| --- | --- | --- | --- | --- | --- | --- | --- |
| C1 | `pnpm-workspace` / root package workspaces → `apps/desktop/ui` | Live UI at `afia-ui/` | manifests; T004 | Stale root workspace | Spec/plan adapt manifests later | Cannot `pnpm -w` as declared | adapt tasks |
| C2 | `Cargo.toml` members live paths | Crates only under `_archived/crates` | Cargo.toml; T008 | Missing members | Accepted ADR-15 (via T030) + adapt; no reactivate | Unbuildable Rust workspace | T030; T008 |
| C3 | `go.work` → `services/operations-go` | Only `_archived/services-operations-go` | go.work; T008 | Missing live Go | Archive disposition Alpha | No Go for Alpha | T008; ADR if return |
| C4 | CI `macos-14` placeholders | Windows-first target | ci.yml; Constitution/plan | Placeholder + platform skew | Honest CI later | No regression gate | T009; T059 |
| C5 | Archived doctrine UI never calls Python | Live `openmed-client` → localhost bridge | T007; T008 | Live contradicts archive docs | Reconstitution evidence | UI-zero-authority target | future ADR package / T030 |
| C6 | `schema.sql` + undated migrations + root duplicate | Single migration authority needed | T006 | CONTRADICTORY Supabase authority | Spec 002 | No migration mutation in 001 | Spec 002 |
| C7 | `services/README` Go/AI layout | Flat `*.py` bridges only | services/ | README stale | Docs later | Confusion risk | T007 |
| C8 | FHIR R4B prototype | commandF canonical FHIR unresolved | T007; contracts | Prototype ≠ canonical | commandF / accepted ADR | Do not claim validated FHIR | commandF spec |
| C9 | Legacy AFIA product plans | Fanatir Constitution | docs/product; .specify | Legacy not governing | Constitution wins | Cite as history only | T010 |
| C10 | Product identity Fanatir | Technical names `afia*` | package names; UI title | Dual identity unresolved | Rename spec later | Keep AFIA working | rename spec |
| C11 | Active Supabase document storage | Target local Rust Artifact Store | T006; ADR drafts | Cloud content vs local SoT target | Decision C / accepted ADRs | No PHI expand on crypto edge | Spec 002; T030 package |
| C12 | Direct UI authority / OpenMed calls | UI-zero-authority Trusted Host target | frontend; ADR drafts | Present ≠ target | Accepted ADR package at T030 | Host before expansion | R1–R2 |
| C13 | `.github/README` / tests READMEs claim gates/suites | Echo CI; empty tests | T009 | Overclaim | Honest docs later | No “CI green” product claim | T009 |
| C14 | `apps/README` documents desktop paths | `apps/` contains README only | apps/ | Misleading tree docs | Manifest adapt | False layout confidence | adapt |
| C15 | Root pnpm 9 vs afia-ui pnpm 10.4.1 | Two packageManager fields | package.json files | Tooling skew | T003 | Install confusion | tooling |

Do **not** resolve via code changes in T010. Contradiction rows citing ADR drafts mean **future accepted ADRs**, not draft-as-gate.

---

## 8. Orphan / reachability findings

| Finding | Primary class | Verification | Notes |
| --- | --- | --- | --- |
| Routed `pages/**` | VERIFIED ACTIVE | `App.tsx` Route list | Not orphaned |
| `scripts/*-test.ts`, `scripts/fault-injection/**` | ORPHANED | No App/route; no package.json script; no ci.yml; no governance; no conditional activation | Imports `lib/**` as callers; not product entry |
| `AfiaUI.zip` | REQUIRES INVESTIGATION | No importer found; purpose/safety not established | **Not** ORPHANED — incomplete review |
| `afia-ui/gen_patients.py` | ORPHANED | No App/CI/package script/route/governance/conditional activation | — |
| `afia-ui/template.json` | ORPHANED | No importer/script/CI/governance/conditional activation | — |
| `afia-ui/patches/**` | REFERENCED BUT RUNTIME-UNVERIFIED | `pnpm.patchedDependencies` | **Not** orphaned |
| `lib/**` (wired family) | VERIFIED ACTIVE | `@kernel` / kernel-adapter | — |
| Fine-grained `lib/**` without proven importers | REQUIRES INVESTIGATION | Census incomplete | **Not** ORPHANED by Graphify absence |
| `packaging/**`, `tools/**` | SCAFFOLD ONLY | README-only | Not orphaned |
| Graphify | DERIVED / REBUILDABLE | policy | Never orphan proof |

---

## 9. Sensitive / trust-boundary tags (activity unchanged)

| Tag | Example paths |
| --- | --- |
| Auth/session | `AuthContext`, `PrivateRoute`, `lib/supabase.ts` |
| Supabase | `afia-ui/supabase/**`, root duplicate migration |
| PHI / patient | Documents flows, patients pages, documents-crypto, bridges payloads |
| Model inference | `openmed-client`, `openmed_bridge.py` |
| FHIR | `fhir_gate.py`, FHIR export UI |
| Durable document storage | documents-crypto, storage services |
| Service-role | workspace-invites |
| Secrets | env keys, edge secrets (not in source) |
| External network | Supabase, OpenMed localhost, Manus hosts |
| Filesystem | Express static server; future Rust host |
| Sharing/export | invites, FHIR export, social/share libs if present |
| Audit | `client/src/lib/audit*` (wiring; not Trusted Host audit authority) |

---

## 10. Current-authority map

### Authoritative now

| Authority | Scope |
| --- | --- |
| Founder-ratified **Constitution** (`.specify/memory/constitution.md`) | Product/architecture governance |
| Founder-accepted **Spec 001** (`spec.md`) | Inventory, dispositions, requirements |
| Founder-accepted **plan 001** (`plan.md`) | Staged reconstitution plan |
| Founder-accepted **task program 001** (`tasks.md`) | Authorized task sequence and gates |
| Already accepted governance decisions recorded through that process | Binding when explicitly accepted |
| **Current source code** (`afia-ui/`, `lib/`, bridges, etc.) | **Implementation fact only** — subordinate to governance; describes what exists, not what is allowed as target architecture |

### Non-authoritative

| Item | Status |
| --- | --- |
| **ADR drafts** (including ADR-15) | PLANNING EVIDENCE — not architecture gates until T030 (or the owning acceptance task) |
| Legacy AFIA architecture / product documents (`docs/product/AFIA_*`, `v1-*`) | Historical / planning evidence only |
| `_archived/**` | Evidence only; not active authority |
| Graphify output | Derived / rebuildable |
| R0 baseline evidence (`docs/program-memory/baseline/R0-*.md`) | **Authoritative as records of inspected facts**; **not** product or architecture authority |
| Program-memory continuity docs | Planning evidence / continuity; subordinate to Constitution |
| AI conclusions / chat memory | Non-authoritative |
| Sibling repository plans not separately accepted | Non-authoritative for Fanatir gates |
| Placeholder CI “pass”, scaffolds, README overclaims | Not quality or security authority |

### Concern map (implementation vs target)

| Concern | Current fact | Explicitly not authority |
| --- | --- | --- |
| Active UI behavior | `afia-ui/` source | Target Trusted Host (unimplemented) |
| Auth/session freeze | Current PrivateRoute/AuthContext contracts (plan retain) | ProfileGate (absent) |
| Local Artifact/policy/secrets/audit | **Absent** as Rust Trusted Host | UI/Python as final SoT |
| Supabase | Provisional collab/auth evidence; contradictory schema authority | Patient/Artifact SoT |
| Fehrest / DeepMed | Sibling repos (empty / README) | Fanatir subfolders |
| CI quality | None substantive | Placeholder workflow “pass” |

---

## 11. R0 prohibited-action summary

- No move/rename/delete/restore/copy of archived or production paths
- No manifest/CI/script/test/auth/Supabase/SQL/service/env repairs
- No installs, lockfile regeneration, Rust/Tauri/Go/Python implementation
- No Fehrest/DeepMed init; no OpenMed/Graphify import
- No claiming CI green, HIPAA, validated FHIR, or Trusted Host as present
- No treating `retain` disposition as archive reactivation
- No citing ADR drafts (including ADR-15) as accepted implementation gates before T030

---

## 12. Unresolved investigation items

1. `AfiaUI.zip` purpose and safety
2. Canonical Supabase migration location / Spec 002
3. OpenMed license/runtime maturity (Decision B bounded later)
4. Root vs afia-ui packageManager and workspace repair sequencing
5. Fine-grained `lib/**` importer census (REQUIRES INVESTIGATION until complete)
6. Hosted GitHub protection/status (out of T009/T010 scope)
7. `afia-ui/template.json` provenance (orphaned; content not treated as authority)
8. Exact social-sharing client inventory depth beyond T004 seams

---

## 13. Exact classification totals

Counted from every primary-class cell in §2 register rows (one primary class per row). Secondary notes and plan dispositions are excluded.

| Primary class | Exact total |
| --- | --- |
| VERIFIED ACTIVE | 11 |
| CONDITIONALLY ACTIVE | 2 |
| REFERENCED BUT RUNTIME-UNVERIFIED | 15 |
| SCAFFOLD ONLY | 6 |
| ARCHIVED | 6 |
| ORPHANED | 3 |
| CONTRADICTORY | 13 |
| DERIVED / REBUILDABLE | 1 |
| GOVERNANCE AUTHORITY | 4 |
| PLANNING EVIDENCE | 10 |
| EXTERNAL SIBLING PRODUCT | 2 |
| GENERATED / IGNORED | 4 |
| REQUIRES INVESTIGATION | 4 |
| **Sum of register rows** | **81** |

### Count verification (row inventory)

- **VERIFIED ACTIVE (11):** `.gitignore`; `.specify/scripts/powershell/**`; bootstrap chain; App routes/PrivateRoute/AppShell; `pages/**`; `contexts/**`; `hooks/**`; `kernel-adapter.ts`; `shared/**`; `afia-ui` package.json/vite/tsconfig; `lib/**` wired family
- **CONDITIONALLY ACTIVE (2):** data fixtures; Manus tooling
- **REFERENCED BUT RUNTIME-UNVERIFIED (15):** `afia-ui/pnpm-lock.yaml`; `.node-version`/`.python-version`; `afia-ui/` package; `supabase.ts`; `documents.ts`; `team-workspaces.ts`; `openmed-client.ts`; `server/index.ts`; `patches/**`; `openmed_bridge.py`; `fhir_gate.py`; `requirements-bridge.txt`; `afia-ui/.../workspaces.sql`; `documents-crypto`; `workspace-invites`
- **SCAFFOLD ONLY (6):** `deny.toml`; `packaging/**`; `tools/README.md`; `ci.yml`; `tests/**`; `docs/adr/**`
- **ARCHIVED (6):** `ARCHIVED.md`; crates; apps-desktop; operations-go; ai-python; archived contracts
- **ORPHANED (3):** `scripts/*-test.ts`+fault-injection; `template.json`; `gen_patients.py`
- **CONTRADICTORY (13):** root `package.json`; `pnpm-workspace.yaml`; root `pnpm-lock.yaml`; `Cargo.toml`; `Cargo.lock`; `rust-toolchain.toml`; `go.work`; `apps/README.md`; `.github/README.md`; `agent-boundary-check.ts`; `services/README.md`; `schema.sql`; root `workspaces.sql`
- **DERIVED / REBUILDABLE (1):** `graphify-out/**`
- **GOVERNANCE AUTHORITY (4):** Constitution; `spec.md`; `plan.md`; `tasks.md`
- **PLANNING EVIDENCE (10):** `AGENTS.md`; `.specify/**` (excl. constitution & powershell scripts); Spec contracts; ADR-15 draft; research/data-model/quickstart/checklists; `R0-*.md`; program-memory continuity; `AFIA_*`; `v1-*`+product README; `ideas.md`
- **EXTERNAL SIBLING PRODUCT (2):** Fehrest; DeepMed-AI
- **GENERATED / IGNORED (4):** supabase `.temp`; `.cursor/**`; node_modules/dist/cache family; env secret files
- **REQUIRES INVESTIGATION (4):** root `README.md`; root `tsconfig.json`; `AfiaUI.zip`; fine-grained `lib/**` without proven importers

---

## 14. Acceptance

| Criterion | Result |
| --- | --- |
| Classification table/register complete for required coverage | **Met** (family-level; fine-grained lib = investigation) |
| Contradiction register covers required themes | **Met** |
| ADR drafts not elevated to governance authority | **Met** (correction applied) |
| Exact totals (no approximate notation) | **Met** |
| Orphan criteria applied; incomplete reachability → investigation | **Met** |
| No implementation disposition beyond planning authority | **Met** |
| Docs only | **Met** |
| Tier B independent review | **APPROVE WITH NOTES** (complete); precision corrections applied |

## 15. Rollback

Delete this file; revert CURRENT-STATE / NEXT-ACTION pointer edits.

## 16. Notes (PASS WITH NOTES)

- Activity classes separate wiring verification from runtime proof.
- Plan dispositions from Spec 001 are recommendations until executed under authorized tasks; T010 does not authorize adapt/replace.
- **Tier B** ([review](dee2c675-d977-472e-b062-1c613cac2bc6)): **APPROVE WITH NOTES**.
- **Post-Tier-B founder precision corrections:** ADR-15 → PLANNING EVIDENCE (non-authoritative draft; T030 acceptance); exact totals recomputed; orphan criteria tightened; authority map split authoritative vs non-authoritative.
