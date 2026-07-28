# R0 Dependency Managers & Toolchain (T003)

**Task**: T003 — Verify dependency managers (pnpm/node/rust/python/uv) and versions  
**Recorded**: 2026-07-24 (local)  
**Baseline / start HEAD**: `5f968c164b14c4afb0e14836b8f1b42ebb7675f2` (T002 completion)  
**Branch**: `docs/f0-planning-memory-bootstrap`  
**Nature**: Read-only environment verification. **No** install, update, enable, repair, or PATH mutation.  
**Review**: Tier C  
**T003 decision**: **PASS WITH NOTES**

Absolute paths and host resolution paths below are **local machine facts**, not portable architecture authority.

---

## 1. Starting state

| Check | Result |
| --- | --- |
| HEAD | `5f968c164b14c4afb0e14836b8f1b42ebb7675f2` |
| Branch | `docs/f0-planning-memory-bootstrap` |
| Worktree | Clean before evidence write |

---

## 2. JavaScript toolchain matrix

| Tool | Command | Exit | Exact output / path | Declared | Classification |
| --- | --- | --- | --- | --- | --- |
| Node.js | `node --version` | 0 | `v22.23.1` | Root `engines.node`: `>=20` | **available and healthy** (satisfies `>=20`) |
| Node path | `where.exe node` | 0 | `C:\Users\Shehr\AppData\Local\hermes\node\node.exe` | — | local resolution fact |
| npm | `npm --version` | 0 | `10.9.8` | Not primary for afia-ui | **available**; not package-manager authority for afia-ui |
| npm path | `where.exe npm` | 0 | `...\hermes\node\npm` / `npm.cmd` | — | local fact |
| pnpm (PATH) | `pnpm --version` | 0 | `9.0.0` | Root `packageManager`: `pnpm@9.0.0`; afia-ui: `pnpm@10.4.1` | **available but version-mismatched** vs afia-ui |
| pnpm path | `where.exe pnpm` | 0 | `...\hermes\node\pnpm` / `pnpm.CMD` | — | local fact |
| Corepack | `corepack --version` | 0 | `0.34.6` | — | **available**; **not enabled/prepared** in T003 |
| Corepack path | `where.exe corepack` | 0 | `...\hermes\node\corepack` / `.cmd` | — | local fact |
| Vite (global) | `vite --version` | 1 | not recognized | Declared as afia-ui **devDependency** | **declared but unavailable** globally; needs `afia-ui/node_modules` |
| Tauri CLI | `tauri --version` / `cargo tauri --version` | 1 / 101 | not found / `no such command: tauri` | Declared historically via archived `apps/desktop/src-tauri` | **declared (archived/stale workspace) but unavailable** |

### Package-manager declarations vs installability

| Location | Declaration | Notes |
| --- | --- | --- |
| Root `package.json` | `packageManager: pnpm@9.0.0`, `engines.node: >=20`, workspaces `apps/desktop/ui` | PATH pnpm **matches root** 9.0.0; workspace path **missing** |
| `afia-ui/package.json` | `packageManager: pnpm@10.4.1+sha512…` | PATH pnpm **9.0.0 ≠ 10.4.1** |
| `afia-ui/pnpm-lock.yaml` | Present (`lockfileVersion: '9.0'`) | Reproducible **after** authorized install with matching pnpm |
| Root `pnpm-lock.yaml` / `pnpm-workspace.yaml` | Workspace `apps/desktop/ui` | Stale vs active `afia-ui/` |

**Can declared afia-ui package manager be reproduced now?** Corepack is **available** but was **not** run (`corepack enable` / `prepare` prohibited). PATH pnpm cannot currently satisfy afia-ui’s pinned 10.4.1 without an authorized Corepack/pnpm action later. Root and afia-ui **conflict** on pnpm major.

**Affects**: R0 evidence + blocks trustworthy afia-ui install/build until package-manager reconciliation (later authorized task). Does not by itself block documentation R0 tasks.

---

## 3. Rust toolchain matrix

| Tool | Command | Exit | Exact output / path | Classification |
| --- | --- | --- | --- | --- |
| rustup | `rustup --version` | 0 | `rustup 1.29.0 (28d1352db 2026-03-05)`; also reported `(timeout reading rustc version)` during this probe | Manager **available**; transient rustc-read timeout observed once |
| rustup path | `where.exe rustup` | 0 | `C:\Users\Shehr\.cargo\bin\rustup.exe` | local fact |
| Default | `rustup default` | 0 | `stable-x86_64-pc-windows-msvc (default)` | recorded |
| Toolchains | `rustup toolchain list` | 0 | `stable-… (active, default)`; `1.85.0-…-gnu`; `1.85.0-…-msvc` | multiple installed |
| rustup show | `rustup show` | 0 | active `stable-x86_64-pc-windows-msvc`; **overridden by** repo `rust-toolchain.toml`; targets include `x86_64-pc-windows-msvc` and `aarch64-apple-darwin` | recorded |
| cargo (default) | `cargo --version` | 0 | `cargo 1.97.1 (c980f4866 2026-06-30)` | **available** (via rustup proxy at `.cargo\bin`) |
| cargo path | `where.exe cargo` | 0 | `C:\Users\Shehr\.cargo\bin\cargo.exe` | rustup proxy location |
| rustc (default) | `rustc --version` | 0 | `rustc 1.97.1 (8bab26f4f 2026-07-14)` | **available and healthy** *at T003 recording time* |
| rustc path | `where.exe rustc` | 0 | `C:\Users\Shehr\.cargo\bin\rustc.exe` | rustup proxy location |
| rustc 1.85.0 pin test | `rustc +1.85.0-x86_64-pc-windows-msvc --version` | 0 | `rustc 1.85.0 (4d91de4e4 2025-02-17)` | older toolchain also healthy |
| cargo 1.85.0 | `cargo +1.85.0-x86_64-pc-windows-msvc --version` | 0 | `cargo 1.85.0 (d73d2caf9 2024-12-31)` | healthy |

### Repo Rust declarations

| Artifact | Status |
| --- | --- |
| `rust-toolchain.toml` | Present; **channel not pinned** (TODO); requests `rustfmt`, `clippy`, target `aarch64-apple-darwin` |
| Root `Cargo.toml` | Workspace members point at **missing** `apps/desktop/src-tauri` and `crates/afia-*` (archived counterparts exist) |

### Health interpretation (critical)

- **Do not** treat Cargo alone as sufficient: at T003, **both** `cargo` and `rustc` returned 1.97.1 successfully → classify default toolchain as **available and healthy now**.
- **T001 note remains historically true**: earlier probes saw `rustc` fail during rustup recovery; T003 re-verification succeeded without repair commands in this task.
- One `rustup --version` invocation still printed `(timeout reading rustc version)` before later `rustc --version` succeeded — residual **instability signal**, not treated as current hard failure.
- **Rust product implementation** remains blocked by **missing workspace members / stale manifests**, not by absence of `rustc` at T003 time. Later R1/R2 work still needs authorized reconstitution; toolchain is no longer the primary blocker recorded here.

**Affects**: Clears the T001 “rustc unavailable” environment block for *host capability*; R1+ still blocked by repository structure until reconstitution tasks.

---

## 4. Python / uv matrix

| Tool | Command | Exit | Exact output / path | Classification |
| --- | --- | --- | --- | --- |
| python | `python --version` | 0 | `Python 3.11.15` | **available** globally on PATH |
| python path | `where.exe python` | 0 | First: `...\hermes\hermes-agent\venv\Scripts\python.exe`; also WindowsApps shim | **local fact** — first hit is a Hermes agent venv, not a Fanatir project venv |
| `py` launcher | `py --version` / `py -0p` | 1 | not recognized | **unavailable** |
| uv | `uv --version` | 0 | `uv 0.11.24 (5e04460c0 2026-06-23 x86_64-pc-windows-msvc)` | **available and healthy** |
| uv path | `where.exe uv` | 0 | `C:\Users\Shehr\AppData\Local\hermes\bin\uv.exe` | local fact |
| pip | `pip --version` | 0 | `pip 26.1.2` from hermes-agent venv (python 3.11) | **available** via same PATH python; not a Fanatir project env |

### Project Python declarations

| Artifact | Observation |
| --- | --- |
| Active `pyproject.toml` / `uv.lock` at Fanatir root | **None** |
| Project `.venv` | **None** found (root / shallow search) |
| `services/requirements-bridge.txt` | Present; pins some bridge packages; comments assume separate OpenMed/fastapi install |
| `_archived/services-ai-python/pyproject.toml` | `requires-python = ">=3.12"` — **mismatched** vs PATH Python 3.11.15 |

**Reproducible active Fanatir Python environment?** **No** — no project venv/lock at active root; global/Hermes interpreter is a host convenience only.

**Affects**: R0 documentation OK; later Python worker / Lab / bridge stages need authorized env definition (and likely ≥3.12 if archived constraint is revived).

---

## 5. Other declared vs available tooling

| Tool | Declared in tree? | Available on PATH? | Classification |
| --- | --- | --- | --- |
| Go | `go.work` → `./services/operations-go` (**missing**; archived) | No | **declared but unavailable**; path stale |
| R | Plan/spec Lab mentions (not active root pin) | No | **not available**; Lab later |
| DuckDB / SQL CLI | Spec/plan Lab mentions | `duckdb` not on PATH | **declared in program, unavailable as CLI** |
| Specify CLI | Spec Kit in use | `specify 0.14.0` at `~\.local\bin\specify.exe` | **available and healthy** (planning) |
| Graphify | Ecosystem index (non-authority) | `graphify 0.9.25` | **available**; not product authority |
| Vite | afia-ui dependency | Not global; no `node_modules` | **declared but unavailable** until install |
| Tauri CLI | Archived desktop app | Not installed | **declared but unavailable** |

---

## 6. Declared-versus-installed mismatches (summary)

1. **afia-ui pnpm 10.4.1** vs **PATH pnpm 9.0.0** (root declares 9.0.0 — matches PATH, conflicts with afia-ui).
2. **Root workspace / Cargo / Go paths** declare missing members; active UI is `afia-ui/`.
3. **Archived AI `requires-python >=3.12`** vs **PATH Python 3.11.15**.
4. **`rust-toolchain.toml` channel unpinned** while host stable resolves to 1.97.1.
5. **T001 rustc failure** vs **T003 rustc success** — environment changed/recovered; both recorded honestly.

---

## 7. Toolchain blockers by future stage

| Stage / concern | Blocker? | Classification |
| --- | --- | --- |
| Remaining R0 docs (T004+) | No hard toolchain block | proceed with observe-only tasks |
| afia-ui install/build (when authorized) | Yes until pnpm 10.4.1 reconciled + `node_modules` | package-manager mismatch + missing deps |
| R1+ Rust reconstitution | Host `rustc`/`cargo` OK now; **repo members missing** | structural, not toolchain-absent |
| R2 Tauri desktop | Tauri CLI absent; members missing | declared but unavailable + structural |
| R3 Python workers / Lab | No project venv; Go/R/DuckDB CLI absent; Python may need ≥3.12 | environment definition required |
| Spec Kit planning | Specify available | OK |

---

## 8. Acceptance

| Criterion | Result |
| --- | --- |
| Tooling matrix recorded | **Met** |
| No install/repair/config mutation | **Met** |
| Documentation only | **Met** |

## 9. Rollback

Delete this file; revert related program-memory pointer edits.

## 10. Notes (PASS WITH NOTES)

- Corepack available but unused (prohibited to enable).
- Rust currently healthy; do not erase T001 failure history.
- PATH Python is Hermes-venv-first — record as host fact, not Fanatir SoT.
- No packages installed; no rustup mutation; no Corepack mutation.
