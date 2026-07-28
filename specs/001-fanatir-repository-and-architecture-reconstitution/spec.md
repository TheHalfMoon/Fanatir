# Feature Specification: Fanatir Repository and Architecture Reconstitution

**Feature Branch**: `docs/f0-planning-memory-bootstrap` (planning branch; feature directory independent)

**Created**: 2026-07-23

**Status**: Accepted (founder decisions Q1-Q9 ratified 2026-07-23; acceptance commit `46f55c4`)

**Input**: User description: "Create the product and repository reconstitution specification for the Fanatir ecosystem — audit current Fanatir repository, reconcile AFIA/Fanatir identity conflicts, classify assets, define target topology and cross-repo boundaries, Founder Alpha scope, migration sequence, and acceptance gates before feature implementation."

**Review note**: Graphify MCP server `fanatir-ecosystem-graph` was **not loaded** in the reviewing agent session. Discovery used local Graphify CLI against `graphify-out/graph.json`, then path verification on disk. Graphify remains non-authoritative.

## Constitution Constraints *(mandatory)*

Specs MUST align with `.specify/memory/constitution.md` (v1.0.0+):

- Stay within ratified product-space status and repository boundaries.
- Require Spec Kit traceability; do not authorize implementation from chat alone.
- Declare data classification / PHI posture when healthcare or patient data is in scope.
- Preserve Artifact/Run/provenance; forbid treating AI summaries as original sources.
- Keep AI/clinical tooling assistive; no silent clinical approval or fabricated mappings.
- Route models/MCP/plugins through Capability Gateway requirements when applicable.
- Use only evidenced compliance language (no unearned HIPAA/clinical/hospital-ready claims).
- Record unresolved founder decisions explicitly rather than inventing authority.

## Clarifications

### Session 2026-07-23

Founder-ratified answers recorded without agent choice among alternatives:

- Q1: Frontend migration starting point → A: fia-ui is the migration starting point and reusable shell; Constitution/ADRs remain architectural authority; auth/session/PrivateRoute/profile flows preserved until dedicated migration spec; no ProfileGate invention; full rewrite unauthorized; incremental adaptation authorized after planning.
- Q2: Founder Alpha boundary → A: One integrated golden journey (project → ingest → Studio → DeepMed → Fehrest → CoLab → commandF → Lab → provenance/memory → secure share) with required/bounded/preview/excluded tiers as specified by founder; must not collapse to a single isolated feature.
- Q3: Desktop trusted host timing → A: Minimal Tauri/Rust Trusted Host is an early foundational requirement after reconstitution plan + required ADRs, before major feature expansion; establish local project access, filesystem mediation, secrets, capability enforcement, worker supervision, audit events, secure desktop composition; not full kernel before first vertical slice.
- Q4: Supabase role → A: Provisional reusable integration evidence; optional adapter for auth/invites/membership/collab sync/approved cloud metadata; not local/patient/Artifact/policy authority; no PHI by default; preserve current auth/session until dedicated migration spec; disposition investigate with likely dapt after security/local-first review — not
eplace/
etain yet.
- Q5: Fehrest Alpha slice → A: Independent product in IamShehri/Fehrest and embedded Fanatir capability; Alpha requires bounded Fehrest usable independently and inside Fanatir (Markdown vault/editor/notes/sources/quotations/wikilinks/backlinks/typed relationships/search/project memory/current-state/next-action/graph/human-vs-extracted-vs-inferred/portable export); Graphify fork/import remains separate Fehrest spec.
- Q6: DeepMed Alpha and OpenMed sequencing → A: DeepMed is active first-integrated-release capability, not placeholder-only; during 001 define contracts/process/security/source-span/review/Fanatir expectations without importing OpenMed; separate DeepMed spec governs OpenMed fork/import; Alpha requires functioning bounded DeepMed pipeline in Studio with listed minimum capabilities.
- Q7: Canonical migration strategy → A: Incremental vertical-slice / strangler migration (not big-bang); preserve baseline → ADRs → minimal trusted host + contracts → one integrated vertical journey → adapt assets → adapters → migrate one capability at a time → verify → archive only via authorized migration → remove compatibility after acceptance; every stage needs entry/exit/rollback/compatibility/evidence/prohibitions.
- Q8: AFIA to Fanatir rename → A: Fanatir is canonical product identity immediately for docs/specs/architecture/new product-facing work; do not mass-rename technical identifiers during reconstitution; technical rename needs inventory/impact/ADR/plan/shims/verification; product-facing rename and technical migration are separate.
- Q9: Codex integration → A: Optional and non-blocking; Cursor Spec Kit sufficient for planning; Codex may be added later for review/implementation/verification; absence must not block 001 or its plan.

## Classification Legend

Every finding in this specification is labeled as one of:

| Label | Meaning |
| --- | --- |
| **Verified current fact** | Observed in the repository or remotes during audit |
| **Founder-ratified decision** | Binding via Constitution v1.0.0 |
| **Target architecture** | Ratified direction; not proof of current implementation |
| **Unresolved decision** | Requires founder or later Spec Kit clarification |
| **Recommendation** | Non-binding guidance for later planning; NOT accepted fact |

### Activity status (for inventory assets)

| Status | Meaning |
| --- | --- |
| **verified active** | Present and on the observed product/runtime path (routed UI, imported modules, or callable services) |
| **referenced but unverified** | Named by manifests/docs but runtime use not proven |
| **scaffold only** | Placeholder structure, README, or TODO CI without working gates |
| **archived** | Under `_archived/` or explicitly archived by `ARCHIVED.md` |
| **orphaned** | Exists but disconnected from active entry points / missing consumers |
| **contradictory** | Claims conflict with tree (e.g., workspace member path missing) |
| **requires runtime verification** | Needs live run/config (env keys, service up) to confirm behavior |

## Disposition Definitions *(mandatory)*

Exactly one **primary proposed disposition** per major asset. Compound labels are forbidden.

| Term | Definition |
| --- | --- |
| **Retain** | Keep as authoritative or reusable with no substantial structural change. |
| **Adapt** | Reuse with bounded changes while preserving its current responsibility. |
| **Migrate** | Move responsibility to a ratified target boundary through an explicit compatibility path. |
| **Replace** | Build a successor because the existing asset cannot satisfy the target requirements. Requires rationale and migration risk. |
| **Archive** | Preserve for history or evidence but remove from active authority and build paths later through an authorized migration. Requires rationale and migration risk. |
| **Investigate** | Insufficient evidence for disposition; no implementation action authorized. |

Proposed dispositions are **recommendations** until accepted through Spec Kit plan/tasks and any required founder/ADR decisions. They are not implementation authorization.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Founder baseline clarity (Priority: P1)

As the founder, I need one authoritative description of what currently exists across Fanatir, Fehrest, and DeepMed-AI so I can decide what to keep, move, archive, or replace without relying on chat memory or conflicting legacy docs.

**Why this priority**: Without a shared baseline, every later feature risks building on the wrong identity or dead scaffolding.

**Independent Test**: A reviewer can answer “what is active vs archived vs empty” using only this specification’s inventory and disposition matrix, without reading chat history.

**Acceptance Scenarios**:

1. **Given** the reconstitution specification, **When** the founder asks what is the active Fanatir product surface, **Then** the answer identifies `afia-ui/` as the verified-active frontend and separates `_archived/` scaffolds.
2. **Given** conflicting AFIA README or blueprint claims, **When** authority is questioned, **Then** the Constitution is cited as governing and legacy claims appear only in the authority-conflict ledger.
3. **Given** Fehrest and DeepMed-AI remotes, **When** ecosystem completeness is assessed, **Then** empty or README-only states are stated as verified facts without inventing product maturity.

---

### User Story 2 - Architect asset validity (Priority: P1)

As an architect, I need every major existing asset classified with a single proposed disposition and full disposition fields so reconstitution planning does not silently revive frozen or obsolete paths.

**Why this priority**: The tree mixes an active UI/kernel/bridge path with a large archived monorepo skeleton and stale workspace manifests.

**Independent Test**: For each major asset in the disposition matrix, a reviewer can inspect path, activity status, proposed disposition, rationale, risk, and follow-up ADR/spec fields.

**Acceptance Scenarios**:

1. **Given** root workspace manifests that point at missing paths, **When** disposition is reviewed, **Then** those manifests are `investigate` or `adapt` recommendations, not proof the missing layout exists.
2. **Given** `_archived/` contents, **When** disposition is reviewed, **Then** proposed disposition is `archive` unless a later accepted specification reactivates them.
3. **Given** Pictorial and Montada, **When** product-space status is checked, **Then** both remain frozen and are excluded from reconstitution activation.

---

### User Story 3 - Builder non-goals (Priority: P1)

As a builder, I need explicit “must not change / must not start” boundaries so reconstitution work cannot drift into package rename, OpenMed/Graphify import, UI redesign, auth behavior change, or feature implementation.

**Why this priority**: Constitution and founder constraints forbid several high-risk actions during this phase.

**Independent Test**: Hard exclusions and acceptance gates block `/speckit-implement` for product features until reconstitution gates pass.

**Acceptance Scenarios**:

1. **Given** this specification, **When** a builder proposes renaming `afia` packages, **Then** the proposal is out of scope until a dedicated rename specification is accepted.
2. **Given** this specification, **When** a builder proposes importing OpenMed or Graphify, **Then** the proposal is rejected as requiring separate dedicated specifications.
3. **Given** this specification, **When** a builder proposes changing auth/session/`PrivateRoute`/profile contracts, **Then** the change is excluded from reconstitution. (`ProfileGate` symbol/file is absent; do not invent it.)

---

### User Story 4 - Reviewer traceability (Priority: P2)

As a reviewer, I need every migration decision to be traceable to verified facts, founder-ratified decisions, target architecture, unresolved decisions, or explicit recommendations.

**Why this priority**: Prevents silent elevation of recommendations into settled facts.

**Independent Test**: Spot-check five disposition rows; each has Classification Legend labels and required matrix fields.

**Acceptance Scenarios**:

1. **Given** a migration stage, **When** reviewed, **Then** entry and exit criteria are observable without code changes.
2. **Given** an unresolved founder decision, **When** reviewed, **Then** it appears in Unresolved Decisions / Clarification Record and is not marked ratified.

---

### User Story 5 - Future specs depend on one baseline (Priority: P2)

As authors of later Spec Kit features, we need one accepted platform baseline so Studio, CoLab, Lab, Fehrest, DeepMed, and commandF specifications do not re-litigate repository identity.

**Why this priority**: Constitution requires Spec Kit for architectural changes and cross-repository contracts.

**Independent Test**: Later feature specs can reference this feature directory as the platform baseline dependency.

**Acceptance Scenarios**:

1. **Given** an accepted reconstitution specification, **When** a later feature starts, **Then** it can cite this baseline for repo boundaries and active/frozen product spaces.
2. **Given** Founder Alpha exclusions, **When** a later feature pulls excluded scope, **Then** reviewers can reject it using this specification’s Alpha boundary.

---

### Edge Cases

- Root manifests declare members that do not exist in the active tree (`Cargo.toml`, `go.work`, `pnpm-workspace.yaml`).
- Duplicate Supabase migration files: `supabase/migrations/workspaces.sql` and `afia-ui/supabase/migrations/workspaces.sql`.
- Auth/profile flows exist; no `ProfileGate` symbol/file outside this spec’s text.
- Fehrest remote has zero commits; DeepMed-AI tip is README-only.
- CI workflow jobs are placeholder echoes (`.github/workflows/ci.yml`).
- Branding strings still say AFIA while governance docs say Fanatir.
- `commandF` is Constitution-active but has no dedicated implementation path in the active tree.
- `lib/patients.ts` does **not** exist; patient kernel lives under `lib/patient/` and related modules.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Publish a current-state inventory with exact paths and activity status for applications, desktop scaffolding, host scaffolds, Python bridges, FHIR assets, collaboration/Supabase assets, auth/profile assets, patient/project workflows, research/Studio surfaces, tests/CI, manifests, governance docs, archived content, naming remnants, and license/notice posture.
- **FR-002**: Publish an authority-conflict ledger separating Constitution rules from legacy AFIA documents, package metadata, and stale READMEs.
- **FR-003**: Classify every major asset with exactly one proposed disposition using the Disposition Definitions and full disposition matrix fields.
- **FR-004**: Define a target product map aligned to Constitution active/frozen spaces without activating Pictorial or Montada.
- **FR-005**: Define a target repository map for Fanatir, Fehrest, and DeepMed-AI as **target architecture** without implementing moves/renames.
- **FR-006**: Define cross-repository contract boundaries as planning requirements (not schemas).
- **FR-007**: Define a legacy compatibility and rollback strategy for AFIA naming and archived scaffolds.
- **FR-008**: Define ordered migration stages with observable entry/exit gates before product-feature implementation begins.
- **FR-009**: Define Founder Alpha as one integrated golden journey with required, bounded-adapter, preview, and excluded tiers.
- **FR-010**: Include risk register, assumptions, clarification record, unresolved founder decisions, and measurable acceptance criteria/evidence list.
- **FR-011**: Later plan/tasks derived from this spec MUST NOT perform production feature implementation, package rename, file moves/deletions for migration, OpenMed import, Graphify import, Fehrest initialization beyond planning, DeepMed product implementation, UI redesign, auth/session behavior change, Pictorial/Montada activation, push, or PR unless a later accepted specification explicitly authorizes that action.
- **FR-012**: Durable reconstitution conclusions MUST be recorded in Spec Kit artifacts and program memory; AI chat MUST NOT be the sole record.
- **FR-013**: commandF ownership MUST be stated as Fanatir (product/integration authority) even when no dedicated commandF codebase yet exists.
- **FR-014**: Patient Longitudinal Memory and Project Long Memory ownership MUST follow Constitution memory-authority boundaries; Fehrest does not become clinical patient authority.
- **FR-015**: Windows-first MUST NOT be read as Windows-only; local-first MUST NOT be read as forbidding optional collaboration.

### Key Entities

- **Asset**: A repository path or capability cluster under audit.
- **Disposition**: retain | adapt | migrate | archive | replace | investigate (exactly one proposed).
- **AuthorityClaim**: A statement from Constitution, ADR, legacy doc, README, or package metadata.
- **RepositoryBoundary**: Ownership and integration edge among Fanatir, Fehrest, DeepMed-AI.
- **MigrationStage**: Ordered reconstitution phase with entry/exit criteria.
- **AlphaBoundary**: Required / bounded-adapter / preview / excluded Alpha capability tiers.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A reviewer can classify active vs archived vs empty-remote assets for every FR-001 inventory section using only this specification and cited paths.
- **SC-002**: Every major disposition-matrix row has exactly one primary proposed disposition and all required matrix fields filled or explicitly marked `investigate`/`n/a`.
- **SC-003**: The authority-conflict ledger lists Constitution, root README legacy claims, package name `afia`, stale workspace manifests, and `docs/execution-rules.md`, each with a governing outcome.
- **SC-004**: Five sampled out-of-scope proposals (rename, OpenMed import, Graphify import, Pictorial activation, auth redesign) are rejectable by reference to this spec alone.
- **SC-005**: Ownership questions for Studio, commandF, Patient Memory, Project Memory, Fehrest knowledge, and DeepMed runtime can be answered without contradicting Constitution v1.0.0.
- **SC-006**: Acceptance gates for starting product-feature implementation require listed evidence artifacts, not chat affirmation.
- **SC-007**: Unresolved founder decisions and clarification-record items are enumerated and not silently marked ratified.
- **SC-008**: Founder Alpha golden journey steps are listed and each step is tagged required, bounded-adapter, preview, or excluded.

## Current-State Inventory *(verified current fact)*

### Ecosystem remotes

| Repository | Verified fact | Activity |
| --- | --- | --- |
| `IamShehri/Fanatir` | Primary product repo; root/`afia-ui` package name `afia` | verified active (code present) |
| `IamShehri/Fehrest` | Empty remote (no commits) | scaffold only / empty |
| `IamShehri/DeepMed-AI` | Tip exists; local content README-only at audit | scaffold only |

### Root manifests and workspace configuration

| Path | Role | Activity |
| --- | --- | --- |
| `package.json` | Root meta package name `afia`; workspace points at `apps/desktop/ui` | contradictory (member missing) |
| `pnpm-workspace.yaml` | Declares `apps/desktop/ui` only | contradictory |
| `pnpm-lock.yaml` | Root lockfile | referenced but unverified for active app (app has own lock) |
| `Cargo.toml` | Lists `crates/afia-*` members | contradictory (no active `crates/`) |
| `go.work` | Uses `./services/operations-go` | contradictory (path missing; archived copy exists) |
| `rust-toolchain.toml`, `deny.toml` | Toolchain/deny stubs | scaffold only |
| `tsconfig.json` | Root TS | referenced but unverified |
| `.node-version`, `.python-version` | `20` / `3.12` | verified present |

### Active frontend application

| Path | Role | Activity |
| --- | --- | --- |
| `afia-ui/` | Primary React/Vite app (package name `afia`) | verified active |
| `afia-ui/vite.config.ts` | Vite config | verified active |
| `afia-ui/client/src/App.tsx` | Router + `PrivateRoute` | verified active |
| `afia-ui/client/index.html` | App HTML shell (AFIA title remnants) | verified active |
| `afia-ui/server/index.ts` | Express static server | verified active |
| `_archived/apps-desktop/ui/` | Archived desktop UI package `@afia/desktop-ui` | archived |

### Routes and entry points *(from `afia-ui/client/src/App.tsx`)*

Verified active routes include: `/login`, `/invite/:token`, `/`, `/patients`, `/patients/:id`, `/schedule`, `/inbox`, `/assistant`, `/documents`, `/analytics`, `/analytics/report`, `/research`, `/workspace/:id`, `/batch`, `/compare`, `/deidentify`, `/models`, `/insights`, `/settings`, `/settings/:section`.

### Authentication and profile flows

| Path | Role | Activity |
| --- | --- | --- |
| `afia-ui/client/src/contexts/AuthContext.tsx` | Supabase session/OTP; profile consent sync | verified active |
| `afia-ui/client/src/App.tsx` (`PrivateRoute`) | Auth gate for private routes | verified active |
| `afia-ui/client/src/pages/Login.tsx` | Login UI | verified active |
| `afia-ui/client/src/pages/InviteAccept.tsx` | Invite accept after auth | verified active |
| `afia-ui/client/src/pages/Settings.tsx` | Profile/settings | verified active |
| `afia-ui/supabase/schema.sql` (`profiles`) | Profile table/RLS reference | verified present; requires runtime verification |
| `ProfileGate` | Not found as file/symbol in repo (except this spec text) | absent |

**Preservation rule**: Auth/session/`PrivateRoute`/profile contracts are **retain** for behavior during reconstitution. No behavior change is authorized by this specification.

### Patient and clinical functionality

| Path | Role | Activity |
| --- | --- | --- |
| `afia-ui/client/src/pages/Patients.tsx`, `PatientDetail.tsx` | Patient UI | verified active |
| `afia-ui/client/src/data/patients.ts` | Mock patient data | verified active (mock) |
| `afia-ui/client/src/data/kernel-adapter.ts` | Seeds in-memory kernel | verified active; persistence requires runtime verification |
| `lib/patient/`, `lib/episode/`, `lib/encounter/`, `lib/clinical-events/`, `lib/clinical-timeline.ts`, `lib/clinical-registry.ts`, `lib/control/`, `lib/runtime/` | Clinical TS kernel | verified active modules |
| `scripts/patient-journey-test.ts` (and related scripts) | Offline script checks | referenced; not CI-gated |

### Studio / research / Lab-like surfaces

| Path | Role | Activity |
| --- | --- | --- |
| `afia-ui/client/src/pages/DocumentStudio.tsx` | Document Studio | verified active |
| `afia-ui/client/src/pages/BatchProcess.tsx`, `Deidentify.tsx`, `ModelCompare.tsx`, `Models.tsx` | Studio tools | verified active |
| `afia-ui/client/src/pages/MyResearch.tsx` | Research library | verified active |
| `afia-ui/client/src/pages/Analytics.tsx`, `afia-ui/client/src/lib/research-export.ts` | Analytics/export | verified active |
| `afia-ui/client/src/data/nav.ts` | Studio/Lab menu deep-links | verified active |
| Dedicated `commandF` app/module | None found | absent / investigate |

### Collaboration / Teams

| Path | Role | Activity |
| --- | --- | --- |
| `afia-ui/client/src/contexts/TeamWorkspaceContext.tsx` | Team workspace context | verified active |
| `afia-ui/client/src/lib/team-workspaces.ts` | Workspace API helpers | verified active |
| `afia-ui/client/src/components/workspace/*` (switcher/settings/invite/move) | Collaboration UI | verified active |
| `afia-ui/supabase/migrations/workspaces.sql` | Workspaces/members/invites | verified present; requires runtime verification |
| `supabase/migrations/workspaces.sql` | Duplicate migration copy | contradictory / duplicate |
| Montada | Frozen product space | frozen (not code) |

### FHIR and Python services

| Path | Role | Activity |
| --- | --- | --- |
| `services/openmed_bridge.py` | FastAPI “AFIA OpenMed Bridge” | verified present; requires runtime verification |
| `services/fhir_gate.py` | FHIR Gate v1 Bundle builder | verified present; requires runtime verification |
| `services/requirements-bridge.txt` | Bridge dependency pins | verified present |
| `afia-ui/client/src/services/openmed-client.ts` | UI client to bridge | verified active |
| `afia-ui/client/src/components/FhirExportModal.tsx` | FHIR export UI | verified active |

### Rust / desktop / Go

| Path | Role | Activity |
| --- | --- | --- |
| `apps/desktop/**` | Documented in `apps/README.md` / root README layout | missing in active tree; contradictory docs |
| `_archived/apps-desktop/src-tauri/` | Tauri v2 scaffold (`tauri.conf.json`) | archived / scaffold only |
| `_archived/crates/afia-*` | Eleven Rust crates | archived |
| `_archived/services-operations-go/` | Go operations service | archived |
| `_archived/services-ai-python/` | Managed Python AI package skeleton | archived |
| `_archived/contracts/` | Multi-language contracts tree | archived |

### Tests and CI

| Path | Role | Activity |
| --- | --- | --- |
| `.github/workflows/ci.yml` | Placeholder echo jobs | scaffold only |
| `tests/**` | README placeholders | scaffold only |
| `scripts/*-test.ts`, `scripts/fault-injection/**` | Local scripts | referenced but unverified as release gates |

### Documentation authorities

| Path | Role | Activity |
| --- | --- | --- |
| `.specify/memory/constitution.md` | Governing constitution | verified active authority |
| `AGENTS.md` | Agent hierarchy | verified active authority (level guidance) |
| `docs/execution-rules.md` | Module isolation (level 7) | verified active subordinate |
| `docs/program-memory/**` | Continuity memory | verified active |
| `docs/product/AFIA_*.md`, `v1-*.md` | Legacy plans/ledgers | evidence only |
| `docs/adr/`, `docs/architecture/` | Placeholder indexes | scaffold only |
| `_archived/ARCHIVED.md` | Archive rationale (2026-07-04) | verified archival statement |

### Naming / licensing

| Item | Verified fact |
| --- | --- |
| AFIA remnants | package names, UI copy, FHIR extension host strings, docs titles |
| Fanatir remnants | Constitution, AGENTS, program memory |
| License files | No root `LICENSE`/`NOTICE` found; `afia-ui/package.json` has `"license": "MIT"` field only |
| `AfiaUI.zip` | Present at repo root; purpose unknown → investigate |

## Authority-Conflict Ledger

| Claim source | Claim (summary) | Outcome |
| --- | --- | --- |
| `.specify/memory/constitution.md` v1.0.0 | Governing constitution | **Founder-ratified decision** — highest authority |
| Root `README.md` legacy layout body | Implies active `apps/desktop`, `crates/`, etc. | **Verified contradictory evidence** — subordinated by authority notice |
| `package.json` / `afia-ui/package.json` name `afia` | Product identity AFIA | **Verified current fact** — rename unresolved |
| `docs/execution-rules.md` | Module isolation rules | **Verified current fact** — hierarchy level 7 only |
| `docs/product/AFIA_*` | Build/master plans | **Evidence / recommendation inputs** — not auto-binding |
| `_archived/ARCHIVED.md` | Active path is `afia-ui`/`lib`/bridge | **Verified archival statement** — aligns with observed active tree |
| Stale `Cargo.toml` / `go.work` / `pnpm-workspace.yaml` | Implies missing layout present | **Contradictory** — not proof of presence |
| Chat / AI memory / Graphify | Informal or derived continuity | **Non-authoritative** |

## Asset Disposition Matrix *(proposed dispositions; Q1/Q4 founder constraints applied)*

Required columns: path, current role, activity, authority status, proposed disposition, rationale, dependencies, migration risk, compatibility requirement, follow-up ADR/spec, acceptance evidence.

| Path / family | Current role | Activity | Authority now | Proposed disposition | Rationale | Dependencies | Migration risk | Compatibility requirement | Follow-up ADR/spec | Acceptance evidence |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `afia-ui/` | Migration starting point and reusable frontend shell | verified active | implementation code (not architecture authority) | **adapt** | Founder Q1: starting point only; Constitution/ADRs are architecture authority; incremental adaptation after planning; no full rewrite | auth, supabase, bridges, `lib/` | Medium — dual AFIA/Fanatir identity | Preserve verified useful journeys; freeze auth/session/`PrivateRoute`/profile behavior until dedicated migration spec | Host ADR; later UI adaptation tasks; rename ADR separate | Inventory + route list from `App.tsx` |
| `lib/` | Clinical TS kernel | verified active | implementation + execution-rules | **adapt** | Active modules; align to primitives later | UI kernel-adapter, scripts | Medium | Preserve module-isolation rules until ADR supersedes | Primitives/memory specs | Path inventory of `lib/*` |
| `services/openmed_bridge.py`, `services/fhir_gate.py` | AI/FHIR bridges | requires runtime verification | implementation evidence | **investigate** | Present and wired from UI client, but OpenMed relationship/license/runtime maturity unproven | `openmed-client.ts`, requirements-bridge | High if treated as production DeepMed | Do not expand import surface without dedicated specs | DeepMed-AI / OpenMed import spec | File presence + client call sites; runtime check later |
| `afia-ui/supabase/**` | Provisional collab/auth integration evidence | requires runtime verification | implementation (not local SoT) | **investigate** | Founder Q4: optional adapter candidate; likely `adapt` after security/schema/local-first/compatibility review; not `retain`/`replace` yet; no PHI by default | AuthContext, TeamWorkspace* | High if treated as patient/Artifact authority | Preserve current auth/session until dedicated migration spec; optional cloud collab only | Supabase-role ADR + security review; filesystem location follow-up | Schema/migration file list + review record |
| `supabase/migrations/workspaces.sql` | Duplicate migration | contradictory | none | **investigate** | Duplicate of `afia-ui` migration; canonical location unresolved | `afia-ui/supabase/migrations/workspaces.sql` | Medium drift risk | Keep both until canonical decision | Clarification Q7 | Byte/path comparison evidence |
| Auth/`PrivateRoute`/`AuthContext`/`profiles` | Auth gate contracts | verified active | implementation | **retain** | Must not change behavior in this phase | Supabase | High if changed | Freeze behavior; document contracts | None for behavior change | Contract list in clarification record |
| `_archived/**` | Historical monorepo | archived | evidence only | **archive** | Explicitly archived; not active authority | stale root manifests | High if silently revived | Keep readable; no delete in this phase | Host reconstitution ADR before revival | `ARCHIVED.md` + tree |
| `Cargo.toml`, `go.work`, `pnpm-workspace.yaml` | Stale workspace truth | contradictory | misleading | **adapt** | Must stop implying missing members exist | archived counterparts | Medium false-confidence | Compatibility: do not delete archives when fixing manifests | Manifest-truthfulness tasks in plan | Diff members vs filesystem |
| `.github/workflows/ci.yml`, `tests/**` | Placeholder quality gates | scaffold only | none | **adapt** | Cannot claim release quality on echo jobs | packaging | Medium | Keep honest about non-gating status | Quality ADR before Alpha claims | Workflow content inspection |
| `docs/product/AFIA_*` | Legacy plans | evidence | not governing | **investigate** | Useful history; selective reuse unknown | Constitution | Low | Cite as evidence only | Per-topic specs | Ledger row |
| `.specify/**`, `docs/program-memory/**`, `AGENTS.md` | Governance/planning | verified active | governing/continuity | **retain** | Required Spec Kit + memory system | Constitution | Low | Preserve | Amendments via constitution process | File presence |
| Fehrest remote | Independent knowledge product | empty | ratified active space | **investigate** | Empty; needs Fehrest scaffold + Graphify specs | Fanatir integration contracts | High for Alpha journey step | Standalone usable later | Fehrest init spec; Graphify import spec | Remote empty status |
| DeepMed-AI remote | Medical intelligence runtime | README-only | ratified active; required in first integrated release | **investigate** | Cannot defer outside first integrated release, but import/implement not authorized here | Fanatir gateway | High | Assistive only; no clinical autonomy | OpenMed/DeepMed import + DeepMed product specs | Remote tip status |
| commandF capability | Interop workbench | absent in code | Fanatir-owned (Constitution) | **investigate** | Owned by Fanatir; no dedicated module yet | FHIR bridges may be precursors | High if over-claimed | Do not invent full commandF in reconstitution | commandF capability spec | Search shows no module |
| Pictorial / Montada | Frozen product spaces | n/a | frozen | n/a (product-space freeze) | Not repository assets to “archive-as-code”; remain frozen | — | High if activated | Must not activate | Founder reauthorization only | Constitution product-space section |
| Package rename `afia`→Fanatir | Identity migration | unresolved | unresolved | **investigate** | Not authorized now | all packages/UI | High churn | Keep AFIA names working | Dedicated rename spec | Constitution unresolved list |
| `AfiaUI.zip` | Unknown zip | orphaned/unknown | none | **investigate** | Purpose unknown | — | Low–Med secret/binary risk | Do not treat as authority | Inventory follow-up | Path exists |

## Target Product Map *(founder-ratified + target)*

**Active (founder-ratified):** Studio; CoLab; Fanatir Lab; Fehrest; DeepMed; commandF; Patient Longitudinal Memory; Project Long Memory; Collaboration; Secure Sharing; Capability Gateway; MCP/approved plugins; MedScale; MESC.

**Frozen (founder-ratified):** Pictorial; Montada only.

**CoLab vs Montada:** CoLab is active shared governance/work; Montada remains frozen and distinct.

**DeepMed rule (founder-ratified):** Must not be frozen, removed, or postponed outside the first integrated Fanatir release. This specification authorizes planning of DeepMed contracts/integration expectations and requires a bounded Alpha DeepMed pipeline, but does **not** authorize OpenMed import or DeepMed product implementation inside 001 itself.

**Implementation status:** Not claimed beyond inventory.

## Target Repository Map *(target architecture only)*

| Repository | Target role |
| --- | --- |
| Fanatir | Product and integration authority: desktop experience, Studio, CoLab, Lab, **commandF**, patient dashboards, patient memory, project memory integration, collaboration, sharing, capability gateway, local policy, ecosystem contracts |
| Fehrest | Independent second-brain **and** Fanatir knowledge/citation/graph capability; Markdown authoritative for human knowledge; graphs derived |
| DeepMed-AI | Independent medical intelligence runtime used by Fanatir; future governed OpenMed-based line requires **separate** import specification |

**Host direction (target architecture, not current fact):** desktop-first; local-first; **Windows-first release with architectural support for macOS and Linux** (not Windows-only); React/Vite UI; Tauri composition; Rust trusted host; isolated workers; optional cloud collaboration (local-first ≠ no collaboration). Per Q3, a **minimal** trusted host is an early foundational requirement after reconstitution plan + ADRs and before major feature expansion — not proof archived crates are active.

**What remains inside Fanatir monorepo (recommendation pending plan):** product UI, integration layer, commandF, patient/project integration, gateway/policy, CoLab/Lab surfaces, and contracts to Fehrest/DeepMed — exact packaging is plan/ADR work, not settled by this spec beyond Constitution ownership.

## Cross-Repository Contract Boundaries *(requirements, not schemas)*

- Fanatir owns commandF and patient/project integration surfaces.
- Fanatir MAY consume Fehrest releases for knowledge/memory/citation without making Fehrest mandatory for Fehrest standalone use.
- Fanatir MAY consume DeepMed-AI releases for task-first assistance without granting autonomous clinical authority.
- Fehrest MUST NOT be system of record for FHIR, databases, model weights, or original healthcare files.
- DeepMed-AI invocations from Fanatir MUST pass Capability Gateway requirements.
- Patient access authority remains separate from ordinary project membership.
- Patient Longitudinal Memory is Fanatir patient-scoped; Project Long Memory is Fanatir project-scoped; Fehrest notes may support projects but do not become clinical patient authority.
- No OpenMed or Graphify import is authorized by this specification.

## Memory Ownership Restatement *(founder-ratified)*

- **Patient Longitudinal Memory** → Fanatir patient-scoped clinical continuity.
- **Project Long Memory** → Fanatir project-scoped authoritative continuity.
- **Fehrest** → human knowledge, citations, claims, evidence, derived graphs; standalone and embeddable.

## Legacy Compatibility and Rollback Strategy

1. Keep AFIA package/string identity functioning until a rename specification is accepted.
2. Keep `_archived/` readable; do not delete during this phase.
3. Prefer authority notices and Spec Kit/ADRs over mass doc rewrites.
4. Manifest truthfulness changes must be reversible (commit-level rollback) and must not orphan archived evidence.
5. If a migration stage fails exit criteria, halt feature implementation and roll back to last accepted Spec Kit artifact state; do not “push through” on the 60-day objective.
6. Deduplicate Supabase migrations only after canonical-location decision.

## Migration Stages *(founder-ratified strategy Q7; no implementation authorized here)*

Strategy: incremental vertical-slice / strangler migration. **Not** a big-bang rewrite.

Required sequence:

1. Preserve the current repository baseline.
2. Ratify target boundaries through ADRs.
3. Establish the minimal trusted host and shared contracts.
4. Build one integrated vertical journey beside or through existing assets.
5. Adapt reusable existing assets.
6. Introduce compatibility adapters where required.
7. Migrate responsibility one bounded capability at a time.
8. Verify behavior before retiring legacy paths.
9. Archive superseded assets only through a separate authorized migration.
10. Remove compatibility layers only after explicit acceptance.

Every stage MUST declare: entry criteria; exit criteria; rollback path; compatibility requirement; evidence; prohibited changes.

| Stage | Intent | Entry gate | Exit gate | Rollback | Prohibited |
| --- | --- | --- | --- | --- | --- |
| M0 | Accept reconstitution baseline | Constitution v1.0.0; this spec clarified | Spec accepted for planning; clarifications recorded | Revert to prior accepted Spec Kit artifacts | Product implementation |
| M1 | Authority normalization | M0 exit | Ledger accepted; Fanatir identity in new docs/specs | Restore prior docs via git | Technical mass-rename |
| M2 | Manifest/tree truthfulness | M1 exit | Plan/tasks for stale manifests accepted | Revert manifest edits | Deleting `_archived/` |
| M3 | Active-path hardening | M2 exit | `afia-ui`/`lib`/bridges/supabase/auth dispositions confirmed | Revert unauthorized adapts | Auth/session behavior change; full UI rewrite |
| M4 | Cross-repo contracts | M3 exit; Fehrest & DeepMed planning specs identified | Boundaries reviewed; OpenMed/Graphify import specs queued not executed | Drop draft contracts | OpenMed/Graphify import execution |
| M5 | Founder Alpha lock | M4 exit | Golden journey tiers locked per Q2 | Keep prior Alpha boundary text | Weakening integrated journey; activating Pictorial/Montada |
| M6 | Minimal Trusted Host + ADRs | Reconstitution plan accepted; required host ADRs ready | Minimal Tauri/Rust host boundaries established (project access, FS mediation, secrets, capability enforcement, worker supervision, audit, secure composition) OR explicit interim exception documented | Disable host feature branch; return to prior shell | Treating archived full kernel as already active; major feature expansion before host foundations |
| V1 | First integrated vertical slice | M5–M6 applicable exits | One golden-journey slice demonstrable with bounded adapters | Feature-flag off / revert slice | Big-bang cutover; live EHR writes |
| GX | Broader feature implementation gate | V1 evidence accepted | Acceptance Evidence complete | Halt feature specs | `/speckit-implement` without gates |

## Founder Alpha Capability Boundary *(founder-ratified Q2)*

Founder Alpha MUST demonstrate one integrated golden journey (must not be reduced to a single isolated feature):

1. Create or open a Project.
2. Import a supported healthcare document, dataset, or FHIR resource.
3. Read, annotate, quote, and work in Studio.
4. Process a supported medical document through a functioning DeepMed pipeline.
5. Preserve notes, sources, links, and project memory in Fehrest.
6. Share, comment, review, or approve through CoLab.
7. Validate or transform supported healthcare data through commandF.
8. Analyze approved data using a guided Fanatir Lab workflow.
9. Preserve Artifact revisions, Runs, provenance, and project memory.
10. Securely export or share an approved result after sensitive-data review.

### Required Alpha capabilities

- Installable Windows desktop application
- Project creation and opening
- Supported PDF/text/CSV/JSON/FHIR ingestion
- Studio document workspace
- Functioning bounded DeepMed document workflow
- Fehrest Markdown notes, sources, quotations, backlinks, graph, search, and project memory
- Bounded CoLab comments, sharing, review, and approval
- commandF validation and selected conversions with explicit result states
- Guided Python and SQL Lab workflows
- Patient document history and bounded longitudinal timeline
- Project resume and long-memory experience
- Secure export and sharing review

### Bounded Alpha capabilities

- Explicitly supported file types only
- Approved DeepMed models only
- Small-team collaboration
- Optional Supabase-backed cloud collaboration
- Limited initial external integrations
- Windows-first release

### Preview capabilities

- R execution
- Broader Hugging Face model access
- Advanced Zotero and storage synchronization
- macOS and Linux packaged releases
- Additional commandF adapters
- Advanced real-time collaborative editing

### Explicitly excluded

- Live EHR writes
- Hospital-production certification
- Autonomous diagnosis or treatment
- Universal healthcare-file conversion
- Public unreviewed plugin marketplace
- Unrestricted PHI egress
- Pictorial
- Montada
- Mass technical identifier rename during reconstitution
- OpenMed or Graphify import execution during 001
- Auth/session/`PrivateRoute`/profile behavior changes without dedicated migration spec

## Minimal Trusted Host *(founder-ratified Q3)*

Distinguish:

| Layer | Meaning |
| --- | --- |
| Target architecture | Desktop-first, local-first, Windows-first with macOS/Linux support; React/Vite UI; Tauri composition; Rust trusted host; isolated workers |
| Minimum early host capability | After reconstitution plan + required ADRs, before major feature expansion: local project access; filesystem mediation; secrets; capability enforcement; worker supervision; audit events; secure desktop composition |
| Later hardening | Full kernel expansion, broader workers, packaging maturity — after first vertical slice |

The minimal host does **not** need the full final kernel before the first vertical slice. Archived crates are evidence, not active authority.

## Fehrest Alpha Boundary *(founder-ratified Q5)*

Fehrest is an independent product in `IamShehri/Fehrest` and an embedded Fanatir capability. Alpha requires a usable bounded Fehrest that works independently for a non-Fanatir user and contextually inside Fanatir.

Required Fehrest Alpha capabilities: local Markdown vault; Markdown editor; pages/notes; sources/attachments; quotations with provenance; wikilinks/backlinks; typed relationships; local search; project memory; current-state and next-action memory; graph visualization; distinction between human, extracted, and inferred relationships; portable export.

Not required for Founder Alpha: full Notion parity; full Obsidian plugin ecosystem; large-scale multiplayer editing; public marketplace; complete Graphify feature parity.

Graphify import/fork mechanics require a separate Fehrest specification and license review. This reconstitution specification defines repository/integration boundary only and does **not** execute any fork.

## DeepMed Alpha and OpenMed Sequencing *(founder-ratified Q6)*

DeepMed is an active first-integrated-release capability and MUST NOT be represented as placeholder-only.

During specification 001 and its planning phase:

- Define versioned DeepMed contracts
- Define process and security boundaries
- Define source-span and review requirements
- Define Fanatir integration expectations
- Do **not** import OpenMed yet

A separate DeepMed specification must govern: OpenMed fork/import; license/NOTICE; upstream history; upstream sync; DeepMed extensions; evaluation; packaging; release contracts.

Founder Alpha requires a functioning bounded DeepMed pipeline integrated into Studio supporting at minimum: supported document input; task-first workflow; PHI detection or de-identification workflow; selected clinical entity extraction; source spans; confidence/uncertainty; review/correction; output handoff to Fehrest and commandF; recorded model and Run provenance.

Existing `services/openmed_bridge.py` / `fhir_gate.py` remain **investigate** evidence pending that DeepMed specification — not an OpenMed import authorization.

## Identity and Rename *(founder-ratified Q8)*

- Fanatir is the canonical product identity immediately for documentation, new specifications, new architecture, and new product-facing work.
- Existing technical names (`afia` packages, legacy folders, imports, DB identifiers, env keys, historical docs) MUST NOT be mass-renamed during reconstitution.
- Technical renaming requires inventory, compatibility impact analysis, dedicated ADR, migration plan, aliases/shims where required, and packaging/upgrade verification.
- Product-facing rename and technical identifier migration are separate operations.

## Codex Integration *(founder-ratified Q9)*

Codex integration is optional and non-blocking. Cursor with installed Spec Kit remains sufficient for planning. Codex may be added later for independent review, bounded implementation, verification, or adversarial inspection. Its absence MUST NOT block specification 001 or its plan.

## Decisions Required Before Planning *(updated after clarifications)*

| # | Decision | Status | Latest stage |
| --- | --- | --- | --- |
| 1 | Frontend migration starting point | **Resolved Q1:** `afia-ui` | M3 |
| 2 | Desktop/minimal trusted host timing | **Resolved Q3:** early foundational after plan+ADRs | M6 |
| 3 | Python bridge role | **Investigate** pending DeepMed spec (Q6) | DeepMed spec |
| 4 | Supabase role | **Resolved Q4:** optional adapter; disposition investigate→likely adapt after review | security ADR |
| 5 | Auth/session contracts | **Resolved:** preserve until dedicated migration spec | auth migration spec |
| 6 | commandF ownership | **Resolved:** Fanatir | commandF spec |
| 7 | Fehrest/DeepMed consumption | Requirements-level; packaging channels still plan detail | M4 |
| 8 | Fanatir monorepo contents | Constitution ownership; packaging detail in plan/ADRs | M4/M6 |
| 9 | Founder Alpha includes | **Resolved Q2** | M5 |
| 10 | ADRs required | Host; Supabase security/local-first; quality; primitives | before related build |
| 11 | Fehrest/DeepMed/OpenMed/Graphify specs | Required before those imports/builds | before import/build |
| 12 | Migration strategy | **Resolved Q7:** strangler / vertical slice | all stages |
| 13 | Rename policy | **Resolved Q8:** Fanatir product identity now; no mass technical rename | rename ADR later |
| 14 | Codex | **Resolved Q9:** optional non-blocking | anytime later |

## Clarification Record *(founder-ratified)*

| ID | Question | Ratified answer | Rationale | Impact | Resolve stage | Follow-up ADR/spec | Consequence for `/speckit-plan` |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Q1 | Frontend starting point | `afia-ui` is migration starting point/reusable shell; not architecture authority; preserve auth/session; no ProfileGate redesign; no full rewrite; incremental adaptation after planning | Only verified-active frontend | Plan must adapt `afia-ui`, not revive archived desktop UI as authority | M3 | Later UI adaptation tasks; auth migration spec if behavior changes ever needed | Plan substrate locked |
| Q2 | Alpha boundary | Integrated golden journey + required/bounded/preview/excluded lists | Prevents vapor and single-feature Alpha | Plan must span Studio/DeepMed/Fehrest/CoLab/commandF/Lab/provenance/share | M5 | Per-capability specs | Plan cannot drop journey steps |
| Q3 | Trusted host timing | Minimal Tauri/Rust host early after plan+ADRs, before major expansion | Local-first/security foundations | Plan schedules host ADRs and minimal host before major features | M6 | Host reconstitution ADR(s) | Plan must include early host work |
| Q4 | Supabase | Optional adapter; not SoT/patient/Artifact/policy authority; no PHI by default; preserve auth; disposition investigate (likely adapt) | Local-first + reuse evidence | Plan must not centralize PHI in Supabase | security/local-first ADR | Plan treats Supabase as optional adapter |
| Q5 | Fehrest Alpha | Independent + embedded bounded Alpha capabilities listed; Graphify fork separate | Constitution Fehrest role | Plan needs Fehrest repo/spec work, not Fanatir-only stub | Fehrest Alpha + Graphify import specs | Plan references Fehrest deliverables |
| Q6 | DeepMed/OpenMed | DeepMed required; define contracts in 001/plan; no OpenMed import yet; separate DeepMed spec; Alpha pipeline minimums listed | First-release DeepMed without illegal import | Plan defines contracts + Studio integration path; queues OpenMed spec | DeepMed/OpenMed import spec | Plan must not schedule OpenMed import in 001 implementation |
| Q7 | Migration strategy | Strangler/vertical-slice sequence with stage gates/rollback | Avoid big-bang risk | Plan phases match sequence | migration stage tasks | Plan structure constrained |
| Q8 | Rename | Fanatir product identity now; no mass technical rename in reconstitution | Reduce churn | Plan uses Fanatir in new docs; keeps `afia` identifiers | rename ADR later | Plan must not include mass rename tasks |
| Q9 | Codex | Optional non-blocking | Avoid tool deadlock | Plan proceeds with Cursor Spec Kit | optional later integration | No Codex prerequisite |

## Assumptions

- `afia-ui` is the migration starting point (Q1) but not final architecture authority.
- `_archived/` remains reference-only unless ADR reactivates specific pieces.
- ProfileGate symbol absence does not authorize access redesign.
- Graphify findings are discovery aids only.
- Detailed tasking and tech sequencing beyond ratified direction belong in `/speckit-plan`.

## Unresolved Decisions *(genuine remainders only)*

1. Exact filesystem canonicalization of duplicate Supabase migrations (`afia-ui/supabase/...` vs root `supabase/...`) after Q4 security/local-first review.
2. Shared primitive schemas (Workspace/Project/Patient/Artifact/...) — deferred to later specifications/ADRs.
3. Concrete Fehrest and DeepMed release packaging/version channels consumed by Fanatir (contract detail for M4/plan).
4. Exact interim packaging choice for the first vertical slice while minimal trusted host lands (plan/ADR detail under Q3).

## Acceptance Gates Before Feature Implementation

Product-feature implementation MUST NOT begin until:

1. This specification is founder-accepted and advanced through plan → tasks → independent review for architecture-sensitive work.
2. Authority-conflict ledger outcomes remain true in agent/docs guidance.
3. Founder Alpha golden journey remains intact per Q2 (DeepMed first-integrated-release rule honored; OpenMed import still separate).
4. Cross-repo boundaries for Alpha are reviewed; Fehrest/DeepMed planning specs identified.
5. Minimal trusted host ADRs accepted and early host work scheduled before major feature expansion (Q3).
6. Acceptance Evidence list below is complete.
7. Program-memory `CURRENT-STATE` / `NEXT-ACTION` point at active plan/tasks.

## Acceptance Evidence *(required)*

- Repository inventory report (this spec’s inventory section with path citations)
- Authority-conflict ledger
- Complete disposition matrix (definitions + single disposition rows)
- Target product map
- Target repository map
- Cross-repository boundary document (this spec section)
- Migration-stage gates table
- Founder Alpha boundary (golden journey + tiers + exclusions)
- Risk register
- Unresolved-decision ledger + clarification record
- Proof that no production implementation occurred during specification work (`git status` / diff scoped to planning artifacts)

## Hard Constraints Restated

No production implementation; no package rename; no file moves/deletions for migration; no OpenMed import; no Graphify import; no Fehrest initialization; no DeepMed implementation; no UI redesign; no auth/session/ProfileGate behavior change; no Pictorial/Montada activation; no push/PR; no unsupported factual assumptions; no compliance/clinical-safety claims without evidence.

## Out of Scope

- Implementation plan details (`/speckit-plan`)
- Task breakdown (`/speckit-tasks`)
- Coding, dependency installation, or repository mutation beyond planning artifacts
- Technology selection beyond Constitution target direction
