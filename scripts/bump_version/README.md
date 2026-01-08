# bump-version

Automatically bump version across all project files.

## Usage

```bash
uv run bump-version <version>
```

Where `<version>` is the new version in format `X.Y` or `X.Y.Z` (e.g., `0.13.0` or `0.13`).

## Files Updated

- `README.md` - Updates the AppImage filename in installation instructions
- `cat-launcher/package.json` - Updates the package version
- `cat-launcher/src-tauri/Cargo.toml` - Updates the crate version
- `cat-launcher/src-tauri/Cargo.lock` - Updates the locked cat-launcher package version
- `cat-launcher/src-tauri/tauri.conf.json` - Updates the Tauri app version

## Example

```bash
uv run bump-version 0.13.0
```
