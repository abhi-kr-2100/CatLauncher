# Schema Validator

A tool to verify SQLite schema compatibility between two Git commits.

## Installation

```bash
uv sync
```

## Usage

```bash
uv run verify-schema <commit_prev> <commit_curr>
```

## Example

```bash
uv run verify-schema 6eacef8 bef5efc
```

## How it works

The script:
1. Retrieves the schema at the previous commit
2. Retrieves the schema at the current commit
3. Creates a temporary SQLite database
4. Executes the previous schema
5. Executes the current schema on top of it
6. Reports success if both executions complete without errors
