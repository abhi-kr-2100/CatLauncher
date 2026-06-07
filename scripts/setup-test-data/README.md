# Setup Test Data

Downloads release assets from game repositories for testing purposes.

## Usage

```bash
cd scripts/setup-test-data
uv run setup-test-data
```

### Options

- `--variants`: Game variants to fetch releases for (default: all)
- `--stable-count`: Number of stable releases to select per variant (default: 5)
- `--rc-count`: Number of release candidates to select per variant (default: 3)
- `--experimental-count`: Number of experimental releases to select per variant (default: 2)
- `--output-dir`: Output directory for downloaded assets (default: cat-launcher/src-tauri/src/infra/testing/data/assets)
- `--resources-dir`: Resources directory for releases metadata (default: cat-launcher/src-tauri)

### Example

```bash
# Download assets for all variants with default counts
uv run setup-test-data

# Download assets for only DDA and BN with custom counts
uv run setup-test-data --variants dda bn --stable-count 3 --experimental-count 1

# Use a custom output directory
uv run setup-test-data --output-dir ./test-assets
```

## Release Type Classification

Release types are determined per variant, matching the Rust logic in `game_variant.rs`:

- **DDA**: `!prerelease` → Stable, `prerelease + "experimental" in tag` → Experimental, else → Release Candidate
- **BN/TLG**: `!prerelease` → Stable, `prerelease` → Experimental

## Platform Asset Selection

Asset filenames are matched per platform, matching the Rust logic in `game_release/utils.rs`:

| Variant | Windows | macOS | Linux |
|---------|---------|-------|-------|
| DDA | `windows-with-graphics-and-sounds` | `osx-with-graphics` or `osx-terminal-only` | `linux-with-graphics-and-sounds` |
| BN | `windows-tiles` | `osx-tiles-arm` or `osx-tiles-x64` | `linux-tiles` |
| TLG | `windows-tiles-sounds-x64-msvc` | `osx-tiles-universal` | `linux-tiles-sounds` |

## Authentication

The script uses the `GITHUB_TOKEN` environment variable for authenticated requests to avoid rate limiting. Without a token, you'll be limited to 60 requests per hour.

```bash
export GITHUB_TOKEN=your_token_here
uv run setup-test-data
```

## Output

The script downloads actual binary release assets and saves release metadata:

```
cat-launcher/src-tauri/src/infra/testing/data/assets/
├── dda/
│   ├── cdda-windows-with-graphics-and-sounds-x64-*.zip
│   ├── cdda-linux-with-graphics-and-sounds-x64-*.tar.gz
│   ├── cdda-osx-with-graphics-universal-*.dmg
│   └── ...
├── bn/
│   ├── cbn-windows-tiles-x64-*.zip
│   ├── cbn-linux-tiles-x64-*.tar.gz
│   ├── cbn-osx-tiles-*.dmg
│   └── ...
└── tlg/
    ├── tlg-windows-tiles-sounds-x64-msvc-*.zip
    ├── tlg-linux-tiles-sounds-x64-*.tar.gz
    ├── tlg-osx-tiles-universal-*.dmg
    └── ...

cat-launcher/src-tauri/releases/
├── dark_days_ahead.json
├── bright_nights.json
└── the_last_generation.json
```
