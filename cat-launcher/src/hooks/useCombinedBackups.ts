import { useMemo } from "react";

import { GameVariant } from "@/generated-types/GameVariant";
import { CombinedBackup } from "@/pages/BackupsPage/types/backups";
import { useBackups } from "./useBackups";
import { useManualBackups } from "./useManualBackups";
import { useDeleteBackup } from "./useDeleteBackup";
import { useDeleteManualBackup } from "./useDeleteManualBackup";
import { useRestoreBackup } from "./useRestoreBackup";
import { useRestoreManualBackup } from "./useRestoreManualBackup";
import { useCreateManualBackup } from "./useCreateManualBackup";

/**
 * Options for the {@link useCombinedBackups} hook.
 */
interface UseCombinedBackupsOptions {
  /**
   * Callback function executed when a backup is successfully deleted.
   */
  onDeleteSuccess?: () => void;
  /**
   * Callback function executed when backup deletion fails.
   */
  onDeleteError?: (error: unknown) => void;
  /**
   * Callback function executed when a backup is successfully restored.
   */
  onRestoreSuccess?: () => void;
  /**
   * Callback function executed when backup restoration fails.
   */
  onRestoreError?: (error: unknown) => void;
  /**
   * Callback function executed when a manual backup is successfully created.
   */
  onCreateSuccess?: () => void;
  /**
   * Callback function executed when manual backup creation fails.
   */
  onCreateError?: (error: unknown) => void;
}

/**
 * A custom hook that combines automatic and manual backups for a specific game variant.
 * It provides unified functions for listing, creating, deleting, and restoring backups.
 *
 * @param variant - The game variant to manage backups for.
 * @param options - Optional callbacks for various backup operations.
 * @returns An object containing the combined backups, loading state, and management functions.
 */
export function useCombinedBackups(
  variant: GameVariant,
  {
    onDeleteSuccess,
    onDeleteError,
    onRestoreSuccess,
    onRestoreError,
    onCreateSuccess,
    onCreateError,
  }: UseCombinedBackupsOptions = {},
) {
  const { backups, isLoading: backupsLoading } = useBackups(variant);
  const { manualBackups, isLoading: manualBackupsLoading } =
    useManualBackups(variant);

  const { deleteBackup: deleteAutoBackup } = useDeleteBackup(
    variant,
    {
      onSuccess: onDeleteSuccess,
      onError: onDeleteError,
    },
  );

  const { deleteManualBackup } = useDeleteManualBackup(variant, {
    onSuccess: onDeleteSuccess,
    onError: onDeleteError,
  });

  const { restoreBackup: restoreAutoBackup } = useRestoreBackup({
    onSuccess: onRestoreSuccess,
    onError: onRestoreError,
  });

  const { restoreManualBackup } = useRestoreManualBackup({
    onSuccess: onRestoreSuccess,
    onError: onRestoreError,
  });

  const { createManualBackup, isCreatingManualBackup } =
    useCreateManualBackup(variant, {
      onSuccess: onCreateSuccess,
      onError: onCreateError,
    });

  const combinedBackups = useMemo(() => {
    if (backupsLoading || manualBackupsLoading) {
      return [];
    }

    return [
      ...backups.map((b) => ({
        ...b,
        type: "automatic" as const,
        name: `Automatic-${b.id}`,
        notes: "Automatic backup",
      })),
      ...manualBackups.map((b) => ({
        ...b,
        type: "manual" as const,
      })),
    ];
  }, [backups, manualBackups, backupsLoading, manualBackupsLoading]);

  const deleteBackup = (backup: CombinedBackup) => {
    if (backup.type === "manual") {
      deleteManualBackup(backup.id);
    } else {
      deleteAutoBackup(backup.id);
    }
  };

  const restoreBackup = (backup: CombinedBackup) => {
    if (backup.type === "manual") {
      restoreManualBackup(backup.id);
    } else {
      restoreAutoBackup(backup.id);
    }
  };

  return {
    combinedBackups,
    isLoading: backupsLoading || manualBackupsLoading,
    createManualBackup,
    isCreatingManualBackup,
    deleteBackup,
    restoreBackup,
  };
}
