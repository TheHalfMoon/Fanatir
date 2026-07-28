# R0 Python / FHIR / OpenMed Prototype Services (T007)

**Task**: T007 — Inspect Python/FHIR/OpenMed prototype services  
**Recorded**: 2026-07-24 (local)  
**Baseline / start HEAD**: `21b2ceed551301e7889d3e793f56a1ee5c626675` (T006 completion)  
**Branch**: `docs/f0-planning-memory-bootstrap`  
**Nature**: Read-only source inventory. **No** service start, OpenMed import execution, model download, `uv sync`, or package install.  
**Review**: **Tier C** — normal verification (implementing agent may self-verify; independent Tier A **not** required by tasks.md for T007).  
**T007 decision**: **PASS WITH NOTES**

Absolute paths are local facts. Runtime behavior is **REQUIRES RUNTIME VERIFICATION** unless noted as source-only.

Taxonomy: **VERIFIED BEHAVIOR** · **VERIFIED TRUST SEAM** · **VERIFIED SAFETY DEFECT** · **POTENTIAL RISK — runtime/model verification required** · **NOT OBSERVED** · **OUT OF SCOPE**.

---

## 1. Complete service-path inventory

| Path | Role | Activity class |
| --- | --- | --- |
| `services/openmed_bridge.py` | FastAPI “AFIA OpenMed Bridge” | **PROTOTYPE** + **REFERENCED BUT RUNTIME-UNVERIFIED** (wired from UI; not executed in T007) |
| `services/fhir_gate.py` | FHIR R4B Bundle builder from NER entities | **PROTOTYPE** (imported by bridge `/export-fhir`) |
| `services/requirements-bridge.txt` | Partial deps (`sentence-transformers`, `fhir.resources`, `markitdown`); notes openmed/fastapi/uvicorn/pymupdf installed separately | **SCAFFOLD / incomplete pin** |
| `services/README.md` | Describes `operations-go/` and `ai-python/` under services — **those dirs absent** at `services/` | **CONTRADICTORY** vs on-disk contents |
| `afia-ui/client/src/services/openmed-client.ts` | UI HTTP client → `http://127.0.0.1:8765` | **VERIFIED ACTIVE** wiring |
| `afia-ui/client/src/services/model-preference.ts` | Model preference + calls `getModels` | **VERIFIED ACTIVE** wiring |
| `afia-ui/client/src/services/storage.ts` | Local UI storage helpers (not Python) | **OUT OF SCOPE** for Python services |
| `_archived/services-ai-python/**` | Scaffold AI runtime (`requires-python >=3.12`; OpenMed adapter stubs) | **ARCHIVED** |
| `_archived/services-operations-go/**` | Go ops service archive | **ARCHIVED** (not Python) |
| Root `Cargo.toml` / `go.work` | Point at missing `services/operations-go` | **CONTRADICTORY** (T001) |
| `.github/workflows/ci.yml` | Has a `python:` job placeholder | **SCAFFOLD ONLY** (echo TODO pattern from T001) |
| DeepMed-AI sibling | Separate repo | **OUT OF SCOPE** product boundary — not initialized here |

No other active `*.py` under `services/` beyond the two modules above.

---

## 2. OpenMed bridge contract (`services/openmed_bridge.py`)

| Aspect | Source fact | Class |
| --- | --- | --- |
| Framework | FastAPI + uvicorn | VERIFIED BEHAVIOR |
| OpenMed import | **Eager** `import openmed` at module load | VERIFIED BEHAVIOR |
| Comment pin note | Mentions `openmed 1.7.0` attention backend behavior | VERIFIED BEHAVIOR (comment); package not pinned in requirements-bridge |
| Bind | `host="127.0.0.1"`, `port=8765` under `__main__` | VERIFIED BEHAVIOR |
| CORS | `localhost:5173` and `localhost:3000` only | VERIFIED BEHAVIOR |
| Auth | **None** on HTTP API | VERIFIED TRUST SEAM (localhost-only mitigates; not Trusted Host) |

### Routes

| Method | Path | Input | Output (summary) |
| --- | --- | --- | --- |
| GET | `/health` | — | `{ok: true}` |
| GET | `/models` | — | grouped `{ner,pii,zeroshot,other}` after HF HEAD filter |
| POST | `/analyze` | `{text, model?}` | text, model, processing_time, timestamp, entities[{text,label,confidence,start,end}] |
| POST | `/deidentify` | `{text, language?}` | original, deidentified |
| POST | `/extract-pii` | `{text}` | text, entities (no model/timing fields) |
| POST | `/upload` | multipart file | document_id (sha256[:16]), filename, page_count, full_text, pages[] |
| POST | `/upload-pdf` | multipart PDF | same |
| POST | `/render-page` | multipart PDF + page_number | PNG bytes |
| POST | `/analyze-document` | `{text, chunk_size?, model?}` | text, entities (offset-adjusted), chunk_count, model_used |
| POST | `/ask-document` | `{text, question, top_k?, document_id?}` | question, passages[{text,score,char_start,char_end}], retrieval_model |
| POST | `/export-fhir` | `{entities[], doc_meta?}` | fhir_gate `{bundle, summary}` |

### Notable behaviors

- Forces `OPENMED_TORCH_ATTENTION_BACKEND=eager` and passes `attn_implementation=eager` into analyze pipeline.
- HF model existence cache under `~/.cache/afia/model_availability.json` (7-day TTL); network HEAD to huggingface.co.
- Document analyze chunks text (default 3000 chars); adjusts entity offsets; requires model identity.
- Ask-document: extractive retrieval via `sentence-transformers` `all-MiniLM-L6-v2` (lazy import); **not** generative clinical Q&A.
- Upload supports pdf/docx/pptx/xlsx/txt/html via PyMuPDF / MarkItDown.
- Returns entity source spans (`start`/`end`) for analyze paths.
- Size limits / timeouts / concurrency caps for analyze: **NOT OBSERVED** (beyond HF check semaphore=10).
- Graceful shutdown / readiness beyond `/health`: **NOT OBSERVED**.

---

## 3. OpenMed dependency / license boundary

| Fact | Detail |
| --- | --- |
| Declared in `requirements-bridge.txt` | `sentence-transformers==5.6.0`, `fhir.resources==8.3.0`, `markitdown[...]>=0.1.0` |
| openmed / fastapi / uvicorn / pymupdf | Comment: “assumed installed separately” — **not pinned** in this file |
| PyPI runtime assumption | Yes — hard `import openmed`; research.md cites Apache-2.0 PyPI package |
| NOTICE / Apache attribution in repo for OpenMed | **NOT OBSERVED** under `services/` |
| Per-model license manifest | **NOT OBSERVED** |
| Model IDs | From `openmed.list_models()` + optional request `model`; HF filter may drop IDs |
| Network downloads | HF HEAD checks; OpenMed/transformers may download weights on first use — **POTENTIAL RISK — runtime/model verification required** |
| Offline behavior | **NOT OBSERVED** as defined |

**Boundary (Decision B / ADR-09):**

- Current PyPI use = **baseline prototype evidence**.
- Temporary R3 DeepMed PyPI use may later be authorized under ADR-09 (pin, isolation, NOTICE, model licenses).
- **No** governed OpenMed fork/import under specification 001.
- DeepMed-AI remains a **separate** repository/product boundary.

---

## 4. FHIR gate contract (`services/fhir_gate.py`)

| Aspect | Source fact |
| --- | --- |
| Library | `fhir.resources.R4B.*` via `fhir.resources==8.3.0` |
| Release assumed | **FHIR R4B** (R4-compatible), docstring |
| Capability | Transform NER entities → collection **Bundle** with Patient, DocumentReference, Condition / MedicationStatement / Observation |
| Validation level | **Pydantic structural model validation** on resource construction — **not** FHIR profile/IG/terminology/business-rule validation |
| PII handling | Label blocklist skips PII-typed entities; regex second layer on Observation text; anonymous Patient id |
| Document bytes | DocumentReference attachment has contentType only — **no embedded document text/bytes** |
| Terminology / IG | **NOT OBSERVED** |
| Bundle reference resolution | Minimal internal refs to anonymous Patient |
| commandF-equivalent result states | **NOT OBSERVED** — prototype transform only |
| Network | None in fhir_gate itself |
| Entry | `entities_to_fhir(entities, doc_meta)` → `{bundle, summary}` |

**Not a complete FHIR validator.** Classify as **prototype transform / structural emit**.

---

## 5. FHIR-version findings

| Fact | Value |
| --- | --- |
| Current implementation | **R4B** (`fhir.resources.R4B`) |
| Product doc note | `docs/product/AFIA_FULL_BUILD_PLAN.md` mentions FHIR R4 Bundle for v1 |
| Spec/plan commandF | Ownership via ADR-10; no single founder-ratified R5 pin found in tasks/plan contracts during T007 |
| Mismatch | Possible future R5/commandF canonical baseline vs prototype R4B — **requires ADR-10 / commandF worker resolution** |
| Action | Do **not** change deps in T007; record contradiction risk as **CONTRADICTORY / unresolved canonical version** until ADR |

---

## 6. Frontend integration map

Client: `afia-ui/client/src/services/openmed-client.ts` → `BASE_URL = http://127.0.0.1:8765`.

| Caller areas | Endpoints used |
| --- | --- |
| DocumentStudio, Batch, Models, Compare, Deidentify, Assistant, StatusBar, AnalysisModelPicker, FhirExportModal | health, models, analyze, analyze-document, upload, deidentify, extract-pii, ask-document, export-fhir, render-page (as applicable) |

| Integration attribute | Observation |
| --- | --- |
| Auth to bridge | **None** |
| Timeouts/cancel/retry | Mostly raw `fetch` — limited/no standardized timeout — **REQUIRES RUNTIME VERIFICATION** |
| PHI | Full text / files / entities sent to localhost bridge | **VERIFIED TRUST SEAM** |
| Optional vs required | StatusBar probes health; Studio features depend on bridge when used | optional process, required for those features |

### Contract mismatches (TypeScript vs Python)

| Item | Detail | Class |
| --- | --- | --- |
| `/extract-pii` response | Client types as `AnalyzeResult` (expects `model`, `processing_time`, `timestamp`); server returns `{text, entities}` only | **CONTRADICTORY** / wiring risk |
| CORS | Bridge allows Vite 5173 + 3000; afia-ui Vite defaults port 3000 — OK for that config | VERIFIED BEHAVIOR |
| Otherwise | analyze / deidentify / upload / analyze-document / ask-document / export-fhir shapes largely align | VERIFIED BEHAVIOR (static) |

---

## 7. Process / packaging blockers

| Item | Fact |
| --- | --- |
| Startup | `python services/openmed_bridge.py` → uvicorn `127.0.0.1:8765` (implied by `__main__`) |
| Documented under services/README | **Wrong** — describes missing `operations-go` / `ai-python` |
| Python version | Host has 3.11.15 (T003); archived AI wants `>=3.12`; bridge unconstrained in-file |
| Project venv / uv.lock | **Absent** for active Fanatir Python (T003) |
| Install | requirements-bridge incomplete; openmed not pinned |
| Health | GET `/health` |
| Windows packaging | Localhost prototype only — **NOT OBSERVED** as packaged sidecar |
| CI | No real Python service test gate observed |

---

## 8. Clinical / safety findings

| Observation | Class |
| --- | --- |
| NER / entity extraction with confidence + spans | VERIFIED BEHAVIOR |
| Ask-document is extractive retrieval, not treatment advice generator | VERIFIED BEHAVIOR |
| No explicit clinical-use disclaimer in bridge module | NOT OBSERVED |
| No abstention / “unsupported clinically” gate beyond errors | NOT OBSERVED |
| Model identity required for document analyze | VERIFIED BEHAVIOR (anti-silent-empty) |
| FHIR Condition `clinicalStatus=active` default for mapped diagnoses | VERIFIED BEHAVIOR — **POTENTIAL RISK** if treated as clinical truth without review |
| Human review required by product process | OUT OF SCOPE for this service (UI may review) |
| **VERIFIED SAFETY DEFECT** | **None** established solely from source (prototype mis-use risk remains process/governance) |

---

## 9. Privacy / authority findings

| Capability | Present? | Class |
| --- | --- | --- |
| Bind localhost only | Yes | VERIFIED BEHAVIOR |
| No auth on API | Yes | VERIFIED TRUST SEAM |
| Accept uploaded files into memory; return full text | Yes | VERIFIED TRUST SEAM |
| Write HF availability cache under user home | Yes | VERIFIED BEHAVIOR |
| Network to HuggingFace | Yes | VERIFIED TRUST SEAM / POTENTIAL RISK (downloads) |
| Mutate Fanatir patient DB / Supabase | No direct | NOT OBSERVED |
| Bypass Rust policy | N/A — Rust host not supervising yet | VERIFIED TRUST SEAM vs target |
| Log PHI | Exception details may include text fragments | POTENTIAL RISK — runtime verification required |
| Persist results as Artifact/Run | No | NOT OBSERVED |

**Target (not implemented):** Rust supervises workers; owns filesystem/secrets/policy/Artifact/Run/audit; Python receives bounded IPC inputs only.

---

## 10. Archived / contradictory assets

| Asset | Class |
| --- | --- |
| `_archived/services-ai-python` | ARCHIVED scaffold; OpenMed adapter TODO |
| `services/README.md` vs actual files | CONTRADICTORY |
| Root manifests referencing missing services | CONTRADICTORY (T001) |
| `_archived/ARCHIVED.md` mentions openmed_bridge | Historical note — live file still at `services/` |

---

## 11. Current-versus-target boundary

| Current | Target |
| --- | --- |
| Local FastAPI OpenMed PyPI prototype | DeepMed worker under ADR-09; optional temporary PyPI |
| fhir_gate R4B transform | Fanatir-owned commandF worker (ADR-10); version TBD |
| UI calls localhost directly | Rust-supervised IPC; UI-zero-authority |
| Unpinned openmed | Exact pin + NOTICE + model licenses |

---

## 12. Runtime-verification gaps

- Service not started; OpenMed not imported in this task.
- Model download / weight presence unknown.
- HF filtering behavior unknown offline.
- extract-pii TS mismatch impact at runtime unknown.
- Performance / memory bounds unknown.

---

## 13. Migration implications

1. Treat `openmed_bridge` / `fhir_gate` as **prototypes**, not DeepMed/commandF products.
2. ADR-09 owns OpenMed pin/NOTICE/model licenses before R3 DeepMed.
3. ADR-10 owns commandF vs fhir_gate replacement/wrap and FHIR version.
4. Do not expand OpenMed import surface under spec 001.
5. Fix services README contradiction in a later authorized docs task (not T007 code change).

---

## 14. Acceptance

| Criterion | Result |
| --- | --- |
| openmed_bridge / fhir_gate classified as prototypes | **Met** |
| Documentation only; license note only; no weights | **Met** |
| No OpenMed import/execution | **Met** |

## 15. Rollback

Delete this file; revert program-memory pointer edits.

## 16. Notes (PASS WITH NOTES)

- Incomplete dependency pinning and contradictory services README.
- extract-pii client/server schema mismatch.
- FHIR R4B vs future commandF baseline unresolved.
- Localhost unauthenticated bridge is a trust seam vs target architecture.
