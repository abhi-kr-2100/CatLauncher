# CatLauncher Scripts

A collection of utility scripts for CatLauncher development and maintenance.

## Available Scripts

### Schema Validator (`schema_validator/`)

Verifies that SQLite schema changes between two commits are compatible.

```bash
cd schema_validator
uv run verify-schema <commit_prev> <commit_curr>
```

**Example:**
```bash
cd schema_validator
uv run verify-schema 6eacef8 bef5efc
```

For more details, see [schema_validator/README.md](schema_validator/README.md).
