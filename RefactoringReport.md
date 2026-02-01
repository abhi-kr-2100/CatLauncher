# Refactoring Report

This report details the inconsistencies found in the codebase compared to the standards defined in `AGENTS.md`.

## Frontend

### `cat-launcher/src/hooks/useBackups.ts`

*   **Issue**: The `useBackups` hook does not accept an `onError` callback, which is a violation of the `tanstack-query` guidelines.
*   **Suggested Fix**: Add an `onError` callback to the hook and call it when the query fails.

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

### `cat-launcher/src/hooks/useManualBackups.ts`

*   **Issue**: The `useManualBackups` hook does not accept an `onError` callback, which is a violation of the `tanstack-query` guidelines.
*   **Suggested Fix**: Add an `onError` callback to the hook and call it when the query fails.

## Backend

### `cat-launcher/src-tauri/src/active_release/commands.rs`

*   **Issue**: The `get_active_release` command combines argument preparation and the business logic call into a single line. This is a stylistic deviation from the `AGENTS.md` examples, which suggest separating them for clarity.
*   **Suggested Fix**: Separate the argument preparation and the business logic call into two distinct steps.

    ```rust
    #[command]
    pub async fn get_active_release(
      variant: GameVariant, active_release_repo: State<'_, SqliteActiveReleaseRepository>,
    ) -> Result<Option<String>, ActiveReleaseCommandError> {
      // Collect all arguments to pass to the business logic function
      let repo = active_release_repo.inner();

      // Call the business logic function
      let active_release = variant.get_active_release(repo).await?;

      // Return the results
      Ok(active_release)
    }
    ```

### `cat-launcher/src-tauri/src/backups/commands.rs`

*   **Issue**: The `list_backups_for_variant`, `delete_backup_by_id`, and `restore_backup_by_id` commands combine argument preparation and the business logic call into a single line. This is a stylistic deviation from the `AGENTS.md` examples.
*   **Suggested Fix**: Separate the argument preparation and the business logic call into two distinct steps.

### `cat-launcher/src-tauri/src/fetch_releases/commands.rs`

*   **Issue**: The `fetch_releases_for_variant` command passes a Tauri-dependent function (`app_handle.emit`) to the business logic. This creates a tight coupling between the business logic and the Tauri framework, which is a violation of the command guidelines.
*   **Suggested Fix**: The business logic should return the data to the command, and the command should be responsible for emitting the event.

*   **Issue**: The `fetch_release_notes` command combines argument preparation and the business logic call into a single line. This is a stylistic deviation from the `AGENTS.md` examples.
*   **Suggested Fix**: Separate the argument preparation and the business logic call into two distinct steps.
