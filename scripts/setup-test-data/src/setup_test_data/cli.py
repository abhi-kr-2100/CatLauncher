import argparse
import json
import os
import sys
from concurrent.futures import ThreadPoolExecutor, as_completed
from dataclasses import dataclass
from enum import Enum
from pathlib import Path
from typing import Optional

import httpx
from tenacity import retry, stop_after_attempt, wait_exponential, RetryError

GITHUB_API = "https://api.github.com"

REPOS: dict[str, str] = {
    "dda": "CleverRaven/Cataclysm-DDA",
    "bn": "cataclysmbnteam/Cataclysm-BN",
    "tlg": "Cataclysm-TLG/Cataclysm-TLG",
}

EXPERIMENTAL_COUNT = 2

SCRIPT_DIR = Path(__file__).resolve().parent.parent.parent.parent.parent
DEFAULT_OUTPUT_DIR = SCRIPT_DIR / "cat-launcher/src-tauri/src/infra/testing/data/assets"
DEFAULT_METADATA_DIR = SCRIPT_DIR / "cat-launcher/src-tauri/src/infra/testing/data/metadata"
RESOURCES_DIR = SCRIPT_DIR / "cat-launcher/src-tauri"


class ReleaseType(Enum):
    STABLE = "stable"
    RELEASE_CANDIDATE = "release_candidate"
    EXPERIMENTAL = "experimental"


class OS(Enum):
    WINDOWS = "windows"
    LINUX = "linux"
    MACOS = "macos"


class Arch(Enum):
    X64 = "x86_64"
    ARM64 = "aarch64"


@dataclass
class Release:
    id: int
    tag_name: str
    name: str
    prerelease: bool
    body: Optional[str]
    published_at: str
    assets: list[dict]

    def __post_init__(self) -> None:
        if isinstance(self.id, str):
            self.id = int(self.id)


def determine_release_type(
    variant: str, tag_name: str, prerelease: bool
) -> ReleaseType:
    if variant == "dda":
        if not prerelease:
            return ReleaseType.STABLE
        if "experimental" in tag_name:
            return ReleaseType.EXPERIMENTAL
        return ReleaseType.RELEASE_CANDIDATE
    else:
        if prerelease:
            return ReleaseType.EXPERIMENTAL
        return ReleaseType.STABLE


def get_platform_asset_substrs(
    variant: str, os: OS, arch: Arch
) -> list[str]:
    mapping: dict[tuple[str, OS, Arch], list[str]] = {
        ("dda", OS.WINDOWS, Arch.X64): [
            "windows-with-graphics-and-sounds"
        ],
        ("dda", OS.WINDOWS, Arch.ARM64): [
            "windows-with-graphics-and-sounds"
        ],
        ("dda", OS.MACOS, Arch.X64): [
            "osx-with-graphics",
            "osx-terminal-only",
        ],
        ("dda", OS.MACOS, Arch.ARM64): [
            "osx-with-graphics",
            "osx-terminal-only",
        ],
        ("dda", OS.LINUX, Arch.X64): [
            "linux-with-graphics-and-sounds",
        ],
        ("dda", OS.LINUX, Arch.ARM64): [
            "linux-with-graphics-and-sounds",
        ],
        ("bn", OS.WINDOWS, Arch.X64): ["windows-tiles"],
        ("bn", OS.WINDOWS, Arch.ARM64): ["windows-tiles"],
        ("bn", OS.MACOS, Arch.X64): ["osx-tiles-x64"],
        ("bn", OS.MACOS, Arch.ARM64): ["osx-tiles-arm"],
        ("bn", OS.LINUX, Arch.X64): ["linux-tiles"],
        ("bn", OS.LINUX, Arch.ARM64): ["linux-tiles"],
        ("tlg", OS.WINDOWS, Arch.X64): [
            "windows-tiles-sounds-x64-msvc",
        ],
        ("tlg", OS.WINDOWS, Arch.ARM64): [
            "windows-tiles-sounds-x64-msvc",
        ],
        ("tlg", OS.MACOS, Arch.X64): ["osx-tiles-universal"],
        ("tlg", OS.MACOS, Arch.ARM64): ["osx-tiles-universal"],
        ("tlg", OS.LINUX, Arch.X64): ["linux-tiles-sounds"],
        ("tlg", OS.LINUX, Arch.ARM64): ["linux-tiles-sounds"],
    }
    return mapping.get((variant, os, arch), [])


def find_matching_asset(
    assets: list[dict], substrings: list[str]
) -> Optional[dict]:
    for asset in assets:
        name: str = asset.get("name", "")
        for substr in substrings:
            if substr in name:
                return asset
    return None


def variant_id_pascal(variant: str) -> str:
    return {"dda": "DarkDaysAhead", "bn": "BrightNights", "tlg": "TheLastGeneration"}[variant]


def get_headers() -> dict[str, str]:
    headers = {
        "Accept": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
    }
    token = os.environ.get("GITHUB_TOKEN")
    if token:
        headers["Authorization"] = f"Bearer {token}"
    return headers


@retry(
    wait=wait_exponential(multiplier=1, min=2, max=10),
    stop=stop_after_attempt(3),
)
def fetch_experimental_releases(client: httpx.Client, variant: str, count: int) -> list[Release]:
    repo = REPOS[variant]
    url = f"{GITHUB_API}/repos/{repo}/releases"
    params = {"per_page": 100}
    response = client.get(url, params=params)
    response.raise_for_status()
    data = response.json()

    releases: list[Release] = []
    for item in data:
        tag_name = item["tag_name"]
        prerelease = item["prerelease"]
        if determine_release_type(variant, tag_name, prerelease) == ReleaseType.EXPERIMENTAL:
            body = item.get("body")
            releases.append(
                Release(
                    id=int(item["id"]),
                    tag_name=tag_name,
                    name=item["name"],
                    prerelease=prerelease,
                    body=body if body and body.strip() else None,
                    published_at=item["published_at"],
                    assets=item.get("assets", []),
                )
            )
            if len(releases) >= count:
                break
    return releases


def load_stable_releases(variant: str) -> list[Release]:
    filepath = RESOURCES_DIR / "releases" / f"{variant_id_pascal(variant)}.json"
    if not filepath.exists():
        return []

    with open(filepath, "r") as f:
        data = json.load(f)

    releases: list[Release] = []
    for item in data:
        releases.append(
            Release(
                id=int(item["id"]),
                tag_name=item["tag_name"],
                name=item.get("name", item["tag_name"]),
                prerelease=item["prerelease"],
                body=item.get("body"),
                published_at=item.get("published_at", item.get("created_at", "")),
                assets=item.get("assets", []),
            )
        )
    return releases


@retry(
    wait=wait_exponential(multiplier=1, min=2, max=10),
    stop=stop_after_attempt(3),
)
def download_asset(
    client: httpx.Client, url: str, dest: Path, quiet: bool = False
) -> None:
    dest.parent.mkdir(parents=True, exist_ok=True)
    if dest.exists():
        if not quiet:
            print(f"    Already exists, skipping: {dest.name}")
        return

    with client.stream("GET", url) as response:
        response.raise_for_status()
        with open(dest, "wb") as f:
            for chunk in response.iter_bytes(chunk_size=8192):
                f.write(chunk)


def download_asset_task(
    client: httpx.Client, url: str, dest: Path, description: str, quiet: bool = False
) -> tuple[str, Optional[Exception]]:
    try:
        download_asset(client, url, dest, quiet)
        return (description, None)
    except Exception as e:
        return (description, e)


def save_releases_metadata(
    releases: list[Release], variant: str, metadata_dir: Path, quiet: bool = False
) -> None:
    metadata_dir.mkdir(parents=True, exist_ok=True)
    filepath = metadata_dir / f"{variant_id_pascal(variant)}.json"
    data: list[dict] = []
    for r in releases:
        assets_out: list[dict] = []
        for a in r.assets:
            assets_out.append(
                {
                    "id": int(a["id"]),
                    "browser_download_url": a.get(
                        "browser_download_url", ""
                    ),
                    "name": a.get("name", ""),
                    "digest": a.get("digest"),
                }
            )
        data.append(
            {
                "id": r.id,
                "tag_name": r.tag_name,
                "prerelease": r.prerelease,
                "body": r.body,
                "assets": assets_out,
                "created_at": r.published_at,
            }
        )

    with open(filepath, "w") as f:
        json.dump(data, f, indent=2)
    if not quiet:
        print(f"  Saved releases metadata: {filepath}")


def main(args: Optional[list[str]] = None) -> None:
    parser = argparse.ArgumentParser(
        description="Download release assets from game repositories for testing"
    )
    parser.add_argument(
        "--variants",
        nargs="+",
        choices=list(REPOS.keys()),
        default=list(REPOS.keys()),
        help="Game variants to fetch releases for",
    )
    parser.add_argument(
        "--experimental-count",
        type=int,
        default=EXPERIMENTAL_COUNT,
        help="Number of experimental releases to select per variant",
    )
    parser.add_argument(
        "--output-dir",
        type=Path,
        default=DEFAULT_OUTPUT_DIR,
        help="Output directory for downloaded assets",
    )
    parser.add_argument(
        "--metadata-dir",
        type=Path,
        default=DEFAULT_METADATA_DIR,
        help="Output directory for releases metadata",
    )
    parser.add_argument(
        "--quiet",
        action="store_true",
        help="Suppress output and only show errors",
    )
    parsed = parser.parse_args(args)

    output_dir = parsed.output_dir
    metadata_dir = parsed.metadata_dir
    quiet = parsed.quiet

    headers = get_headers()
    # Disable timeout as requested
    with httpx.Client(
        headers=headers, timeout=None, follow_redirects=True
    ) as client:
        all_download_tasks = []
        variant_releases: dict[str, list[Release]] = {}

        for variant in parsed.variants:
            if not quiet:
                print(f"\nProcessing {variant} ({REPOS[variant]})...")

            try:
                # Load stable releases from file
                stable_releases = load_stable_releases(variant)
                if not quiet:
                    print(f"  Loaded {len(stable_releases)} stable/RC releases from local files")

                # Fetch experimental releases from API
                experimental_releases = fetch_experimental_releases(
                    client, variant, parsed.experimental_count
                )
                if not quiet:
                    print(f"  Fetched {len(experimental_releases)} experimental releases from API")

                selected = stable_releases + experimental_releases
                variant_releases[variant] = selected

                for release in selected:
                    downloaded: set[str] = set()
                    for platform in OS:
                        for arch in Arch:
                            substrings = get_platform_asset_substrs(
                                variant, platform, arch
                            )
                            if not substrings:
                                continue

                            asset = find_matching_asset(
                                release.assets, substrings
                            )
                            if not asset:
                                continue

                            asset_name: str = asset.get("name", "")
                            cache_key = (
                                f"{platform.value}/{asset_name}"
                            )
                            if cache_key in downloaded:
                                continue
                            downloaded.add(cache_key)

                            download_url: str = asset.get(
                                "browser_download_url", ""
                            )
                            if not download_url:
                                continue

                            dest = output_dir / variant / asset_name
                            description = f"{variant} {release.tag_name} {platform.value}/{arch.value}: {asset_name}"
                            all_download_tasks.append((download_url, dest, description))

            except Exception as e:
                print(f"Error preparing tasks for {variant}: {e}", file=sys.stderr)

        if all_download_tasks:
            if not quiet:
                print(f"\nDownloading {len(all_download_tasks)} assets in parallel...")
            with ThreadPoolExecutor(max_workers=8) as executor:
                futures = [
                    executor.submit(
                        download_asset_task, client, url, dest, desc, quiet
                    )
                    for url, dest, desc in all_download_tasks
                ]
                for future in as_completed(futures):
                    desc, error = future.result()
                    if error:
                        print(f"Failed {desc}: {error}", file=sys.stderr)
                    elif not quiet:
                        print(f"Completed: {desc}")

        for variant, releases in variant_releases.items():
            try:
                save_releases_metadata(releases, variant, metadata_dir, quiet)
            except Exception as e:
                print(f"Error saving metadata for {variant}: {e}", file=sys.stderr)

    if not quiet:
        print("\nDone!")


if __name__ == "__main__":
    main()
