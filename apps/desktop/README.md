# Fanatir desktop shell (T031)

Minimal **Tauri 2** window that loads the existing `afia-ui` client as an **untrusted** WebView presentation surface.

## Scope

- In scope: window shell + client consumption for Windows-first smoke.
- Out of scope: Trusted Host, IPC, workers, capabilities/plugins beyond window render, persistence, auth, Supabase, PHI, Fehrest, DeepMed, commandF.

## Layout

- `ui/` — pnpm orchestration package (`build`, `tauri:dev`); does not own application UI source.
- `src-tauri/` — Tauri 2 / Rust shell (nested Cargo workspace).

## Commands (from repository root)

```text
pnpm --dir apps/desktop/ui run build
pnpm --dir apps/desktop/ui run tauri:dev
```

Client package (repository-defined; Vite root is `afia-ui/client`):

```text
pnpm --dir afia-ui run build
pnpm --dir afia-ui run dev
```
