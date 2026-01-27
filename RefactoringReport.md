# Refactoring Report

## Frontend

### `cat-launcher/src/hooks/useBackups.ts`

**Inconsistency:** The `useBackups` hook directly uses `useQuery` without the required error handling callback mechanism. The `AGENTS.md` file specifies that custom hooks should accept an `onError` callback and use `useRef` and `useEffect` to manage it.

**Suggested Fix:** Refactor the hook to accept an `onBackupLoadError` callback and implement the error handling logic as described in `AGENTS.md`.

```typescript
import { useQuery } from "@tanstack/react-query";
import { useRef, useEffect } from "react";

import { GameVariant } from "@/generated-types/GameVariant";
import { listBackupsForVariant } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useBackups(
  variant: GameVariant,
  onBackupLoadError?: (error: Error) => void,
) {
  const onBackupLoadErrorRef = useRef(onBackupLoadError);

  useEffect(() => {
    onBackupLoadErrorRef.current = onBackupLoadError;
  }, [onBackupLoadError]);

  const {
    data: backups = [],
    isLoading,
    isError,
    error,
  } = useQuery({
    queryKey: queryKeys.backups(variant),
    queryFn: () => listBackupsForVariant(variant),
  });

  useEffect(() => {
    if (error && onBackupLoadErrorRef.current) {
      onBackupLoadErrorRef.current(error);
    }
  }, [error]);

  return {
    backups,
    isLoading,
    isError,
    error,
  };
}
```

### `cat-launcher/src/hooks/useCreateManualBackup.ts`

**Inconsistency:** The `useCreateManualBackup` hook uses `useMutation` with `onSuccess` and `onError` callbacks passed directly to the hook. This is inconsistent with the `AGENTS.md` guideline to use a `useRef` and `useEffect` pattern for callbacks to prevent stale closures.

**Suggested Fix:** Refactor the hook to manage `onSuccess` and `onError` callbacks using the `useRef` and `useEffect` pattern as specified in `AGENTS.md`.

```typescript
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useRef, useEffect } from "react";

import { GameVariant } from "@/generated-types/GameVariant";
import { createManualBackupForVariant } from "@/lib/commands";
import { ManualBackupEntry } from "@/generated-types/ManualBackupEntry";
import { queryKeys } from "@/lib/queryKeys";

export function useCreateManualBackup(
  variant: GameVariant,
  onSuccess?: () => void,
  onError?: (error: unknown) => void,
) {
  const queryClient = useQueryClient();
  const onSuccessRef = useRef(onSuccess);
  const onErrorRef = useRef(onError);

  useEffect(() => {
    onSuccessRef.current = onSuccess;
  }, [onSuccess]);

  useEffect(() => {
    onErrorRef.current = onError;
  }, [onError]);

  const { mutate, isPending } = useMutation({
    mutationFn: async (values: { name: string; notes?: string }) => {
      await createManualBackupForVariant(
        variant,
        values.name,
        values.notes,
      );
    },
    onMutate: async (newBackup) => {
      await queryClient.cancelQueries({
        queryKey: queryKeys.manualBackups(variant),
      });

      const previousBackups = queryClient.getQueryData<
        ManualBackupEntry[]
      >(queryKeys.manualBackups(variant));

      queryClient.setQueryData<ManualBackupEntry[]>(
        queryKeys.manualBackups(variant),
        (old) => [
          ...(old ?? []),
          {
            id: BigInt(-1), // Temporary ID
            name: newBackup.name,
            game_variant: variant,
            timestamp: BigInt(Math.floor(Date.now() / 1000)),
            notes: newBackup.notes ?? null,
          },
        ],
      );

      return { previousBackups };
    },
    onError: (err, _newBackup, context) => {
      queryClient.setQueryData(
        queryKeys.manualBackups(variant),
        context?.previousBackups,
      );
      onErrorRef.current?.(err);
    },
    onSettled: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.manualBackups(variant),
      });
    },
    onSuccess: () => {
      onSuccessRef.current?.();
    },
  });

  return {
    createManualBackup: mutate,
    isCreatingManualBackup: isPending,
  };
}
```

## Backend

### `cat-launcher/src-tauri/src/backups/commands.rs`

**Inconsistency:** None.

**Analysis:** The file is fully compliant with the `AGENTS.md` guidelines.
- Tauri commands are straightforward and do not contain business logic.
- Arguments are prepared correctly before being passed to business logic functions.
- Framework-dependent data like `app_handle` is not passed to business logic.
- Error handling derives the necessary traits and is well-structured.
