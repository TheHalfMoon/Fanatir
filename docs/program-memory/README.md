# Fanatir Program Memory

Human-readable project memory for the Fanatir ecosystem planning vault.

## Authority model

| Layer | Role |
| --- | --- |
| GitHub repositories | Canonical shared history for code and tracked docs |
| GitHub Spec Kit specifications | Requirements authority for planned work |
| Markdown files in `docs/program-memory/` | Human project-memory authority |
| Obsidian | Interface over those Markdown files (not a separate source of truth) |
| Graphify | Derived, rebuildable cross-repository index (not authoritative) |
| AI chat memory | Non-authoritative; discard when it conflicts with tracked docs |

## Hard rules

- Do **not** store PHI, patient documents, secrets, credentials, API keys, or private datasets in this vault.
- Obsidian Sync must never sync patient PHI from this development vault.
- The Obsidian account is **not** an authorization boundary for Fanatir clinical data.

## Session closeout rules

Before ending a planning or implementation session:

1. Update `CURRENT-STATE.md` with verified facts only.
2. Update `NEXT-ACTION.md` with the single next authorized action.
3. Add unresolved items to `OPEN-QUESTIONS.md`.
4. Record material decisions under `decisions/` (see decision-record rules).
5. Leave AI chat history out of the authoritative record unless captured in Markdown.

## Milestone snapshot rules

At each milestone boundary:

1. Snapshot verified repository HEADs and branch names into `PROJECT-TIMELINE.md`.
2. Confirm Spec Kit artifacts (constitution / spec / plan / tasks) that apply to the milestone.
3. Do not invent product completion status; mark unknowns explicitly.

## Decision-record rules

- One decision per file under `decisions/`.
- Filename: `YYYY-MM-DD-short-title.md`.
- Include: context, decision, alternatives considered, consequences, and links to Spec Kit artifacts when relevant.
- Do not bury decisions only in chat transcripts.

## Context-loading order

Before any planning or implementation work, load context in this order:

1. Constitution
2. Active Spec Kit specification
3. Active implementation plan
4. Active task list
5. `docs/program-memory/CURRENT-STATE.md`
6. `docs/program-memory/NEXT-ACTION.md`
7. Relevant decisions and ADRs
8. Graphify impact/query results
9. Repository state
