# Fanatir Agent Instructions

## Authority hierarchy (mandatory)

Exact order from the Fanatir Constitution. Higher layers win on conflict:

1. Founder-ratified Constitution: `.specify/memory/constitution.md`
2. Founder-approved product and architecture decisions
3. Accepted ADRs
4. Active Spec Kit specification
5. Active implementation plan
6. Active tasks
7. Repository execution instructions such as `docs/execution-rules.md`
8. Implementation code
9. AI suggestions

Required context (not a substitute for the hierarchy above): load
`docs/program-memory/CURRENT-STATE.md`, `NEXT-ACTION.md`, and relevant decisions
per the Constitution context-loading order before planning or implementation.

Older AFIA documentation, README claims, package metadata, and chat memory are
evidence inputs only. They MUST NOT override the Constitution.

Module-isolation constraints in
[docs/execution-rules.md](docs/execution-rules.md) remain in force for existing
code paths until reconstituted through Spec Kit and ADRs, but only as hierarchy
level 7 and never above the Constitution, ADRs, or active Spec Kit artifacts.

---

## Executor Prompt (Use this every time)

```text
Follow the Fanatir Constitution and Spec Kit workflow.

Primary authority (highest to lowest):
1. .specify/memory/constitution.md
2. founder-approved decisions
3. accepted ADRs
4. active Spec Kit specification
5. active plan
6. active tasks
7. docs/execution-rules.md (existing module constraints only)
8. implementation code
9. AI suggestions

Also load docs/program-memory/CURRENT-STATE.md and NEXT-ACTION.md as required context.

<<< TASK START >>>

PUT YOUR TASK HERE

<<< TASK END >>>

Rules:
- Do not implement from informal chat alone
- Make minimal changes only
- Do not change founder-ratified scope implicitly
- Do not invent compliance or clinical claims
- Do not create new files unless required by an accepted task
- If anything is unclear, stop and explain instead of guessing

After completion:
- List modified files
- Explain what changed
- Provide verification steps
- Update program-memory closeout when the session is significant
```
