# ADR-11 — Auth and Session Preservation

| Field | Value |
| --- | --- |
| **ADR** | ADR-11 |
| **Title** | Auth and Session Preservation |
| **Status** | **Proposed** (draft only) |
| **Task origin** | T023 |
| **Acceptance / review posture** | Tier A — auth preservation ADR after drafting. **T030** mandatory Accepted set is ADR-15/01/02/06 only; ADR-11 is **not** listed in T030’s explicit Reviewed-or-Accepted group (04/05/07/08/09/10/14). Plan R3 entry requires ADR-11 to be **current**. Downstream **T053** / **T058** gate on ADR-11. Exact later acceptance-state expectations remain explicitly ambiguous where tasks/plan differ. |
| **Feature** | `001-fanatir-repository-and-architecture-reconstitution` |
| **Required gate (tasks.md)** | No behavior change under 001 |
| **Planning dependency (plan.md)** | Q1 — Authentication/session preservation strategy; no behavior change without dedicated spec |
| **Dependency (tasks.md)** | T005 |
| **Global constraint** | Auth/session behavior change requires **ADR-11 plus a dedicated specification** |
| **Constraining drafts** | [ADR-15](./ADR-15-rust-first-polyglot-runtime.md) (Proposed); [ADR-02](./ADR-02-rust-trusted-host-boundary.md) (Proposed); [ADR-03](./ADR-03-afia-ui-strangler-migration.md) (Proposed); [ADR-04](./ADR-04-shared-primitive-ownership-and-versioning.md) (Proposed); [ADR-05](./ADR-05-artifact-revision-run-storage.md) (Proposed); [ADR-06](./ADR-06-worker-and-ipc-contracts.md) (Proposed); [ADR-07](./ADR-07-supabase-adapter-and-local-first-boundary.md) (Proposed); [ADR-08](./ADR-08-fehrest-integration-and-release-model.md) (Proposed); [ADR-09](./ADR-09-deepmed-integration-and-openmed-runtime-fork-boundary.md) (Proposed); [ADR-10](./ADR-10-commandf-ownership-and-process-boundary.md) (Proposed) |
| **Related drafts** | [ADR-01](./ADR-01-platform-and-desktop-composition-tauri-2.md) (Proposed) |
| **Planning evidence** | [spec.md](../spec.md) Q1 / Q4 / FR-011; [plan.md](../plan.md) ADR-11 row / Decision C; [research.md](../research.md) R6; Constitution local-first / UI-zero-authority / PolicyDecision ([constitution.md](../../../.specify/memory/constitution.md)); [R0-auth-session.md](../../../docs/program-memory/baseline/R0-auth-session.md); [supabase-adapter.md](../contracts/supabase-adapter.md); [data-model.md](../data-model.md); [shared-primitives.md](../contracts/shared-primitives.md); [trusted-host-ipc.md](../contracts/trusted-host-ipc.md); [worker-runtime.md](../contracts/worker-runtime.md); [tasks.md](../tasks.md) T023 / T005 / T030 / T053 / T058 |
| **Architecture authority of this file** | **NO** — until a valid stage-gate acceptance action records Accepted |
| **Implementation authorization** | **NO** |
| **Behavior-change authorization** | **NO** — redesign requires this ADR **plus** a dedicated migration specification |

```text
This document is a planning draft and is not accepted architecture authority.
```

```text
Status: Proposed
Draft ADR ≠ architecture acceptance
T023 completion ≠ T030 acceptance
Passing Tier A review ≠ Accepted
tasks.md founder-acceptance field for T023: No — draft only
Founder acceptance of a draft work product ≠ architecture acceptance
ADR-11 alone ≠ permission to change auth/session behavior
Behavior change requires ADR-11 + dedicated specification
ADR-15 / ADR-01 / ADR-02 / ADR-03 / ADR-04 / ADR-05 / ADR-06 / ADR-07 / ADR-08 / ADR-09 / ADR-10 remain Proposed and non-authoritative
```

```text
This draft is not:
- an authentication redesign
- a session redesign
- permission to modify existing auth/session behavior
- permission to configure Supabase Auth
- permission to create migrations or RLS
- permission to modify afia-ui
- permission to modify AuthContext.tsx or PrivateRoute
- a final AuthContext schema
- a final PolicyDecision schema
- permission to modify Artifact access
- permission to transmit identity data or PHI
- permission to begin R2 or R3
```

```text
No production implementation is authorized by this draft.
```

This draft **must not** be used as justification to: redesign authentication or sessions; modify [`afia-ui`](../../../afia-ui); modify `AuthContext.tsx`, `PrivateRoute`, profile, onboarding, invitations, or membership code; configure Supabase Auth; create migrations or RLS; publish final AuthContext or PolicyDecision schemas; change Artifact access; transmit identity data or PHI; implement Fehrest/DeepMed/commandF auth; begin R2 (`T031+`) or R3; or treat Proposed ADRs as Accepted.

---

## 1. Context and problem

### 1.1 Assigned architectural question

**ADR-11 proposes** how Fanatir **preserves** current authentication and session behavior during Spec **001** reconstitution — freezing observable prototype contracts recorded by **T005**, while isolating any future redesign behind **ADR-11 plus a dedicated migration specification**, and keeping authorization, Artifact, and PHI authority with the Trusted Host and related Proposed ADRs.

It does **not** redesign auth; change OTP/session/`PrivateRoute`/profile behavior; configure Supabase Auth; execute migrations; finalize host AuthContext schemas (**ADR-04** / later); issue PolicyDecision (**ADR-02**); redefine Artifact access (**ADR-05**); redefine Supabase/local-first (**ADR-07**); or finalize PHI egress (**ADR-14**, not authored).

### 1.2 Why preservation matters during reconstitution

Founder-ratified Q1 requires `afia-ui` as migration starting shell with auth/session/`PrivateRoute`/profile flows preserved until a dedicated migration specification ([spec.md](../spec.md); [plan.md](../plan.md); [research.md](../research.md) R6). Global task prohibitions forbid auth/session behavior change without ADR-11 + dedicated spec ([tasks.md](../tasks.md)).

Without an explicit preservation boundary, planning identifies these **risks** (prospective — not claimed as currently measured production failures unless separately evidenced):

| Risk | Why it matters |
| --- | --- |
| Auth redesign breaks routes or onboarding mid-reconstitution | Blocks Alpha journey / strangler |
| Prototype behavior mistaken for final architecture | Trust seams frozen as product |
| Login conflated with authorization | Capability Gateway bypass |
| React `AuthContext` treated as host authority | UI-zero-authority violation |
| Supabase session state treated as Artifact/PHI authority | Decision C / ADR-07 violation |
| Local-first features become cloud-dependent | Offline continuity collapse |
| Offline mode becomes permanent auth bypass | Least-privilege failure |
| Migrations change behavior under 001 | Uncontrolled schema drift |
| Invented role matrices without evidence | False ACL confidence |
| Workers trust UI-provided identity | Impersonation / audit collapse |
| Sensitive operations under-authorized | Clinical/PHI risk |

### 1.3 Accepted program and planning constraints

| Constraint | Source | Treatment |
| --- | --- | --- |
| Preserve auth/session/`PrivateRoute`/profile until dedicated migration spec; no ProfileGate invention; no full rewrite | Q1 — [spec.md](../spec.md) | Binding preservation; **not** ADR-11 acceptance |
| Supabase optional collab/identity adapter; not SoT; preserve auth until dedicated migration spec | Q4 / Decision C — [spec.md](../spec.md); [plan.md](../plan.md); [supabase-adapter.md](../contracts/supabase-adapter.md) | Binding; **not** ADR-11 acceptance |
| Freeze behavior contracts; observation/docs/adapters only | R6 — [research.md](../research.md) | Planning resolution |
| T005 behavioral contract recorded; ProfileGate absent | [R0-auth-session.md](../../../docs/program-memory/baseline/R0-auth-session.md) | Inventory evidence |
| No production code; draft ≠ accepted; no migration mutation; no auth/session behavior change without ADR-11 + dedicated spec | [tasks.md](../tasks.md) T023 | Binding for this task |
| UI-zero-authority; PolicyDecision; local-first; session locking doctrines | Constitution | Active program direction; **not** ADR-11 acceptance |

### 1.4 Naming

| Name | Role in this ADR |
| --- | --- |
| **Fanatir** | Product/integration authority |
| **Auth and Session Preservation** | This ADR’s decision scope |
| **`afia-ui`** | Migration shell; hosts prototype auth UI (**ADR-03** Proposed) |
| **React `AuthContext`** | UI prototype context (`AuthContext.tsx`) — not host authority |
| **`PrivateRoute`** | Prototype session gate in `App.tsx` |
| **Supabase** | Optional identity/collaboration adapter (**ADR-07** Proposed) |
| **Trusted Host** | Privileged authorization and audit owner (**ADR-02** Proposed) |
| **Capability Gateway** / **PolicyDecision** | Host-mediated authorization path |
| **Artifact Store** | Local content authority (**ADR-05** Proposed) |
| **Fehrest** / **DeepMed** / **commandF** | Product boundaries (**ADR-08** / **09** / **10** Proposed) |

Never confuse login with authorization, React AuthContext with Trusted Host policy, or Supabase rows with Artifact authority.

### 1.5 What this draft is not

This file is **not**: an authentication redesign; session redesign; Supabase Auth configuration; migration/RLS execution; `afia-ui` modification; final AuthContext or PolicyDecision schema; Artifact-access redesign; PHI-egress authorization; or R2/R3 authorization.

---

## 2. Decision (proposed)

### 2.1 Core preservation decision

**Propose** that Spec **001** **preserves** externally observable current authentication and session behavior recorded by T005 / [R0-auth-session.md](../../../docs/program-memory/baseline/R0-auth-session.md), and that **any behavioral redesign** would require **Accepted ADR-11 plus a dedicated auth/session migration specification** — not ADR-11 alone, not chat, and not incidental UI edits.

**Preservation means:**

- no behavior change during Spec 001;
- no silent removal or addition of auth flows under 001 tasks;
- no architecture endorsement of prototype code as final target;
- no claim that current behavior is production-ready;
- future redesign requires a dedicated specification.

### 2.2 Observable behaviors frozen under Spec 001 (proposed)

Preserve current observed behavior for:

- sign-in;
- sign-out;
- sign-up where currently exposed;
- OTP / magic-link behavior where currently used;
- session restoration and loading;
- route protection via `PrivateRoute`;
- current profile/consent/onboarding side effects as observed;
- current invitation and workspace membership behavior where observed;
- existing error and missing-environment behavior (including module throw when Supabase env absent).

Internal details listed as replaceable in [R0-auth-session.md](../../../docs/program-memory/baseline/R0-auth-session.md) §12 remain eligible for later ADR/spec replacement **only** when a dedicated migration specification authorizes change — not under T023.

### 2.3 Identity, authentication, session, and authorization separation (proposed)

| Concept | Planning meaning |
| --- | --- |
| Identity | Who an actor claims to be |
| Account | Durable account record (provider-specific) |
| Authentication | Mechanism that establishes actor claim (e.g. OTP) |
| Session | Continuity of authenticated state |
| Authorization | Whether an operation is allowed |
| Membership | Collaboration metadata (workspace/org/project) |
| Role | Label within membership — not automatic Artifact ACL |
| Capability | Described privilege; not self-executing |
| PolicyDecision | Host-issued authorization outcome (**ADR-02** / data shape **ADR-04**) |
| Profile | Editable metadata / consent — not proof of identity |
| Reviewer identity | Provenance of human review — may differ from account |
| Worker/service identity | Supervised process actor (**ADR-06** / **ADR-15**) |

**Explicitly:**

- authentication proves only the mechanism’s actor claim;
- session validity does **not** prove operation authorization;
- membership metadata does **not** grant Artifact access automatically;
- UI route access is **not** PolicyDecision;
- profile data is **not** proof of identity;
- reviewer identity may require separate provenance.

### 2.4 Current prototype evidence (inventory only)

From [R0-auth-session.md](../../../docs/program-memory/baseline/R0-auth-session.md) and related inventory (source observation; not runtime proof; not final architecture):

| Element | Observed role | Label |
| --- | --- | --- |
| `afia-ui/client/src/contexts/AuthContext.tsx` | React AuthProvider; `user`, `session`, `loading`, `signIn`, `signOut`, `signUp` | Prototype evidence; preserved |
| Supabase OTP | `signInWithOtp` / sign-up OTP; `detectSessionInUrl`; persist/auto-refresh | Prototype evidence; preserved |
| `PrivateRoute` in `App.tsx` | Loading spinner; unauthenticated → `/login` | Prototype evidence; preserved |
| Profile/consent sync | `profiles.consent_given_at` upsert; fail-open sync on error | Prototype evidence; preserved (imperfect) |
| Invitations / workspace membership | InviteAccept; team-workspaces; UI role gates | Prototype evidence; preserved as observed |
| Missing `VITE_SUPABASE_URL` / anon key | Client module throws; app cannot mount AuthProvider path | Observed failure behavior; preserved |
| `getLoginUrl` in `const.ts` | Defined with **no importers** | Unused helper — **not** architectural adoption |
| `ProfileGate` | Absent as file/symbol | Do **not** invent |
| Auth event audit in AuthContext | Mandatory login/logout audit **NOT OBSERVED** | Incomplete prototype evidence |
| UI-only role checks / direct client DB writes | **VERIFIED TRUST SEAM** (T005) | Preserve behavior; **not** endorse as final architecture |

RLS, Edge Function authorization, and live token-storage details remain **unverified** where T005 deferred them to T006+.

### 2.5 Product and deployment boundary (proposed)

Preservation applies across planning contexts:

- desktop-first local Fanatir;
- founder-controlled single-user operation;
- optional collaboration;
- Supabase-connected operation when configured;
- offline / degraded network conditions;
- future hosted deployment (**not** authorized here);
- Fehrest, DeepMed, commandF, and supervised workers as **consumers** of host-mediated policy — not auth redesign owners.

Do **not** impose a cloud-first requirement. Future hosted or multi-user production behavior would require separate authorization and specifications.

### 2.6 Local-first identity boundary (proposed)

- Local Artifact and PHI authority must **not** depend silently on Supabase availability (**ADR-07** / Decision C).
- Network loss must **not** change durable-content authority.
- Offline operation does **not** automatically bypass identity or authorization.
- Local continuity rules beyond preservation of current prototype failure modes remain **unresolved**.
- Stale remote membership must **not** silently authorize new privileged host operations.
- Preserved prototype failure behavior (including env-missing throw) may remain temporarily even if imperfect.

Do **not** invent a new offline-auth mechanism in this draft.

### 2.7 Supabase Auth boundary (proposed)

- Current Supabase session issuance is **provisionally preserved** ([supabase-adapter.md](../contracts/supabase-adapter.md)).
- Preservation does **not** make Supabase final architecture.
- Supabase remains an optional identity/collaboration adapter per Decision C / **ADR-07** Proposed.
- Supabase is **not** Artifact, clinical, PHI-content, or PolicyDecision source of truth.
- Remote identity rows do **not** grant local content authority.
- Future replacement, canonicalization, or migration → dedicated specification (including future Spec **002** for local-first/migration canonicalization — authorized by name; **not** created in 001).
- **T023 performs no Supabase Auth configuration.**

Do **not** select a new identity provider.

### 2.8 Session boundary (proposed)

Preserve observed session creation, restoration, refresh (SDK auto-refresh), logout, loading, and missing-env failure modes.

**Remain unresolved** (dedicated specification): final token format; secure-storage backend; rotation policy; inactivity timeout; concurrent-session policy; device revocation; final offline continuity; lock-screen design.

Do **not** define production token storage.

### 2.9 React AuthContext versus host AuthContext (proposed)

| Layer | Role |
| --- | --- |
| React `AuthContext` | UI convenience state; presentation and client SDK session |
| Future Trusted Host auth/authorization context | Separately versioned host contract (if introduced) |

**Propose:**

- React context is **not** authoritative PolicyDecision;
- workers must **not** trust React AuthContext directly;
- future host context requires a separately versioned contract;
- naming/type resolution remains a later specification matter.

Do **not** publish the future schema.

### 2.10 PolicyDecision boundary (proposed)

- Trusted Host evaluates privileged operation authorization (**ADR-02** Proposed).
- ADR-11 preserves authentication/session behavior but does **not** issue PolicyDecision.
- Shared PolicyDecision / Approval semantics remain **ADR-04** Proposed.
- UI requests cannot self-authorize.
- Workers receive only bounded, host-mediated authorization evidence (**ADR-06** / **ADR-15**).
- Cached UI permissions are **not** authorization.
- Login success cannot substitute for capability evaluation.

### 2.11 Membership and scope boundary (proposed)

Canonical evidence supports workspaces, invitations, and UI-level role checks as **collaboration metadata** prototypes ([R0-auth-session.md](../../../docs/program-memory/baseline/R0-auth-session.md)). Patient access is distinct from ordinary project membership (Constitution; T005).

**Clarify:** membership does **not** automatically grant Artifact, PHI, DeepMed, commandF, or Fehrest authority; final role/permission matrix is **unresolved**; RLS/migration rules are **outside T023**.

Do **not** invent production multi-tenancy or an unsupported clinician/researcher/guest role matrix.

### 2.12 Profile boundary (proposed)

Distinguish authenticated identity, account, editable profile, onboarding/consent state, professional metadata, display preferences, and reviewer identity.

Profile metadata does **not** prove identity; profile completion does **not** grant privileged capabilities; current behavior is preserved; future redesign requires separate specification where needed.

### 2.13 Worker and service identity (proposed)

- Workers are supervised actors (**ADR-06** / **ADR-15**).
- Workers cannot impersonate users by default.
- Workers do **not** consume React AuthContext as authority.
- Audit should distinguish user, host, and worker actors when implemented.
- Final certificates, tokens, or service-account systems remain **unresolved**.

### 2.14 Secrets and credential custody (proposed)

Session tokens, refresh tokens, OTP links, Supabase keys, provider credentials, device keys, encryption keys, recovery codes, and signing keys:

- UI must **not** become the long-term secret source of truth;
- secrets must **not** enter ordinary logs, IPC payloads, Artifacts, or profile fields;
- environment-variable use is **prototype evidence**, not final custody;
- Trusted Host or approved secure storage would require a later decision (**ADR-02**);
- **no secret backend is selected** here.

### 2.15 UI and `afia-ui` boundary (proposed)

**ADR-03** owns `afia-ui` migration. ADR-11 may discuss preservation of sign-in route, protected routes, logout, session-loading, permission-denied presentation, and current onboarding/profile behavior.

ADR-11 must **not**: modify routes; redesign screens; create a lock screen; add MFA UI; change onboarding; modify `PrivateRoute`; implement offline indicators. UI redesign requires a dedicated specification and later tasks.

### 2.16 PHI, privacy, and audit (proposed)

Identity metadata (email, phone, clinician identifiers, patient-account metadata, session identifiers, IP/device identifiers, invitations, membership, access patterns, authentication logs) may itself be sensitive. Minimum-necessary handling applies; no PHI or identity secrets in ordinary logs; full classification/egress/retention → **ADR-14** (not authored). No compliance certification claimed.

**Audit:** future relevance includes sign-in success/failure, logout, session lifecycle, invitation/membership changes, authorization denial, recovery, device changes, suspicious events. Authoritative audit ownership remains **ADR-02** / Trusted Host. Current AuthContext login/logout audit may be absent (**NOT OBSERVED**). Do not finalize audit schemas.

### 2.17 Failure, lifecycle, external IdPs, multi-tenancy (proposed)

**Failure / degraded:** address Supabase unavailable, network unavailable, missing env, invalid/expired session, refresh failure, revoked membership, clock skew, corrupted local state, conflicting local/remote identity, process restart, invitation expiry — distinguishing **observed current behavior**, **desirable future behavior**, and **behavior requiring a dedicated specification**. Preservation must **not** invent new fallback authentication. No stale or uncertain state may be presented as newly authorized.

**Account lifecycle:** creation/sign-up and invitations are partially observed; recovery, deletion, suspension, device revocation largely **unresolved** / future specification. Destructive account operations are **not** authorized.

**External IdPs:** unused `getLoginUrl` helper does **not** constitute adoption. No Google/Microsoft/Apple/GitHub/OIDC/SAML/national/healthcare IdP is selected.

**Multi-tenancy:** single-user desktop + workspace membership metadata; RLS unverified where T005 deferred; T023 does **not** grant SaaS production multi-tenant authority; remote row separation ≠ local content authority.

### 2.18 Fehrest, DeepMed, commandF, Artifact Store (proposed)

| Boundary | Proposal |
| --- | --- |
| **Fehrest (ADR-08)** | Embedded Fehrest consumes host-mediated context; vault access requires governed authorization; standalone Fehrest identity remains its product concern; Supabase must not silently synchronize authoritative vault content |
| **DeepMed (ADR-09)** | Invocation requires Capability Gateway; clinical-assistive ops may need stronger authorization; reviewer identity distinguishable; remote inference/PHI egress separately governed; ADR-11 does not authorize model/provider ops |
| **commandF (ADR-10)** | Local validation may not inherently require cloud identity; sensitive transforms, network, live-system access, or publication require host PolicyDecision; commandF is not auth authority. Founder-accepted long-term FHIR workbench strategy does **not** alter ADR-10 or create T023 auth requirements; ADR-11 does **not** modify ADR-10 |
| **Artifact Store (ADR-05)** | Login success does not automatically authorize Artifact reads/writes/publication/deletion/export/pointer changes; Supabase identity rows do not grant storage authority; UI must not bypass host mediation |

### 2.19 Migration, schema, and version domains (proposed)

- **T023 prohibits migration mutation.**
- ADR-11 may identify future migration requirements only.
- Supabase tables, RLS, local databases, user-ID propagation, and schema canonicalization require a dedicated specification (and Spec **002** where applicable).
- ADR-11 alone is **insufficient** to execute auth/session redesign.

**Version domains remain separate:** identity-schema; session-protocol; UI AuthContext; future host AuthContext; PolicyDecision; provider-adapter; Supabase schema; local-profile; IPC protocol; application version. Do not collapse. Negotiation/migration/compatibility testing are **not** claimed implemented.

---

## 3. Alternatives considered

| Option | Rationale (evidence-based; not claimed as tested) |
| --- | --- |
| A. Redesign auth immediately during Spec 001 | Violates Q1 / global prohibitions / R6 freeze |
| B. Remove auth for local-first Alpha | Not canonically authorized; would be a behavior change under 001 |
| C. Make Supabase Auth permanent authority | Violates Decision C / ADR-07 non-SoT posture |
| D. Trust UI AuthContext as authorization | Violates UI-zero-authority / ADR-02 |
| E. Use membership rows as Artifact ACL | Violates ADR-05 / patient≠membership doctrines |
| F. Replace current auth without dedicated migration spec | Violates ADR-11 + dedicated-spec global constraint |
| **G (proposed).** Preserve current observable behavior; isolate redesign behind ADR-11 + dedicated specification | Aligns Q1/Q4, T005, tasks.md, plan ADR-11 row |

---

## 4. Consequences

### 4.1 Prospective benefits

- lower regression risk during reconstitution;
- preserved routes and onboarding;
- clear authentication ≠ authorization;
- protection of Artifact and PHI authority;
- local-first continuity of content authority;
- explicit dual-gate for redesign (ADR-11 + dedicated spec);
- reduced accidental cloud lock-in;
- separation of prototype UI state from Trusted Host policy.

### 4.2 Costs and risks

- preserving imperfect prototype behavior temporarily;
- unresolved offline continuity;
- current Supabase dependency when env present;
- missing AuthContext audit evidence;
- AuthContext naming collision (UI vs host);
- deferred secure token custody;
- migration debt;
- ambiguous role/membership model;
- limited UX improvement during Spec 001;
- future compatibility burden;
- T030/R3/T053/T058 acceptance-state ambiguity for ADR-11.

### 4.3 Planning-only rollback

Per [tasks.md](../tasks.md) T023: **Revert ADR file**. No production rollback surface is created by this draft.

---

## 5. Non-goals

ADR-11 does **not**:

- implement authentication;
- redesign sessions;
- modify `AuthContext.tsx`;
- modify `PrivateRoute`;
- modify profile or onboarding;
- configure Supabase Auth;
- add external IdPs;
- select MFA, passkeys, passwords, or OAuth;
- create migrations or RLS;
- define final AuthContext or PolicyDecision schemas;
- modify Artifact access;
- implement audit;
- define complete PHI policy;
- modify Fehrest, DeepMed, or commandF;
- alter ADR-10 or commandF strategy artifacts;
- publish packages or releases;
- begin R2 or R3.

---

## 6. Relationship to other ADRs

| ADR | Status in repo | Relationship |
| --- | --- | --- |
| ADR-15 | Proposed | Supervised workers; no ambient trust of UI identity |
| ADR-01 | Proposed | Platform composition |
| ADR-02 | Proposed | Trusted Host / Capability Gateway / PolicyDecision / audit |
| ADR-03 | Proposed | `afia-ui` strangler; presentation only |
| ADR-04 | Proposed | Shared PolicyDecision / Approval semantics as data |
| ADR-05 | Proposed | Artifact persistence and access mediation |
| ADR-06 | Proposed | Worker propagation; no direct durable authority |
| ADR-07 | Proposed | Supabase/local-first; preserve auth until dedicated spec |
| ADR-08 | Proposed | Fehrest; host-mediated context |
| ADR-09 | Proposed | DeepMed; Capability Gateway |
| ADR-10 | Proposed | commandF; not auth authority; unchanged by this draft |
| ADR-12 | Not authored | Rename planning (peripheral) |
| ADR-13 | Not authored | Packaging |
| ADR-14 | Not authored | PHI classification and egress |

ADR-11 decides only **Auth and Session Preservation**.

---

## 7. Gates and authority

```text
T023 produces a Proposed ADR-11 draft only.
Tier A — auth preservation ADR is required.
Passing Tier A review is not architecture acceptance.
tasks.md founder-acceptance field for T023 is: No — draft only.
Founder draft acceptance is not required by the T023 contract for the draft work product.
Any auth/session behavior change requires ADR-11 plus a dedicated specification.
ADR-11 alone does not authorize redesign or implementation.
T030 remains incomplete.
T030 mandatory Accepted set is ADR-15/01/02/06 only — ADR-11 is not in that set.
T030 explicit Reviewed-or-Accepted group lists ADR-04/05/07/08/09/10/14 — ADR-11 is not listed there.
Plan R3 entry requires ADR-11 to be current.
T053 and T058 depend on ADR-11; exact Accepted-versus-current expectations remain explicitly ambiguous where sources differ.
T031+ and R2 remain separately gated.
R3 and production auth work remain separately gated.
Authentication, sessions, Supabase Auth configuration, migrations, UI changes, AuthContext publication, PolicyDecision issuance, Artifact-access changes, PHI handling, and production work require separate authorization.
```

| Gate | Meaning for ADR-11 |
| --- | --- |
| T023 | Draft authoring (this task) |
| Tier A | Mandatory independent review |
| Founder draft acceptance | Not required by tasks.md for T023 work product |
| Dedicated auth migration spec | Required for any behavior change |
| T030 | R1 gate; ADR-11 not in mandatory Accepted set; not in listed Reviewed-or-Accepted group |
| Plan R3 entry | ADR-11 current |
| T053 / T058 | Depend on ADR-11 |
| T031+ / R2 / R3 | Separately gated; not authorized here |

---

## 8. Validation and acceptance plan

```text
T023 completion produces a draft for Tier A review.
Architecture acceptance is not performed by T023.
Behavior change is not authorized by T023.
```

Planning reviews **may** include: preservation-not-redesign fidelity; dual-gate language; login≠authorization; React AuthContext ≠ host authority; ADR-02/04/05/07 non-theft; T005/R0 fidelity; T030/R3/T053/T058 ambiguity honesty; Tier A independent review ([tasks.md](../tasks.md) T023).

Verification method from tasks.md: doc review; link from plan roadmap (plan already contains ADR-11 row — **not** modified by T023).

---

## 9. Unresolved questions

T023 does **not** resolve these. Attempted resolution: **NO**.

| ID | Question | May remain open in draft review? | Must resolve before behavior change? | Belongs elsewhere? |
| --- | --- | --- | --- | --- |
| U-ADR11-1 | Exact Alpha auth requirement (when cloud optional) | YES | YES before redesign | Dedicated auth migration spec |
| U-ADR11-2 | Local single-user identity without Supabase | YES | YES before redesign | Dedicated spec; ADR-07 |
| U-ADR11-3 | Offline continuity rules | YES | YES before redesign | Dedicated spec |
| U-ADR11-4 | Missing Supabase environment long-term UX | YES | YES if changing throw behavior | Dedicated spec |
| U-ADR11-5 | Future Supabase Auth role | YES | YES before redesign | ADR-07; Spec 002; dedicated auth spec |
| U-ADR11-6 | Migration to another provider | YES | YES before redesign | Dedicated auth spec |
| U-ADR11-7 | UI AuthContext vs host AuthContext naming | YES | YES before schema publication | ADR-04 / dedicated spec |
| U-ADR11-8 | Session token custody | YES | YES before redesign | ADR-02 secrets |
| U-ADR11-9 | Secure storage backend | YES | YES before redesign | ADR-02 |
| U-ADR11-10 | Session expiry policy | YES | YES before redesign | Dedicated auth spec |
| U-ADR11-11 | Refresh and revocation | YES | YES before redesign | Dedicated auth spec |
| U-ADR11-12 | Inactivity lock | YES | YES before redesign | Dedicated auth / UI spec |
| U-ADR11-13 | Concurrent sessions | YES | YES before redesign | Dedicated auth spec |
| U-ADR11-14 | Device loss | YES | YES before redesign | Dedicated auth spec |
| U-ADR11-15 | Local/cloud identity reconciliation | YES | YES before redesign | Dedicated spec; Spec 002 |
| U-ADR11-16 | Workspace and organization model | YES | YES before redesign | ADR-07; collab specs |
| U-ADR11-17 | Invitation behavior finalization | YES | YES before redesign | Dedicated auth / collab spec |
| U-ADR11-18 | Role and permission model | YES | YES before redesign | ADR-02; dedicated ACL spec |
| U-ADR11-19 | Patient versus project membership | YES | YES for clinical surfaces | Constitution; patient specs |
| U-ADR11-20 | Profile versus identity | YES | YES before redesign | Dedicated auth / profile spec |
| U-ADR11-21 | Reviewer identity | YES | YES for CoLab/clinical review | ADR-04 Approval; T053 |
| U-ADR11-22 | Worker/service identity | YES | YES before worker auth claims | ADR-06 / ADR-15 |
| U-ADR11-23 | Audit events for auth lifecycle | YES | YES before claiming audit complete | ADR-02 |
| U-ADR11-24 | Account recovery | YES | YES before enabling recovery UX | Dedicated auth spec |
| U-ADR11-25 | Account deletion | YES | YES before destructive ops | Dedicated auth / privacy spec; ADR-14 |
| U-ADR11-26 | External IdPs | YES | YES before enabling | Dedicated auth spec |
| U-ADR11-27 | Multi-tenancy | YES | YES before SaaS claims | Later product specs |
| U-ADR11-28 | RLS verification | YES | YES before trusting client writes | T006 evidence; Spec 002 |
| U-ADR11-29 | Artifact-access policy detail | YES | YES before ACL claims | ADR-05; dedicated ACL |
| U-ADR11-30 | Fehrest access identity | YES | YES before Fehrest auth claims | ADR-08 |
| U-ADR11-31 | DeepMed invocation authority coupling | YES | YES before clinical ops | ADR-09; ADR-02 |
| U-ADR11-32 | commandF sensitive-operation authority | YES | YES before live/network ops | ADR-10; ADR-02 |
| U-ADR11-33 | PHI-bearing identity metadata | YES | YES before cloud identity expansion | ADR-14 |
| U-ADR11-34 | Schema and migration ownership | YES | YES before migrations | Spec 002; dedicated auth migration |
| U-ADR11-35 | Spec 002 scope vs auth migration spec | YES | YES before creating either | Plan / founder sequencing |
| U-ADR11-36 | R3 “ADR-11 current” exact meaning | YES | YES before R3 entry claims | Plan vs tasks reconciliation |
| U-ADR11-37 | T053/T058 Accepted-versus-current expectations | YES | YES before those tasks | tasks.md; stage gates |

---

## 10. Security and privacy claim language

This draft **proposes** a preservation posture only. It does **not** claim production-ready authentication, verified RLS, implemented audit, offline auth success, or compliance certification. Preserving current behavior does **not** endorse prototype trust seams as final architecture.

---

## 11. Document control

| Item | Value |
| --- | --- |
| Created by | T023 draft authoring |
| Status | Proposed |
| Supersedes | None |
| Superseded by | None |
| Next expected actions | Tier A independent review; later dedicated auth migration spec before any behavior change; T053/T058 only under their own gates |

```text
End of ADR-11 draft.
Status: Proposed
Implementation authorization: NO
Behavior-change authorization: NO
```
