# Refactoring Report

This report details the inconsistencies found in the codebase and the suggested fixes to align with the coding standards mentioned in `AGENTS.md`.

## Frontend

### Data Fetching and Mutations

**Inconsistency:** Raw `useQuery` and `useMutation` hooks are not used. Instead create custom hooks that wrap `useQuery` and `useMutation`. The custom hooks should take one or more callbacks depending on the number of different possible errors.

**Files with Inconsistencies:**

- `cat-launcher/src/hooks/useBackups.ts`
- `cat-launcher/src/hooks/useManualBackups.ts`

**Suggested Fix:**

- Modify `useBackups.ts` to accept an `onError` callback and handle errors as described in `AGENTS.md`.
- Modify `useManualBackups.ts` to accept an `onError` callback and handle errors as described in `AGENTS.md`.

### `useBackups.ts`

**Current Implementation:**

```typescript
export function useBackups(variant: GameVariant) {
  const {
    data: backups = [],
    isLoading,
    isError,
    error,
  } = useQuery({
    queryKey: queryKeys.backups(variant),
    queryFn: () => listBackupsForVariant(variant),
  });

  return {
    backups,
    isLoading,
    isError,
    error,
  };
}
```

**Suggested Fix:**

```typescript
import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { GameVariant } from "@/generated-types/GameVariant";
import { listBackupsForVariant } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useBackups(
  variant: GameVariant,
  onBackupsLoadError?: (error: Error) => void,
) {
  const onBackupsLoadErrorRef = useRef(onBackupsLoadError);

  useEffect(() => {
    onBackupsLoadErrorRef.current = onBackupsLoadError;
  }, [onBackupsLoadError]);

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
    if (error && onBackupsLoadErrorRef.current) {
      onBackupsLoadErrorRef.current(error);
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

### `useManualBackups.ts`

**Current Implementation:**

```typescript
export function useManualBackups(variant: GameVariant) {
  const { data: manualBackups, isLoading } = useQuery({
    queryKey: queryKeys.manualBackups(variant),
    queryFn: () => listManualBackupsForVariant(variant),
  });

  return { manualBackups: manualBackups ?? [], isLoading };
}
```

**Suggested Fix:**

```typescript
import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { GameVariant } from "@/generated-types/GameVariant";
import { listManualBackupsForVariant } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useManualBackups(
  variant: GameVariant,
  onManualBackupsLoadError?: (error: Error) => void,
) {
  const onManualBackupsLoadErrorRef = useRef(onManualBackupsLoadError);

  useEffect(() => {
    onManualBackupsLoadErrorRef.current = onManualBackupsLoadError;
  }, [onManualBackupsLoadError]);

  const {
    data: manualBackups,
    isLoading,
    error,
  } = useQuery({
    queryKey: queryKeys.manualBackups(variant),
    queryFn: () => listManualBackupsForVariant(variant),
  });

  useEffect(() => {
    if (error && onManualBackupsLoadErrorRef.current) {
      onManualBackupsLoadErrorRef.current(error);
    }
  }, [error]);

  return { manualBackups: manualBackups ?? [], isLoading };
}
```
