# Refactoring Report

This document outlines the inconsistencies found in the codebase when compared against the coding standards and advice mentioned in `AGENTS.md`.

## Frontend

### Data Fetching and Mutations

The following files use raw `useQuery` and `useMutation` hooks instead of the custom hook pattern that wraps these hooks and uses callbacks for error handling:

- `cat-launcher/src/hooks/useBackups.ts`
- `cat-launcher/src/hooks/useGameVariants.ts`
- `cat-launcher/src/hooks/useInstallAndMonitor.ts`

**Suggested Fix:**

Refactor the hooks to use the custom hook pattern as described in `AGENTS.md`. This involves:

1. Creating a custom hook that wraps `useQuery` or `useMutation`.
2. Using `useRef` and `useEffect` to handle error callbacks.

Example from `AGENTS.md`:

```typescript
export default function useGames(onGameLoadError?: (error: Error) => void) {
  const onGameLoadErrorRef = useRef(onGameLoadError);

  useEffect(() => {
    onGameLoadErrorRef.current = onGameLoadError;
  }, [onGameLoadError]);

  const { data, isLoading, error } = useQuery({
    queryKey: queryKeys.games(),
    queryFn: getGames,
  });

  useEffect(() => {
    if (error && onGameLoadErrorRef.current) {
      onGameLoadErrorRef.current(error);
    }
  }, [error]);

  return { data, isLoading, error };
}
```

## Backend

### Directory Structure

The `cat-launcher/src-tauri/src/active_release` feature is missing the following files:

- `types.rs`
- `lib.rs`

**Suggested Fix:**

Create the missing `types.rs` and `lib.rs` files in the `cat-launcher/src-tauri/src/active_release` directory.

### Error Handling

The `cat-launcher/src-tauri/src/active_release/repository/active_release_repository.rs` file uses `Box<dyn Error + Send + Sync>` for error handling, which is not in line with the `AGENTS.md` guidelines.

**Suggested Fix:**

Refactor the `ActiveReleaseRepositoryError` enum to use `thiserror` and the `#[from]` attribute to compose errors, as shown in the example below.

```rust
#[derive(thiserror::Error, Debug)]
pub enum ActiveReleaseRepositoryError {
  #[error("failed to get active release: {0}")]
  Get(#[from] r2d2::Error),

  #[error("failed to set active release: {0}")]
  Set(#[from] rusqlite::Error),
}
```
The `sqlite_active_release_repository.rs` will also need to be updated to map the errors correctly.
