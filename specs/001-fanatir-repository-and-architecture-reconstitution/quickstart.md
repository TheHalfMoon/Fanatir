# Quickstart: Validate Planning Artifacts (001)

**Purpose**: Runnable checks that prove planning completeness — **not** product implementation.

## Prerequisites

- Local Fanatir checkout on `docs/f0-planning-memory-bootstrap`
- Spec 001 accepted commit present
- Fehrest and DeepMed-AI clones present (may be empty/README-only)
- No requirement to start Supabase or build Tauri for this quickstart

## 1. Authority chain

```powershell
Set-Location C:\Projects\Fanatir-Ecosystem\Fanatir
Test-Path .specify/memory/constitution.md
Test-Path specs/001-fanatir-repository-and-architecture-reconstitution/spec.md
Test-Path specs/001-fanatir-repository-and-architecture-reconstitution/plan.md
Test-Path specs/001-fanatir-repository-and-architecture-reconstitution/research.md
```

Expect all `True`.

## 2. Planning artifact set

```powershell
$base = "specs/001-fanatir-repository-and-architecture-reconstitution"
@(
  "plan.md","research.md","data-model.md","quickstart.md",
  "adrs/ADR-15-rust-first-polyglot-runtime.md",
  "contracts/trusted-host-ipc.md","contracts/worker-runtime.md","contracts/shared-primitives.md",
  "contracts/fehrest-integration.md","contracts/deepmed-integration.md",
  "contracts/supabase-adapter.md","contracts/commandf.md"
) | ForEach-Object { Join-Path $base $_; Test-Path (Join-Path $base $_) }
```

## 3. No production mutation from planning

```powershell
git status --short
git diff --name-only
```

Expect only planning/docs paths under `specs/001-...`, optionally `docs/program-memory/*`.  
Must **not** include `afia-ui/`, `lib/`, `services/`, `apps/` production sources (unless explicitly unrelated and rejected).

## 4. Sibling repos unchanged

```powershell
git -C C:\Projects\Fanatir-Ecosystem\Fehrest status --short
git -C C:\Projects\Fanatir-Ecosystem\DeepMed-AI status --short
```

Expect empty.

## 5. Checklist still green

Open `checklists/requirements.md` — all 27 items remain `[x]`.

## 6. Conceptual golden-journey dry run (documentation)

Walk [plan.md](./plan.md) stage **R3** against Spec Alpha tiers and confirm each step maps to a contract:

| Step | Contract |
| --- | --- |
| Project open/create | `trusted-host-ipc.md` |
| Import document | Artifact/Source in `data-model.md` |
| Studio | UI shell (`afia-ui`) + host |
| DeepMed | `deepmed-integration.md` |
| Fehrest | `fehrest-integration.md` |
| commandF | `commandf.md` |
| Lab / Run / share | Run + ExportManifest + Approval |

## Expected outcomes

- Plan reviewable without code changes
- P1–P4 answers visible in `research.md`
- ADR roadmap sequenced in `plan.md`
- Ready for `/speckit-tasks` (do not run until founder requests)
