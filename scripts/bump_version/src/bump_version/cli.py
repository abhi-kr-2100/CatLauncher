#!/usr/bin/env python3

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Optional

import semver
import tomlkit

"""Command-line interface for bumping the version across the project."""

BASE_PATH = Path(__file__).parent.parent.parent.parent.parent
"""Path: The root directory of the project."""


def update_readme(readme_path: Path, new_version: str) -> None:
    """Updates the version number in the README.md file.

    Args:
        readme_path: The path to the README.md file.
        new_version: The new version string to insert.
    """
    content = readme_path.read_text()
    content = re.sub(
        r"cat-launcher_\d+\.\d+(\.\d+)?_amd64\.AppImage",
        f"cat-launcher_{new_version}_amd64.AppImage",
        content,
    )
    readme_path.write_text(content)


def update_json(json_path: Path, new_version: str) -> None:
    """Updates the version field in a JSON file.

    Args:
        json_path: The path to the JSON file.
        new_version: The new version string to set.
    """
    with json_path.open("r") as f:
        content = json.load(f)
    content["version"] = new_version
    with json_path.open("w") as f:
        json.dump(content, f, indent=2)
        f.write("\n")


def update_cargo_toml(toml_path: Path, new_version: str) -> None:
    """Updates the package version in a Cargo.toml file.

    Args:
        toml_path: The path to the Cargo.toml file.
        new_version: The new version string to set.
    """
    with toml_path.open("r") as f:
        doc = tomlkit.load(f)
    doc["package"]["version"] = new_version
    toml_path.write_text(tomlkit.dumps(doc))


def update_cargo_lock(lock_path: Path, new_version: str) -> None:
    """Updates the cat-launcher package version in a Cargo.lock file.

    Args:
        lock_path: The path to the Cargo.lock file.
        new_version: The new version string to set.
    """
    with lock_path.open("r") as f:
        doc = tomlkit.load(f)

    for package in doc["package"]:
        if package.get("name") == "cat-launcher":
            package["version"] = new_version
            break

    lock_path.write_text(tomlkit.dumps(doc))


def main(args: Optional[list[str]] = None) -> None:
    """Main entry point for the version bumping script.

    Parses command line arguments, validates the new version, and updates
    all necessary files in the project.

    Args:
        args: Optional list of command line arguments. Defaults to sys.argv[1:].
    """
    parser = argparse.ArgumentParser(description="Bump version in all project files")
    parser.add_argument("version", help="New version (e.g., 0.13.0 or 0.13)")
    parsed_args = parser.parse_args(args)

    try:
        semver.VersionInfo.parse(parsed_args.version)
    except ValueError:
        print(
            "Error: Version must be a valid semantic version (e.g., 0.13.0)",
            file=sys.stderr,
        )
        sys.exit(1)

    readme_path = BASE_PATH / "README.md"
    package_json_path = BASE_PATH / "cat-launcher" / "package.json"
    cargo_lock_path = BASE_PATH / "cat-launcher" / "src-tauri" / "Cargo.lock"
    cargo_toml_path = BASE_PATH / "cat-launcher" / "src-tauri" / "Cargo.toml"
    tauri_conf_path = BASE_PATH / "cat-launcher" / "src-tauri" / "tauri.conf.json"

    update_readme(readme_path, parsed_args.version)
    print(f"Updated {readme_path}")

    update_json(package_json_path, parsed_args.version)
    print(f"Updated {package_json_path}")

    update_cargo_lock(cargo_lock_path, parsed_args.version)
    print(f"Updated {cargo_lock_path}")

    update_cargo_toml(cargo_toml_path, parsed_args.version)
    print(f"Updated {cargo_toml_path}")

    update_json(tauri_conf_path, parsed_args.version)
    print(f"Updated {tauri_conf_path}")

    print(f"\nVersion bumped to {parsed_args.version}")


if __name__ == "__main__":
    main()
