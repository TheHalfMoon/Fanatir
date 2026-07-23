# R0 Build / Dev Commands (T002)

**Task**: T002 — Verify actual build/dev commands for afia-ui and record evidence  
**Recorded**: 2026-07-24 (local)  
**Baseline commit (T001 completion / T002 start HEAD)**: `ea2c3f66c62758e058fbe89a2781473054e2fe8e`  
**Branch**: `docs/f0-planning-memory-bootstrap`  
**Nature**: Read-only inspection + bounded command probes. **No** dependency install, manifest repair, or production mutation.  
**Review**: Tier C  
**T002 decision**: **PASS WITH NOTES**

Absolute paths below are **local machine facts**, not architecture authority.

---

## 1. Starting state

| Check | Result |
| --- | --- |
| `git rev-parse HEAD` | `ea2c3f66c62758e058fbe89a2781473054e2fe8e` |
| Branch | `docs/f0-planning-memory-bootstrap` |
| `git status --short` before T002 | empty (clean) |

---

## 2. Actual application root

| Question | Answer |
| --- | --- |
| Runnable application root | **`afia-ui/`** |
| Package name | `"afia"` (`afia-ui/package.json`) |
| Active UI path vs root workspace | Root `pnpm-workspace.yaml` / root `package.json` workspaces point at **`apps/desktop/ui`**, which is **MISSING**. Related history under `_archived/apps-desktop/ui`. Active app is **`afia-ui/`**, not in the root pnpm workspace. |
| Client entry | `afia-ui/client/index.html` → `client/src/main.tsx` → `App.tsx` |
| Server entry (production start path) | `afia-ui/server/index.ts` (Express static server for `dist/public`) |
| Vite root | `afia-ui/vite.config.ts` sets `root` to `client/`; build `outDir` = `dist/public` |
| Dev README claiming scripts | **None** found under `afia-ui/` (only `ideas.md`). No `.env*` files present in the repo tree at inspection time. |

---

## 3. Package-manager authority

| Source | Declared manager |
| --- | --- |
| `afia-ui/package.json` `packageManager` | **`pnpm@10.4.1`** (+ sha512 pin) |
| `afia-ui/pnpm-lock.yaml` | Present; `lockfileVersion: '9.0'` |
| Root `package.json` `packageManager` | `pnpm@9.0.0` |
| Root `pnpm-lock.yaml` | Present; references `apps/desktop/ui: {}` (stale workspace) |
| Root `pnpm-workspace.yaml` | packages: `apps/desktop/ui` only (**missing on disk**) |
| npm / yarn lockfiles under `afia-ui/` | **None** |
| Patches | `afia-ui/patches/wouter@3.7.1.patch` referenced by `pnpm.patchedDependencies` |

**Authority for afia-ui**: **pnpm**, driven by **`afia-ui/package.json` + `afia-ui/pnpm-lock.yaml`**, not the root workspace (which does not include `afia-ui`).

**Host tool versions (actual output)**:

| Tool | Output |
| --- | --- |
| Node.js | `v22.23.1` |
| pnpm (PATH) | `9.0.0` |
| npm | `10.9.8` |
| corepack | `0.34.6` |
| vite / esbuild on PATH | **Not found** (no global; no local `node_modules`) |

**Note**: Declared `packageManager` for afia-ui is **pnpm 10.4.1**; PATH pnpm is **9.0.0**. Not repaired in T002. Future install (out of scope) must reconcile Corepack / pnpm major.

---

## 4. Detected scripts (`afia-ui/package.json`)

| Script | Exact command |
| --- | --- |
| `dev` | `vite --host` |
| `build` | `vite build && esbuild server/index.ts --platform=node --packages=external --bundle --format=esm --outdir=dist` |
| `start` | `NODE_ENV=production node dist/index.js` |
| `preview` | `vite preview --host` |
| `check` | `tsc --noEmit` |
| `format` | `prettier --write .` |

Vite config also hard-codes `server.host: true`, `server.port: 3000`, `strictPort: false`.

---

## 5. Dependency state

| Fact | Result |
| --- | --- |
| `afia-ui/node_modules` | **Absent** |
| Repo-root `node_modules` | **Absent** |
| `afia-ui/dist` | **Absent** |
| Lockfile present | Yes (`afia-ui/pnpm-lock.yaml`) |

**Blocker**: Required dependencies are **not** installed. Per T002 policy, **no install** was performed. Build and dev are **not currently executable**.

---

## 6. Build verification

| Field | Value |
| --- | --- |
| Working directory | `C:\Projects\Fanatir-Ecosystem\Fanatir\afia-ui` |
| Exact command | `pnpm run build` |
| Started | Yes |
| Exit code | **1** |
| Outcome | **Failed** — missing local toolchain binaries |
| Relevant output (summary) | `'vite' is not recognized as an internal or external command` … `ELIFECYCLE Command failed with exit code 1` … `WARN Local package.json exists, but node_modules missing, did you mean to install?` |
| Generated output directory | None created |
| Tracked files changed by build | **None** |
| Proves | Compilation/build **not** proven; only proves missing deps |

**First actionable root cause**: `afia-ui/node_modules` missing → `vite` / `esbuild` unavailable to the script.

---

## 7. Development-server verification (bounded)

| Field | Value |
| --- | --- |
| Working directory | `C:\Projects\Fanatir-Ecosystem\Fanatir\afia-ui` |
| Exact command tested | `pnpm run dev` (manifest: `vite --host`) |
| Exit code | **1** |
| Outcome | **Failed immediately** — same missing-`vite` / missing-`node_modules` blocker |
| Host / port reached | **N/A** — process did not start a server |
| Frontend / backend processes | **None** started |
| Multi-process spawn | **No** |
| Termination | N/A (never started); post-check found **no** leftover `node`/`vite` processes from this probe |
| Safe localhost-only smoke | **Not possible** without unauthorized install |

**Claimed vs safe binding**: Manifest `dev` / `preview` use `--host`, and `vite.config.ts` sets `host: true` (all interfaces). If deps were present, a localhost-only smoke would need an **authorized** non-mutating override later; T002 does **not** invent alternate scripts. Recorded as a **future safety note**, not executed.

---

## 8. Root-level command validity

| Root script | Command | Validity for afia-ui |
| --- | --- | --- |
| `check:boundaries` | `npx --yes tsx scripts/agent-boundary-check.ts` | Skeleton monorepo utility; **not** an afia-ui build/dev command. Uses `npx --yes` (would download). **Not executed** (would install tooling). |
| `typecheck` | `npx --yes -p typescript tsc --noEmit` | Root typecheck; does not target afia-ui app scripts. **Not executed**. |
| `validate` | `npm run check:boundaries && npm run typecheck` | Same. **Not executed**. |
| Workspace path `apps/desktop/ui` | Referenced by root `package.json` workspaces + `pnpm-workspace.yaml` | **Stale / contradictory** — path missing; archived under `_archived/apps-desktop/ui`. |

Root has **no** `dev` / `build` scripts for the live UI. Root-level commands are **not** the authority for running `afia-ui`.

---

## 9. Application-level command validity

| Command | Claimed | Proven operational? | Notes |
| --- | --- | --- | --- |
| `pnpm run build` (in `afia-ui`) | Yes | **No** — exit 1, missing `node_modules` | Exact failure recorded |
| `pnpm run dev` | Yes | **No** — exit 1, missing `node_modules` | Exact failure recorded |
| `pnpm run preview` | Yes | **Not tested** | Would need vite + prior build; blocked by deps |
| `pnpm run check` | Yes | **Not tested** | Needs local `tsc` |
| `pnpm run format` | Yes | **Not tested** | Would mutate files; out of T002 scope |
| `pnpm run start` | Yes | **Not tested** | Needs `dist/`; Unix-style `NODE_ENV=production` prefix may be fragile on Windows CMD without cross-env |

**Classification of current application**: **Blocked by missing dependencies**. Also subject to **repository contradictions** (root workspace ≠ `afia-ui`). Not proven directly runnable or buildable in this checkout without install (install prohibited in T002).

---

## 10. Environment assumptions (observed, not modified)

- No `.env` / `.env.example` files present in the Fanatir tree at T002 inspection.
- Vite `envDir` is `afia-ui/` (directory of `vite.config.ts`).
- Production `start` serves static files from `dist/public` via Express on `PORT` or **3000**.
- Supabase client dependency (`@supabase/supabase-js`) is declared; **no** env files or live calls exercised in T002.
- Vite plugins of note (present in config; **not changed**):
  - `vite-plugin-manus-runtime`
  - Manus **debug collector** (writes `.manus-logs/`, injects `/__manus__/debug-collector.js` in non-production)
  - Manus **storage proxy** (`/manus-storage`) requiring `BUILT_IN_FORGE_API_URL` + `BUILT_IN_FORGE_API_KEY` if used
- `allowedHosts` includes various `*.manus*.computer` hosts plus localhost.

---

## 11. Generated artifacts

| Artifact | Status |
| --- | --- |
| `afia-ui/dist/` | Not created; would be gitignored (`dist/` in `afia-ui/.gitignore` and root `.gitignore`) |
| `.manus-logs/` | Not created (dev server never ran) |
| Lockfile / `package.json` | **Unchanged** |
| Tracked working tree from probes | **Clean** before evidence write |

---

## 12. Security / privacy observations (T002)

Verification did **not**:

- load PHI
- use production credentials
- create or modify `.env` files
- call patient-data or external clinical services
- send data to external model providers
- modify Supabase
- modify auth/session behavior

**Present but unchanged** (future risk awareness):

- Manus debug collector / runtime plugins in Vite config
- Storage proxy that can call an external forge API when env vars are set
- `@supabase/supabase-js` dependency (unused in this task)
- `dev`/`preview` `--host` / `host: true` would bind beyond localhost if started with deps

---

## 13. Commands claimed but invalid / contradictory

- Root workspace commands targeting `apps/desktop/ui` — **path missing**
- Treating root `pnpm` workspace as including `afia-ui` — **false**
- Treating script **names** as proof of operability — **disproven** for `dev`/`build` without `node_modules`

---

## 14. Acceptance criteria

| Criterion | Result |
| --- | --- |
| Documented scripts with run/fail evidence | **Met** |
| No package script / production mutation | **Met** |
| No secrets in logs | **Met** |

## 15. Rollback

Delete this file; revert related program-memory pointer edits if any.

## 16. Notes (accepted with PASS WITH NOTES)

1. Build/dev blocked solely by missing `node_modules` (install intentionally withheld).
2. afia-ui declares pnpm **10.4.1**; host PATH pnpm is **9.0.0**.
3. Root manifests remain stale vs active `afia-ui/`.
4. Manifest `dev` uses `--host`; localhost-only smoke deferred until deps exist under an authorized task.
5. Rust/`rustc` status unchanged from T001; irrelevant to this Vite/Node app path except ecosystem context.
