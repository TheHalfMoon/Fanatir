# R0 Repository Baseline (T001)

**Task**: T001 — Record exact repository baseline  
**Recorded**: 2026-07-24 (local)  
**Authority**: Accepted task program commit `797a35ac78b2599b7ef231dbd0a49b3d586ce245`  
**Nature**: Read-only inspection + documentation evidence only. No production mutation.

Absolute paths below are **local machine facts** for this checkout, not portable constitution requirements.

---

## 1. Fanatir

| Field | Value |
| --- | --- |
| Local root | `C:\Projects\Fanatir-Ecosystem\Fanatir` |
| Remote (fetch/push) | `https://github.com/IamShehri/Fanatir.git` |
| GitHub | `IamShehri/Fanatir` — **PUBLIC** (`gh repo view`); default branch `main` |
| Current branch | `docs/f0-planning-memory-bootstrap` |
| Upstream/tracking | **None** — no upstream configured for this branch |
| Ahead/behind vs upstream | **Unavailable** — no upstream (`fatal: no upstream configured`) |
| Remote branch existence | `git ls-remote --heads origin docs/f0-planning-memory-bootstrap` → **empty** (branch remains local-only) |
| Exact HEAD SHA | `797a35ac78b2599b7ef231dbd0a49b3d586ce245` |
| Exact parent SHA | `ac5c777e91b74fc30903a364f03337a5ac8a63f6` |
| HEAD subject | `docs(tasks): define Fanatir reconstitution execution program` |
| Working-tree state at T001 start (after tasks commit) | Clean (`git status --short` empty) |
| Active Spec Kit feature | `.specify/feature.json` → `specs/001-fanatir-repository-and-architecture-reconstitution` |

### Accepted planning commits (verified on this branch)

| Artifact | Full SHA | Subject (short) |
| --- | --- | --- |
| Bootstrap | `aa5db3f7cb0d8fac414d6ec0a1247c387422b682` | `chore(planning): bootstrap Fanatir spec and memory system` |
| Constitution | `1ee7c42ea0e06f182318522c232f598680268d2a` | `docs(governance): ratify Fanatir constitution` |
| Spec acceptance | `46f55c4e9a6b69aecbd85007e98688141939f869` | `docs(spec): define Fanatir architecture reconstitution` |
| Plan acceptance | `ac5c777e91b74fc30903a364f03337a5ac8a63f6` | `docs(plan): define Fanatir reconstitution program` |
| Tasks acceptance | `797a35ac78b2599b7ef231dbd0a49b3d586ce245` | `docs(tasks): define Fanatir reconstitution execution program` |

`main` at `c19e3e1` tracks `origin/main` (product tip; not the planning branch).

### Root manifests and contradictions

| Manifest | Status | Referenced paths | On-disk status |
| --- | --- | --- | --- |
| `Cargo.toml` (workspace) | Exists | `apps/desktop/src-tauri`, `crates/afia-*` (11 crates) | **All MISSING** at those paths (contradictory vs active tree) |
| `go.work` | Exists | `./services/operations-go` | **MISSING** (`_archived/services-operations-go` exists — archived evidence only) |
| `pnpm-workspace.yaml` | Exists | `apps/desktop/ui` | **MISSING** (active UI is `afia-ui/`) |
| `afia-ui/package.json` | Exists | package `"name": "afia"` | Verified active frontend package |
| `.specify/feature.json` | Exists | feature directory as above | Verified |

**Contradiction classification**: Root `Cargo.toml`, `go.work`, and `pnpm-workspace.yaml` are **contradictory / stale** relative to the verified-active `afia-ui/` path. Archived counterparts under `_archived/` (e.g. `apps-desktop`, `crates`, `services-operations-go`) are **archived**, not active authority.

### CI and tests

| Path | Observed state |
| --- | --- |
| `.github/workflows/ci.yml` | Present; placeholder jobs that `echo TODO...` only (scaffold / non-gating) |
| `tests/` | Directories: `contract/`, `e2e/`, `fixtures/`, `integration/`, `performance/`, `security/` + `README.md` — scaffold layout; not treated as proven green gates |

### Production / runtime paths left untouched by T001

Top-level dirs present and **not modified** by this task: `afia-ui/`, `apps/`, `lib/`, `services/`, `supabase/`, `packaging/`, `scripts/`, `tools/`, `_archived/`, plus planning trees `.specify/`, `docs/`, `specs/`.

T001 writes **only** under `docs/program-memory/baseline/` (this file) and may update program-memory pointers.

---

## 2. Fehrest (read-only)

| Field | Value |
| --- | --- |
| Local path | `C:\Projects\Fanatir-Ecosystem\Fehrest` |
| Remote | `https://github.com/IamShehri/Fehrest.git` |
| Branch | `main` |
| Commits | **None** (`rev-list --all --count` = 0) |
| Status | `## No commits yet on main...origin/main [gone]` |
| Working tree | Clean (no short-status lines) |
| Initialization / implementation during T001 | **None** — read-only inspection only |

---

## 3. DeepMed-AI (read-only)

| Field | Value |
| --- | --- |
| Local path | `C:\Projects\Fanatir-Ecosystem\DeepMed-AI` |
| Remote | `https://github.com/IamShehri/DeepMed-AI.git` |
| Branch | `main` |
| Exact HEAD | `796168f821ff468e533234c5f05b74a1b8cc407f` |
| Subject | `Update project description in README.md` |
| Tracked content at HEAD | `README.md` only |
| Working tree | Clean |
| Implementation during T001 | **None** — read-only inspection only |

---

## 4. Environment / tool versions

| Tool | Version / result |
| --- | --- |
| git | `git version 2.55.0.windows.3` |
| gh | `gh version 2.96.0 (2026-07-02)` |
| uv | `uv 0.11.24 (5e04460c0 2026-06-23 x86_64-pc-windows-msvc)` |
| specify | `specify 0.14.0` |
| Node.js | `v22.23.1` |
| pnpm | `9.0.0` |
| Python | `Python 3.11.15` |
| Graphify | `graphify 0.9.25` |
| Rust `rustc` | **Failed** — `rustc --version` triggered rustup sync toward `stable` **1.97.1 (8bab26f4f 2026-07-14)**, then failed with download/rename errors (e.g. os error 2 / os error 145 under `%USERPROFILE%\.rustup`). No clean version string. **Do not** infer a healthy Rust toolchain. Rust implementation remains blocked until a later authorized environment/toolchain task proves `rustc` operational. No Fanatir files changed; toolchain was not repaired in T001. |
| Cargo | Eventually reported **`cargo 1.97.1 (c980f4866 2026-06-30)`** after rustup recovery attempts. Cargo printing a version does **not** mean the Rust toolchain is healthy while `rustc` fails. |

**Python**: `Python 3.11.15` verified.

---

## 5. Contradictions and unavailable facts (honest)

1. Stale root workspace manifests point at missing paths (Cargo/Go/pnpm) while `_archived/` holds related historical trees.
2. Planning branch has **no upstream** and is **not** on `origin`.
3. Fehrest has **zero commits**; `origin/main` reported as `[gone]` from this clone’s perspective.
4. Ahead/behind vs upstream for Fanatir planning branch: **N/A**.
5. Rust toolchain limitation (accepted T001 note): `cargo 1.97.1` eventually succeeded after rustup recovery; `rustc` still failed. Not a healthy toolchain; do not repair rustup in T001; Rust implementation blocked until a later authorized task proves `rustc` operational.
6. Absolute ecosystem path `C:\Projects\Fanatir-Ecosystem` is a local fact only.

---

## 6. Acceptance criteria check (T001)

| Criterion | Result |
| --- | --- |
| Baseline markdown lists SHAs/branches/status | **Met** (this document) |
| Verification commands runnable conceptually (`git rev-parse`, status, sibling status) | **Met** — values recorded above |
| Documentation only; no production / sibling mutation | **Met** |
| No secrets/PHI in evidence | **Met** |

## 7. Rollback

Delete this file (`docs/program-memory/baseline/R0-repo-baseline.md`) and revert any related program-memory pointer edits.

## 8. Review classification

Tier C — normal verification (per tasks.md). Implementing agent may self-verify; not a Tier A sole-review prohibition case.
