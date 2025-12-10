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

interface UseCombinedBackupsOptions {
  onDeleteSuccess?: () => void;
  onDeleteError?: (error: unknown) => void;
  onRestoreSuccess?: () => void;
  onRestoreError?: (error: unknown) => void;
  onCreateSuccess?: () => void;
  onCreateError?: (error: unknown) => void;
}

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

  const { deleteBackup: deleteAutoBackup } = useDeleteBackup(variant, {
    onSuccess: onDeleteSuccess,
    onError: onDeleteError,
  });

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

  const { createManualBackup } = useCreateManualBackup(variant, {
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
      ...manualBackups.map((b) => ({ ...b, type: "manual" as const })),
    ];
  }, [backups, manualBackups, backupsLoading, manualBackupsLoading]);

  const deleteBackup = (backup: CombinedBackup) => {
    if (backup.type === "manual") {
      deleteManualBackup(Number(backup.id));
    } else {
      deleteAutoBackup(Number(backup.id));
    }
  };

  const restoreBackup = (backup: CombinedBackup) => {
    if (backup.type === "manual") {
      restoreManualBackup(Number(backup.id));
    } else {
      restoreAutoBackup(Number(backup.id));
    }
  };

  return {
    combinedBackups,
    isLoading: backupsLoading || manualBackupsLoading,
    createManualBackup,
    deleteBackup,
    restoreBackup,
  };
}
