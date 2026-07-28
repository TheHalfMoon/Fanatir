<!--
Sync Impact Report
- Version change: (template/unratified) → 1.0.0
- Modified principles: template placeholders → Fanatir-specific ratified principles
- Added sections: Mission; Product Spaces; Repository Boundaries; Planning & Memory;
  Primitives; Artifact & Provenance; Patient/Project Memory; Fehrest; DeepMed; commandF;
  Studio/CoLab/Lab; UX; Desktop/Local-First; Security & Privacy; Secure Sharing;
  Capability Gateway; Integrations Direction; Fork Governance; Quality & Evidence;
  Governance Hierarchy; Spec Lifecycle; PR & Acceptance; Amendment; Unresolved Decisions;
  Acceptance Checklist
- Removed sections: generic Spec Kit example principles only
- Templates requiring updates:
  - .specify/templates/plan-template.md ✅ Constitution Check gates filled
  - .specify/templates/spec-template.md ✅ mandatory constraints note added
  - .specify/templates/tasks-template.md ✅ quality/security/privacy task categories noted
  - AGENTS.md ✅ authority hierarchy updated to place Constitution above legacy AFIA rules
  - docs/program-memory/CURRENT-STATE.md ✅ constitution ratified status
  - docs/program-memory/NEXT-ACTION.md ✅ next Spec Kit command
- Follow-up TODOs:
  - Formal reconstitution ADR for desktop host stack (Windows-first Tauri target vs legacy AFIA README)
  - Separate specs for OpenMed fork/import and Graphify fork/import (not authorized here)
  - Primitive schema finalization deferred to later specs
- Adversarial review corrections (pre-commit):
  - Removed machine-specific absolute path from verified facts
  - Clarified Patient / Project / Fehrest memory authority boundaries
  - Aligned AGENTS.md hierarchy to the nine-level Constitution order
  - Added supersession notices to legacy README and execution-rules authority claims
-->

# Fanatir Ecosystem Constitution

## Core Principles

### I. Mission Accessibility
Fanatir is a desktop-first, local-first healthcare intelligence and research studio
where students, researchers, scientists, health professionals, interoperability
engineers, educators, teams, and institutions work with documents, patients,
projects, evidence, models, computation, collaboration, and FHIR in one coherent
environment. Advanced healthcare and research capabilities MUST remain accessible
to users with little technical knowledge through progressive disclosure, guided
workflows, and clear next actions.

### II. Spec-Driven Delivery (NON-NEGOTIABLE)
GitHub Spec Kit is mandatory for new product capabilities, architectural changes,
cross-repository contracts, security boundaries, storage decisions, model
integrations, healthcare interoperability, major refactors, and release milestones.
No implementation may begin from an informal chat prompt alone. Every
implementation task MUST be traceable to an accepted specification, an
implementation plan, an actionable task, acceptance criteria, and a verification
method.

### III. Authority of Memory and Evidence
GitHub history is the canonical shared repository history. Spec Kit specifications
are the requirements authority. Accepted ADRs are the architectural decision
authority. Markdown program memory is the human-readable continuity authority.
Obsidian is an interface over Markdown memory, not a separate source of truth.
Graphify is a derived, rebuildable knowledge index. AI conversation memory is
useful context and MUST NEVER be treated as canonical truth.

### IV. Artifact, Run, and Provenance Integrity
Every durable user result is an Artifact with immutable revisions. Every
computation, transformation, model invocation, and external tool invocation creates
a Run. Derived results MUST preserve provenance to inputs. Original documents and
imported clinical records are immutable sources. AI-generated summaries MUST NEVER
be treated as original sources. Approved results MUST remain reproducible from
recorded inputs, versions, configuration, and environment where technically
possible. No important result may exist only inside transient chat history.

### V. Clinical and AI Assistive Boundaries
DeepMed, commandF, Lab models, MCP tools, and plugins are assistive. They MUST NOT
claim autonomous clinical authority. AI may propose mappings, extractions, and edits
but MUST NOT silently approve clinically significant transformations. Ambiguity,
loss, unsupported mappings, and review requirements MUST remain visible. Claims of
accuracy, safety, compliance, or clinical readiness require documented evidence.

### VI. Security, Privacy, and Least Privilege
UI-zero-authority, least privilege, minimum necessary access, explicit data
classification, patient-scoped and project-scoped authorization, encrypted local
storage, encrypted transport, no-PHI logging, immutable audit events, egress
control, destination declaration, session locking, secure export, revocation,
controlled deletion, recovery procedures, plugin isolation, and signed trusted
updates are mandatory doctrines. Local storage alone is NOT sufficient to claim
HIPAA compliance. Language MUST use designed-with-HIPAA-aligned-safeguards,
PDPL-aware, and local-first privacy architecture unless stronger claims are
evidenced.

### VII. Repository and Fork Discipline
Fanatir is the product and integration authority. Fehrest and DeepMed-AI are
independent repositories with ratified boundaries. Upstream history, attribution,
licenses, NOTICE requirements, identifiable modifications, documented remotes, and
reviewed sync branches are mandatory for open-source forks. No fork is imported
without a dedicated specification and license review.

## Mission and Delivery Objective

### Mission
Fanatir is a desktop-first, local-first healthcare intelligence and research studio
where students, researchers, scientists, health professionals, interoperability
engineers, educators, teams, and institutions can work with documents, patients,
projects, evidence, models, computation, collaboration, and FHIR within one
coherent environment.

### Founder Alpha Objective
Deliver an integrated and installable Fanatir Founder Alpha within approximately
60 days using AI-accelerated specification, implementation, testing, and review.

Clarifications that MUST accompany this objective:

- It is an aggressive execution target.
- It is NOT a claim of hospital-production certification.
- It does NOT authorize unsafe shortcuts.
- It does NOT justify fabricated accuracy, security, compliance, or clinical claims.
- Requirements MAY be delivered through bounded Alpha capability rather than
  universal production maturity.

## Product-Space Status

### Active
Studio; CoLab; Fanatir Lab; Fehrest; DeepMed; commandF; Patient Longitudinal
Memory; Project Long Memory; Collaboration; Secure Sharing; Capability Gateway;
MCP and approved plugins; MedScale integration; MESC integration.

### Frozen until explicit founder reauthorization
Pictorial; Montada.

### DeepMed release rule
DeepMed MUST NOT be frozen, removed, or postponed outside the first integrated
Fanatir release.

## Repository Boundaries

### Fanatir (`IamShehri/Fanatir`)
Fanatir is the product and integration authority. It owns desktop product
experience; Studio; CoLab; Fanatir Lab; commandF; patient dashboards; patient
memory; project memory integration; collaboration; secure sharing; capability and
plugin gateway; local policy enforcement; and ecosystem integration contracts.

### Fehrest (`IamShehri/Fehrest`)
Fehrest is both:

1. an independent second-brain product usable without Fanatir; and
2. Fanatir’s knowledge, memory, citation, and graph capability.

Fehrest combines Markdown-first knowledge; Obsidian-style links and vault
portability; Notion-style structured content; Graphify-style knowledge graphs;
notes; sources; quotations; claims; evidence; citations; project memory; search;
and local AI-assisted synthesis.

Markdown is authoritative for human-authored knowledge. Graph indexes are derived
and rebuildable. The planned Graphify fork and import mechanics require a separate
specification and are NOT authorized by this constitution.

### DeepMed-AI (`IamShehri/DeepMed-AI`)
DeepMed-AI is the independent medical intelligence runtime used by Fanatir. It will
be developed from a governed OpenMed fork and extended with task-first workflows;
clinical entity extraction; PHI detection and de-identification; source-span
provenance; assertions; negation; temporality; relations; Arabic and bilingual
healthcare support; Saudi identifiers and policy support; human review; model
selection and routing; commandF handoff; MedScale evaluation hooks; and MESC
integration.

The OpenMed fork/import and upstream-sync mechanics require a separate
specification and are NOT authorized by this constitution.

## Planning and Memory Authority

Required context-loading order before planning or implementation:

1. Constitution
2. Active Spec Kit specification
3. Active implementation plan
4. Active task list
5. `docs/program-memory/CURRENT-STATE.md`
6. `docs/program-memory/NEXT-ACTION.md`
7. Relevant decisions and ADRs
8. Graphify impact/query results
9. Current repository state

Every significant session MUST end with a durable Markdown closeout. Every
milestone MUST produce a project-memory snapshot. PHI, patient documents, secrets,
credentials, API keys, and private datasets MUST NEVER be stored in the planning
vault.

## Specification Lifecycle

Mandatory lifecycle:

1. Constitution
2. Specify
3. Clarify
4. Plan
5. Tasks
6. Independent review
7. Implement
8. Verify
9. Converge
10. Accept or reject

## Core Product Primitives

All Fanatir spaces MUST converge on shared primitives:

Workspace; Project; Patient; Artifact; Revision; Run; Source; Relationship;
Review; Approval; Decision; PolicyDecision; DataClassification; Capability;
ExportManifest.

Complete schemas are NOT finalized in this constitution and require later
specifications and ADRs.

## Artifact and Provenance Principles

- Every durable user result is an Artifact.
- Every Artifact has immutable revisions.
- Every computation, transformation, model invocation, and external tool
  invocation creates a Run.
- Every derived result preserves provenance to its inputs.
- Original documents and imported clinical records are immutable sources.
- AI-generated summaries are never treated as original sources.
- Approved results remain reproducible from recorded inputs, versions,
  configuration, and environment where technically possible.
- No important result may exist only inside transient chat history.

## Patient Longitudinal Memory

Patient Longitudinal Memory is an active first-class capability. It MUST support
patient overview; longitudinal timeline; documents; encounters; conditions;
medications; allergies; observations; laboratories; procedures; clinical notes;
FHIR resources; DeepMed results; commandF transformations; reviews and
corrections; related projects; and provenance.

Patient history MUST NOT be overwritten. Clinical statements MUST support temporal
validity, status, source, review state, contradictions, and supersession. Patient
access authority MUST remain separate from ordinary project membership.

## Project Long Memory

Project Long Memory is an active first-class capability. Projects may continue for
years and MUST preserve charter; objectives; participants; protocols; decisions and
rationale; sources; datasets; experiments; notebooks; failures; findings; claims
and evidence; open questions; risks; milestones; current state; and next authorized
action.

Distinguish personal memory, team memory, and authoritative project memory.
Informal notes do NOT become authoritative merely because they exist.

### Memory authority boundaries (non-ambiguous)

- **Patient Longitudinal Memory** is Fanatir patient-scoped clinical continuity. It
  is NOT governed by ordinary project membership and is NOT replaced by Fehrest
  notes.
- **Project Long Memory** is Fanatir project-scoped authoritative continuity for
  Fanatir projects (charter, decisions, milestones, current state, next action).
- **Fehrest** owns standalone and embeddable human knowledge, citations, claims,
  evidence, and derived graphs. Fehrest project-linked notes may support Project
  Long Memory but do NOT become patient clinical authority and do NOT supersede
  Fanatir project governance records merely by existing.
- Markdown is authoritative for human-authored knowledge in Fehrest. It is NOT
  authoritative for FHIR resources, original healthcare files, databases, or
  model artifacts.

## Fehrest Authority

Markdown is truth for human knowledge; the graph is derived; AI is advisory;
sources are preserved.

Fehrest MUST remain local-first, portable, inspectable, usable independently from
Fanatir, and capable of operating without mandatory cloud services. FHIR,
databases, model artifacts, and original healthcare files MUST remain in
standards-appropriate formats and MUST NOT be replaced by Markdown.

## DeepMed Principles

- DeepMed is task-first rather than model-first.
- Nontechnical users SHOULD choose an objective, not manually select from thousands
  of models.
- Models MUST pass eligibility checks before execution.
- Eligibility includes task, language, license, hardware, privacy, evaluation,
  location, and project policy.
- DeepMed MUST return no eligible model rather than silently weakening mandatory
  constraints.
- Clinical extraction MUST preserve source spans.
- AI-generated clinical structures REQUIRE review where risk or uncertainty demands
  it.
- DeepMed is assistive and MUST NOT claim autonomous clinical authority.

## commandF Principles

commandF is the universal healthcare interoperability workbench. Its goal is to
accept healthcare sources and produce one of: VERIFIED; REVIEW REQUIRED; PARTIAL;
UNSUPPORTED; REJECTED.

commandF MUST NOT hide ambiguity or fabricate mappings. It MUST progressively
support healthcare file ingestion; source profiling; FHIR conversion; FHIR
validation; FHIR search; explanation; comparison; research; safe draft editing;
provenance; loss reporting; and reproducibility packages.

Live clinical writes are NOT automatically authorized by this constitution. AI may
propose mappings and edits but MUST NOT silently approve clinically significant
transformations.

## Studio, CoLab, and Fanatir Lab

### Studio
The individual work surface for reading; writing; annotation; quoting; analysis;
notes; document processing; patient review; DeepMed; commandF; Fehrest; and model
assistance.

### CoLab
The shared work and governance surface for teams; professors and students;
researchers; review; comments; assignments; branches; approvals; shared projects;
shared artifacts; project onboarding; and institutional collaboration. CoLab is
not Montada and MUST remain active.

### Fanatir Lab
The accessible model, data, and computation surface for health users with limited
technical experience. It MUST progressively support Hugging Face models; MESC;
DeepMed models; approved commercial models; local models; Python; R; SQL;
notebooks; no-code and guided analysis; and reproducible experiments. Lab
capabilities MUST also be accessible contextually from Studio and CoLab.

## UX Principles

- The interface MUST feel like a professional studio controlled by the user.
- Advanced power MUST NOT require technical expertise.
- Complexity SHOULD be progressively disclosed.
- The same Artifact MUST be usable across Studio, CoLab, Lab, Fehrest, DeepMed, and
  commandF without copying it.
- The UI MUST make source, model, privacy boundary, execution state, and review
  state visible.
- No invisible clinical automation.
- No dead-end dashboards.
- Every page SHOULD support an obvious next action.
- Accessibility and keyboard navigation are mandatory.
- v0 MAY be used for design exploration but has no architecture, security, storage,
  or clinical authority.
- Final UI implementation REQUIRES reviewed design specifications and interaction
  acceptance criteria.

Exact visual layout details are NOT ratified here.

## Desktop and Local-First Architecture Target

Ratified target direction:

- desktop-first
- local-first
- Windows-first release
- architectural support for macOS and Linux
- optional cloud collaboration rather than mandatory cloud authority

Target composition direction:

- React and Vite for the UI
- Tauri as the desktop composition boundary
- Rust as the trusted local host
- isolated workers for Python, AI, FHIR, and notebook execution
- replaceable external service adapters

This is an architectural target and still REQUIRES formal reconstitution and
implementation ADRs. Legacy AFIA documents that describe a macOS-only or different
host layout are historical or aspirational unless re-accepted through Spec Kit and
ADRs.

## Security and Privacy Doctrine

Mandatory doctrines include UI-zero-authority; least privilege; minimum necessary
access; explicit data classification; patient-scoped authorization; project-scoped
authorization; encrypted local storage; encrypted transport; no-PHI logging;
immutable audit events; egress control; tool and model destination declaration;
session locking; secure export; revocation; controlled deletion; recovery
procedures; plugin isolation; and signed updates with trusted distribution.

Permitted claim language when safeguards are designed in:

- designed with HIPAA-aligned safeguards
- PDPL-aware
- local-first privacy architecture

Forbidden claim language without documented legal, operational, technical, and
security evidence:

- HIPAA certified
- fully compliant
- clinically safe
- hospital ready

## Secure Sharing

Fanatir supports controlled sharing through Fanatir username; email; CoLab; secure
links; approved cloud destinations; and approved external platforms.

Before external or public sharing the system MUST classify the Artifact; detect
sensitive information; offer de-identification or redaction; show a preview;
explain remaining risk; require confirmation where appropriate; record the share
event; and support expiration and revocation where possible.

Social sharing MUST NEVER appear for PHI or restricted data unless an authorized,
reviewed public derivative has been created.

## Models, MCP, and Plugins

All models, MCP servers, plugins, storage services, and external tools MUST pass
through the Fanatir Capability Gateway. Every capability MUST declare publisher;
version; permissions; data read; data written; network destinations; supported
classifications; PHI policy; human-confirmation requirements; audit behavior;
license; and update and revocation policy.

No plugin receives unrestricted database, filesystem, secret, or patient access.

Initial trust tiers: first-party; organization-private; explicitly approved
external integrations. A public unreviewed marketplace is NOT authorized.

## Required Integrations Direction

Recognized planned integration families (recognition is NOT implementation
authorization): GitHub; Zotero; PubMed and PMC; Crossref; OpenAlex;
ClinicalTrials.gov; scientific search; Google Drive and Docs; Microsoft 365 and
OneDrive; Box; Dropbox; Overleaf; LaTeX; Quarto and Pandoc; REDCap; Hugging Face;
OpenAI; Anthropic; Google models; local model runtimes; FHIR servers; terminology
services.

Each integration REQUIRES a specification and security review.

## Open-Source Fork Governance

- Upstream history and attribution MUST be preserved.
- Licenses and NOTICE requirements MUST be followed.
- Modified upstream files MUST be identifiable where required.
- Upstream remotes MUST be documented.
- Upstream sync MUST occur through reviewed branches.
- No fork is imported without a dedicated specification and license review.
- Fanatir MAY remain proprietary while properly using permissively licensed
  components.
- Trademarks MUST NOT be implied by open-source license permissions.

## Quality and Evidence

No acceptance without evidence. Required quality dimensions include functionality;
type safety; unit testing; integration testing; security testing; privacy testing;
deterministic behavior where required; failure-mode testing; accessibility;
performance; packaging; migration; reproducibility; and documentation.

Claims about accuracy REQUIRE defined benchmarks and representative evaluation
data. AI-generated code REQUIRES the same or stronger review as human-authored
code.

## Governance Hierarchy

1. Founder-ratified Constitution
2. Founder-approved product and architecture decisions
3. Accepted ADRs
4. Active Spec Kit specification
5. Active implementation plan
6. Active tasks
7. Repository execution instructions
8. Implementation code
9. AI suggestions

Older AFIA documentation MUST NOT override this Constitution. No AI agent may
change founder-ratified scope or governance status implicitly.

## Pull Request and Acceptance Policy

Require bounded branches; traceability to a specification and task; explicit
changed-file scope; verification evidence; no unrelated changes; independent review
for security, clinical, interoperability, or architecture-sensitive work; no
automatic merge unless separately authorized; post-merge verification for critical
changes; and durable project-memory closeout.

## Architecture Authority

Accepted ADRs are the architectural decision authority. Existing repository layout,
legacy README claims, archived scaffolds, and chat proposals are evidence inputs
only. They become binding only when re-accepted through Spec Kit specification,
planning, and ADR acceptance under this Constitution.

## Verified Repository Facts vs Non-Authoritative Legacy Claims

### Verified at ratification (2026-07-23)
- Primary product repository: `IamShehri/Fanatir` (package name currently `afia`;
  rename timeline remains an unresolved founder decision).
- Independent repos: `IamShehri/Fehrest` (empty remote); `IamShehri/DeepMed-AI`
  (README-only tip).
- Local multi-repo ecosystem checkout is used for planning; machine-specific
  absolute paths are recorded only in local program-memory state files, not as
  portable governance requirements.
- Fanatir contains active application surfaces including `afia-ui/`, `lib/`,
  `apps/`, `services/`, `docs/`, and `_archived/`.
- Spec Kit is initialized with `cursor-agent` integration.
- Program memory exists under `docs/program-memory/`.
- This Constitution does NOT authorize product implementation, package rename,
  OpenMed import, or Graphify import.

### Founder-ratified by this Constitution
Mission; active/frozen product spaces; repository boundaries; Spec Kit lifecycle;
memory authority; primitives list; Patient/Project memory; Fehrest/DeepMed/commandF
principles; Studio/CoLab/Lab roles; UX doctrines; desktop/local-first target
direction; security/privacy doctrine; Capability Gateway; fork governance; quality
gates; governance hierarchy; PR policy.

### Architectural targets (require reconstitution ADRs)
Windows-first Tauri + React/Vite + Rust host with isolated workers. This target is
ratified as direction, not as a claim that current code already implements it.

### Unresolved implementation decisions
See Unresolved Founder Decisions below. Complete primitive schemas, OpenMed import
mechanics, Graphify import mechanics, and host reconstitution details are deferred.

## Unresolved Founder Decisions

1. Exact package/brand rename timeline from AFIA naming to Fanatir across the
   monorepo (rename is NOT authorized by this constitution task).
2. Formal host reconstitution plan resolving legacy AFIA README (macOS-centric /
   archived crates layout) against the Windows-first Tauri target.
3. Fehrest first scaffold and import of Graphify (separate specification required).
4. DeepMed-AI OpenMed fork/import and upstream-sync mechanics (separate
   specification required).
5. Final schemas for shared primitives.
6. Whether Codex Spec Kit integration should be installed alongside `cursor-agent`.
7. Exact Alpha capability bounds for the ~60-day Founder Alpha.

## Amendment Procedure

1. Propose amendment with rationale, impacted principles, and migration notes.
2. Update `.specify/memory/constitution.md` with semantic version bump:
   - MAJOR: incompatible principle removal or redefinition
   - MINOR: new principle/section or material expansion
   - PATCH: clarifications and non-semantic refinements
3. Propagate dependent Spec Kit templates and runtime guidance.
4. Record decision under `docs/program-memory/decisions/`.
5. Update `CURRENT-STATE.md` and `NEXT-ACTION.md`.
6. Obtain founder ratification before treating the amendment as binding.

## Acceptance Checklist

- [x] Mission ratified
- [x] Active and frozen product spaces ratified
- [x] Repository boundaries ratified
- [x] Spec Kit lifecycle mandatory
- [x] Planning/memory authority and context-loading order ratified
- [x] Shared primitives listed without premature schema finalization
- [x] Artifact/provenance principles ratified
- [x] Patient Longitudinal Memory and Project Long Memory ratified
- [x] Fehrest, DeepMed, commandF, Studio, CoLab, Lab principles ratified
- [x] UX, desktop/local-first, security/privacy, secure sharing ratified
- [x] Capability Gateway and integration recognition ratified
- [x] Fork governance and quality gates ratified
- [x] Governance hierarchy and PR/acceptance policy ratified
- [x] Constitution version and ratification date set
- [x] Unresolved founder decisions listed
- [x] No production implementation performed by this constitution task

## Governance

This Constitution supersedes conflicting legacy AFIA guidance, informal chat
instructions, and non-accepted documents. Compliance review for planning and
implementation MUST verify Spec Kit traceability, security/privacy doctrine,
clinical/AI assistive boundaries, repository boundaries, and evidence requirements.
Complexity and exceptions MUST be justified in an accepted specification or ADR.
Runtime agent guidance lives in `AGENTS.md` and module rules in
`docs/execution-rules.md`, both subordinate to this Constitution until formally
reconstituted.

**Version**: 1.0.0 | **Ratified**: 2026-07-23 | **Last Amended**: 2026-07-23
