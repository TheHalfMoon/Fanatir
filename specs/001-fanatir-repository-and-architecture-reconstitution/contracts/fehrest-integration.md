# Contract: Fehrest ↔ Fanatir

**Version**: 0.1.0-draft  
**Boundary**: Process/sidecar primary for Alpha; shared primitives via contracts package

## Fehrest owns

- Markdown vault content
- Human notes, quotations, wikilinks/backlinks
- Typed relationships with `origin` (human|extracted|inferred)
- Local search index for vault
- Portable export of vault subset

## Fanatir owns

- Project/Patient integration surfaces
- When/how Fehrest is launched or embedded
- Capability Gateway invocation of Fehrest workers
- Project Long Memory references to Fehrest exports (not Fehrest as clinical SoT)

## Alpha integration surface (minimum)

| Operation | Direction | Notes |
| --- | --- | --- |
| `vault.open` | Fanatir → Fehrest | Project-scoped or standalone |
| `note.upsert` | Bidirectional | Preserve provenance |
| `graph.query` | Fanatir → Fehrest | Bounded graph viz/search |
| `export.portable` | Fehrest → Fanatir | ExportManifest compatible |
| `memory.current_state` / `next_action` | Fehrest | Project memory helpers |

## Explicit exclusions

- Graphify fork/import (separate Fehrest spec)
- Fehrest as FHIR/Database/model-weight SoT
- Full Notion/Obsidian parity
