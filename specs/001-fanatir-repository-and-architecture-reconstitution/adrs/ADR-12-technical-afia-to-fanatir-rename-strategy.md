# ADR-12 — Technical AFIA-to-Fanatir Rename Strategy

| Field | Value |
| --- | --- |
| **ADR** | ADR-12 |
| **Title** | Technical AFIA-to-Fanatir Rename Strategy |
| **Status** | **Proposed** (draft only) |
| **Task origin** | T024 |
| **Acceptance / review posture** | Tier C — normal verification after drafting. **T030** mandatory Accepted set is ADR-15/01/02/06 only; ADR-12 is **not** listed in T030’s explicit Reviewed-or-Accepted group (04/05/07/08/09/10/14). Plan sequencing: ADR-12 deferred after Alpha unless accelerated. Downstream **T056** gates on **ADR-12 deferred** (compatibility adapters without mass rename). |
| **Feature** | `001-fanatir-repository-and-architecture-reconstitution` |
| **Required gate (tasks.md)** | Deferred mass rename |
| **Planning dependency (plan.md)** | Q8 — Technical AFIA→Fanatir rename strategy; inventory + shims; not in reconstitution mass rename |
| **Dependency (tasks.md)** | T011 |
| **Constraining drafts** | [ADR-15](./ADR-15-rust-first-polyglot-runtime.md) (Proposed); [ADR-02](./ADR-02-rust-trusted-host-boundary.md) (Proposed); [ADR-03](./ADR-03-afia-ui-strangler-migration.md) (Proposed); [ADR-04](./ADR-04-shared-primitive-ownership-and-versioning.md) (Proposed); [ADR-05](./ADR-05-artifact-revision-run-storage.md) (Proposed); [ADR-06](./ADR-06-worker-and-ipc-contracts.md) (Proposed); [ADR-07](./ADR-07-supabase-adapter-and-local-first-boundary.md) (Proposed); [ADR-08](./ADR-08-fehrest-integration-and-release-model.md) (Proposed); [ADR-09](./ADR-09-deepmed-integration-and-openmed-runtime-fork-boundary.md) (Proposed); [ADR-10](./ADR-10-commandf-ownership-and-process-boundary.md) (Proposed); [ADR-11](./ADR-11-auth-and-session-preservation.md) (Proposed) |
| **Related drafts** | [ADR-01](./ADR-01-platform-and-desktop-composition-tauri-2.md) (Proposed) |
| **Planning evidence** | [spec.md](../spec.md) Q8 / Identity and Rename / FR-002 / FR-005 / FR-007 / FR-011 / inventory; [plan.md](../plan.md) ADR-12 row / sequencing; [research.md](../research.md) R0 topology / R3 / R5; Constitution verified facts and unresolved rename timeline ([constitution.md](../../../.specify/memory/constitution.md)); [tasks.md](../tasks.md) T024 / T011 / T030 / T056 |
| **Architecture authority of this file** | **NO** — until a valid stage-gate acceptance action records Accepted |
| **Implementation authorization** | **NO** |
| **Rename / migration authorization** | **NO** — this draft defines strategy principles only |

```text
This document is a planning draft and is not accepted architecture authority.
```

```text
Status: Proposed
Draft ADR ≠ architecture acceptance
T024 completion ≠ T030 acceptance
Passing Tier C review ≠ Accepted
tasks.md founder-acceptance field for T024: No — draft only
Founder acceptance of a draft work product ≠ architecture acceptance
Founder-ratified Q8 Fanatir product identity ≠ permission to mass-rename technical identifiers
Constitution unresolved package/brand rename timeline ≠ resolved by this draft
ADR-15 / ADR-01 / ADR-02 / ADR-03 / ADR-04 / ADR-05 / ADR-06 / ADR-07 / ADR-08 / ADR-09 / ADR-10 / ADR-11 remain Proposed and non-authoritative
```

```text
This draft is not:
- permission to rename files, directories, repositories, packages, namespaces, or imports
- permission to rename executables, environment variables, database or Supabase objects
- permission to rename UI labels, persistent identifiers, or external accounts
- permission to rename Fehrest, DeepMed, or commandF
- permission to create migrations, compatibility aliases, or redirects in the repository
- permission to publish packages or releases
- permission to modify afia-ui, auth/session behavior, or production code
- a final inventory of every occurrence
- a final set of machine-readable Fanatir identifier forms
- permission to begin R2 or R3
```

```text
No production implementation is authorized by this draft.
```

This draft **must not** be used as justification to: rename any file, directory, repository, package, namespace, import, executable, environment variable, database object, Supabase object, UI label, persistent identifier, or external account; rename Fehrest, DeepMed, or commandF; create migrations, aliases, or redirects; modify [`afia-ui`](../../../afia-ui); change authentication or sessions; publish packages or releases; begin R2 (`T031+`) or R3; or treat Proposed ADRs as Accepted.

---

## 1. Context and problem

### 1.1 Assigned architectural question

**ADR-12 proposes** a **technical AFIA-to-Fanatir rename strategy** for Spec **001**: preserve the founder-ratified **Fanatir** product identity for documentation, specifications, architecture, and new product-facing work, while **deferring mass technical rename** of legacy **AFIA** / `afia` identifiers until a later dedicated rename/migration specification is accepted — and defining only **inventory**, **compatibility-shim principles**, **migration gates**, **verification**, and **rollback** principles.

It does **not** rename anything; invent package-safe / namespace-safe / domain forms; execute inventory searches as mutation; create aliases or shims in code; migrate persistence; change auth/session keys or routes (**ADR-11**); redefine Fehrest (**ADR-08**), DeepMed (**ADR-09**), or commandF (**ADR-10**); finalize packaging names (**ADR-13**, not authored); or authorize R2/R3.

### 1.2 Why an explicit rename strategy matters

Verified inventory shows dual identity: governance and new docs say **Fanatir**, while package metadata, UI shell paths, and legacy documents still say **AFIA** / `afia` ([spec.md](../spec.md); Constitution; [research.md](../research.md)). Founder-ratified Q8 separates product-facing identity from technical identifier migration ([spec.md](../spec.md) Identity and Rename). Immediate mass rename was rejected as reconstitution churn ([research.md](../research.md); [plan.md](../plan.md) ADR-12 row).

Without an explicit strategy, planning identifies these **risks** (prospective — not claimed as currently measured production failures unless separately evidenced):

| Risk | Why it matters |
| --- | --- |
| Mass `afia` rename mid-reconstitution | Breaks imports, builds, CI, and Alpha journey |
| Treating Fanatir docs identity as package rename authority | Premature technical migration |
| Collapsing Fehrest / DeepMed / commandF into Fanatir naming | Cross-product identity damage |
| Renaming durable identifiers with display strings | Persistence / provenance fork |
| Auth/session key or redirect rename without ADR-11 + dedicated spec | Session breakage |
| Supabase / schema rename under 001 | Decision C / ADR-07 violation |
| GitHub or local directory rename mistaken for ADR-12 | Operational scope creep |
| Inventing machine-readable Fanatir forms without founder decision | False lock-in |
| Compatibility aliases without removal gate | Permanent dual identity debt |
| External trademark/domain claims without evidence | Legal and availability risk |

### 1.3 Accepted program and planning constraints

| Constraint | Source | Treatment |
| --- | --- | --- |
| Fanatir is canonical product identity immediately for docs/specs/architecture/new product-facing work; no mass technical rename during reconstitution; technical rename needs inventory/impact/ADR/plan/shims/verification; product-facing rename and technical migration are separate | Q8 — [spec.md](../spec.md) | Binding rename policy; **not** ADR-12 acceptance |
| Keep AFIA package/string identity functioning until a rename specification is accepted | [spec.md](../spec.md) | Binding preservation of technical names |
| Package name currently `afia`; rename timeline remains an unresolved founder decision | Constitution verified facts | Unresolved timeline; **not** resolved here |
| ADR-12 = inventory + shims; not reconstitution mass rename; deferred after Alpha unless accelerated | [plan.md](../plan.md) | Planning sequencing |
| T024: ADR draft markdown only; Deferred mass rename gate; no migration mutation; draft ≠ accepted | [tasks.md](../tasks.md) T024 | Binding for this task |
| T056: compatibility adapters without mass rename; gate ADR-12 deferred; prohibit mass `afia` rename | [tasks.md](../tasks.md) T056 | Downstream; not authorized by this draft |
| No production implementation; package rename; file moves for migration; push/PR under Spec 001 unless later accepted specification authorizes | FR-011 — [spec.md](../spec.md) | Binding out-of-scope |

### 1.4 Naming

| Name | Role in this ADR |
| --- | --- |
| **Fanatir** | Canonical **product** identity for new docs/specs/architecture/product-facing work (Q8) |
| **AFIA** | Legacy product/branding identity appearing in docs, titles, and historical materials |
| **`afia`** | Current technical package / identifier family (Constitution; package manifests) |
| **`afia-ui`** | Migration starting shell directory/package surface (**ADR-03** Proposed) — not the whole product |
| **Technical rename** | Changing machine-readable identifiers (packages, imports, env keys, schemas, etc.) |
| **Product-facing rename** | Human-visible product naming in docs and new product surfaces — already Fanatir for new work |
| **Fehrest** | Independent product/repo — **excluded** from AFIA→Fanatir technical rename of Fanatir packages |
| **DeepMed** / DeepMed-AI | Independent medical intelligence runtime — **excluded** |
| **commandF** | Fanatir-owned capability name — **not** renamed by this strategy |
| **Artifact Store** / **Trusted Host** / **Capability Gateway** | Architectural concepts — distinguish labels from machine-stable identifiers |
| **PolicyDecision** / **AuthContext** / **DataClassification** | Shared primitive / UI names — not AFIA package renames |

Never confuse Fanatir product identity with permission to mass-rename `afia*`; never rename Fehrest as Fanatir; never treat `afia-ui` as the entire product.

### 1.5 What this draft is not

This file is **not**: a rename execution plan for Spec 001; a final occurrence inventory; a founder decision of package rename timeline; a set of invented Fanatir machine forms; permission to mutate the repository; packaging/distribution rename (**ADR-13**); auth/session redesign (**ADR-11**); or R2/R3 authorization.

---

## 2. Decision (proposed)

### 2.1 Core rename strategy decision

**Propose** that:

1. **Fanatir** remains the canonical **product** identity for documentation, new specifications, new architecture, and new product-facing work (Q8 — already founder-ratified).
2. Legacy **AFIA** / `afia` **technical** identifiers **MUST NOT** be mass-renamed during Spec **001** reconstitution.
3. Any later technical renaming requires: inventory; compatibility impact analysis; Accepted ADR-12 (or successor acceptance); a **dedicated rename/migration specification**; aliases/shims where required; packaging/upgrade verification; and explicit stage authorization.
4. Product-facing naming and technical identifier migration remain **separate operations**.
5. This ADR defines **principles and gates only** — not an execution of rename, and not resolution of the Constitution’s unresolved package/brand rename **timeline**.

**Deferred mass rename** (tasks.md gate) means: no Spec 001 task may treat ADR-12 draft completion as authority to rename technical identifiers.

### 2.2 Source and target (proposed)

| Dimension | Status |
| --- | --- |
| Source product/brand family | **AFIA** (legacy human-facing) and technical family **`afia`** / **`afia-ui`** (inventory evidence) |
| Target product identity | **Fanatir** — ratified for docs/specs/architecture/new product-facing work |
| Target technical identifier forms (package-safe, namespace-safe, CLI, env-prefix, domain, bundle ID, etc.) | **Unresolved** — this draft does **not** invent or select them |
| Package/brand rename timeline | **Unresolved founder decision** (Constitution) |
| Acceleration of ADR-12 after Alpha | Plan allows “unless accelerated”; acceleration authority remains **unresolved** here |

Do **not** derive machine-readable Fanatir forms automatically from the display name.

### 2.3 Dual-identity operating mode under Spec 001 (proposed)

Until a dedicated rename specification is accepted:

- new governance and architecture documents use **Fanatir**;
- existing `afia` package names, imports, folders, and related technical strings **remain functioning**;
- dual AFIA/Fanatir appearance in inventory is **expected**, not a defect to “fix” by mass rename under 001;
- legacy `docs/product/AFIA_*` and similar materials remain **evidence only**, not governing authority (Constitution; FR-002).

### 2.4 Inventory principles (proposed)

A future authorized inventory **SHOULD** classify occurrences across categories such as:

| Category | Typical examples | Classification under this strategy |
| --- | --- | --- |
| Product / display names | README titles, UI shell titles, marketing copy | Product-facing vs technical separation |
| Repository / hosting | Local directory, GitHub repo name, remotes, badges | Repository/Git hosting boundary — separate ops authority |
| Packages / manifests | npm `afia`, Python modules, Cargo crates if any | Technical; deferred mass rename |
| Imports / namespaces | JS/TS/Python import paths | Technical; deferred |
| Executables / CLI | Binary names, shortcuts | Technical / packaging |
| Env vars / secrets names | Prefixes, config keys | Technical; may couple to auth (**ADR-11**) |
| Database / Supabase | Project, tables, functions, buckets, generated types | Supabase boundary — dedicated spec |
| Persistence / serialization | Artifact kinds, Run metadata, JSON type names, URIs | ADR-04 / ADR-05 authority |
| IPC / capabilities | Operation names, schema ids, PolicyDecision fields | ADR-02 / ADR-06 — distinguish labels vs stable IDs |
| UI / `afia-ui` | Routes, a11y labels, screenshots, localization | ADR-03 + copy inventory; not automatic rename |
| CI / containers / releases | Workflow names, image tags, installers | Packaging (**ADR-13**) / release boundary |
| External identity | Domains, registries, app stores, trademarks | External/legal boundary |
| Cross-product | Fehrest, DeepMed, commandF strings | Product-boundary protection |

**T024 does not execute** the inventory beyond what was required for contract recovery and this strategy draft.

### 2.5 Display-name versus machine-identifier boundary (proposed)

| Layer | Proposal |
| --- | --- |
| Product display name | **Fanatir** for new product-facing work |
| Durable machine identifiers | Remain stable unless a dedicated rename/migration specification authorizes change |
| Serialized types / hashes / provenance URIs | Must not be casually rewritten; risk of identity fork (**ADR-04** / **ADR-05**) |
| Human-facing UI copy | May be inventoried; mutation requires separate UI/copy authorization |
| Architectural concept names | Trusted Host, Artifact Store, Capability Gateway — not AFIA package names |

Changing a display string must **not** silently change durable identifiers.

### 2.6 Compatibility and shim principles (proposed)

Technical renaming, **when later authorized**, would require aliases/shims where impact analysis shows breakage (`spec.md` Q8). Principles only:

- compatibility may be **required** for a defined period when technical rename is authorized;
- permanent dual public identity without a removal gate is **undesirable**;
- shim ownership and removal gate belong to the dedicated rename/migration specification;
- **T024 / this draft create no aliases, redirects, or compatibility packages**.

T056 may later introduce strangler adapters **without** mass rename while ADR-12 remains deferred — that task is separately gated and **not** authorized here.

### 2.7 Migration-gate principles (proposed)

Propose phased gates for any future technical rename (principles — not an executed sequence):

1. **Naming inventory** complete and reviewed.
2. **Canonical technical forms** selected by founder/authorized decision (not invented here).
3. **Impact analysis** across packages, persistence, auth, Supabase, IPC, UI, CI, releases.
4. **Dedicated rename/migration specification** accepted.
5. **Compatibility period** defined (if required).
6. Controlled migration of documentation → UI labels → packages/namespaces → persistence → external services → release transition → legacy-name removal — each under its own authorization.
7. **Verification** before claiming rename complete.
8. **Rollback** assessment realistic for each layer.

ADR-12 may define gates; it does **not** execute them. Exact sequence may be deferred to the dedicated specification.

### 2.8 Repository and Git hosting boundary (proposed)

| Action | Under T024 / Spec 001 |
| --- | --- |
| Local directory rename | **Not authorized** |
| Git repository rename | **Not authorized** |
| GitHub repository rename | **Not authorized** — separate operational authorization |
| Remote URL / upstream changes | **Not authorized** |
| Branch rename, PR rewrite, badge/issue URL churn | Out of scope for this draft |
| Platform redirects | Do **not** replace migration verification |

Primary product repository identity remains planning evidence: `IamShehri/Fanatir` with package name currently `afia` (Constitution).

### 2.9 Package and namespace boundary (proposed)

- Current package name `afia` is **verified fact**, not endorsement of permanence.
- Mass package/import/namespace rename is **prohibited** under Spec 001 reconstitution.
- Breaking vs non-breaking vs aliasable changes are **unresolved** until dedicated specification.
- Lockfiles, generated code, and public API names require impact analysis before any later rename.
- **No package publication** is authorized by this draft.

### 2.10 Persistent data and serialization boundary (proposed)

Preserve **ADR-04** / **ADR-05** authority over Artifact, Revision, Run, Source, Relationship, and related durable forms.

Rename planning must treat as high-risk: stored type names, canonical URIs, provenance, audit records, export manifests, local paths, caches, Fehrest vault metadata references to Fanatir/AFIA strings.

**Propose:** display-name changes can remain separate from durable identifiers; persistence migration requires dedicated specification; **T024 performs no persistence mutation**.

### 2.11 UI and `afia-ui` boundary (proposed)

**ADR-03** owns `afia-ui` strangler migration. ADR-12 may note AFIA title remnants and package name `afia` as inventory evidence ([spec.md](../spec.md)).

ADR-12 must **not**: modify `afia-ui`; change window titles; rewrite routes; change assets; alter localization; or treat UI copy edits as technical package rename.

### 2.12 Authentication and session boundary (proposed)

Preserve **ADR-11**. Rename planning that would touch Supabase redirect URLs, OTP templates, auth routes, session storage keys, cookies, or local-storage keys requires **ADR-11 plus a dedicated specification**.

**T024 authorizes no auth/session behavior change.**

### 2.13 Supabase and remote collaboration boundary (proposed)

Preserve **ADR-07** and Decision C. Supabase project name, tables, functions, policies, buckets, URLs, email templates, RLS, and generated client types are **out of T024 mutation scope**. Remote naming does not control local authority. Schema migration requires a dedicated specification (including future Spec **002** where applicable).

### 2.14 Fehrest, DeepMed, and commandF boundaries (proposed)

| Product | Proposal |
| --- | --- |
| **Fehrest** | Remains named **Fehrest**; excluded from Fanatir package AFIA→Fanatir technical rename; separate repository/release identity (**ADR-08**). Embedded references to Fanatir/AFIA in exports/vault metadata need later coordination — not mutated here. Use exactly `Fehrest`. |
| **DeepMed** / DeepMed-AI | Remains named DeepMed; excluded; independent package/runtime identity (**ADR-09**). |
| **commandF** | Remains named **commandF**; Fanatir-owned capability (**ADR-10**); not renamed by ADR-12. Founder-accepted long-term FHIR workbench strategy does **not** alter ADR-10 and does **not** authorize commandF rename or implementation. |

Do **not** broaden rename across product boundaries.

### 2.15 Trusted Host, IPC, Artifact, and Run (proposed)

- Distinguish human-facing labels from machine-stable IPC operation names, capability identifiers, schema identifiers, PolicyDecision / AuthContext field names (**ADR-02** / **ADR-06** / **ADR-04**).
- Do **not** publish renamed IPC schemas under T024.
- Artifact/Run durable identity remains **ADR-05** / **ADR-04**; display rename ≠ provenance rewrite.

### 2.16 Release, packaging, and distribution (proposed)

Intersection with **ADR-13** (not authored): installers, bundle IDs, signing identities, update channels, release tags, container images, app-store listings, NOTICE/license text.

**Propose:** packaging rename is **not** Spec 001 mass-rename work under T024; no publish/rename of releases here.

### 2.17 External identity and legal boundary (proposed)

Later evaluation **may** be required (before implementation or public launch — not claimed required before this draft): trademarks; business registration; domains; social handles; app-store names; package-registry names; GitHub org/repo conflicts; geographic/language meaning.

| Classification for this draft | Status |
| --- | --- |
| Required before ADR-12 *draft* | **Canonically silent** — T024 does not mandate external research for drafting |
| Required before technical rename implementation | Likely **yes** where public collision risk exists — dedicated rename spec |
| Required before public launch | Likely **yes** for public names — later authorization |
| Availability / legal clearance | **Not claimed** by this draft |

Do **not** perform registrations or reservations.

### 2.18 Documentation and historical record (proposed)

- Retain original names in historical commits, ADR history, release notes, and provenance/audit records.
- Do **not** rewrite repository history to apply a new name.
- New docs use Fanatir; legacy AFIA materials remain evidence (“formerly” language may appear in later authorized docs — not required to rewrite earlier ADRs under T024).
- Do **not** modify earlier ADRs solely to retroactively apply a new technical name.

### 2.19 Verification principles (proposed)

When a dedicated rename specification authorizes work, verification **SHOULD** include (principles only):

- case-sensitive and case-insensitive repository searches;
- exact-word searches for source and target forms;
- manifest and lockfile inspection;
- generated artifact and binary/asset checks where relevant;
- external reference and stale-link review;
- build/test and package-import validation;
- persistence compatibility tests;
- redirect validation if hosting platforms provide redirects (insufficient alone).

**T024 verification method** (tasks.md): doc review; link from plan roadmap (plan already contains ADR-12 row — **not** modified by T024). No rename-specific builds or mutations are performed by this draft task beyond creating this Proposed file.

### 2.20 Versioning and breaking-change principles (proposed)

Potential version domains: product; application; packages; schemas; IPC; persisted data; export formats; plugin/adapter; Fehrest/DeepMed/commandF compatibility matrices.

Whether technical rename requires major version, deprecation window, migration tooling, or release notes remains **unresolved** until dedicated specification. Do **not** decide version bumps in this draft.

### 2.21 Rollback principles (proposed)

| Layer | Planning realism |
| --- | --- |
| Revert ADR-12 draft file | Realistic — tasks.md rollback for T024 |
| Restore display copy | Realistic if only docs/UI copy changed under later auth |
| Retain aliases temporarily | May be required during migration |
| Rollback package names | Possible with cost; lockfiles/consumers |
| Rollback repository rename | Hosting-dependent; not trivial |
| Rollback persistent-data migration | High risk; may be non-trivial or irreversible |
| External registrations | Often **not** trivially reversible |

Do **not** claim irreversible external renames are trivially reversible.

---

## 3. Alternatives considered

| Option | Rationale (evidence-based; not claimed as tested) |
| --- | --- |
| A. Mass-rename `afia*` during Spec 001 | Violates Q8 / FR-011 / tasks.md Deferred mass rename / research rejection |
| B. Treat Fanatir docs identity as already-complete technical rename | False; Constitution package timeline unresolved; dual identity verified |
| C. Choose and lock machine-readable Fanatir forms in this draft | Unauthorized invention; timeline unresolved |
| D. Rename Fehrest / DeepMed / commandF as part of ADR-12 | Violates product boundaries ADR-08/09/10 |
| E. Rename repository/GitHub under T024 | Out of scope; hosting/ops separate; no remotes/push |
| F. Create aliases/shims/redirects now | Violates T024 prohibited mutation; draft-only |
| **G (proposed).** Fanatir product identity now; defer mass technical rename; ADR defines inventory/shim/gate/verification/rollback principles only | Aligns Q8, plan ADR-12, Constitution, tasks.md T024 |

---

## 4. Consequences

### 4.1 Prospective benefits

- reduced reconstitution churn;
- preserved builds and imports under dual identity;
- clear separation of product identity vs technical migration;
- protection of Fehrest / DeepMed / commandF boundaries;
- protection of persistence, auth, and Supabase from casual rename;
- explicit dual-gate for later technical rename (Accepted ADR-12 path + dedicated specification);
- honest unresolved timeline for package forms.

### 4.2 Costs and risks

- continued AFIA/`afia` dual identity in inventory;
- documentation vs package naming inconsistency;
- deferred migration debt;
- risk of accidental “helpful” renames outside gates;
- unresolved machine-readable Fanatir forms;
- packaging and registry collision risk deferred;
- external legal/availability work deferred;
- ADR-12 absent from T030 Accepted and Reviewed lists — sequencing ambiguity if someone expects rename acceptance at R1 exit.

### 4.3 Planning-only rollback

Per [tasks.md](../tasks.md) T024: **Revert ADR file**. No production rename surface is created by this draft.

---

## 5. Non-goals

ADR-12 does **not**:

- rename any file, directory, repository, package, namespace, or import;
- rename executables, environment variables, database or Supabase objects;
- rename UI labels or persistent identifiers;
- rename Fehrest, DeepMed, or commandF;
- create migrations, aliases, redirects, or compatibility packages;
- select final Fanatir machine-readable identifier forms;
- resolve the Constitution package/brand rename timeline;
- modify `afia-ui`, auth/session behavior, or production code;
- modify Trusted Host, IPC schemas, Artifact Store, Fehrest, DeepMed, or commandF implementations;
- publish packages, tags, releases, or installers;
- perform trademark, domain, or registry reservations;
- begin R2 or R3;
- execute T025, T027, T028, T029, or T030.

---

## 6. Relationship to other ADRs

| ADR | Status in repo | Relationship |
| --- | --- | --- |
| ADR-15 | Proposed | Language/runtime authority; not rename |
| ADR-01 | Proposed | Platform composition; packaging names deferred |
| ADR-02 | Proposed | Trusted Host / IPC labels vs stable identifiers |
| ADR-03 | Proposed | `afia-ui` strangler; UI copy ≠ package mass rename |
| ADR-04 | Proposed | Shared primitives; durable identifier stability |
| ADR-05 | Proposed | Artifact/Run persistence; no casual identity rewrite |
| ADR-06 | Proposed | Worker/IPC contracts; no renamed schemas under T024 |
| ADR-07 | Proposed | Supabase/local-first; no remote rename under T024 |
| ADR-08 | Proposed | Fehrest identity protected |
| ADR-09 | Proposed | DeepMed identity protected |
| ADR-10 | Proposed | commandF identity protected; strategy unchanged |
| ADR-11 | Proposed | Auth/session keys and routes; dual-gate with dedicated spec |
| ADR-13 | Not authored | Packaging/distribution naming overlap |
| ADR-14 | Not authored | Security/classification; not rename |

ADR-12 decides only the **Technical AFIA-to-Fanatir Rename Strategy** (principles and gates).

---

## 7. Gates and authority

```text
T024 produces a Proposed ADR-12 draft only.
Tier C — normal verification is required.
Passing Tier C review is not architecture acceptance.
tasks.md founder-acceptance field for T024 is: No — draft only.
Founder draft acceptance is not required by the T024 contract for the draft work product.
Founder-ratified Q8 product identity does not authorize mass technical rename.
Package/brand rename timeline remains an unresolved founder decision.
Technical rename requires inventory, impact analysis, Accepted architecture path, dedicated rename/migration specification, shims where required, and verification.
ADR-12 alone does not authorize rename or implementation.
T030 remains incomplete.
T030 mandatory Accepted set is ADR-15/01/02/06 only — ADR-12 is not in that set.
T030 explicit Reviewed-or-Accepted group lists ADR-04/05/07/08/09/10/14 — ADR-12 is not listed there.
Plan: ADR-12 deferred after Alpha unless accelerated.
T056 may proceed with ADR-12 deferred; mass afia rename remains prohibited.
T031+ and R2 remain separately gated.
R3 and production rename work remain separately gated.
```

| Gate | Meaning for ADR-12 |
| --- | --- |
| T024 | Draft authoring (this task) |
| Tier C | Mandatory independent review |
| Founder draft acceptance | Not required by tasks.md for T024 work product |
| Founder package-form / timeline selection | Required before technical rename execution; not required to draft this strategy |
| Dedicated rename/migration specification | Required before technical rename implementation |
| T030 | R1 gate; ADR-12 not in mandatory Accepted set; not in listed Reviewed-or-Accepted group |
| T056 | Compatibility adapters without mass rename; ADR-12 deferred |
| T031+ / R2 / R3 | Separately gated; not authorized here |

---

## 8. Validation and acceptance plan

```text
T024 completion produces a draft for Tier C review.
Architecture acceptance is not performed by T024.
Rename implementation is not authorized by T024.
```

Planning reviews **may** include: Q8 fidelity; deferred mass rename; product vs technical separation; Fehrest/DeepMed/commandF protection; ADR-04/05/07/11 non-theft; unresolved timeline honesty; no invented machine forms; Tier C independent review ([tasks.md](../tasks.md) T024).

Verification method from tasks.md: doc review; link from plan roadmap (plan already contains ADR-12 row — **not** modified by T024).

---

## 9. Unresolved questions

T024 does **not** resolve these. Attempted resolution: **NO**.

| ID | Question | May remain open in draft review? | Must resolve before technical rename implementation? | Belongs elsewhere? |
| --- | --- | --- | --- | --- |
| U-ADR12-1 | Exact package-safe Fanatir identifier form(s) | YES | YES | Founder decision; dedicated rename spec |
| U-ADR12-2 | Namespace-safe / import-path forms | YES | YES | Dedicated rename spec |
| U-ADR12-3 | CLI / executable forms | YES | YES | Dedicated rename spec; ADR-13 |
| U-ADR12-4 | Environment-variable prefix forms | YES | YES | Dedicated rename spec; ADR-11 if auth-coupled |
| U-ADR12-5 | Bundle ID / application identifier forms | YES | YES | ADR-13; dedicated rename spec |
| U-ADR12-6 | Domain / URL forms | YES | YES | External/legal; dedicated rename spec |
| U-ADR12-7 | Package/brand rename timeline | YES | YES | Constitution unresolved; founder |
| U-ADR12-8 | Whether/when to accelerate ADR-12 before/after Alpha | YES | YES before acceleration claims | Plan sequencing; founder |
| U-ADR12-9 | Full occurrence inventory | YES | YES | Dedicated rename spec execution |
| U-ADR12-10 | Compatibility alias duration and removal gate | YES | YES | Dedicated rename spec |
| U-ADR12-11 | Whether aliases are mandatory vs prohibited in some layers | YES | YES | Dedicated rename spec |
| U-ADR12-12 | GitHub / hosting rename operations | YES | YES before hosting rename | Separate ops authorization |
| U-ADR12-13 | Persistence/serialization migration approach | YES | YES | ADR-04/05; dedicated migration spec |
| U-ADR12-14 | Auth/session key and redirect migration | YES | YES | ADR-11 + dedicated auth/rename specs |
| U-ADR12-15 | Supabase object rename approach | YES | YES | ADR-07; Spec 002; dedicated spec |
| U-ADR12-16 | UI copy vs package rename sequencing | YES | YES | ADR-03; dedicated specs |
| U-ADR12-17 | Fehrest export/vault string compatibility | YES | YES if Fehrest embeds Fanatir/AFIA refs | ADR-08 |
| U-ADR12-18 | DeepMed client string compatibility | YES | YES if DeepMed embeds Fanatir/AFIA refs | ADR-09 |
| U-ADR12-19 | commandF package/executable future naming | YES | YES if changing commandF machine names | ADR-10; not assumed |
| U-ADR12-20 | IPC schema identifier rename policy | YES | YES | ADR-02/06; dedicated spec |
| U-ADR12-21 | Versioning / major-bump policy for rename | YES | YES | Dedicated rename / release specs |
| U-ADR12-22 | Trademark / domain / registry clearance | YES | YES before public launch/publish | External/legal |
| U-ADR12-23 | Exact acceptance state needed before T056 completes | YES | YES before claiming T056 rename-related done | tasks.md T056 (ADR-12 deferred) |
| U-ADR12-24 | Whether ADR-12 must later join a Reviewed/Accepted list | YES | YES before stage claims that require it | Plan vs tasks reconciliation |

---

## 10. Security and privacy claim language

This draft **proposes** rename-strategy principles only. It does **not** claim that dual AFIA/Fanatir identity is cleared for production branding, that technical identifiers are safe to change, that external names are available, or that any compliance/trademark clearance exists. Inventory evidence is not authorization to mutate.

PHI posture: rename planning must not transmit PHI; must not casually rewrite audit/provenance identifiers; claim language remains non-certifying.

---

## 11. Document control

| Item | Value |
| --- | --- |
| Created by | T024 draft authoring |
| Status | Proposed |
| Supersedes | None |
| Superseded by | None |
| Next expected actions | Tier C independent review; keep mass technical rename deferred under Spec 001; dedicated rename/migration specification before any technical rename; T056 only under its own gate with ADR-12 deferred |

```text
End of ADR-12 draft.
Status: Proposed
Implementation authorization: NO
Rename / migration authorization: NO
```
