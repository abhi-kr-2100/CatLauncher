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

STABLE_COUNT = 5
RELEASE_CANDIDATE_COUNT = 3
EXPERIMENTAL_COUNT = 2

SCRIPT_DIR = Path(__file__).resolve().parent.parent.parent.parent.parent
OUTPUT_DIR = SCRIPT_DIR / "cat-launcher/src-tauri/src/infra/testing/data/assets"
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


def variant_id(variant: str) -> str:
    return {"dda": "dark_days_ahead", "bn": "bright_nights", "tlg": "the_last_generation"}[variant]


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
def fetch_releases(client: httpx.Client, repo: str) -> list[Release]:
    url = f"{GITHUB_API}/repos/{repo}/releases"
    params = {"per_page": 100}
    response = client.get(url, params=params)
    response.raise_for_status()
    data = response.json()

    releases: list[Release] = []
    for item in data:
        body = item.get("body")
        releases.append(
            Release(
                id=int(item["id"]),
                tag_name=item["tag_name"],
                name=item["name"],
                prerelease=item["prerelease"],
                body=body if body and body.strip() else None,
                published_at=item["published_at"],
                assets=item.get("assets", []),
            )
        )
    return releases


def classify_and_sort(
    releases: list[Release], variant: str
) -> dict[ReleaseType, list[Release]]:
    classified: dict[ReleaseType, list[Release]] = {
        ReleaseType.STABLE: [],
        ReleaseType.RELEASE_CANDIDATE: [],
        ReleaseType.EXPERIMENTAL: [],
    }

    for r in releases:
        rt = determine_release_type(variant, r.tag_name, r.prerelease)
        classified[rt].append(r)

    for rt in classified:
        classified[rt].sort(key=lambda x: x.published_at, reverse=True)

    return classified


def download_asset(
    client: httpx.Client, url: str, dest: Path
) -> None:
    dest.parent.mkdir(parents=True, exist_ok=True)
    if dest.exists():
        print(f"    Already exists, skipping: {dest.name}")
        return

    with client.stream("GET", url) as response:
        response.raise_for_status()
        with open(dest, "wb") as f:
            for chunk in response.iter_bytes(chunk_size=8192):
                f.write(chunk)


def download_asset_task(
    client: httpx.Client, url: str, dest: Path, description: str
) -> tuple[str, Optional[Exception]]:
    try:
        download_asset(client, url, dest)
        return (description, None)
    except Exception as e:
        return (description, e)


def save_releases_metadata(
    releases: list[Release], variant: str
) -> None:
    releases_dir = RESOURCES_DIR / "releases"
    releases_dir.mkdir(parents=True, exist_ok=True)

    filepath = releases_dir / f"{variant_id(variant)}.json"
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
    print(f"  Saved releases metadata: {filepath}")


def main(args: Optional[list[str]] = None) -> None:
    global OUTPUT_DIR, RESOURCES_DIR
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
        "--stable-count",
        type=int,
        default=STABLE_COUNT,
        help="Number of stable releases to select per variant",
    )
    parser.add_argument(
        "--rc-count",
        type=int,
        default=RELEASE_CANDIDATE_COUNT,
        help="Number of release candidates to select per variant",
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
        default=None,
        help="Output directory for downloaded assets",
    )
    parser.add_argument(
        "--resources-dir",
        type=Path,
        default=None,
        help="Resources directory for releases metadata",
    )
    parsed = parser.parse_args(args)

    if parsed.output_dir is not None:
        OUTPUT_DIR = parsed.output_dir
    if parsed.resources_dir is not None:
        RESOURCES_DIR = parsed.resources_dir

    headers = get_headers()
    with httpx.Client(
        headers=headers, timeout=60.0, follow_redirects=True
    ) as client:
        for variant in parsed.variants:
            repo = REPOS[variant]
            print(f"\nProcessing {variant} ({repo})...")

            try:
                all_releases = fetch_releases(client, repo)
                print(f"  Fetched {len(all_releases)} releases")

                classified = classify_and_sort(all_releases, variant)

                stable = classified[ReleaseType.STABLE][
                    : parsed.stable_count
                ]
                rc = classified[ReleaseType.RELEASE_CANDIDATE][
                    : parsed.rc_count
                ]
                experimental = classified[ReleaseType.EXPERIMENTAL][
                    : parsed.experimental_count
                ]

                selected = stable + rc + experimental

                print(
                    f"  Selected: {len(stable)} stable, {len(rc)} rc, {len(experimental)} experimental"
                )

                for release in selected:
                    print(
                        f"  Release: {release.tag_name} ({release.name})"
                    )

                    downloaded: set[str] = set()
                    download_tasks: list[tuple[str, str, Path, str]] = []

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

                            dest_dir = OUTPUT_DIR / variant
                            dest = dest_dir / asset_name
                            description = f"{platform.value}/{arch.value}: {asset_name}"
                            download_tasks.append((download_url, dest, description))

                    if download_tasks:
                        print(f"    Downloading {len(download_tasks)} assets in parallel...")
                        with ThreadPoolExecutor(max_workers=4) as executor:
                            futures = [
                                executor.submit(
                                    download_asset_task, client, url, dest, desc
                                )
                                for url, dest, desc in download_tasks
                            ]
                            for future in as_completed(futures):
                                desc, error = future.result()
                                if error:
                                    print(f"    Failed {desc}: {error}", file=sys.stderr)
                                else:
                                    print(f"    Completed: {desc}")

                save_releases_metadata(selected, variant)

            except RetryError as e:
                last_attempt = e.last_attempt
                if last_attempt.exception():
                    exc = last_attempt.exception()
                    if isinstance(exc, httpx.HTTPStatusError):
                        print(
                            f"  Error fetching releases for {variant}: {exc.response.status_code} - {exc.response.text}",
                            file=sys.stderr,
                        )
                    else:
                        print(
                            f"  Unexpected error for {variant}: {type(exc).__name__}: {exc}",
                            file=sys.stderr,
                        )
                else:
                    print(
                        f"  Retry failed for {variant}: {e}",
                        file=sys.stderr,
                    )
                sys.exit(1)
            except httpx.HTTPStatusError as e:
                print(
                    f"  Error fetching releases for {variant}: {e.response.status_code} - {e.response.text}",
                    file=sys.stderr,
                )
                sys.exit(1)
            except Exception as e:
                print(
                    f"  Unexpected error for {variant}: {type(e).__name__}: {e}",
                    file=sys.stderr,
                )
                sys.exit(1)

    print("\nDone!")
