# ADR-13 — First Vertical-Slice Packaging

| Field | Value |
| --- | --- |
| **ADR** | ADR-13 |
| **Title** | First Vertical-Slice Packaging |
| **Status** | **Proposed** (draft only) |
| **Task origin** | T025 |
| **Acceptance / review posture** | Tier C — normal verification after drafting. **T030** mandatory Accepted set is ADR-15/01/02/06 only; ADR-13 is **not** listed in T030’s explicit Reviewed-or-Accepted group (04/05/07/08/09/10/14). Downstream **T039** requires an **ADR-13 draft** for an unsigned internal packaging spike; **T060** depends on T025 (+T026, T055); **T061** gates on ADR-13, Decision D, and T060 for signed Windows installer work. Exact later acceptance-state expectations for ADR-13 remain explicitly ambiguous where draft-for-spike vs R5 gate wording differs. |
| **Feature** | `001-fanatir-repository-and-architecture-reconstitution` |
| **Required gate (tasks.md)** | P4 staged hybrid |
| **Planning dependency (plan.md)** | P4, ADR-01–02, **ADR-15** — First vertical-slice packaging (staged hybrid); early Rust host + supervised Python; no Go required for Alpha |
| **Dependencies (tasks.md)** | T012, T013, T014 |
| **Constraining drafts** | [ADR-15](./ADR-15-rust-first-polyglot-runtime.md) (Proposed); [ADR-01](./ADR-01-platform-and-desktop-composition-tauri-2.md) (Proposed); [ADR-02](./ADR-02-rust-trusted-host-boundary.md) (Proposed); [ADR-03](./ADR-03-afia-ui-strangler-migration.md) (Proposed); [ADR-04](./ADR-04-shared-primitive-ownership-and-versioning.md) (Proposed); [ADR-05](./ADR-05-artifact-revision-run-storage.md) (Proposed); [ADR-06](./ADR-06-worker-and-ipc-contracts.md) (Proposed); [ADR-07](./ADR-07-supabase-adapter-and-local-first-boundary.md) (Proposed); [ADR-08](./ADR-08-fehrest-integration-and-release-model.md) (Proposed); [ADR-09](./ADR-09-deepmed-integration-and-openmed-runtime-fork-boundary.md) (Proposed); [ADR-10](./ADR-10-commandf-ownership-and-process-boundary.md) (Proposed); [ADR-11](./ADR-11-auth-and-session-preservation.md) (Proposed); [ADR-12](./ADR-12-technical-afia-to-fanatir-rename-strategy.md) (Proposed) |
| **Planning evidence** | [spec.md](../spec.md) FR-015 / Alpha installable Windows / packaging inventory; [plan.md](../plan.md) ADR-13 row / Decision D / R5 packaging; [research.md](../research.md) P4 / R4 Trusted Host packaging / R15 signing custody; Constitution Windows-first Tauri target ([constitution.md](../../../.specify/memory/constitution.md)); [tasks.md](../tasks.md) T025 / T012 / T013 / T014 / T030 / T039 / T060 / T061; [data-model.md](../data-model.md); [shared-primitives.md](../contracts/shared-primitives.md); [trusted-host-ipc.md](../contracts/trusted-host-ipc.md); [worker-runtime.md](../contracts/worker-runtime.md) |
| **Architecture authority of this file** | **NO** — until a valid stage-gate acceptance action records Accepted |
| **Implementation authorization** | **NO** |
| **Build / package / sign / publish authorization** | **NO** — this draft defines packaging strategy principles only |

```text
This document is a planning draft and is not accepted architecture authority.
```

```text
Status: Proposed
Draft ADR ≠ architecture acceptance
T025 completion ≠ T030 acceptance
Passing Tier C review ≠ Accepted
tasks.md founder-acceptance field for T025: No — draft only
Founder acceptance of a draft work product ≠ architecture acceptance
P4 staged hybrid ≠ permission to build or package
Decision D signing posture ≠ configured signing keys
ADR-15 / ADR-01 / ADR-02 / ADR-03 / ADR-04 / ADR-05 / ADR-06 / ADR-07 / ADR-08 / ADR-09 / ADR-10 / ADR-11 / ADR-12 remain Proposed and non-authoritative
```

```text
This draft is not:
- permission to build Fanatir
- permission to create an installer or archive
- permission to package workers or bundle runtimes
- permission to sign, notarize, or create keys
- permission to create checksums, SBOMs, or attestations
- permission to publish packages or releases
- permission to create tags or GitHub Releases
- permission to upload to registries or app stores
- permission to modify CI or release workflows
- permission to modify lockfiles or dependencies
- permission to modify source, UI, or afia-ui
- permission to modify Supabase or authentication
- permission to migrate persistent data
- permission to begin R2 or R3
```

```text
No production implementation is authorized by this draft.
```

This draft **must not** be used as justification to: build; package; install; sign; notarize; publish; tag; create GitHub Releases; modify CI; modify lockfiles; bundle Fehrest, DeepMed, commandF validator/terminology content, models, or external tools; modify [`afia-ui`](../../../afia-ui); change authentication or sessions; migrate persistent data; begin R2 (`T031+`) or R3; or treat Proposed ADRs as Accepted.

---

## 1. Context and problem

### 1.1 Assigned architectural question

**ADR-13 proposes** a **first vertical-slice packaging** strategy for Spec **001**: a **staged-hybrid** composition of **Rust Trusted Host** + bundled **`afia-ui` production assets** + **supervised non-Rust workers**, with **Windows-first** as the Alpha packaging acceptance vehicle, **unsigned builds internal/non-distributable**, and **signing custody** required before later Alpha distribution — defining principles and gates only.

It does **not** build, package, sign, publish, select an exact Windows installer format, create keys, modify CI or locks, rename `afia*` identifiers (**ADR-12**), redesign UI (**ADR-03**), change auth/session (**ADR-11**), redefine Supabase (**ADR-07**), or authorize R2/R3.

### 1.2 Why packaging architecture before an installer

Founder Alpha requires an **installable Windows desktop application** ([spec.md](../spec.md)). P4 resolves staged hybrid packaging ([research.md](../research.md)). Decision D requires signing custody before R5 distribution ([plan.md](../plan.md); [research.md](../research.md) R15). Without an explicit packaging boundary, planning identifies these **risks** (prospective — not claimed as currently measured production failures unless separately evidenced):

| Risk | Why it matters |
| --- | --- |
| Dev build confused with installable Alpha | False Alpha evidence (Vite-only insufficient) |
| Python / non-Rust code in-process with Trusted Host | ADR-15 / worker isolation violation |
| Accidental dependency on ambient system interpreters | Non-reproducible, insecure Alpha |
| Silently downloading components at install/first launch | Supply-chain and offline failure |
| Bundling unreviewed third-party content | License / redistribution / PHI risk |
| Application package embeds user data or Artifact Store | Persistence and uninstall damage (**ADR-05**) |
| Unsigned distributable artifacts | Decision D violation |
| Implicit AFIA→Fanatir technical rename in package metadata | **ADR-12** violation |
| Cloud access mandatory for install or launch | Decision C / local-first violation |
| Web fallback treated as desktop Alpha evidence | P4 violation |
| Update/signing behavior without custody | Key leakage / untrusted updates |

### 1.3 Accepted program and planning constraints

| Constraint | Source | Treatment |
| --- | --- | --- |
| Staged hybrid: R2 minimal Tauri + `afia-ui`; R3 host IPC + browser fallback ≠ Alpha packaging evidence; R5 Windows packaged build = Alpha acceptance vehicle | P4 — [research.md](../research.md) | Binding packaging path; **not** ADR-13 acceptance |
| Windows-first Alpha installer via Tauri bundler; Decision D custody before distribution; unsigned = internal only | [plan.md](../plan.md) | Planning; **not** implementation |
| Founder is signing authority owner; custody before R5 distribution | Decision D | Binding distribution gate; **not** ADR-13 acceptance |
| Windows-first ≠ Windows-only; local-first ≠ forbidding optional collaboration | FR-015 — [spec.md](../spec.md) | Binding |
| Installable Windows desktop application required for Alpha | [spec.md](../spec.md) Alpha capabilities | Binding Alpha vehicle |
| macOS/Linux packaged releases preview/excluded for Alpha | [spec.md](../spec.md) | Binding Alpha scope |
| No Go required for Founder Alpha | ADR-15 / plan ADR-13 row | Binding |
| T025: ADR draft only; P4 gate; draft ≠ accepted; no production code | [tasks.md](../tasks.md) T025 | Binding for this task |

### 1.4 Naming

| Name | Role in this ADR |
| --- | --- |
| **Fanatir** | Product-facing identity for packaging strategy and Alpha vehicle |
| **`afia` / `afia-ui`** | Current technical package/shell identifiers (**ADR-12** / **ADR-03**) — not silently renamed |
| **Trusted Host** | Rust privileged core (**ADR-02** / **ADR-15**) |
| **Tauri** | Provisional desktop composition / bundler direction (**ADR-01**) |
| **Artifact Store** | Local content authority (**ADR-05**) — not embedded as a second authority in packages |
| **Fehrest** / **DeepMed** / **commandF** | Separate product/capability packaging boundaries |

Never confuse Fanatir display identity with decided machine package names; never treat unsigned internal builds as distributable Alpha.

### 1.5 What this draft is not

This file is **not**: a build; an installer; a selected MSI/NSIS/MSIX/ZIP format; configured signing; a release; CI mutation; lockfile mutation; third-party bundling approval; or R2/R3 authorization.

---

## 2. Decision (proposed)

### 2.1 Core packaging decision

**Propose** that:

1. Fanatir uses a **staged-hybrid** packaging strategy (P4).
2. Early packaged composition centers on a **Rust Trusted Host**.
3. **`afia-ui` production assets** may be consumed by the host without changing UI architecture.
4. Non-Rust capabilities remain **supervised workers or sidecars**.
5. Workers are **not** in-process trusted peers.
6. **Windows-first** is the Alpha packaging acceptance vehicle.
7. The **exact Windows installer format** remains **unresolved**.
8. **Unsigned builds** are **internal and non-distributable**.
9. **Signing custody** must exist before later Alpha distribution (Decision D).
10. **macOS/Linux** packaging is future/preview for the Alpha stage, not the acceptance vehicle.
11. **Web/browser** operation is a compatibility fallback, not Alpha packaging evidence.
12. **No Go** runtime is required for Alpha.
13. Exact package and artifact **machine names** remain constrained by **ADR-12** (unresolved).
14. Packaging must preserve **local-first** operation and authoritative local data.
15. Packaging must **not** silently bundle Fehrest, DeepMed, commandF dependencies, models, validator JARs, terminology content, or external tools.
16. Build, packaging, signing, publishing, and release execution require **later tasks and authority**.

### 2.2 Staged-hybrid composition (proposed)

Planning stages (map across R2 / R3 / R5 — **not** executed by T025):

1. early Rust Trusted Host shell;
2. consume current `afia-ui` production assets (prod: bundled dist; dev: localhost URL per P4);
3. introduce supervised workers with explicit version and capability declaration;
4. later installer creation (R5 path);
5. later signing after Decision D custody (T060);
6. later release publication only under separate authorization.

**T025 does not claim** any packaged build exists.

### 2.3 Core package boundary (proposed)

At planning level, distinguish:

| Category | Proposal |
| --- | --- |
| Trusted Host executable | Core package candidate |
| Bundled UI assets | Consumed from `afia-ui` production build — not UI redesign |
| Configuration / static metadata | Explicit and reviewable |
| Worker manifests | Declare identity, pin, capabilities — not ambient trust |
| Optional sidecars | Supervised; not in-process peers |
| User data / Artifact Store / local DBs | **Outside** application package; preserved across upgrade |
| Caches / logs | Separable from authoritative content |
| Secrets | **Never** packaged as production credentials |

**Propose:** user data must not be embedded in application packages; production secrets must never be packaged; package contents must be explicit and reviewable; package composition must not create a second Artifact Store authority (**ADR-05**).

Do **not** publish a final directory layout.

### 2.4 Windows-first Alpha (proposed)

- Windows is the Alpha packaging **acceptance vehicle**.
- A future packaged Windows build must be installable and founder-controlled ([spec.md](../spec.md)).
- Exact format remains **unresolved**.
- MSI, NSIS, MSIX, portable ZIP, and Tauri bundler defaults are **not selected** by this ADR.
- Provisional direction: Windows-first Alpha installer **via Tauri bundler** ([plan.md](../plan.md)) — format choice deferred.
- Unsigned internal builds may support spikes only (**T039**).
- Public or external distribution requires signing and later gates (**T060** / **T061**).

Do **not** claim Windows packaging is implemented.

### 2.5 macOS and Linux (proposed)

- Architecture should not intentionally prevent macOS/Linux (FR-015; Windows-first ≠ Windows-only).
- Packaged macOS/Linux releases are **not** Alpha acceptance evidence.
- Runtime and accessibility proof remain later work.
- DMG, PKG, DEB, RPM, AppImage, Flatpak, Snap, and similar formats remain **unresolved/future**.
- No platform-support claim exceeds canonical evidence.

### 2.6 Browser and server boundary (proposed)

- Browser fallback is compatibility/development support (P4).
- It does **not** satisfy desktop packaging acceptance; Vite-only is insufficient for Alpha/R2/R5 exit evidence ([tasks.md](../tasks.md)).
- Server, container, and CLI distribution are **outside** the selected Alpha vehicle.
- No hosted deployment is authorized by this draft.

### 2.7 `afia-ui` boundary (proposed)

Preserve **ADR-03** and **ADR-12**:

- `afia-ui` remains the current technical shell/path;
- packaging may later consume its production assets;
- packaging does not redesign routes, components, auth, accessibility, or UI state;
- packaging does not rename `afia-ui`;
- exact UI-build integration remains an implementation-spec concern.

### 2.8 Trusted Host and ADR-15 (proposed)

Preserve **ADR-15** / **ADR-02**:

- Rust owns the Trusted Host;
- privileged lifecycle and signing/update boundaries belong to trusted Rust-controlled architecture;
- non-Rust code runs in supervised processes;
- workers are restartable and replaceable;
- workers receive bounded capabilities;
- workers do not inherit ambient filesystem, network, or secret authority;
- packaging must not convert worker isolation into an in-process shortcut.

Do **not** publish final worker launch schemas.

### 2.9 Worker and sidecar packaging (proposed)

Future requirements (principles only): worker identity; version pin; protocol version; runtime requirement; executable/interpreter discovery; startup; shutdown; health checks; timeout; cancellation; resource limits; crash recovery; integrity verification; license metadata; rollback.

Exact sidecar layout and bundling choices remain **unresolved**. **T040** may spike externalBin feasibility under separate gate — not authorized here.

### 2.10 Dependency and locking (proposed)

Address Cargo, JavaScript, Python, Java artifacts, external binaries, models, FHIR packages, terminology content:

- immutable versions;
- integrity hashes;
- deterministic manifests;
- no floating `latest`;
- no unattended download during packaging;
- provenance for every included dependency;
- separate pin domains for application, worker, runtime, model, validator, FHIR package, and terminology content.

Do **not** modify locks or select new dependencies under T025.

### 2.11 Reproducible builds (proposed)

Future requirements may include: source-commit linkage; pinned toolchains; isolated build environments; deterministic file ordering; normalized timestamps where feasible; checksums; build manifests; builder identity; dependency inventory; reproducibility comparison.

**T025 performs no build.** Full reproducibility verification belongs to later tasks such as **T064**. Do **not** claim byte-for-byte reproducibility is achieved.

### 2.12 Signing and Decision D (proposed)

- Founder owns signing authority.
- Unsigned artifacts are **internal/non-distributable**.
- Signing custody must exist before later Alpha distribution.
- Signing keys must **not** be stored in the repository or ordinary CI variables.
- No certificate provider, CA, HSM, cloud signing service, or hardware token is selected.
- Key rotation, revocation, recovery, and compromised-key response remain **unresolved**.
- App-signing and updater-signing separation remains **unresolved** where applicable.

Do **not** create keys or certificates.

### 2.13 Update boundary (proposed)

Future topics: signed update manifests; Alpha/stable channels; manual versus automatic updates; offline updates; update deferral; rollback; interrupted updates; downgrade compatibility.

**Propose:** updater design is not implemented; Tauri updater requires trusted signing ([research.md](../research.md)); forced updates are **not** selected; exact channel model remains **unresolved**.

### 2.14 Supply-chain security (proposed)

Future controls may include: dependency inventory; SBOM; vulnerability scanning; source verification; hashes; trusted download origins; build isolation; secret scanning; malicious-package prevention; release provenance; signature verification; quarantine of invalid artifacts; rollback on compromise.

Exact tools and formats remain **unresolved**. Do **not** claim controls are operational. Overlap with **ADR-14** (not authored) for PHI/secrets/distribution controls.

### 2.15 Licensing and redistribution (proposed)

Every included component requires per-version license and NOTICE verification. Terminology content may have separate redistribution restrictions. commandF’s future HL7 validator and terminology assets are **not** automatically included. Fehrest and DeepMed retain separate licensing/release boundaries. **No third-party content is approved for bundling by ADR-13 alone.**

Do **not** claim legal clearance.

### 2.16 Secrets and configuration (proposed)

Packages must not contain production Supabase keys, provider credentials, signing keys, API keys, or PHI. Environment examples are not production secrets. Runtime configuration and secure storage remain separately governed (**ADR-02** / **ADR-11**). Ordinary package files, logs, UI assets, worker manifests, or IPC payloads must not expose secrets. Update credentials require separate custody.

Do **not** select a secure-storage backend.

### 2.17 Persistent data and upgrade safety (proposed)

Preserve **ADR-04** / **ADR-05**:

- application files versus data directories;
- Artifact Store and local database preservation;
- schema compatibility; interrupted update; rollback; downgrade; backups; cache invalidation; worker compatibility.

**Propose:** installer replacement must not imply data deletion; uninstall must distinguish application removal from user-data removal; migrations require separate specifications; **T025 authorizes no migration**.

Do **not** define final filesystem paths.

### 2.18 Installation and uninstallation (proposed)

Future concerns: per-user versus machine-wide; privileges; install/data/cache/log directories; shortcuts; file associations; protocol handlers; repair; uninstall; data preservation; secret cleanup.

Exact choices remain **unresolved**. Do **not** install or uninstall anything under T025.

### 2.19 Offline and local-first distribution (proposed)

Preserve Decision C / **ADR-07**:

- core desktop Alpha must not require cloud availability;
- first launch should not silently require network where dependencies are already present;
- optional cloud services are not mandatory package dependencies;
- downloaded-on-demand components require explicit authorization and integrity checks;
- offline installer expectations remain to be specified;
- no cloud account is required merely to validate package installation unless a later auth policy requires it.

Do **not** invent offline-auth behavior (**ADR-11**).

### 2.20 Application and artifact naming (proposed)

Preserve **ADR-12**:

- display-facing artifacts may use Fanatir;
- exact machine artifact names remain unresolved;
- current `afia` technical identifiers must not be silently renamed;
- installer, executable, package, bundle, directory, and registry names require later decisions;
- artifact names may eventually include version, platform, architecture, channel, and hashes — **no final convention selected**.

Do **not** invent artifact names (including `fanatir-ui`).

### 2.21 Release channels and versioning (proposed)

Possible domains (evidence-limited): internal; founder-controlled Alpha; later public/Beta/stable/nightly/enterprise/offline — only what later authorization supports.

Keep separate: application; installer; worker; protocol; schema; updater; package; model/runtime versions.

Do **not** create tags or versions under T025.

### 2.22 Release and publication boundary (proposed)

- T025 produces **no** release.
- GitHub Releases belong to later release engineering.
- App stores and package registries are **not** selected.
- No artifact upload, release notes generation, or public distribution under T025.
- **R5** later owns signed Windows installer acceptance (**T061+**).

### 2.23 Build and CI boundary (proposed)

Future CI may include: clean builds; platform matrix; installer jobs; signing jobs; protected environments; retention; checksums; SBOM; provenance; secret scanning; release approval.

**Propose:** existing CI may be scaffold/inventory only ([spec.md](../spec.md)); **T025 modifies no workflow**; signing secrets must not be available to ordinary untrusted jobs; exact release workflow remains later work.

### 2.24 UI and accessibility (proposed)

Preserve **ADR-03**: installer and updater UI must eventually support keyboard access; understandable errors; status not color-alone; screen-reader compatibility; accessible license/privacy disclosures; Arabic/RTL and localization remain later packaging/UI requirements.

Do **not** modify UI or assert these are implemented.

### 2.25 Authentication and Supabase (proposed)

Preserve **ADR-07** / **ADR-11**: packaging must not modify authentication behavior; no Supabase project is selected by packaging; redirect URLs, callbacks, protocol registrations, origins, and env configuration remain later concerns; packages must not embed production credentials; offline/local installation must not silently transfer authority to remote rows.

### 2.26 Fehrest, DeepMed, and commandF (proposed)

| Boundary | Proposal |
| --- | --- |
| **Fehrest (ADR-08)** | Separate product/repository/release; **not** included in Fanatir Alpha installer by default; future integration via versioned contracts; no Fehrest package/vault/asset bundled under T025. Use exactly `Fehrest`. |
| **DeepMed (ADR-09)** | Separate packaging/release; models/runtimes not silently bundled; licenses/sizes need independent review; optional supervised integration ≠ core installer content; no DeepMed download/package under T025. |
| **commandF (ADR-10)** | Separate supervised capability; no validator JAR, Java runtime, FHIR package, terminology pack, or network terminology service bundled/configured under T025; future bundling needs per-pin license/NOTICE/size/update/security/provenance review. Founder-accepted FHIR workbench strategy does **not** alter ADR-10 and does **not** authorize packaging or implementation. |

### 2.27 Rollback (proposed)

| Layer | Status |
| --- | --- |
| Revert ADR-13 draft | Available — tasks.md T025 rollback |
| Failed build cleanup | Later requirement |
| Installer / release / updater / worker-pin rollback | Later R5 planning where supported |
| Database/schema rollback | Dedicated migration specs |
| Compromised-signing rollback | Decision D / custody docs |

Only **draft rollback** is currently available via this task. Prior installer/tag and sidecar pin downgrade belong to later R5 planning ([plan.md](../plan.md)) — not claimed implemented.

### 2.28 Verification strategy (proposed)

Later verification categories may include: manifest validation; clean build; reproducibility comparison; installer creation; clean-machine installation; upgrade/downgrade/uninstall; application-data preservation; offline first launch; worker startup/failure; protocol compatibility; signature/update/checksum/SBOM/license verification; secret and vulnerability scanning; platform smoke tests; accessibility; rollback rehearsal.

**Do not claim any were executed under T025.** T025 verification method: doc review; link from plan roadmap (plan already contains ADR-13 row — **not** modified by T025).

---

## 3. Alternatives considered

| Option | Rationale (evidence-based; not claimed as tested) |
| --- | --- |
| A. Distribute development builds as Alpha | Violates installable Windows Alpha / Vite-only insufficient |
| B. Package all runtimes and products into one monolith | Violates Fehrest/DeepMed/commandF boundaries; license/size risk |
| C. Require users to install every runtime manually | Weak Alpha UX; ambient interpreter risk |
| D. Web-only Alpha | Violates Q3 / P4 / desktop-first |
| E. Container-first Alpha | Outside selected Alpha vehicle |
| F. Go-dependent Alpha packaging | Violates no-Go-for-Alpha |
| **G (proposed).** Staged hybrid; Windows-first installer vehicle; supervised workers; unsigned=internal; signing custody before distribution | Aligns P4, Decision D, ADR-15, plan ADR-13 |

---

## 4. Consequences

### 4.1 Prospective benefits

- installable Alpha path;
- trusted Rust lifecycle ownership;
- worker isolation and replaceable sidecars;
- explicit signing gate;
- reproducibility direction;
- offline/local-first posture;
- packaging clarity;
- preserved product boundaries;
- no forced Go dependency;
- controlled third-party redistribution.

### 4.2 Costs and risks

- Windows-first operational focus;
- cross-platform packaging debt;
- multi-runtime complexity;
- worker/runtime discovery;
- installer size;
- signing cost and custody;
- update complexity;
- license review burden;
- reproducibility difficulty;
- support burden;
- unresolved artifact names (**ADR-12**);
- dual AFIA/Fanatir technical identity;
- model and terminology redistribution risk;
- packaging drift risk;
- persistence-upgrade risk;
- T039 draft vs T061 acceptance-state ambiguity for ADR-13.

### 4.3 Planning-only rollback

Per [tasks.md](../tasks.md) T025: **Revert ADR file**. No production packaging surface is created by this draft.

---

## 5. Non-goals

ADR-13 does **not**:

- build Fanatir;
- create an installer;
- select MSI, NSIS, MSIX, ZIP, or another exact format;
- sign or notarize;
- create keys;
- publish packages or releases;
- create tags;
- create GitHub Releases;
- modify CI;
- modify locks or dependencies;
- bundle Fehrest;
- bundle DeepMed;
- bundle commandF validator or terminology content;
- bundle models;
- modify UI;
- modify auth or Supabase;
- migrate persistent data;
- rename `afia` identifiers;
- begin R2 or R3;
- execute T026, T027, T028, T029, or T030.

---

## 6. Relationship to other ADRs

| ADR | Status in repo | Relationship |
| --- | --- | --- |
| ADR-15 | Proposed | Trusted Rust core; supervised workers; updater/signing language authority |
| ADR-01 | Proposed | Platform / Tauri composition |
| ADR-02 | Proposed | Trusted Host; privileged lifecycle; signing/security where applicable |
| ADR-03 | Proposed | UI packaging consumer; no redesign |
| ADR-04 | Proposed | Schemas / shared semantics stability |
| ADR-05 | Proposed | Persistent-data stability; Artifact Store authority |
| ADR-06 | Proposed | Worker/IPC compatibility |
| ADR-07 | Proposed | Supabase/local-first; no mandatory cloud package dependency |
| ADR-08 | Proposed | Fehrest separate packaging |
| ADR-09 | Proposed | DeepMed separate packaging |
| ADR-10 | Proposed | commandF separate; strategy unchanged |
| ADR-11 | Proposed | Auth/session preservation; no packaging behavior change |
| ADR-12 | Proposed | Naming; no silent technical rename |
| ADR-14 | Not authored | PHI, secrets, security, distribution controls |

ADR-13 decides only **First Vertical-Slice Packaging**.

---

## 7. Gates and authority

```text
T025 produces a Proposed ADR-13 draft only.
Tier C — normal verification is required.
Passing Tier C review is not architecture acceptance.
tasks.md founder-acceptance field for T025 is: No — draft only.
Founder draft acceptance is not required by the T025 contract for the draft work product.
ADR-13 alone does not authorize build, package, sign, publish, tag, or CI mutation.
T030 remains incomplete.
T030 mandatory Accepted set is ADR-15/01/02/06 only — ADR-13 is not in that set.
T030 explicit Reviewed-or-Accepted group lists ADR-04/05/07/08/09/10/14 — ADR-13 is not listed there.
T039 requires an ADR-13 draft for an unsigned internal spike; unsigned artifacts are non-distributable.
T060 depends on T025, T026, and T055; Decision D custody before distribution.
T061 requires ADR-13, Decision D, and T060 for signed Windows installer work.
Exact later acceptance state of ADR-13 (draft-for-spike vs Accepted-for-distribution) remains an explicit ambiguity.
T031+ and R2 remain separately gated.
R3 and production packaging remain separately gated.
Build/package/sign/publish/tag/release/CI/registry/app-store work requires separate authorization.
```

| Gate | Meaning for ADR-13 |
| --- | --- |
| T025 | Draft authoring (this task) |
| Tier C | Mandatory independent review |
| Founder draft acceptance | Not required by tasks.md for T025 work product |
| T030 | R1 gate; ADR-13 not in mandatory Accepted or listed Reviewed-or-Accepted sets |
| T039 | ADR-13 draft; Decision D unsigned=internal |
| T060 | Signing custody; depends on T025 (+T026, T055) |
| T061 | Signed Windows installer; ADR-13 + Decision D + T060 |
| T031+ / R2 / R3 | Separately gated; not authorized here |

---

## 8. Validation and acceptance plan

```text
T025 completion produces a draft for Tier C review.
Architecture acceptance is not performed by T025.
Build/package/sign/publish are not authorized by T025.
```

Planning reviews **may** include: P4 fidelity; Windows-first ≠ Windows-only; unsigned=non-distributable; Decision D; ADR-15 worker isolation; ADR-12 naming; Fehrest/DeepMed/commandF non-bundling; T030/T039/T060/T061 ambiguity honesty; Tier C independent review ([tasks.md](../tasks.md) T025).

Verification method from tasks.md: doc review; link from plan roadmap (plan already contains ADR-13 row — **not** modified by T025).

---

## 9. Unresolved questions

T025 does **not** resolve these. Attempted resolution: **NO**.

| ID | Question | May remain open in draft review? | Must resolve before implementation? | Belongs elsewhere? |
| --- | --- | --- | --- | --- |
| U-ADR13-1 | Exact Windows installer format (MSI/NSIS/MSIX/ZIP/Tauri default) | YES | YES before T061 format lock | R5 packaging / T039 spike evidence |
| U-ADR13-2 | Portable package need | YES | YES if offering portable | R5 packaging spec |
| U-ADR13-3 | Per-user versus machine-wide install | YES | YES before installer | R5 packaging spec |
| U-ADR13-4 | Install/data/cache/log paths | YES | YES before installer | R5; ADR-05 |
| U-ADR13-5 | Executable and artifact machine names | YES | YES before publish | ADR-12; dedicated rename/release |
| U-ADR13-6 | Bundle / application identifier | YES | YES before store/signing | ADR-12; R5 |
| U-ADR13-7 | Signing provider / CA | YES | YES before T060 completion | Decision D / T060 |
| U-ADR13-8 | Signing-key custody mechanism (HSM vs service) | YES | YES before distribution | Decision D / T060 |
| U-ADR13-9 | App versus updater signing keys | YES | YES where applicable | Decision D / T060 |
| U-ADR13-10 | Unsigned internal-build handling details | YES | YES before T039 distribution claims | T039; Decision D |
| U-ADR13-11 | Alpha channel model | YES | YES before updater | R5 release engineering |
| U-ADR13-12 | Updater behavior (manual/auto/offline/deferral) | YES | YES before shipping updater | ADR-02; R5 |
| U-ADR13-13 | Offline installer packaging | YES | YES if claiming offline install | R5; Decision C |
| U-ADR13-14 | Worker bundling versus external runtime | YES | YES before shipping workers | ADR-06/15; T040 |
| U-ADR13-15 | Python runtime strategy | YES | YES before Python sidecar ship | ADR-15; T040 |
| U-ADR13-16 | Java runtime strategy | YES | YES before any Java sidecar | ADR-10; commandF specs |
| U-ADR13-17 | FHIR validator / terminology packaging | YES | YES before bundling | ADR-10; license review |
| U-ADR13-18 | Fehrest packaging relative to Fanatir installer | YES | YES before including Fehrest | ADR-08 |
| U-ADR13-19 | DeepMed / model packaging | YES | YES before bundling models | ADR-09; license |
| U-ADR13-20 | Package and model license review process | YES | YES before bundling | Legal; ADR-14 |
| U-ADR13-21 | SBOM format | YES | YES before claiming SBOM | Supply-chain / ADR-14 |
| U-ADR13-22 | Provenance attestation format | YES | YES before claiming provenance | Supply-chain / R5 |
| U-ADR13-23 | Reproducibility threshold | YES | YES before T064 claims | T064 |
| U-ADR13-24 | Build environment isolation | YES | YES before release builds | CI / R5 |
| U-ADR13-25 | CI release workflow | YES | YES before automated release | CI; T060 protected env |
| U-ADR13-26 | macOS/Linux packaging proof timing | YES | YES before those platform claims | Post-Alpha; FR-015 |
| U-ADR13-27 | Uninstall data retention policy | YES | YES before uninstall UX | R5; ADR-05 |
| U-ADR13-28 | Rollback mechanism detail | YES | YES before claiming rollback | R5; T065 |
| U-ADR13-29 | Schema migration on upgrade | YES | YES before upgrade migration | ADR-04/05; dedicated specs |
| U-ADR13-30 | Artifact naming under ADR-12 | YES | YES before publish | ADR-12 |
| U-ADR13-31 | T039 draft-versus-T061 acceptance-state for ADR-13 | YES | YES before R5 distribution claims | tasks.md; stage gates |
| U-ADR13-32 | Exact T060/T061 gate sequencing details | YES | YES before distribution | Decision D; T060/T061 |
| U-ADR13-33 | Public distribution target (GitHub/app stores/registries) | YES | YES before public launch | Release engineering |

---

## 10. Security and privacy claim language

This draft **proposes** packaging-strategy principles only. It does **not** claim that an installer exists, builds pass, signing is configured, packages are reproducible, SBOMs exist, CI release workflows exist, macOS/Linux packaging is validated, an updater exists, exact artifact names are decided, third-party redistribution is cleared, or that ADR-13 is Accepted. Claim language remains non-certifying; PHI posture forbids packaging PHI or production secrets.

---

## 11. Document control

| Item | Value |
| --- | --- |
| Created by | T025 draft authoring |
| Status | Proposed |
| Supersedes | None |
| Superseded by | None |
| Next expected actions | Tier C independent review; keep mass technical rename deferred (**ADR-12**); T039 only under its gate with unsigned=internal; T060/T061 only under Decision D and later authorization |

```text
End of ADR-13 draft.
Status: Proposed
Implementation authorization: NO
Build / package / sign / publish authorization: NO
```
