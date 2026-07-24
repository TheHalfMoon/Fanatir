# R0 CI and Tests Honesty (T009)

**Task**: T009 — Inspect current CI and tests honesty  
**Recorded**: 2026-07-24 (local)  
**Baseline / start HEAD**: `bd82b6830164e92455e738127f46923bfba66136` (T008 completion)  
**Branch**: `docs/f0-planning-memory-bootstrap`  
**Nature**: Read-only source inspection. **No** builds, tests, installs, workflow triggers, or `gh` hosted mutation.  
**Review**: **Tier C** — normal verification; independent Tier A **not** required.  
**T009 decision**: **PASS WITH NOTES**

Absolute paths are local facts. A workflow file existing is **not** proof it passes or protects branches.

---

## 1. CI / workflow inventory

| Path | Name | Triggers | Runner | Classification |
| --- | --- | --- | --- | --- |
| `.github/workflows/ci.yml` | `ci` | `push`→`main`; `pull_request` (all) | `macos-14` × 5 jobs | **PLACEHOLDER / ECHO-ONLY** |

**Only** workflow file under `.github/workflows/`.  
No Dependabot, CodeQL, release, signing, or packaging workflows observed in-tree.

### `.github/README.md` claim

States `ci.yml` provides “baseline format, lint, typecheck, build, and unit-test gates” — **stale / unsupported** relative to echo-only jobs (see §9).

---

## 2. Current `ci.yml` behavior (exact)

| Job | Command | Real gate? |
| --- | --- | --- |
| `rust` | `echo "TODO(S03-T06): cargo fmt…"` | **No** — always succeeds |
| `go` | `echo "TODO… gofmt…"` | **No** |
| `python` | `echo "TODO… ruff…"` | **No** |
| `typescript` | `echo "TODO… pnpm lint…"` | **No** |
| `contracts` | `echo "TODO(S04-T02)…"` | **No** |

| Question | Answer |
| --- | --- |
| Checkout / setup-node / cargo install? | **Absent** |
| Dependencies installed? | **No** |
| Paths existence checked? | **No** (would fail for missing Cargo/Go members if real) |
| Failures detect regressions? | **No** — echo cannot fail on code quality |
| Aligns with Windows-first? | **No** — `macos-14` only |
| Permissions block? | Default (none declared) |
| Secrets / artifacts? | **None** |

Header comments claim future deny-warnings, licenses, SBOM, secret scan — **aspirational**, not implemented.

Classification: **PLACEHOLDER / ECHO-ONLY**. Do **not** describe as a passing quality gate.

---

## 3. Executable-test inventory

| Family | Paths | Framework | Wired? | Class |
| --- | --- | --- | --- | --- |
| `*.test.ts(x)` / `*.spec.ts(x)` | **0 files** | — | — | **ABSENT** |
| Vitest config | **ABSENT** | vitest listed in `afia-ui` devDeps | **no `test` script** | scaffold dependency only |
| Playwright/Cypress config | **ABSENT** | — | — | **ABSENT** |
| `tests/**` | README-only dirs: contract, e2e, fixtures, integration, performance, security | docs | not executable | **scaffold only** |
| Rust `#[test]` | **NOT OBSERVED** in live tree (crates missing) | — | — | **ABSENT** |
| Python `test_*.py` / pytest | **ABSENT** under `services/`; archived AI has README-only tests | — | — | **ABSENT** / archived scaffold |
| Go `_test.go` | **ABSENT** | — | — | **ABSENT** |
| Ad-hoc `scripts/*-test.ts` | `patient-journey-test.ts`, `encounter-timeline-test.ts`, `temporal-consistency-test.ts`, fault-injection/* | manual `tsx`/node style | **not** in CI; **not** in package scripts | **ad-hoc** / **REQUIRES HOSTED-RUN / local VERIFICATION** (not executed in T009) |
| `scripts/agent-boundary-check.ts` | Root `package.json` `check:boundaries` | tsx via `npx --yes` | referenced; paths assume old Next-style layout + `lib/` | **broken by missing/stale paths** risk; **not executed** |

`tests/README.md` describes Playwright against packaged Tauri — **future-plan**; no tests present.

---

## 4. Script inventory

| Script / command | Class |
| --- | --- |
| Root `check:boundaries` / `typecheck` / `validate` | **placeholder / risky** — uses `npx --yes` (would download); typecheck has no project tsconfig scope proven; boundary script refs missing Next paths |
| `afia-ui` `dev`/`build`/`check`/`format`/`preview`/`start` | **active and referenced** in manifest; **runtime-unverified** without `node_modules` (T002) |
| `scripts/*-test.ts` + fault-injection | **ad-hoc**; cover TS `lib/` clinical-timeline/patient helpers; **manually executable but unverified** |
| `.specify/scripts/powershell/*` | Spec Kit planning tooling — **active for planning**, not product CI |
| Archived AI test READMEs | **archived** |

**Destructive / credentials:** Not observed in script headers reviewed; still **do not execute** under T009 policy.

---

## 5. Subsystem verification matrix

| Area | Status |
| --- | --- |
| **Frontend** install/typecheck/lint/unit/component/route/auth/build/a11y/e2e | install/build scripts exist; **check**=tsc; **no** unit/e2e wired; auth tests **absent** → mostly **absent** / **scaffold** |
| **Rust/Tauri** fmt/clippy/test/IPC/packaging | **absent** (no live crates; CI echo only) |
| **Python/OpenMed/FHIR** lint/type/unit/contract/model/license | **absent** |
| **Supabase** SQL/RLS/Edge/crypto tests | **absent** |
| **Go** fmt/vet/test/build | **absent** (path missing; CI echo) |
| **Cross-system** TS↔Python contracts, Rust IPC, PHI-egress, Artifact/Run, offline, crash, Windows install, signing, golden journey | **absent** / **future-plan requirement** (R0–R5 tasks) |

---

## 6. Hosted GitHub state

**Not inspected.** T009 task definition in `tasks.md` authorizes workflow **content** inspection; hosted `gh` run history / branch protection was **not** explicitly authorized here and was **not** queried.

Unknowns: whether placeholder `ci` runs on `main` PRs; whether any status checks are required.

Repository visibility was previously PUBLIC (T001) — separate from gate honesty.

---

## 7. Unsupported / stale claim ledger

| Source | Claim | Evidence | Status |
| --- | --- | --- | --- |
| `.github/README.md` | CI provides format/lint/typecheck/build/unit-test **gates** | Jobs only `echo` TODOs | **stale / unsupported** |
| `ci.yml` header | Workspace denies warnings, license/advisory/secret scan, SBOM, contract drift | Not implemented in steps | **aspirational** |
| `tests/README.md` | e2e Playwright against packaged Tauri | No Playwright config/tests | **future / unsupported as present** |
| `tests/*/README.md` TODOs | Contract/integration/security suites | Empty of executables | **scaffold** |
| Spec/plan/constitution | Prohibit unearned HIPAA / hospital-ready claims | Policy language (correct) — not a CI claim | **OUT OF SCOPE** as false CI claim |
| Planning docs describing future secure IPC / gates | Target architecture | Not present CI | **future-plan** — do not treat as implemented |

No rewrite of legacy docs in T009.

---

## 8. Security and release-gate inventory

| Check | In-repo CI? |
| --- | --- |
| Secrets scanning workflow | **ABSENT** (platform features unknown) |
| Dependency vulnerability (npm/cargo/pip) | **ABSENT** |
| Rust advisories / cargo-deny | Echo TODO only |
| License compliance / model licenses | **ABSENT** |
| PHI fixture policy | Documented in `tests/README` (“No live PHI”) — **not enforced by CI** |
| Signing / updater / SBOM / checksums | **ABSENT** |
| Windows signing | **ABSENT** |

Do not claim GitHub platform defaults absent solely because workflow files omit them.

---

## 9. Present quality-gate conclusion

**There is no substantive automated quality gate in source today.**

- CI always “passes” if echo succeeds.
- No executable unit/integration/e2e suite wired to CI.
- Ad-hoc `scripts/*-test.ts` are local kernel smoke helpers, not product CI.
- Windows-first Alpha packaging/signing gates do not exist yet (R5).

---

## 10. Current-versus-target boundary

| Current | Target (plan R0–R5) |
| --- | --- |
| macOS echo CI | Real gates: Rust Trusted Host, IPC, PHI, Artifact/Run, auth compat, sidecars, packaging, signing, golden journey |
| README test folders | Executable suites with fixtures provenance |
| Vite scripts only | Desktop packaging verification |
| Ad-hoc lib scripts | Contract tests across UI/Python/Rust |

Future tests must not be presented as implemented.

---

## 11. R0 / R1 implications

1. Do not claim “CI green” as product quality.
2. R1 ADRs / reconstitution should plan Windows runners and real path-aware jobs.
3. T059 (CI honesty pass) is the later task to document non-gating jobs and add real contracts when ready.
4. Fixing `.github/README.md` wording is a later docs task — not T009 mutation beyond evidence.

---

## 12. Runtime-verification gaps

- Hosted Actions history unknown.
- Ad-hoc scripts not executed.
- `afia-ui` `pnpm run check` not executed (deps absent).

---

## 13. Acceptance

| Criterion | Result |
| --- | --- |
| CI TODOs/scaffolds identified | **Met** |
| No false green-gate claim in evidence | **Met** |
| Documentation only | **Met** |

## 14. Rollback

Delete this file; revert program-memory pointer edits.

## 15. Notes (PASS WITH NOTES)

- Placeholder CI is honest about TODOs in job bodies; README overclaims “gates”.
- Platform skew (macOS) vs Windows-first Alpha.
- Vitest dependency without tests/scripts.
