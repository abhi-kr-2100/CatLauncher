#!/usr/bin/env python3

import argparse
import sqlite3
import sys
from pathlib import Path
from typing import Optional

from git import Repo

SCHEMA_PATH = "cat-launcher/src-tauri/schemas/schema.sql"


def get_schema_at_commit(repo: Repo, commit_sha: str) -> str:
    commit = repo.commit(commit_sha)
    blob = commit.tree / SCHEMA_PATH
    return blob.data_stream.read().decode("utf-8")


def execute_schema(conn: sqlite3.Connection, schema: str) -> bool:
    try:
        cursor = conn.cursor()
        cursor.executescript(schema)
        conn.commit()
        return True
    except sqlite3.Error:
        return False


def main(args: Optional[list[str]] = None) -> None:
    parser = argparse.ArgumentParser(
        description="Verify SQLite schema compatibility between two commits"
    )
    parser.add_argument("commit_prev", help="Previous commit SHA")
    parser.add_argument("commit_curr", help="Current commit SHA")
    parsed_args = parser.parse_args(args)

    current_dir = Path.cwd()
    repo = Repo(current_dir, search_parent_directories=True)

    try:
        schema_prev = get_schema_at_commit(repo, parsed_args.commit_prev)
        schema_curr = get_schema_at_commit(repo, parsed_args.commit_curr)
    except Exception as e:
        print(f"Error retrieving schemas from git: {e}", file=sys.stderr)
        sys.exit(1)

    db_path = Path.cwd() / "test_schema_compatibility.db"
    if db_path.exists():
        db_path.unlink()

    conn = sqlite3.connect(db_path)

    try:
        if not execute_schema(conn, schema_prev):
            print(
                f"FAIL: Previous schema ({parsed_args.commit_prev}) failed to execute",
                file=sys.stderr,
            )
            sys.exit(1)

        if not execute_schema(conn, schema_curr):
            print(
                f"FAIL: Current schema ({parsed_args.commit_curr}) failed to execute on top of previous schema",
                file=sys.stderr,
            )
            sys.exit(1)

        print(
            f"PASS: Schemas are compatible ({parsed_args.commit_prev} -> {parsed_args.commit_curr})"
        )
    finally:
        conn.close()
        if db_path.exists():
            db_path.unlink()


if __name__ == "__main__":
    main()
