# Obsidian setup

## Vault root

Open this folder as the Obsidian vault:

```text
C:\Projects\Fanatir-Ecosystem
```

Do **not** treat the ecosystem root as a git repository. Do **not** commit a parent-level `.obsidian/` folder.

## Where program memory lives

Fanatir program memory:

```text
Fanatir/docs/program-memory
```

Fehrest and DeepMed-AI remain separately versioned repositories under the same vault for cross-repo reading only.

## Sync and clinical data

- Obsidian Sync must **never** sync patient PHI from this development vault.
- The Obsidian account is **not** an authorization boundary for Fanatir clinical data.
- Keep PHI, patient documents, secrets, credentials, API keys, and private datasets out of this vault entirely.
