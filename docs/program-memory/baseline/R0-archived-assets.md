# R0 Archived Rust / Tauri / Go Assets (T008)

**Task**: T008 — Inspect archived Rust/Tauri/Go assets without reactivation  
**Recorded**: 2026-07-24 (local)  
**Baseline / start HEAD**: `1115f181b689e136839493c480469fa4258f6768` (T007 completion)  
**Branch**: `docs/f0-planning-memory-bootstrap`  
**Nature**: Read-only inventory. **No** compile, restore, move, or reactivation.  
**Review**: **Tier C** — normal verification; independent Tier A **not** required by tasks.md.  
**T008 decision**: **PASS WITH NOTES**

Absolute paths are local facts. Archive classified **non-authoritative**.

---

## 1. Archive provenance

`_archived/ARCHIVED.md` (2026-07-04): ambitious monorepo skeleton (2026-06-29) never progressed beyond scaffolding; active path was/is `afia-ui/`, `lib/`, `services/openmed_bridge.py`. Archive retained as **future architectural reference**, not build authority.

---

## 2. Complete archived-asset inventory

| Family | Path | Contents summary | Class |
| --- | --- | --- | --- |
| Rust crates | `_archived/crates/afia-*` (11 crates) | `Cargo.toml` + `lib.rs` TODO stubs only | **ARCHIVED SCAFFOLD** |
| Desktop host | `_archived/apps-desktop/src-tauri/` | Tauri v2 config stubs; empty `main`; empty capabilities | **ARCHIVED SCAFFOLD** |
| Archived UI | `_archived/apps-desktop/ui/` | Separate Vite UI skeleton (not current `afia-ui`) | **ARCHIVED SCAFFOLD** / superseded |
| Go ops | `_archived/services-operations-go/` | `go.mod` + empty `main` + package `doc.go` stubs | **ARCHIVED SCAFFOLD** |
| Python AI | `_archived/services-ai-python/` | pyproject scaffold (T007) | **ARCHIVED** (out of Rust/Go scope note) |
| Contracts | `_archived/contracts/` | README/TODO contract trees | **ARCHIVED SCAFFOLD** |

**Counts (files):** ~11 crate `lib.rs`; ~3 desktop `.rs`; ~14 Go files (mostly docs).

---

## 3. Root-manifest contradiction map

| Manifest | Referenced path | On disk | Archive counterpart | Buildable now? | Classification |
| --- | --- | --- | --- | --- | --- |
| `Cargo.toml` members | `apps/desktop/src-tauri` | **MISSING** | `_archived/apps-desktop/src-tauri` | **No** | CONTRADICTORY |
| `Cargo.toml` members | `crates/afia-*` (11) | **MISSING** | `_archived/crates/afia-*` | **No** | CONTRADICTORY |
| `Cargo.lock` | — | **MISSING** | — | N/A | incomplete workspace |
| `rust-toolchain.toml` | unpinned channel + components | EXISTS | — | host OK (T003) | does not fix missing members |
| `go.work` | `./services/operations-go` | **MISSING** | `_archived/services-operations-go` | **No** | CONTRADICTORY |
| `pnpm-workspace.yaml` | `apps/desktop/ui` | **MISSING** | `_archived/apps-desktop/ui` | **No** | CONTRADICTORY |
| Root `package.json` workspaces | `apps/desktop/ui` | **MISSING** | archived UI | **No** | CONTRADICTORY |
| Active UI | `afia-ui/` | EXISTS | not in root workspace | Vite app active | CONTRADICTORY vs workspace |

**CI / scripts:** No evidence CI builds these missing members (CI is TODO echo — T009).  
**Migration implication:** Reconstitute Trusted Host under ADR-15/01/02 **without** treating root manifests as truth; repair manifests only in authorized later tasks — **not T008**.

---

## 4. Rust crate matrix

All crates: `version = "0.0.0"`, `edition = "2021"`, `publish = false`, **empty `[dependencies]`**, `lib.rs` = module doc + “Skeleton only — no implementation.”

| Crate | Declared responsibility | Completeness | Class | Disposition |
| --- | --- | --- | --- | --- |
| `afia-domain` | IDs, entities, invariants | scaffold | ARCHIVED SCAFFOLD | **investigate** (naming/shape for ADR-15 domain) |
| `afia-contracts` | transport DTOs / versions | scaffold | ARCHIVED SCAFFOLD | **investigate** |
| `afia-application` | use cases / ports | scaffold | ARCHIVED SCAFFOLD | **investigate** |
| `afia-security` | keys, encryption, audit chain | scaffold | ARCHIVED SCAFFOLD | **investigate** (ADR-14) |
| `afia-storage` | SQLCipher / repos / objects | scaffold | ARCHIVED SCAFFOLD | **investigate** (ADR-05) — note SQLCipher vs Artifact Store target |
| `afia-workspace` | package lifecycle | scaffold | ARCHIVED SCAFFOLD | **investigate** |
| `afia-documents` | import/parse/content access | scaffold | ARCHIVED SCAFFOLD | **investigate** |
| `afia-search` | indexing/query | scaffold | ARCHIVED SCAFFOLD | **archive** / later P2 |
| `afia-ai` | Python client / provenance | scaffold | ARCHIVED SCAFFOLD | **investigate** (must not restore UI→Python bypass; ADR-09) |
| `afia-plugins` | plugin gate (disabled) | scaffold | ARCHIVED SCAFFOLD | **archive** until plugin ADR |
| `afia-tauri` | commands/events adapter | scaffold | ARCHIVED SCAFFOLD | **investigate** (ADR-01/02/06) |

**Not reusable implementation** merely because Rust — no executable logic.

Compile status: **NOT OBSERVED** (Cargo not run; members missing from live tree).

---

## 5. Trusted-host capability comparison

| Capability (target) | Archived evidence | Status |
| --- | --- | --- |
| Desktop host | `apps-desktop` + `afia-tauri` stubs | **partial/scaffold only** |
| Project/workspace authority | `afia-workspace` stub | scaffold |
| Artifact/Revision/Run | domain/application TODOs | **absent** as concrete types |
| Encrypted storage | `afia-storage` / `afia-security` TODOs | scaffold; SQLCipher-oriented notes may **contradict** Decision C Artifact Store — **REQUIRES ADR INVESTIGATION** |
| Filesystem mediation | intended deny-by-default capabilities | scaffold intent only |
| Secrets | keychain TODO (macOS-oriented notes) | scaffold |
| Policy/capabilities | empty permissions array + TODOs | scaffold |
| Audit | audit MAC chain TODO | scaffold |
| IPC | no typed commands implemented | **absent** |
| Worker supervision | `afia-ai` Python client TODO; Go runtime TODO | scaffold / optional Go |
| Plugin/MCP gateway | plugins disabled-by-default intent | scaffold |
| Export/sharing | not implemented | **absent** |
| Updater/signing | not configured | **absent** |

Final choices belong to ADR-01/02/05/06/14/15 — T008 uses **INVESTIGATE** where insufficient.

---

## 6. Tauri / Desktop findings

| Item | Observation |
| --- | --- |
| Tauri generation | Comments/`Cargo.toml` target **Tauri v2**; deps **not** declared |
| Product | `productName`: AFIA; `identifier`: `com.tenomes.afia` |
| frontendDist | `../ui/dist`; `devUrl` `http://localhost:5173` |
| Bundle targets | `["app","dmg"]` — **macOS-oriented**, not Windows-first Alpha packaging |
| Capabilities | `permissions: []` with deny-shell/fs/network TODO comments |
| Windows array | empty |
| CSP | TODO / empty security block |
| Updater / signing | **NOT OBSERVED** |
| `main.rs` | empty `main` with TODO |
| Buildable from current layout | **No** — path not at `apps/desktop`; no tauri deps; UI is `afia-ui` not archived `ui` |

**Conclusion:** Composition **scaffolding only**, not a Trusted Host implementation. **Do not reactivate** as-is. Candidate for **bounded adaptation** of deny-by-default intent under ADR-01/02 after Windows-first redesign.

---

## 7. IPC / capability findings

| Item | Observation |
| --- | --- |
| Typed/versioned IPC | **Absent** (no commands registered) |
| Bounds/size limits | **NOT OBSERVED** |
| UI-zero-authority | Intended by comments (`afia-ai`: frontend never calls Python) — **not implemented** |
| Actual current product | Vite UI calls Python/Supabase directly (T004–T007) — **contradicts** archived intent |

Classification: archived intent = **VERIFIED BEHAVIOR** (comments); current live product = separate trust seams already inventoried.

---

## 8. Go inventory

| Item | Fact |
| --- | --- |
| Module | `github.com/tenomes/afia/services/operations-go` |
| Go | 1.22 |
| Binary | `cmd/afia-operations` — empty `main` |
| Packages | internal/* mostly `doc.go` responsibility docs |
| Callers today | **None** |
| Root `go.work` | points to missing live path |
| Founder rule | Go permitted not required; **not** Trusted Host; **not** required for Founder Alpha; future Go needs ADR |

**Disposition:** **archive** — do not reactivate for Alpha; optional later ADR if Rust/Python insufficient for ops.

---

## 9. Security findings

| Finding | Class |
| --- | --- |
| No unrestricted FS/shell in archived Rust/Go (no impl) | NOT OBSERVED as defect |
| Empty Tauri permissions (deny-by-default intent) | VERIFIED BEHAVIOR (scaffold) |
| Broad network listeners | NOT OBSERVED in code |
| Hard-coded credentials | NOT OBSERVED |
| Unsafe Rust | NOT OBSERVED (no impl) |
| Production exposure of these assets | **None** while archived and unreferenced by running UI | OUT OF SCOPE as live risk |
| Reactivation without ADR | would reintroduce contradiction with UI-zero-authority / Windows-first | VERIFIED TRUST SEAM if reactivated naively |

**VERIFIED SECURITY DEFECT:** **None** in archived stubs.

---

## 10. Licensing / provenance

| Item | Fact |
| --- | --- |
| Per-crate `license` field | **NOT OBSERVED** in archived `Cargo.toml`s |
| LICENSE files under `_archived/crates` | **NOT OBSERVED** in inventory |
| Ownership | In-repo Fanatir/AFIA scaffolding; third-party reuse obligations **NOT** fully documented here |
| Go module path `github.com/tenomes/afia/...` | naming provenance note; not proof of remote existence |

Reuse requires later license hygiene before shipping derived code.

---

## 11. Activity & disposition matrix (summary)

| Asset family | Activity | Authority | Disposition | ADR/spec |
| --- | --- | --- | --- | --- |
| `_archived/crates/*` | archived scaffold | **non-authoritative** | **investigate** (shape) / do not reactivate wholesale | ADR-15 + domain ADRs |
| `_archived/apps-desktop` | archived scaffold | non-authoritative | **replace** host layout for Windows-first Tauri 2; adapt deny-default intent | ADR-01/02/06 |
| `_archived/services-operations-go` | archived scaffold | non-authoritative | **archive** (no Alpha Go) | future Go ADR if needed |
| `_archived/contracts` | archived TODOs | non-authoritative | **investigate** vs Spec Kit contracts | contract ADRs |
| Root Cargo/go/pnpm refs | contradictory | stale | **migrate** manifests in authorized R1+ tasks — not T008 | plan R0→R1 |

`retain` under `_archived` = keep as evidence, **not** reactivate.

---

## 12. Current-versus-target boundary

| Current | Target |
| --- | --- |
| Vite `afia-ui` + Python/Supabase direct | Rust Trusted Host, UI-zero-authority |
| Archived empty crates + stale root manifests | Reconstituted workspace under ADR-15 |
| macOS dmg-oriented Tauri stub | Windows-first Alpha packaging (Decision D / R5) |
| Optional Go ops skeleton | Not required for Alpha |

---

## 13. Runtime/build unknowns

- Whether any historical build of these crates ever succeeded: **unknown** (ARCHIVED.md says no real commits/toolchain at time).
- Exact Tauri 2 API surface once deps added: **unknown**.

---

## 14. Migration implications

1. Keep `_archived/**` intact as non-authoritative evidence.
2. Do not copy crates into `crates/` without ADR-gated reconstitution tasks.
3. Fix root manifests only when authorized (R1+); T008 forbids repair.
4. Prefer greenfield Trusted Host composition informed by archive READMEs, not stub reactivation.
5. Ignore Go for Founder Alpha unless a later ADR proves need.

---

## 15. Prohibited reactivation list (T008)

Do **not** without ADR acceptance + authorized tasks:

- Restore `apps/desktop` or `crates/afia-*` from archive as live authority
- Wire root `Cargo.toml`/`go.work`/`pnpm-workspace` to archived paths as “fixed”
- Compile/run archived Tauri/Go as product host
- Treat `afia-ai` stub as license to keep UI→Python calls
- Activate Go operations service for Alpha

---

## 16. Acceptance

| Criterion | Result |
| --- | --- |
| Archive classified non-authoritative | **Met** |
| Documentation only; no reactivation | **Met** |

## 17. Rollback

Delete this file; revert program-memory pointer edits.

## 18. Notes (PASS WITH NOTES)

- Entire Rust/Tauri/Go archive is intentional empty skeleton.
- Root manifests remain contradictory (known since T001).
- Bundle targets skew macOS vs Windows-first Alpha.
