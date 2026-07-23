# Tasks: Fanatir Repository and Architecture Reconstitution

**Input**: Design documents from `/specs/001-fanatir-repository-and-architecture-reconstitution/`

**Prerequisites**: Accepted `spec.md`, accepted `plan.md`, `research.md`, `data-model.md`, `quickstart.md`, `contracts/**`, `adrs/ADR-15-*.md`

**Plan acceptance commit**: `ac5c777e91b74fc30903a364f03337a5ac8a63f6` (parent specification acceptance `46f55c4e9a6b69aecbd85007e98688141939f869`)

**Tests**: Required (contract, IPC, FS, secrets, network, PHI-egress, crash/recovery, unknown contract version, packaging, offline, clean-install, etc.)

**Organization**: Stage-gated **R0→R5**. Spec Kit checklist lines are executable; detail catalog carries full gates.

**Global prohibitions**: No OpenMed/Graphify fork/import; no Pictorial/Montada; no migration mutation; no auth/session behavior change without ADR-11 + dedicated spec; no Go Alpha dependency; no Rust rewrite of OpenMed/Graphify/Jupyter/R; no push/PR

## Blocking matrix

| Gate | Blocks |
| --- | --- |
| ADR-15 + ADR-01 + ADR-02 Accepted (T030) | R2 shell/host (**T031+**) |
| ADR-06 Accepted | Worker/IPC implementation (**T034+**) |
| ADR-04 before ADR-05 | Artifact ADR/store sequencing |
| ADR-05 Accepted | Artifact Store foundation (**T037**) / R3 Artifact contracts |
| ADR-05/07/08/09/10/14 | Relevant **R3** implementation |
| Fehrest functional (**T044–T047**) | DeepMed persist (**T050**); DeepMed pipeline requires Fehrest functional |
| ADR-13 + Decision D + **T060** | **R5 distribution** (**T061+**) |
| Vite-only | **Cannot** satisfy R2 exit, R3 exit, or Founder Alpha (**T042/T055/T066**) |

## Authority note (adversarial review correction)

- ADR authoring tasks (**T012–T026**) produce **drafts only**. Draft ≠ accepted.
- Formal acceptance is recorded at stage gates: **T030** (R1), **T042** (R2), **T055** (R3), **T060** (signing custody), **T066** (Founder Alpha).
- Independent review uses risk tiers: **Tier A** (mandatory), **Tier B** (batch/stage), **Tier C** (normal verification).

## Review risk tiers

| Tier | Meaning |
| --- | --- |
| **A** | Mandatory independent review before treating work as stage-complete |
| **B** | Batch or stage-level independent review |
| **C** | Normal verification / evidence only |

## Rust-first enforcement (all implementation tasks)

Rust owns: Tauri host; project/workspace authority; Artifact/Revision/Run mutation; encrypted local storage; FS mediation; secrets; policy/capabilities; audit; secure IPC; worker supervision; plugin/MCP gateway; secure export; updater/signing boundaries.

React/TypeScript: UI/presentation only. Python: bounded supervised workers. R/SQL: governed Lab. Go: not required for Alpha; only via justifying ADR.

## Format

`- [ ] TaskID [P?] [Stage] Description — paths`

---

## Phase R0 — Verified Baseline

**Purpose**: Read-only / documentation verification; **no production mutation**

- [ ] T001 [R0] Record exact repository baseline (HEAD, branch, remotes, sibling repos) — `docs/program-memory/baseline/R0-repo-baseline.md`
- [ ] T002 [P] [R0] Verify actual build/dev commands for afia-ui and record evidence — `docs/program-memory/baseline/R0-build-commands.md; afia-ui/package.json`
- [ ] T003 [P] [R0] Verify dependency managers (pnpm/node/rust/python/uv) and versions — `docs/program-memory/baseline/R0-tooling.md`
- [ ] T004 [P] [R0] Map active frontend entry points (Vite client App/routes/Shell) — `docs/program-memory/baseline/R0-frontend-map.md; afia-ui/client/src/App.tsx`
- [ ] T005 [R0] Verify auth/session/PrivateRoute/profile behavior (observe-only) — `docs/program-memory/baseline/R0-auth-session.md; afia-ui/client/src/contexts/AuthContext.tsx; afia-ui/client/src/App.tsx`
- [ ] T006 [P] [R0] Inspect Supabase clients and both migration trees (freeze inventory) — `docs/program-memory/baseline/R0-supabase-inventory.md; afia-ui/supabase/**; supabase/migrations/**`
- [ ] T007 [P] [R0] Inspect Python/FHIR/OpenMed prototype services — `docs/program-memory/baseline/R0-python-services.md; services/**`
- [ ] T008 [P] [R0] Inspect archived Rust/Tauri/Go assets without reactivation — `docs/program-memory/baseline/R0-archived-assets.md; _archived/**`
- [ ] T009 [P] [R0] Inspect current CI and tests honesty — `docs/program-memory/baseline/R0-ci-tests.md; .github/workflows/**; tests/**`
- [ ] T010 [R0] Classify paths active/scaffold/archived/orphaned/contradictory — `docs/program-memory/baseline/R0-path-classification.md`
- [ ] T011 [R0] Produce R0 baseline evidence package and program-memory closeout — `docs/program-memory/baseline/**; docs/program-memory/CURRENT-STATE.md; docs/program-memory/NEXT-ACTION.md`

**Checkpoint**: R0 complete — baseline evidence package exists; no production mutation

---

## Phase R1 — Architecture Decisions and Contracts

**Purpose**: ADR **drafting**; freezes; charters; **T030** records acceptance. No R2 code before T030.

- [ ] T012 [R1] Author ADR-15 Rust-First Polyglot Runtime and Language Authority (draft only) — `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-15-*.md`
- [ ] T013 [P] [R1] Author ADR-01 Platform and Desktop Composition (Tauri 2) (draft only) — `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-01-*.md`
- [ ] T014 [R1] Author ADR-02 Rust Trusted Host Boundary (draft only) — `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-02-*.md`
- [ ] T015 [P] [R1] Author ADR-03 afia-ui Strangler Migration (draft only) — `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-03-*.md`
- [ ] T016 [P] [R1] Author ADR-04 Shared Primitive Ownership and Versioning (draft only) — `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-04-*.md`
- [ ] T017 [R1] Author ADR-06 Worker and IPC Contracts (draft only) — `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-06-*.md`
- [ ] T018 [R1] Author ADR-05 Artifact/Revision/Run Storage (draft only) — `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-05-*.md`
- [ ] T019 [P] [R1] Author ADR-07 Supabase Adapter and Local-First Boundary (draft only) — `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-07-*.md`
- [ ] T020 [P] [R1] Author ADR-08 Fehrest Integration and Release Model (draft only) — `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-08-*.md`
- [ ] T021 [P] [R1] Author ADR-09 DeepMed Integration and OpenMed Runtime/Fork Boundary (draft only) — `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-09-*.md`
- [ ] T022 [P] [R1] Author ADR-10 commandF Ownership and Process Boundary (draft only) — `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-10-*.md`
- [ ] T023 [P] [R1] Author ADR-11 Auth and Session Preservation (draft only) — `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-11-*.md`
- [ ] T024 [P] [R1] Author ADR-12 Technical AFIA-to-Fanatir Rename Strategy (draft only) — `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-12-*.md`
- [ ] T025 [P] [R1] Author ADR-13 First Vertical-Slice Packaging (draft only) — `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-13-*.md`
- [ ] T026 [R1] Author ADR-14 Security and Data-Classification Enforcement (draft only) — `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-14-*.md`
- [ ] T027 [R1] Publish/align shared primitive contract boundaries (planning schemas only) — `specs/001-fanatir-repository-and-architecture-reconstitution/contracts/shared-primitives.md; specs/001-fanatir-repository-and-architecture-reconstitution/data-model.md`
- [ ] T028 [R1] Freeze documents-crypto for real patient data; document PHI-egress seam controls — `docs/program-memory/decisions/documents-crypto-freeze.md; specs/001-fanatir-repository-and-architecture-reconstitution/contracts/supabase-adapter.md`
- [ ] T029 [R1] Define future-002 Supabase canonicalization specification charter (do not create 002) — `docs/program-memory/decisions/002-charter.md`
- [ ] T030 [R1] R1 acceptance gate: accept ADR-15/01/02/06; review R3-blocking ADRs; program-memory update — `docs/program-memory/CURRENT-STATE.md; docs/program-memory/NEXT-ACTION.md; specs/001-fanatir-repository-and-architecture-reconstitution/adrs/**`

**Checkpoint**: T030 complete — ADR-15/01/02/06 Accepted; R2 may begin

---

## Phase R2 — Minimal Rust Trusted Desktop Foundation

**Purpose**: Minimum Trusted Host only; avoid final kernel

**Gate**: T030 (ADR-15/01/02/06 as applicable)

- [ ] T031 [R2] Create minimal Tauri 2 desktop shell loading afia-ui client — `apps/desktop/**; afia-ui/client/** (consume only)`
- [ ] T032 [R2] Implement Rust Trusted Host skeleton (project/workspace authority stubs) — `apps/desktop/src-tauri/**`
- [ ] T033 [R2] Implement bounded filesystem/project open/create via Rust mediation — `apps/desktop/src-tauri/**; specs/001-fanatir-repository-and-architecture-reconstitution/contracts/trusted-host-ipc.md`
- [ ] T034 [R2] Implement versioned secure IPC (commands/events/channels) foundation — `apps/desktop/src-tauri/**; specs/001-fanatir-repository-and-architecture-reconstitution/contracts/trusted-host-ipc.md`
- [ ] T035 [R2] Implement worker supervision foundation (spawn/status/stop/restart) — `apps/desktop/src-tauri/**`
- [ ] T036 [R2] Implement audit event foundation in Rust — `apps/desktop/src-tauri/**`
- [ ] T037 [R2] Implement local Artifact Store foundation (encrypted local storage seam) — `apps/desktop/src-tauri/**`
- [ ] T038 [R2] Implement data-classification enforcement seam in gateway — `apps/desktop/src-tauri/**`
- [ ] T039 [P] [R2] Windows development packaging spike (unsigned internal-only) — `apps/desktop/**; docs/program-memory/baseline/R2-packaging-spike.md`
- [ ] T040 [P] [R2] Heavy Python/ML sidecar externalBin feasibility spike — `docs/program-memory/baseline/R2-sidecar-spike.md; apps/desktop/** (config only)`
- [ ] T041 [R2] Retain Vite fallback for development only; label non-Alpha — `docs/program-memory/decisions/vite-fallback.md`
- [ ] T068 [P] [R2] Add filesystem-escape, secret-access-denial, and network-access-denial tests — `tests/**`
- [ ] T042 [R2] R2 exit verification: project open, FS deny, worker lifecycle, audit, no Artifact bypass — `docs/program-memory/baseline/R2-exit-evidence.md; tests/**`

**Checkpoint**: T042 R2 exit evidence (Trusted Host path only)

---

## Phase R3 — First Integrated Vertical Slice

**Purpose**: Exact order — Artifact/Run → Fehrest → DeepMed → persist to Fehrest → commandF/Lab → CoLab/secure share

**Gate**: ADR-05/07/08/09/10/14 as applicable; both Fehrest and DeepMed functioning

- [ ] T043 [R3] Implement Artifact and Run contracts end-to-end (slice step 1) — `apps/desktop/**; specs/001-fanatir-repository-and-architecture-reconstitution/contracts/**; tests/contract/**`
- [ ] T044 [R3] Scaffold Fehrest bounded Alpha product (independent repo) — vault/memory minimum — `Fehrest/**`
- [ ] T045 [R3] Implement Fehrest Markdown vault, notes, sources, quotations, backlinks, typed relationships, local search, and graph boundary — `Fehrest/**`
- [ ] T046 [R3] Implement Fehrest project memory + human/extracted/inferred relationship states + portable export — `Fehrest/**`
- [ ] T047 [R3] Wire versioned Fanatir↔Fehrest integration contract (Rust-supervised) — `apps/desktop/**; specs/001-fanatir-repository-and-architecture-reconstitution/contracts/fehrest-integration.md; Fehrest release pin`
- [ ] T048 [R3] Prepare DeepMed bounded worker with exact OpenMed PyPI pin (no fork/import) — `DeepMed-AI/** and/or worker packaging; lockfile pin; NOTICE; model-license manifest`
- [ ] T049 [R3] Implement bounded DeepMed pipeline (task-first, spans, confidence, review, provenance) — `DeepMed worker; apps/desktop supervision; Studio UI presentation only`
- [ ] T050 [R3] Persist reviewed DeepMed results into Fehrest (slice step 4) — `apps/desktop/**; Fehrest integration; Artifact Store`
- [ ] T051 [R3] Implement commandF validate/transform worker (supervised; Fanatir-owned) — `services/ or workers/commandf/**; apps/desktop/**; specs/001-fanatir-repository-and-architecture-reconstitution/contracts/commandf.md`
- [ ] T052 [R3] Implement guided Lab Python and SQL runtimes (R remains Preview) — `apps/desktop/**; Lab UI presentation; governed runtimes`
- [ ] T053 [R3] Implement CoLab bounded review/comment/approval (not advanced realtime) — `afia-ui presentation; host Approval/Review records`
- [ ] T054 [R3] Implement secure sharing/export controls (classify→scan→redact→preview→confirm→audit→expiry) — `apps/desktop/**; ExportManifest; social-share gating`
- [ ] T069 [P] [R3] Add worker crash/restart/recovery, malformed/oversized IPC, and unknown-contract-version rejection tests — `tests/**`
- [ ] T055 [R3] R3 golden-journey evidence pack (Trusted Host path; not Vite-only) — `docs/program-memory/baseline/R3-golden-journey-evidence.md`

**Checkpoint**: T055 R3 golden-journey evidence on Trusted Host

---

## Phase R4 — Compatibility Migration

**Purpose**: Adapters, Supabase-optional proofs, auth compatibility, CI honesty

- [ ] T056 [R4] Introduce compatibility adapters for lib/services strangler without mass rename — `lib/**; services/**; apps/desktop adapters`
- [ ] T057 [R4] Harden Supabase optional adapter tests (not content/Artifact/patient/policy SoT) — `tests/**; specs/001-fanatir-repository-and-architecture-reconstitution/contracts/supabase-adapter.md`
- [ ] T058 [R4] Auth/session compatibility verification (no behavior change) — `tests/**; docs/program-memory/baseline/R4-auth-compat.md`
- [ ] T059 [R4] CI honesty pass: document non-gating jobs; add real contract test jobs where ready — `.github/workflows/**; tests/**`
- [ ] T070 [R4] Offline mode and Supabase-optional mode verification — `tests/**; docs/program-memory/baseline/R4-offline.md`

**Checkpoint**: Parity/auth matrices green

---

## Phase R5 — Founder Alpha Packaging and Acceptance

**Purpose**: Signing custody, signed Windows Alpha, clean install, Alpha gate

**Gate**: Decision D custody (**T060**) before distribution (**T061+**)

- [ ] T060 [R5] Establish signing custody controls (custodian, HSM/service, rotation/recovery/revocation/emergency) — `docs/program-memory/release/signing-custody.md`
- [ ] T061 [R5] Produce signed Windows Alpha installer (distributable only after T060) — `apps/desktop/bundle/**; release artifacts (out of repo as needed)`
- [ ] T062 [R5] Windows clean-install verification — `docs/program-memory/baseline/R5-clean-install.md`
- [ ] T063 [P] [R5] Accessibility and performance measurement recording for Alpha surfaces — `docs/program-memory/baseline/R5-a11y-perf.md`
- [ ] T064 [P] [R5] Reproducibility and export/import integrity verification — `docs/program-memory/baseline/R5-repro-export.md; tests/**`
- [ ] T065 [R5] Rollback exercise for host/slice/sidecar pins — `docs/program-memory/baseline/R5-rollback-drill.md`
- [ ] T066 [R5] Founder Alpha gate: full Trusted Host golden journey acceptance — `docs/program-memory/baseline/R5-founder-alpha-acceptance.md`
- [ ] T067 [R5] Program-memory closeout after Alpha acceptance — `docs/program-memory/CURRENT-STATE.md; docs/program-memory/NEXT-ACTION.md; docs/program-memory/decisions/**`

**Checkpoint**: T066 Founder Alpha acceptance; T067 program-memory closeout

---

## Task detail catalog

#### T001 — Record exact repository baseline (HEAD, branch, remotes, sibling repos)
- **Stage**: R0
- **Objective**: Record exact repository baseline (HEAD, branch, remotes, sibling repos)
- **Repository**: Fanatir
- **Expected paths**: `docs/program-memory/baseline/R0-repo-baseline.md`
- **Dependencies**: none
- **Required ADR/spec gate**: Spec 001 + Plan accepted
- **Allowed changes**: Documentation only
- **Prohibited changes**: Production code; sibling repos
- **Acceptance criteria**: Baseline markdown lists SHAs/branches/status
- **Verification**: `git rev-parse HEAD`; `git status -sb`; Fehrest/DeepMed status
- **Rollback**: Delete baseline doc
- **Security/privacy**: No secrets/PHI
- **Independent review**: Tier C — normal verification
- **Founder acceptance**: No


#### T002 — Verify actual build/dev commands for afia-ui and record evidence
- **Stage**: R0
- **Objective**: Verify actual build/dev commands for afia-ui and record evidence
- **Repository**: Fanatir
- **Expected paths**: `docs/program-memory/baseline/R0-build-commands.md; afia-ui/package.json`
- **Dependencies**: T001
- **Required ADR/spec gate**: Plan R0
- **Allowed changes**: Read-only + documentation
- **Prohibited changes**: Changing package scripts; production mutation
- **Acceptance criteria**: Documented scripts with run/fail evidence
- **Verification**: Record command outputs (no secrets)
- **Rollback**: Revert doc
- **Security/privacy**: No env secrets in logs
- **Independent review**: Tier C — normal verification
- **Founder acceptance**: No


#### T003 — Verify dependency managers (pnpm/node/rust/python/uv) and versions
- **Stage**: R0
- **Objective**: Verify dependency managers (pnpm/node/rust/python/uv) and versions
- **Repository**: Fanatir
- **Expected paths**: `docs/program-memory/baseline/R0-tooling.md`
- **Dependencies**: T001
- **Required ADR/spec gate**: Plan R0
- **Allowed changes**: Documentation
- **Prohibited changes**: Installing unrelated tools into product tree
- **Acceptance criteria**: Tooling matrix recorded
- **Verification**: `node -v`; `pnpm -v`; `rustc -V` if present; `python --version`
- **Rollback**: Revert doc
- **Security/privacy**: N/A
- **Independent review**: Tier C — normal verification
- **Founder acceptance**: No


#### T004 — Map active frontend entry points (Vite client App/routes/Shell)
- **Stage**: R0
- **Objective**: Map active frontend entry points (Vite client App/routes/Shell)
- **Repository**: Fanatir
- **Expected paths**: `docs/program-memory/baseline/R0-frontend-map.md; afia-ui/client/src/App.tsx`
- **Dependencies**: T001
- **Required ADR/spec gate**: Plan R0
- **Allowed changes**: Documentation
- **Prohibited changes**: UI redesign; route changes
- **Acceptance criteria**: Entry/route map matches disk
- **Verification**: Path existence + import graph notes
- **Rollback**: Revert doc
- **Security/privacy**: No PHI dumps
- **Independent review**: Tier C — normal verification
- **Founder acceptance**: No


#### T005 — Verify auth/session/PrivateRoute/profile behavior (observe-only)
- **Stage**: R0
- **Objective**: Verify auth/session/PrivateRoute/profile behavior (observe-only)
- **Repository**: Fanatir
- **Expected paths**: `docs/program-memory/baseline/R0-auth-session.md; afia-ui/client/src/contexts/AuthContext.tsx; afia-ui/client/src/App.tsx`
- **Dependencies**: T004
- **Required ADR/spec gate**: ADR-11 awareness
- **Allowed changes**: Documentation of current behavior
- **Prohibited changes**: Any auth/session/PrivateRoute/profile code change
- **Acceptance criteria**: Behavioral contract recorded; ProfileGate absent confirmed
- **Verification**: Code inspection evidence
- **Rollback**: Revert doc
- **Security/privacy**: No credentials in docs
- **Independent review**: Tier A — auth/session compatibility observation
- **Founder acceptance**: No


#### T006 — Inspect Supabase clients and both migration trees (freeze inventory)
- **Stage**: R0
- **Objective**: Inspect Supabase clients and both migration trees (freeze inventory)
- **Repository**: Fanatir
- **Expected paths**: `docs/program-memory/baseline/R0-supabase-inventory.md; afia-ui/supabase/**; supabase/migrations/**`
- **Dependencies**: T001
- **Required ADR/spec gate**: Decision C / ADR-07 awareness
- **Allowed changes**: Documentation
- **Prohibited changes**: Any migration create/edit/delete; documents-crypto expansion
- **Acceptance criteria**: Dual trees inventoried; documents-crypto labeled legacy PHI-egress seam
- **Verification**: `git ls-files` migrations
- **Rollback**: Revert doc
- **Security/privacy**: No PHI; no keys
- **Independent review**: Tier A — Supabase/PHI seam inventory
- **Founder acceptance**: No


#### T007 — Inspect Python/FHIR/OpenMed prototype services
- **Stage**: R0
- **Objective**: Inspect Python/FHIR/OpenMed prototype services
- **Repository**: Fanatir
- **Expected paths**: `docs/program-memory/baseline/R0-python-services.md; services/**`
- **Dependencies**: T001
- **Required ADR/spec gate**: Decision B awareness
- **Allowed changes**: Documentation
- **Prohibited changes**: Import OpenMed source; service rewrite
- **Acceptance criteria**: openmed_bridge/fhir_gate classified as prototypes
- **Verification**: File inventory
- **Rollback**: Revert doc
- **Security/privacy**: License note only; no weights
- **Independent review**: Tier C — normal verification
- **Founder acceptance**: No


#### T008 — Inspect archived Rust/Tauri/Go assets without reactivation
- **Stage**: R0
- **Objective**: Inspect archived Rust/Tauri/Go assets without reactivation
- **Repository**: Fanatir
- **Expected paths**: `docs/program-memory/baseline/R0-archived-assets.md; _archived/**`
- **Dependencies**: T001
- **Required ADR/spec gate**: Plan R0
- **Allowed changes**: Documentation
- **Prohibited changes**: Reactivating archived crates as authority; deleting archive
- **Acceptance criteria**: Archive classified non-authoritative
- **Verification**: Directory listing
- **Rollback**: Revert doc
- **Security/privacy**: N/A
- **Independent review**: Tier C — normal verification
- **Founder acceptance**: No


#### T009 — Inspect current CI and tests honesty
- **Stage**: R0
- **Objective**: Inspect current CI and tests honesty
- **Repository**: Fanatir
- **Expected paths**: `docs/program-memory/baseline/R0-ci-tests.md; .github/workflows/**; tests/**`
- **Dependencies**: T001
- **Required ADR/spec gate**: Plan R0
- **Allowed changes**: Documentation
- **Prohibited changes**: Falsely claiming green quality gates
- **Acceptance criteria**: CI TODOs/scaffolds identified
- **Verification**: Workflow content inspection
- **Rollback**: Revert doc
- **Security/privacy**: N/A
- **Independent review**: Tier C — normal verification
- **Founder acceptance**: No


#### T010 — Classify paths active/scaffold/archived/orphaned/contradictory
- **Stage**: R0
- **Objective**: Classify paths active/scaffold/archived/orphaned/contradictory
- **Repository**: Fanatir
- **Expected paths**: `docs/program-memory/baseline/R0-path-classification.md`
- **Dependencies**: T004,T006,T007,T008,T009
- **Required ADR/spec gate**: Spec disposition legend
- **Allowed changes**: Documentation aligning with Spec 001 matrix
- **Prohibited changes**: Changing dispositions in code
- **Acceptance criteria**: Classification table complete
- **Verification**: Cross-check Spec disposition matrix
- **Rollback**: Revert doc
- **Security/privacy**: N/A
- **Independent review**: Tier B — stage baseline classification
- **Founder acceptance**: No


#### T011 — Produce R0 baseline evidence package and program-memory closeout
- **Stage**: R0
- **Objective**: Produce R0 baseline evidence package and program-memory closeout
- **Repository**: Fanatir
- **Expected paths**: `docs/program-memory/baseline/**; docs/program-memory/CURRENT-STATE.md; docs/program-memory/NEXT-ACTION.md`
- **Dependencies**: T001-T010
- **Required ADR/spec gate**: Plan R0 exit
- **Allowed changes**: Docs/program-memory only
- **Prohibited changes**: Production mutation
- **Acceptance criteria**: R0 exit criteria satisfied; no prod diffs
- **Verification**: `git status --short`
- **Rollback**: Revert docs
- **Security/privacy**: No secrets
- **Independent review**: Tier C — normal verification
- **Founder acceptance**: No


#### T012 — Author ADR-15 Rust-First Polyglot Runtime and Language Authority (draft only)
- **Stage**: R1
- **Objective**: Author ADR-15 Rust-First Polyglot Runtime and Language Authority (draft only)
- **Repository**: Fanatir
- **Expected paths**: `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-15-*.md`
- **Dependencies**: T011
- **Required ADR/spec gate**: Decision A; MUST accept at T030 before R2
- **Allowed changes**: ADR draft markdown only; status remains Proposed until stage-gate acceptance (draft ≠ accepted)
- **Prohibited changes**: No OpenMed/Graphify fork/import; no Pictorial/Montada; no migration mutation; no auth/session behavior change without ADR-11 + dedicated spec; no Go Alpha dependency; no Rust rewrite of OpenMed/Graphify/Jupyter/R; no push/PR; no production code; draft ≠ accepted
- **Acceptance criteria**: ADR-15 draft complete with decision/consequences/gates; status Proposed
- **Verification**: Doc review; link from plan roadmap
- **Rollback**: Revert ADR file
- **Security/privacy**: Claim language; PHI posture where relevant
- **Independent review**: Tier A — authority-boundary ADR
- **Founder acceptance**: No — draft only; accept at T030


#### T013 — Author ADR-01 Platform and Desktop Composition (Tauri 2) (draft only)
- **Stage**: R1
- **Objective**: Author ADR-01 Platform and Desktop Composition (Tauri 2) (draft only)
- **Repository**: Fanatir
- **Expected paths**: `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-01-*.md`
- **Dependencies**: T012
- **Required ADR/spec gate**: ADR-15 constrains
- **Allowed changes**: ADR draft markdown only; status remains Proposed until stage-gate acceptance (draft ≠ accepted)
- **Prohibited changes**: No OpenMed/Graphify fork/import; no Pictorial/Montada; no migration mutation; no auth/session behavior change without ADR-11 + dedicated spec; no Go Alpha dependency; no Rust rewrite of OpenMed/Graphify/Jupyter/R; no push/PR; no production code; draft ≠ accepted
- **Acceptance criteria**: ADR-01 draft complete with decision/consequences/gates; status Proposed
- **Verification**: Doc review; link from plan roadmap
- **Rollback**: Revert ADR file
- **Security/privacy**: Claim language; PHI posture where relevant
- **Independent review**: Tier A — platform ADR
- **Founder acceptance**: No — draft only


#### T014 — Author ADR-02 Rust Trusted Host Boundary (draft only)
- **Stage**: R1
- **Objective**: Author ADR-02 Rust Trusted Host Boundary (draft only)
- **Repository**: Fanatir
- **Expected paths**: `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-02-*.md`
- **Dependencies**: T012,T013
- **Required ADR/spec gate**: ADR-15+01
- **Allowed changes**: ADR draft markdown only; status remains Proposed until stage-gate acceptance (draft ≠ accepted)
- **Prohibited changes**: No OpenMed/Graphify fork/import; no Pictorial/Montada; no migration mutation; no auth/session behavior change without ADR-11 + dedicated spec; no Go Alpha dependency; no Rust rewrite of OpenMed/Graphify/Jupyter/R; no push/PR; no production code; draft ≠ accepted
- **Acceptance criteria**: ADR-02 draft complete with decision/consequences/gates; status Proposed
- **Verification**: Doc review; link from plan roadmap
- **Rollback**: Revert ADR file
- **Security/privacy**: Claim language; PHI posture where relevant
- **Independent review**: Tier A — Trusted Host ADR
- **Founder acceptance**: No — draft only


#### T015 — Author ADR-03 afia-ui Strangler Migration (draft only)
- **Stage**: R1
- **Objective**: Author ADR-03 afia-ui Strangler Migration (draft only)
- **Repository**: Fanatir
- **Expected paths**: `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-03-*.md`
- **Dependencies**: T012,T013
- **Required ADR/spec gate**: Q1
- **Allowed changes**: ADR draft markdown only; status remains Proposed until stage-gate acceptance (draft ≠ accepted)
- **Prohibited changes**: No OpenMed/Graphify fork/import; no Pictorial/Montada; no migration mutation; no auth/session behavior change without ADR-11 + dedicated spec; no Go Alpha dependency; no Rust rewrite of OpenMed/Graphify/Jupyter/R; no push/PR; no production code; draft ≠ accepted
- **Acceptance criteria**: ADR-03 draft complete with decision/consequences/gates; status Proposed
- **Verification**: Doc review; link from plan roadmap
- **Rollback**: Revert ADR file
- **Security/privacy**: Claim language; PHI posture where relevant
- **Independent review**: Tier C — normal verification
- **Founder acceptance**: No — draft only


#### T016 — Author ADR-04 Shared Primitive Ownership and Versioning (draft only)
- **Stage**: R1
- **Objective**: Author ADR-04 Shared Primitive Ownership and Versioning (draft only)
- **Repository**: Fanatir
- **Expected paths**: `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-04-*.md`
- **Dependencies**: T012
- **Required ADR/spec gate**: ADR-15
- **Allowed changes**: ADR draft markdown only; status remains Proposed until stage-gate acceptance (draft ≠ accepted)
- **Prohibited changes**: No OpenMed/Graphify fork/import; no Pictorial/Montada; no migration mutation; no auth/session behavior change without ADR-11 + dedicated spec; no Go Alpha dependency; no Rust rewrite of OpenMed/Graphify/Jupyter/R; no push/PR; no production code; draft ≠ accepted
- **Acceptance criteria**: ADR-04 draft complete with decision/consequences/gates; status Proposed
- **Verification**: Doc review; link from plan roadmap
- **Rollback**: Revert ADR file
- **Security/privacy**: Claim language; PHI posture where relevant
- **Independent review**: Tier A — shared primitives ADR
- **Founder acceptance**: No — draft only


#### T017 — Author ADR-06 Worker and IPC Contracts (draft only)
- **Stage**: R1
- **Objective**: Author ADR-06 Worker and IPC Contracts (draft only)
- **Repository**: Fanatir
- **Expected paths**: `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-06-*.md`
- **Dependencies**: T012,T014
- **Required ADR/spec gate**: ADR-15+02
- **Allowed changes**: ADR draft markdown only; status remains Proposed until stage-gate acceptance (draft ≠ accepted)
- **Prohibited changes**: No OpenMed/Graphify fork/import; no Pictorial/Montada; no migration mutation; no auth/session behavior change without ADR-11 + dedicated spec; no Go Alpha dependency; no Rust rewrite of OpenMed/Graphify/Jupyter/R; no push/PR; no production code; draft ≠ accepted
- **Acceptance criteria**: ADR-06 draft complete with decision/consequences/gates; status Proposed
- **Verification**: Doc review; link from plan roadmap
- **Rollback**: Revert ADR file
- **Security/privacy**: Claim language; PHI posture where relevant
- **Independent review**: Tier A — IPC/worker ADR
- **Founder acceptance**: No — draft only


#### T018 — Author ADR-05 Artifact/Revision/Run Storage (draft only)
- **Stage**: R1
- **Objective**: Author ADR-05 Artifact/Revision/Run Storage (draft only)
- **Repository**: Fanatir
- **Expected paths**: `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-05-*.md`
- **Dependencies**: T012,T014,T016
- **Required ADR/spec gate**: ADR-15+02+04; Decision C; ADR-04 before ADR-05
- **Allowed changes**: ADR draft markdown only; status remains Proposed until stage-gate acceptance (draft ≠ accepted)
- **Prohibited changes**: No OpenMed/Graphify fork/import; no Pictorial/Montada; no migration mutation; no auth/session behavior change without ADR-11 + dedicated spec; no Go Alpha dependency; no Rust rewrite of OpenMed/Graphify/Jupyter/R; no push/PR; no production code; draft ≠ accepted
- **Acceptance criteria**: ADR-05 draft complete with decision/consequences/gates; status Proposed
- **Verification**: Doc review; link from plan roadmap
- **Rollback**: Revert ADR file
- **Security/privacy**: Claim language; PHI posture where relevant
- **Independent review**: Tier A — Artifact Store ADR
- **Founder acceptance**: No — draft only


#### T019 — Author ADR-07 Supabase Adapter and Local-First Boundary (draft only)
- **Stage**: R1
- **Objective**: Author ADR-07 Supabase Adapter and Local-First Boundary (draft only)
- **Repository**: Fanatir
- **Expected paths**: `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-07-*.md`
- **Dependencies**: T012,T006
- **Required ADR/spec gate**: Decision C; freeze documents-crypto; future 002 charter only
- **Allowed changes**: ADR draft markdown only; status remains Proposed until stage-gate acceptance (draft ≠ accepted)
- **Prohibited changes**: No OpenMed/Graphify fork/import; no Pictorial/Montada; no migration mutation; no auth/session behavior change without ADR-11 + dedicated spec; no Go Alpha dependency; no Rust rewrite of OpenMed/Graphify/Jupyter/R; no push/PR; no production code; draft ≠ accepted
- **Acceptance criteria**: ADR-07 draft complete with decision/consequences/gates; status Proposed
- **Verification**: Doc review; link from plan roadmap
- **Rollback**: Revert ADR file
- **Security/privacy**: Claim language; PHI posture where relevant
- **Independent review**: Tier A — Supabase/local-first ADR
- **Founder acceptance**: No — draft only; Decision C already ratified


#### T020 — Author ADR-08 Fehrest Integration and Release Model (draft only)
- **Stage**: R1
- **Objective**: Author ADR-08 Fehrest Integration and Release Model (draft only)
- **Repository**: Fanatir
- **Expected paths**: `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-08-*.md`
- **Dependencies**: T012
- **Required ADR/spec gate**: ADR-15; Q5
- **Allowed changes**: ADR draft markdown only; status remains Proposed until stage-gate acceptance (draft ≠ accepted)
- **Prohibited changes**: No OpenMed/Graphify fork/import; no Pictorial/Montada; no migration mutation; no auth/session behavior change without ADR-11 + dedicated spec; no Go Alpha dependency; no Rust rewrite of OpenMed/Graphify/Jupyter/R; no push/PR; no production code; draft ≠ accepted
- **Acceptance criteria**: ADR-08 draft complete with decision/consequences/gates; status Proposed
- **Verification**: Doc review; link from plan roadmap
- **Rollback**: Revert ADR file
- **Security/privacy**: Claim language; PHI posture where relevant
- **Independent review**: Tier A — Fehrest integration ADR
- **Founder acceptance**: No — draft only


#### T021 — Author ADR-09 DeepMed Integration and OpenMed Runtime/Fork Boundary (draft only)
- **Stage**: R1
- **Objective**: Author ADR-09 DeepMed Integration and OpenMed Runtime/Fork Boundary (draft only)
- **Repository**: Fanatir
- **Expected paths**: `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-09-*.md`
- **Dependencies**: T012,T007
- **Required ADR/spec gate**: Decision B; no fork/import task under 001
- **Allowed changes**: ADR draft markdown only; status remains Proposed until stage-gate acceptance (draft ≠ accepted)
- **Prohibited changes**: No OpenMed/Graphify fork/import; no Pictorial/Montada; no migration mutation; no auth/session behavior change without ADR-11 + dedicated spec; no Go Alpha dependency; no Rust rewrite of OpenMed/Graphify/Jupyter/R; no push/PR; no production code; draft ≠ accepted
- **Acceptance criteria**: ADR-09 draft complete with decision/consequences/gates; status Proposed
- **Verification**: Doc review; link from plan roadmap
- **Rollback**: Revert ADR file
- **Security/privacy**: Claim language; PHI posture where relevant
- **Independent review**: Tier A — DeepMed/OpenMed boundary ADR
- **Founder acceptance**: No — draft only; Decision B already ratified


#### T022 — Author ADR-10 commandF Ownership and Process Boundary (draft only)
- **Stage**: R1
- **Objective**: Author ADR-10 commandF Ownership and Process Boundary (draft only)
- **Repository**: Fanatir
- **Expected paths**: `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-10-*.md`
- **Dependencies**: T012
- **Required ADR/spec gate**: ADR-15
- **Allowed changes**: ADR draft markdown only; status remains Proposed until stage-gate acceptance (draft ≠ accepted)
- **Prohibited changes**: No OpenMed/Graphify fork/import; no Pictorial/Montada; no migration mutation; no auth/session behavior change without ADR-11 + dedicated spec; no Go Alpha dependency; no Rust rewrite of OpenMed/Graphify/Jupyter/R; no push/PR; no production code; draft ≠ accepted
- **Acceptance criteria**: ADR-10 draft complete with decision/consequences/gates; status Proposed
- **Verification**: Doc review; link from plan roadmap
- **Rollback**: Revert ADR file
- **Security/privacy**: Claim language; PHI posture where relevant
- **Independent review**: Tier C — normal verification
- **Founder acceptance**: No — draft only


#### T023 — Author ADR-11 Auth and Session Preservation (draft only)
- **Stage**: R1
- **Objective**: Author ADR-11 Auth and Session Preservation (draft only)
- **Repository**: Fanatir
- **Expected paths**: `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-11-*.md`
- **Dependencies**: T005
- **Required ADR/spec gate**: No behavior change under 001
- **Allowed changes**: ADR draft markdown only; status remains Proposed until stage-gate acceptance (draft ≠ accepted)
- **Prohibited changes**: No OpenMed/Graphify fork/import; no Pictorial/Montada; no migration mutation; no auth/session behavior change without ADR-11 + dedicated spec; no Go Alpha dependency; no Rust rewrite of OpenMed/Graphify/Jupyter/R; no push/PR; no production code; draft ≠ accepted
- **Acceptance criteria**: ADR-11 draft complete with decision/consequences/gates; status Proposed
- **Verification**: Doc review; link from plan roadmap
- **Rollback**: Revert ADR file
- **Security/privacy**: Claim language; PHI posture where relevant
- **Independent review**: Tier A — auth preservation ADR
- **Founder acceptance**: No — draft only


#### T024 — Author ADR-12 Technical AFIA-to-Fanatir Rename Strategy (draft only)
- **Stage**: R1
- **Objective**: Author ADR-12 Technical AFIA-to-Fanatir Rename Strategy (draft only)
- **Repository**: Fanatir
- **Expected paths**: `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-12-*.md`
- **Dependencies**: T011
- **Required ADR/spec gate**: Deferred mass rename
- **Allowed changes**: ADR draft markdown only; status remains Proposed until stage-gate acceptance (draft ≠ accepted)
- **Prohibited changes**: No OpenMed/Graphify fork/import; no Pictorial/Montada; no migration mutation; no auth/session behavior change without ADR-11 + dedicated spec; no Go Alpha dependency; no Rust rewrite of OpenMed/Graphify/Jupyter/R; no push/PR; no production code; draft ≠ accepted
- **Acceptance criteria**: ADR-12 draft complete with decision/consequences/gates; status Proposed
- **Verification**: Doc review; link from plan roadmap
- **Rollback**: Revert ADR file
- **Security/privacy**: Claim language; PHI posture where relevant
- **Independent review**: Tier C — normal verification
- **Founder acceptance**: No — draft only


#### T025 — Author ADR-13 First Vertical-Slice Packaging (draft only)
- **Stage**: R1
- **Objective**: Author ADR-13 First Vertical-Slice Packaging (draft only)
- **Repository**: Fanatir
- **Expected paths**: `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-13-*.md`
- **Dependencies**: T012,T013,T014
- **Required ADR/spec gate**: P4 staged hybrid
- **Allowed changes**: ADR draft markdown only; status remains Proposed until stage-gate acceptance (draft ≠ accepted)
- **Prohibited changes**: No OpenMed/Graphify fork/import; no Pictorial/Montada; no migration mutation; no auth/session behavior change without ADR-11 + dedicated spec; no Go Alpha dependency; no Rust rewrite of OpenMed/Graphify/Jupyter/R; no push/PR; no production code; draft ≠ accepted
- **Acceptance criteria**: ADR-13 draft complete with decision/consequences/gates; status Proposed
- **Verification**: Doc review; link from plan roadmap
- **Rollback**: Revert ADR file
- **Security/privacy**: Claim language; PHI posture where relevant
- **Independent review**: Tier C — normal verification
- **Founder acceptance**: No — draft only


#### T026 — Author ADR-14 Security and Data-Classification Enforcement (draft only)
- **Stage**: R1
- **Objective**: Author ADR-14 Security and Data-Classification Enforcement (draft only)
- **Repository**: Fanatir
- **Expected paths**: `specs/001-fanatir-repository-and-architecture-reconstitution/adrs/ADR-14-*.md`
- **Dependencies**: T012,T018,T019
- **Required ADR/spec gate**: Decision C/D
- **Allowed changes**: ADR draft markdown only; status remains Proposed until stage-gate acceptance (draft ≠ accepted)
- **Prohibited changes**: No OpenMed/Graphify fork/import; no Pictorial/Montada; no migration mutation; no auth/session behavior change without ADR-11 + dedicated spec; no Go Alpha dependency; no Rust rewrite of OpenMed/Graphify/Jupyter/R; no push/PR; no production code; draft ≠ accepted
- **Acceptance criteria**: ADR-14 draft complete with decision/consequences/gates; status Proposed
- **Verification**: Doc review; link from plan roadmap
- **Rollback**: Revert ADR file
- **Security/privacy**: Claim language; PHI posture where relevant
- **Independent review**: Tier A — security/classification ADR
- **Founder acceptance**: No — draft only


#### T027 — Publish/align shared primitive contract boundaries (planning schemas only)
- **Stage**: R1
- **Objective**: Publish/align shared primitive contract boundaries (planning schemas only)
- **Repository**: Fanatir
- **Expected paths**: `specs/001-fanatir-repository-and-architecture-reconstitution/contracts/shared-primitives.md; specs/001-fanatir-repository-and-architecture-reconstitution/data-model.md`
- **Dependencies**: T016
- **Required ADR/spec gate**: ADR-04 drafted
- **Allowed changes**: Contract docs only
- **Prohibited changes**: Finalizing DB schemas; production migration; packages/contracts production publish not required
- **Acceptance criteria**: Language-neutral primitive boundaries published in named contract paths
- **Verification**: Contract review vs data-model.md
- **Rollback**: Revert docs
- **Security/privacy**: Classification fields present
- **Independent review**: Tier B — contract group
- **Founder acceptance**: No


#### T028 — Freeze documents-crypto for real patient data; document PHI-egress seam controls
- **Stage**: R1
- **Objective**: Freeze documents-crypto for real patient data; document PHI-egress seam controls
- **Repository**: Fanatir
- **Expected paths**: `docs/program-memory/decisions/documents-crypto-freeze.md; specs/001-fanatir-repository-and-architecture-reconstitution/contracts/supabase-adapter.md`
- **Dependencies**: T019
- **Required ADR/spec gate**: ADR-07 draft; Decision C
- **Allowed changes**: Documentation + policy notices
- **Prohibited changes**: Editing migrations; expanding documents-crypto; deleting path
- **Acceptance criteria**: Freeze policy recorded; synthetic/test-only rule explicit; preserve for inspection
- **Verification**: Reviewer sign-off note
- **Rollback**: Revert docs
- **Security/privacy**: PHI-egress control
- **Independent review**: Tier A — PHI-egress freeze
- **Founder acceptance**: No — operationalizes Decision C


#### T029 — Define future-002 Supabase canonicalization specification charter (do not create 002)
- **Stage**: R1
- **Objective**: Define future-002 Supabase canonicalization specification charter (do not create 002)
- **Repository**: Fanatir
- **Expected paths**: `docs/program-memory/decisions/002-charter.md`
- **Dependencies**: T019,T028
- **Required ADR/spec gate**: Decision C
- **Allowed changes**: Charter doc only
- **Prohibited changes**: Creating specs/002 or editing migrations
- **Acceptance criteria**: 002 name+scope charter exists; specs/002 directory absent
- **Verification**: Path exists; no specs/002 directory
- **Rollback**: Revert charter
- **Security/privacy**: N/A
- **Independent review**: Tier C — normal verification
- **Founder acceptance**: No


#### T030 — R1 acceptance gate: accept ADR-15/01/02/06; review R3-blocking ADRs; program-memory update
- **Stage**: R1
- **Objective**: R1 acceptance gate: accept ADR-15/01/02/06; review R3-blocking ADRs; program-memory update
- **Repository**: Fanatir
- **Expected paths**: `docs/program-memory/CURRENT-STATE.md; docs/program-memory/NEXT-ACTION.md; specs/001-fanatir-repository-and-architecture-reconstitution/adrs/**`
- **Dependencies**: T012-T029
- **Required ADR/spec gate**: Plan R1 exit; ADR-15+01+02+06 Accepted before any R2 implementation
- **Allowed changes**: Program-memory + ADR status metadata only
- **Prohibited changes**: Starting R2 implementation without ADR-15/01/02/06 Accepted
- **Acceptance criteria**: Program-memory states ADR-15/01/02/06 Accepted; no R2 impl commits exist; ADR-04/05/07/08/09/10/14 listed Reviewed or Accepted
- **Verification**: Plan blocking matrix checklist
- **Rollback**: Revert status flips
- **Security/privacy**: N/A
- **Independent review**: Tier A — R1 architecture gate
- **Founder acceptance**: Yes — R1 architecture gate


#### T031 — Create minimal Tauri 2 desktop shell loading afia-ui client
- **Stage**: R2
- **Objective**: Create minimal Tauri 2 desktop shell loading afia-ui client
- **Repository**: Fanatir
- **Expected paths**: `apps/desktop/**; afia-ui/client/** (consume only)`
- **Dependencies**: T030
- **Required ADR/spec gate**: ADR-15 + ADR-01 + ADR-02 Accepted; R1 exit (T030). ADR-06 NOT required for shell-only
- **Allowed changes**: New apps/desktop Tauri scaffold; wire to existing UI
- **Prohibited changes**: Auth changes; full kernel; Go; archive reactivation
- **Acceptance criteria**: Desktop shell launches UI in Windows-first dev
- **Verification**: tauri/vite smoke
- **Rollback**: Remove apps/desktop
- **Security/privacy**: WebView untrusted
- **Independent review**: Tier A — Trusted Host shell
- **Founder acceptance**: No


#### T032 — Implement Rust Trusted Host skeleton (project/workspace authority stubs)
- **Stage**: R2
- **Objective**: Implement Rust Trusted Host skeleton (project/workspace authority stubs)
- **Repository**: Fanatir
- **Expected paths**: `apps/desktop/src-tauri/**`
- **Dependencies**: T031
- **Required ADR/spec gate**: ADR-15 + ADR-01 + ADR-02 Accepted; R1 exit (T030)
- **Allowed changes**: Rust host module skeleton
- **Prohibited changes**: TS owning host authority
- **Acceptance criteria**: Rust module is sole privileged entry
- **Verification**: Compile + unit stubs
- **Rollback**: Revert host modules
- **Security/privacy**: Capability deny-by-default
- **Independent review**: Tier A — Trusted Host
- **Founder acceptance**: No


#### T033 — Implement bounded filesystem/project open/create via Rust mediation
- **Stage**: R2
- **Objective**: Implement bounded filesystem/project open/create via Rust mediation
- **Repository**: Fanatir
- **Expected paths**: `apps/desktop/src-tauri/**; specs/001-fanatir-repository-and-architecture-reconstitution/contracts/trusted-host-ipc.md`
- **Dependencies**: T032
- **Required ADR/spec gate**: ADR-02 Accepted; ADR-15
- **Allowed changes**: Host FS commands
- **Prohibited changes**: Unrestricted FS from WebView/Python
- **Acceptance criteria**: Path escape denied; project open works
- **Verification**: FS escape tests
- **Rollback**: Revert commands
- **Security/privacy**: Path sandbox
- **Independent review**: Tier A — filesystem authority
- **Founder acceptance**: No


#### T034 — Implement versioned secure IPC (commands/events/channels) foundation
- **Stage**: R2
- **Objective**: Implement versioned secure IPC (commands/events/channels) foundation
- **Repository**: Fanatir
- **Expected paths**: `apps/desktop/src-tauri/**; specs/001-fanatir-repository-and-architecture-reconstitution/contracts/trusted-host-ipc.md`
- **Dependencies**: T032,T017,T030
- **Required ADR/spec gate**: ADR-06 Accepted; ADR-15
- **Allowed changes**: IPC layer
- **Prohibited changes**: Bypassing IPC for privileged ops
- **Acceptance criteria**: Versioned IPC works; malformed/oversized messages rejected; unknown contract versions rejected
- **Verification**: IPC contract tests
- **Rollback**: Revert IPC
- **Security/privacy**: Reject unknown commands
- **Independent review**: Tier A — IPC
- **Founder acceptance**: No


#### T035 — Implement worker supervision foundation (spawn/status/stop/restart)
- **Stage**: R2
- **Objective**: Implement worker supervision foundation (spawn/status/stop/restart)
- **Repository**: Fanatir
- **Expected paths**: `apps/desktop/src-tauri/**`
- **Dependencies**: T034,T030
- **Required ADR/spec gate**: ADR-06 Accepted; ADR-15
- **Allowed changes**: Supervisor for process-isolated workers
- **Prohibited changes**: Workers owning secrets/FS
- **Acceptance criteria**: Crash marks failed; restartable
- **Verification**: Crash/restart smoke
- **Rollback**: Disable supervisor
- **Security/privacy**: No unrestricted network grants
- **Independent review**: Tier A — worker supervision
- **Founder acceptance**: No


#### T036 — Implement audit event foundation in Rust
- **Stage**: R2
- **Objective**: Implement audit event foundation in Rust
- **Repository**: Fanatir
- **Expected paths**: `apps/desktop/src-tauri/**`
- **Dependencies**: T032,T026,T030
- **Required ADR/spec gate**: ADR-14 Reviewed/Accepted for audit foundation; ADR-15
- **Allowed changes**: Append-only audit API
- **Prohibited changes**: Authoritative audit in TS/Python
- **Acceptance criteria**: Privileged actions emit audit
- **Verification**: Audit emission tests
- **Rollback**: Revert audit module
- **Security/privacy**: No PHI in audit payloads by default
- **Independent review**: Tier A — audit
- **Founder acceptance**: No


#### T037 — Implement local Artifact Store foundation (encrypted local storage seam)
- **Stage**: R2
- **Objective**: Implement local Artifact Store foundation (encrypted local storage seam)
- **Repository**: Fanatir
- **Expected paths**: `apps/desktop/src-tauri/**`
- **Dependencies**: T032,T018,T030
- **Required ADR/spec gate**: ADR-05 Accepted before Artifact Store foundation; ADR-15; Decision C
- **Allowed changes**: Rust Artifact/Revision/Run foundation
- **Prohibited changes**: Worker-direct Artifact mutation; Supabase content SoT
- **Acceptance criteria**: put/get via host only
- **Verification**: Immutability/identity unit tests
- **Rollback**: Revert store
- **Security/privacy**: Encryption keys via secrets API
- **Independent review**: Tier A — Artifact Store
- **Founder acceptance**: No


#### T038 — Implement data-classification enforcement seam in gateway
- **Stage**: R2
- **Objective**: Implement data-classification enforcement seam in gateway
- **Repository**: Fanatir
- **Expected paths**: `apps/desktop/src-tauri/**`
- **Dependencies**: T036,T037,T026,T030
- **Required ADR/spec gate**: ADR-14 Accepted for classification enforcement seam
- **Allowed changes**: Classification + PolicyDecision hooks
- **Prohibited changes**: PHI default allow
- **Acceptance criteria**: Deny-by-default for phi egress
- **Verification**: PHI-egress denial tests
- **Rollback**: Revert gateway hooks
- **Security/privacy**: PHI posture
- **Independent review**: Tier A — data classification
- **Founder acceptance**: No


#### T039 — Windows development packaging spike (unsigned internal-only)
- **Stage**: R2
- **Objective**: Windows development packaging spike (unsigned internal-only)
- **Repository**: Fanatir
- **Expected paths**: `apps/desktop/**; docs/program-memory/baseline/R2-packaging-spike.md`
- **Dependencies**: T031
- **Required ADR/spec gate**: ADR-13 draft; Decision D (unsigned=internal)
- **Allowed changes**: Dev bundle spike docs + config
- **Prohibited changes**: Distributing unsigned builds; storing private keys
- **Acceptance criteria**: Spike notes; non-distributable label
- **Verification**: Build log evidence
- **Rollback**: Delete spike artifacts
- **Security/privacy**: No private keys in repo/CI vars
- **Independent review**: Tier B — packaging spike
- **Founder acceptance**: No


#### T040 — Heavy Python/ML sidecar externalBin feasibility spike
- **Stage**: R2
- **Objective**: Heavy Python/ML sidecar externalBin feasibility spike
- **Repository**: Fanatir
- **Expected paths**: `docs/program-memory/baseline/R2-sidecar-spike.md; apps/desktop/** (config only)`
- **Dependencies**: T035
- **Required ADR/spec gate**: ADR-06 Accepted; ADR-13 draft
- **Allowed changes**: Spike documentation + optional config
- **Prohibited changes**: Shipping unpinned OpenMed; fork/import
- **Acceptance criteria**: Feasibility recorded for R3
- **Verification**: Spike report
- **Rollback**: Revert config
- **Security/privacy**: License awareness
- **Independent review**: Tier B — sidecar spike
- **Founder acceptance**: No


#### T041 — Retain Vite fallback for development only; label non-Alpha
- **Stage**: R2
- **Objective**: Retain Vite fallback for development only; label non-Alpha
- **Repository**: Fanatir
- **Expected paths**: `docs/program-memory/decisions/vite-fallback.md`
- **Dependencies**: T031
- **Required ADR/spec gate**: ADR-13 draft; P4
- **Allowed changes**: Dev docs
- **Prohibited changes**: Using Vite-only as Alpha or R2/R5 exit evidence
- **Acceptance criteria**: Fallback explicitly non-Alpha
- **Verification**: Doc review
- **Rollback**: Revert docs
- **Security/privacy**: N/A
- **Independent review**: Tier C — normal verification
- **Founder acceptance**: No


#### T068 — Add filesystem-escape, secret-access-denial, and network-access-denial tests
- **Stage**: R2
- **Objective**: Add filesystem-escape, secret-access-denial, and network-access-denial tests
- **Repository**: Fanatir
- **Expected paths**: `tests/**`
- **Dependencies**: T033,T032
- **Required ADR/spec gate**: ADR-15/02/14
- **Allowed changes**: Tests
- **Prohibited changes**: Weakening denies to pass tests
- **Acceptance criteria**: Named negative tests fail-closed for FS escape, secret access, and unauthorized network access
- **Verification**: Automated tests
- **Rollback**: Revert tests
- **Security/privacy**: Security
- **Independent review**: Tier A — security negative tests
- **Founder acceptance**: No


#### T042 — R2 exit verification: project open, FS deny, worker lifecycle, audit, no Artifact bypass
- **Stage**: R2
- **Objective**: R2 exit verification: project open, FS deny, worker lifecycle, audit, no Artifact bypass
- **Repository**: Fanatir
- **Expected paths**: `docs/program-memory/baseline/R2-exit-evidence.md; tests/**`
- **Dependencies**: T033-T038,T068
- **Required ADR/spec gate**: Plan R2 exit
- **Allowed changes**: Tests + evidence docs
- **Prohibited changes**: Proceeding to R3 without evidence; Vite-only as R2 exit
- **Acceptance criteria**: R2 exit checklist green on Trusted Host path; Vite-only evidence rejected as R2 exit substitute
- **Verification**: Automated tests + Windows smoke
- **Rollback**: Revert failing features
- **Security/privacy**: Security tests included
- **Independent review**: Tier A — R2 foundation gate
- **Founder acceptance**: Yes — R2 trusted-foundation gate


#### T043 — Implement Artifact and Run contracts end-to-end (slice step 1)
- **Stage**: R3
- **Objective**: Implement Artifact and Run contracts end-to-end (slice step 1)
- **Repository**: Fanatir
- **Expected paths**: `apps/desktop/**; specs/001-fanatir-repository-and-architecture-reconstitution/contracts/**; tests/contract/**`
- **Dependencies**: T042,T018,T027
- **Required ADR/spec gate**: ADR-05/04/15 Accepted; ADR-14
- **Allowed changes**: Contract implementations + tests
- **Prohibited changes**: Supabase as Artifact SoT; worker mutation
- **Acceptance criteria**: Artifact immutability + Revision identity + Run provenance tests pass
- **Verification**: contract tests
- **Rollback**: Feature-flag off
- **Security/privacy**: Classification required
- **Independent review**: Tier A — Artifact/Run contracts
- **Founder acceptance**: No


#### T044 — Scaffold Fehrest bounded Alpha product (independent repo) — vault/memory minimum
- **Stage**: R3
- **Objective**: Scaffold Fehrest bounded Alpha product (independent repo) — vault/memory minimum
- **Repository**: Fehrest
- **Expected paths**: `Fehrest/**`
- **Dependencies**: T020,T043
- **Required ADR/spec gate**: ADR-08 Accepted; no Graphify import
- **Allowed changes**: Fehrest repo init for Alpha capabilities
- **Prohibited changes**: Graphify fork/import; Fehrest as FHIR/patient SoT
- **Acceptance criteria**: Independent vault runnable
- **Verification**: Fehrest smoke
- **Rollback**: Reset Fehrest to empty
- **Security/privacy**: Local-only vault default
- **Independent review**: Tier A — Fehrest trust boundary scaffold
- **Founder acceptance**: No — governed by accepted ADR-08


#### T045 — Implement Fehrest Markdown vault, notes, sources, quotations, backlinks, typed relationships, local search, and graph boundary
- **Stage**: R3
- **Objective**: Implement Fehrest Markdown vault, notes, sources, quotations, backlinks, typed relationships, local search, and graph boundary
- **Repository**: Fehrest
- **Expected paths**: `Fehrest/**`
- **Dependencies**: T044
- **Required ADR/spec gate**: ADR-08 Accepted
- **Allowed changes**: Fehrest features
- **Prohibited changes**: Notion parity; Graphify import
- **Acceptance criteria**: Alpha Fehrest checklist met including search and graph boundary; provenance on quotations
- **Verification**: Feature tests
- **Rollback**: Revert Fehrest commits
- **Security/privacy**: Provenance on quotations
- **Independent review**: Tier B — Fehrest feature batch
- **Founder acceptance**: No


#### T046 — Implement Fehrest project memory + human/extracted/inferred relationship states + portable export
- **Stage**: R3
- **Objective**: Implement Fehrest project memory + human/extracted/inferred relationship states + portable export
- **Repository**: Fehrest
- **Expected paths**: `Fehrest/**`
- **Dependencies**: T045
- **Required ADR/spec gate**: ADR-08; Relationship.origin
- **Allowed changes**: Memory/export
- **Prohibited changes**: Clinical patient authority in Fehrest
- **Acceptance criteria**: origin states + portable export
- **Verification**: Export integrity test
- **Rollback**: Revert
- **Security/privacy**: No PHI SoT claims
- **Independent review**: Tier B — Fehrest memory/export batch
- **Founder acceptance**: No


#### T047 — Wire versioned Fanatir↔Fehrest integration contract (Rust-supervised)
- **Stage**: R3
- **Objective**: Wire versioned Fanatir↔Fehrest integration contract (Rust-supervised)
- **Repository**: Fanatir + Fehrest
- **Expected paths**: `apps/desktop/**; specs/001-fanatir-repository-and-architecture-reconstitution/contracts/fehrest-integration.md; Fehrest release pin`
- **Dependencies**: T035,T046
- **Required ADR/spec gate**: ADR-08 Accepted; ADR-15
- **Allowed changes**: Integration adapter + pin
- **Prohibited changes**: Embedding Fehrest with host privileges
- **Acceptance criteria**: IPC-bounded Fehrest worker works
- **Verification**: Integration smoke
- **Rollback**: Disable integration
- **Security/privacy**: Capability constrained
- **Independent review**: Tier A — Fehrest integration
- **Founder acceptance**: No


#### T048 — Prepare DeepMed bounded worker with exact OpenMed PyPI pin (no fork/import)
- **Stage**: R3
- **Objective**: Prepare DeepMed bounded worker with exact OpenMed PyPI pin (no fork/import)
- **Repository**: DeepMed-AI and/or Fanatir
- **Expected paths**: `DeepMed-AI/** and/or worker packaging; lockfile pin; NOTICE; model-license manifest`
- **Dependencies**: T021,T040,T035,T042
- **Required ADR/spec gate**: ADR-09 Accepted; Decision B; R2 worker supervision available
- **Allowed changes**: Pinned worker packaging + license docs
- **Prohibited changes**: OpenMed/Graphify source fork/import; unrestricted network
- **Acceptance criteria**: Pin+NOTICE+manifest; local-by-default; replacement path documented
- **Verification**: License review checklist
- **Rollback**: Unpin/disable worker
- **Security/privacy**: License compliance
- **Independent review**: Tier A — DeepMed pin/license
- **Founder acceptance**: No — governed by accepted ADR-09


#### T049 — Implement bounded DeepMed pipeline (task-first, spans, confidence, review, provenance)
- **Stage**: R3
- **Objective**: Implement bounded DeepMed pipeline (task-first, spans, confidence, review, provenance)
- **Repository**: DeepMed-AI / Fanatir
- **Expected paths**: `DeepMed worker; apps/desktop supervision; Studio UI presentation only`
- **Dependencies**: T047,T048,T043
- **Required ADR/spec gate**: ADR-09 Accepted; ADR-15; Fehrest functional (T044–T047)
- **Allowed changes**: Worker + UI invoke path
- **Prohibited changes**: Claiming final DeepMed architecture; silent approval; OpenMed fork/import
- **Acceptance criteria**: Source spans+uncertainty+review+provenance evidenced; not represented as final architecture
- **Verification**: Contract tests + Studio smoke
- **Rollback**: Feature-flag off
- **Security/privacy**: No unrestricted FS/secrets/network/patient-store
- **Independent review**: Tier A — DeepMed pipeline
- **Founder acceptance**: No — journey acceptance at T055


#### T050 — Persist reviewed DeepMed results into Fehrest (slice step 4)
- **Stage**: R3
- **Objective**: Persist reviewed DeepMed results into Fehrest (slice step 4)
- **Repository**: Fanatir + Fehrest
- **Expected paths**: `apps/desktop/**; Fehrest integration; Artifact Store`
- **Dependencies**: T049,T047
- **Required ADR/spec gate**: ADR-08/09/05 Accepted
- **Allowed changes**: Persistence path after review
- **Prohibited changes**: Auto-persist without review
- **Acceptance criteria**: Reviewed results in Fehrest with provenance
- **Verification**: E2E step evidence
- **Rollback**: Revert persistence path
- **Security/privacy**: Classification on persisted notes
- **Independent review**: Tier A — DeepMed→Fehrest persistence
- **Founder acceptance**: No


#### T051 — Implement commandF validate/transform worker (supervised; Fanatir-owned)
- **Stage**: R3
- **Objective**: Implement commandF validate/transform worker (supervised; Fanatir-owned)
- **Repository**: Fanatir
- **Expected paths**: `services/ or workers/commandf/**; apps/desktop/**; specs/001-fanatir-repository-and-architecture-reconstitution/contracts/commandf.md`
- **Dependencies**: T022,T043,T050
- **Required ADR/spec gate**: ADR-10 Accepted; ADR-15
- **Allowed changes**: commandF worker + host wiring
- **Prohibited changes**: Live EHR writes; universal conversion
- **Acceptance criteria**: Guided validate/transform with explicit result states
- **Verification**: commandF tests
- **Rollback**: Disable worker
- **Security/privacy**: Bounded I/O
- **Independent review**: Tier A — commandF interoperability
- **Founder acceptance**: No


#### T052 — Implement guided Lab Python and SQL runtimes (R remains Preview)
- **Stage**: R3
- **Objective**: Implement guided Lab Python and SQL runtimes (R remains Preview)
- **Repository**: Fanatir
- **Expected paths**: `apps/desktop/**; Lab UI presentation; governed runtimes`
- **Dependencies**: T051,T035
- **Required ADR/spec gate**: ADR-15; Spec Lab tiers
- **Allowed changes**: Governed Lab runtimes via Rust supervision
- **Prohibited changes**: Promoting R to Required without founder change; unrestricted notebook FS
- **Acceptance criteria**: Python+SQL guided path works; R remains Preview
- **Verification**: Lab smoke tests
- **Rollback**: Disable Lab
- **Security/privacy**: Capability constrained
- **Independent review**: Tier A — Lab runtime supervision
- **Founder acceptance**: No


#### T053 — Implement CoLab bounded review/comment/approval (not advanced realtime)
- **Stage**: R3
- **Objective**: Implement CoLab bounded review/comment/approval (not advanced realtime)
- **Repository**: Fanatir
- **Expected paths**: `afia-ui presentation; host Approval/Review records`
- **Dependencies**: T050,T023
- **Required ADR/spec gate**: ADR-11; Review/Approval model
- **Allowed changes**: Bounded CoLab UX + records
- **Prohibited changes**: Advanced multiplayer editing; auth redesign
- **Acceptance criteria**: Approval required before share
- **Verification**: CoLab flow test
- **Rollback**: Feature-flag off
- **Security/privacy**: No silent clinical approval
- **Independent review**: Tier A — CoLab approval boundary
- **Founder acceptance**: No


#### T054 — Implement secure sharing/export controls (classify→scan→redact→preview→confirm→audit→expiry)
- **Stage**: R3
- **Objective**: Implement secure sharing/export controls (classify→scan→redact→preview→confirm→audit→expiry)
- **Repository**: Fanatir
- **Expected paths**: `apps/desktop/**; ExportManifest; social-share gating`
- **Dependencies**: T053,T038,T026
- **Required ADR/spec gate**: ADR-14 Accepted; Constitution secure share
- **Allowed changes**: Secure share pipeline
- **Prohibited changes**: Ungated social share of PHI; skipping audit on deny/fail
- **Acceptance criteria**: Denied/failed shares audited; approved public derivatives only; destination disclosed; confirmation required
- **Verification**: PHI-egress denial + share audit tests
- **Rollback**: Disable share
- **Security/privacy**: PHI controls
- **Independent review**: Tier A — secure sharing
- **Founder acceptance**: No — journey acceptance at T055


#### T069 — Add worker crash/restart/recovery, malformed/oversized IPC, and unknown-contract-version rejection tests
- **Stage**: R3
- **Objective**: Add worker crash/restart/recovery, malformed/oversized IPC, and unknown-contract-version rejection tests
- **Repository**: Fanatir
- **Expected paths**: `tests/**`
- **Dependencies**: T034,T035
- **Required ADR/spec gate**: ADR-06 Accepted
- **Allowed changes**: Tests
- **Prohibited changes**: Swallowing crashes as success
- **Acceptance criteria**: Crash→failed Run; IPC rejects bad messages; unknown contract versions rejected; restart evidence in audit where required
- **Verification**: Automated tests
- **Rollback**: Revert
- **Security/privacy**: Security
- **Independent review**: Tier A — IPC/worker negative tests
- **Founder acceptance**: No


#### T055 — R3 golden-journey evidence pack (Trusted Host path; not Vite-only)
- **Stage**: R3
- **Objective**: R3 golden-journey evidence pack (Trusted Host path; not Vite-only)
- **Repository**: Fanatir
- **Expected paths**: `docs/program-memory/baseline/R3-golden-journey-evidence.md`
- **Dependencies**: T043-T054,T069
- **Required ADR/spec gate**: Plan R3 exit; Fehrest and DeepMed both functioning
- **Allowed changes**: Evidence documentation + tests
- **Prohibited changes**: Claiming Alpha/R3 exit on Vite-only
- **Acceptance criteria**: All journey steps evidenced on Trusted Host path; Fehrest+DeepMed functioning
- **Verification**: Checklist against Spec Alpha tiers
- **Rollback**: Revert slice flags
- **Security/privacy**: No PHI in evidence pack
- **Independent review**: Tier A — R3 journey gate
- **Founder acceptance**: Yes — R3 integrated-journey gate


#### T056 — Introduce compatibility adapters for lib/services strangler without mass rename
- **Stage**: R4
- **Objective**: Introduce compatibility adapters for lib/services strangler without mass rename
- **Repository**: Fanatir
- **Expected paths**: `lib/**; services/**; apps/desktop adapters`
- **Dependencies**: T055
- **Required ADR/spec gate**: ADR-03; ADR-12 deferred
- **Allowed changes**: Adapters
- **Prohibited changes**: Mass afia rename; deleting _archived
- **Acceptance criteria**: Parity notes for adapted paths
- **Verification**: Regression subset
- **Rollback**: Disable adapters
- **Security/privacy**: No authority bypass
- **Independent review**: Tier B — compatibility adapters
- **Founder acceptance**: No


#### T057 — Harden Supabase optional adapter tests (not content/Artifact/patient/policy SoT)
- **Stage**: R4
- **Objective**: Harden Supabase optional adapter tests (not content/Artifact/patient/policy SoT)
- **Repository**: Fanatir
- **Expected paths**: `tests/**; specs/001-fanatir-repository-and-architecture-reconstitution/contracts/supabase-adapter.md`
- **Dependencies**: T028,T055
- **Required ADR/spec gate**: ADR-07/14 Accepted
- **Allowed changes**: Tests + adapter boundaries
- **Prohibited changes**: Migration edits; real PHI on documents-crypto
- **Acceptance criteria**: Tests prove non-authority of Supabase for content
- **Verification**: Adapter + offline/optional mode tests
- **Rollback**: Revert tests
- **Security/privacy**: PHI-egress
- **Independent review**: Tier A — Supabase non-authority tests
- **Founder acceptance**: No


#### T058 — Auth/session compatibility verification (no behavior change)
- **Stage**: R4
- **Objective**: Auth/session compatibility verification (no behavior change)
- **Repository**: Fanatir
- **Expected paths**: `tests/**; docs/program-memory/baseline/R4-auth-compat.md`
- **Dependencies**: T005,T023,T055
- **Required ADR/spec gate**: ADR-11
- **Allowed changes**: Tests/docs only unless bugfix authorized
- **Prohibited changes**: OTP/session/PrivateRoute changes
- **Acceptance criteria**: Compatibility matrix green
- **Verification**: Auth/session tests
- **Rollback**: Revert accidental changes
- **Security/privacy**: No credential logging
- **Independent review**: Tier A — auth compatibility
- **Founder acceptance**: No


#### T059 — CI honesty pass: document non-gating jobs; add real contract test jobs where ready
- **Stage**: R4
- **Objective**: CI honesty pass: document non-gating jobs; add real contract test jobs where ready
- **Repository**: Fanatir
- **Expected paths**: `.github/workflows/**; tests/**`
- **Dependencies**: T055
- **Required ADR/spec gate**: Plan R4
- **Allowed changes**: CI truthfulness
- **Prohibited changes**: Fake green gates
- **Acceptance criteria**: CI labels accurate
- **Verification**: Workflow review
- **Rollback**: Revert workflow
- **Security/privacy**: No secrets in CI logs
- **Independent review**: Tier C — normal verification
- **Founder acceptance**: No


#### T070 — Offline mode and Supabase-optional mode verification
- **Stage**: R4
- **Objective**: Offline mode and Supabase-optional mode verification
- **Repository**: Fanatir
- **Expected paths**: `tests/**; docs/program-memory/baseline/R4-offline.md`
- **Dependencies**: T057,T037
- **Required ADR/spec gate**: ADR-07; local-first
- **Allowed changes**: Tests/docs
- **Prohibited changes**: Requiring cloud for local Artifact ops
- **Acceptance criteria**: Local features work offline
- **Verification**: Offline test matrix
- **Rollback**: Revert
- **Security/privacy**: No forced PHI cloud
- **Independent review**: Tier B — offline/optional mode suite
- **Founder acceptance**: No


#### T060 — Establish signing custody controls (custodian, HSM/service, rotation/recovery/revocation/emergency)
- **Stage**: R5
- **Objective**: Establish signing custody controls (custodian, HSM/service, rotation/recovery/revocation/emergency)
- **Repository**: Fanatir
- **Expected paths**: `docs/program-memory/release/signing-custody.md`
- **Dependencies**: T055,T025,T026
- **Required ADR/spec gate**: Decision D; before distribution
- **Allowed changes**: Custody documentation + process
- **Prohibited changes**: Private keys in repo/ordinary CI vars; distributing unsigned
- **Acceptance criteria**: Custody checklist complete; Founder is authority owner; named operational custodian recorded
- **Verification**: Document review; no key material present
- **Rollback**: Revoke/replace process
- **Security/privacy**: Key custody
- **Independent review**: Tier A — signing/updater custody
- **Founder acceptance**: Yes — signing custody (Decision D)


#### T061 — Produce signed Windows Alpha installer (distributable only after T060)
- **Stage**: R5
- **Objective**: Produce signed Windows Alpha installer (distributable only after T060)
- **Repository**: Fanatir
- **Expected paths**: `apps/desktop/bundle/**; release artifacts (out of repo as needed)`
- **Dependencies**: T060,T039,T055
- **Required ADR/spec gate**: ADR-13; Decision D; T060 complete
- **Allowed changes**: Signed packaging
- **Prohibited changes**: Unsigned distribution
- **Acceptance criteria**: Signed installer verifies
- **Verification**: Signed release verification commands
- **Rollback**: Withdraw release
- **Security/privacy**: App/updater key separation where applicable
- **Independent review**: Tier A — signed release
- **Founder acceptance**: No — blocked by T060; Tier A security review of signed artifact


#### T062 — Windows clean-install verification
- **Stage**: R5
- **Objective**: Windows clean-install verification
- **Repository**: Fanatir
- **Expected paths**: `docs/program-memory/baseline/R5-clean-install.md`
- **Dependencies**: T061
- **Required ADR/spec gate**: Plan R5
- **Allowed changes**: Evidence docs
- **Prohibited changes**: Skipping clean install
- **Acceptance criteria**: Clean install evidence recorded
- **Verification**: Clean install steps
- **Rollback**: N/A
- **Security/privacy**: No PHI dumps
- **Independent review**: Tier C — normal verification
- **Founder acceptance**: No


#### T063 — Accessibility and performance measurement recording for Alpha surfaces
- **Stage**: R5
- **Objective**: Accessibility and performance measurement recording for Alpha surfaces
- **Repository**: Fanatir
- **Expected paths**: `docs/program-memory/baseline/R5-a11y-perf.md`
- **Dependencies**: T055
- **Required ADR/spec gate**: Plan R5
- **Allowed changes**: Checks/docs
- **Prohibited changes**: Inventing ratified pass/fail thresholds; unearned performance claims
- **Acceptance criteria**: Provisional budgets declared as measurement targets only; measurements recorded
- **Verification**: a11y smoke notes + recorded metrics without false compliance claims
- **Rollback**: N/A
- **Security/privacy**: N/A
- **Independent review**: Tier C — normal verification
- **Founder acceptance**: No


#### T064 — Reproducibility and export/import integrity verification
- **Stage**: R5
- **Objective**: Reproducibility and export/import integrity verification
- **Repository**: Fanatir
- **Expected paths**: `docs/program-memory/baseline/R5-repro-export.md; tests/**`
- **Dependencies**: T054,T046
- **Required ADR/spec gate**: ADR-14; ExportManifest
- **Allowed changes**: Tests/evidence
- **Prohibited changes**: Claiming reproducibility without evidence
- **Acceptance criteria**: Export/import integrity passes
- **Verification**: Repro checklist
- **Rollback**: N/A
- **Security/privacy**: Classification on exports
- **Independent review**: Tier B — repro/export suite
- **Founder acceptance**: No


#### T065 — Rollback exercise for host/slice/sidecar pins
- **Stage**: R5
- **Objective**: Rollback exercise for host/slice/sidecar pins
- **Repository**: Fanatir
- **Expected paths**: `docs/program-memory/baseline/R5-rollback-drill.md`
- **Dependencies**: T061
- **Required ADR/spec gate**: Plan rollback strategy
- **Allowed changes**: Drill documentation
- **Prohibited changes**: Irreversible prod deletes
- **Acceptance criteria**: Rollback drill evidenced
- **Verification**: Drill report
- **Rollback**: N/A
- **Security/privacy**: N/A
- **Independent review**: Tier C — normal verification
- **Founder acceptance**: No


#### T066 — Founder Alpha gate: full Trusted Host golden journey acceptance
- **Stage**: R5
- **Objective**: Founder Alpha gate: full Trusted Host golden journey acceptance
- **Repository**: Fanatir
- **Expected paths**: `docs/program-memory/baseline/R5-founder-alpha-acceptance.md`
- **Dependencies**: T055,T061-T065
- **Required ADR/spec gate**: Plan R5 exit; Vite-only insufficient
- **Allowed changes**: Acceptance evidence pack
- **Prohibited changes**: Accepting Vite-only; expanding excluded Alpha; unearned HIPAA claims
- **Acceptance criteria**: Founder acceptance recorded for integrated journey on signed desktop build
- **Verification**: Journey checklist on signed desktop build
- **Rollback**: Withdraw Alpha tag
- **Security/privacy**: Claim language review
- **Independent review**: Tier A — Founder Alpha evidence
- **Founder acceptance**: Yes — required Founder Alpha acceptance


#### T067 — Program-memory closeout after Alpha acceptance
- **Stage**: R5
- **Objective**: Program-memory closeout after Alpha acceptance
- **Repository**: Fanatir
- **Expected paths**: `docs/program-memory/CURRENT-STATE.md; docs/program-memory/NEXT-ACTION.md; docs/program-memory/decisions/**`
- **Dependencies**: T066
- **Required ADR/spec gate**: Constitution closeout
- **Allowed changes**: Program-memory only
- **Prohibited changes**: Silent scope expansion
- **Acceptance criteria**: CURRENT-STATE/NEXT-ACTION accurate
- **Verification**: `git status`
- **Rollback**: Revert docs
- **Security/privacy**: No secrets
- **Independent review**: Tier C — normal verification
- **Founder acceptance**: No


---

## Dependencies and critical path

```text
T001 → T002..T010 → T011 (R0)
  → T012..T026 ADR drafts (ADR-04 before ADR-05; ADR-02 before ADR-06)
  → T027..T029 freezes/charter
  → T030 R1 accept ADR-15/01/02/06
  → T031 shell → T032 host
  → T033 FS; T034 IPC (ADR-06) → T035 workers
  → T036 audit; T037 Artifact (ADR-05); T038 classification
  → T039/T040/T041/T068 parallelizable where marked
  → T042 R2 gate
  → T043 Artifact/Run
  → T044..T047 Fehrest
  → T048..T049 DeepMed (after Fehrest functional + R2 supervision)
  → T050 persist to Fehrest
  → T051 commandF → T052 Lab
  → T053 CoLab → T054 secure share
  → T069 tests; T055 R3 gate
  → T056..T059 (+T070) R4
  → T060 signing custody → T061 signed install → T062..T065
  → T066 Founder Alpha → T067 closeout
```

## Safe parallel work groups

- R0: T002,T003,T004,T006,T007,T008,T009 after T001
- R1: many ADR drafts after T012 (respect ADR-02→06 and ADR-04→05)
- R2: T039,T040,T068 after their deps
- R3: T069 after T034/T035; Fehrest vs DeepMed prep only after gates
- R5: T063,T064 after their deps

## Future specifications (NOT under 001)

| Spec | Bound |
| --- | --- |
| `002-supabase-local-first-and-migration-canonicalization` | Charter only (**T029**); no migration mutation |
| Separate DeepMed OpenMed **fork/import** spec | Forbidden under 001 |
| Separate Fehrest Graphify **fork/import** spec | Forbidden under 001 |
| Dedicated auth/session migration spec | ADR-11 preserves behavior under 001 |

## Exclusions (no implementation tasks)

Pictorial; Montada; public plugin marketplace; live EHR writes; autonomous diagnosis/treatment; hospital certification; universal healthcare-file conversion; unrestricted cloud-model PHI; OpenMed fork/import; Graphify fork/import; Supabase migration mutation; Go-required Alpha; mass `afia` rename.

## Review-burden normalization (adversarial review)

| Metric | Before | After |
| --- | --- | --- |
| Independent review Yes (untiered) | 48 | 0 |
| Tier A | — | 42 |
| Tier B | — | 9 |
| Tier C | — | 19 |
| Founder acceptance Yes | 14 | 5 |

Founder acceptance retained only for: **T030**, **T042**, **T055**, **T060**, **T066**.

## Counts

R0=11, R1=19, R2=13, R3=14, R4=5, R5=8, **total=70**.

## First executable task

**T001** (R0 baseline). First implementation: **T031** only after **T030**.

## Confirmation

Planning artifact only. **No implementation was performed** by this review/regeneration.
