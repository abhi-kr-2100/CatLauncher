import { useState } from "react";

import { DataTable } from "@/components/DataTable";
import { columns } from "./columns";
import { useDeleteBackup } from "@/hooks/useDeleteBackup";
import { useRestoreBackup } from "@/hooks/useRestoreBackup";
import { DeleteBackupDialog } from "./DeleteBackupDialog";
import { RestoreBackupDialog } from "./RestoreBackupDialog";
import { BackupEntry } from "@/generated-types/BackupEntry";
import { useBackups } from "@/hooks/useBackups";
import { GameVariant } from "@/generated-types/GameVariant";
import { toastCL } from "@/lib/utils";

interface BackupsTableProps {
  variant: GameVariant;
}

export function BackupsTable({ variant }: BackupsTableProps) {
  const { backups, isLoading: backupsLoading } = useBackups(variant);
  const { deleteBackup } = useDeleteBackup(variant, {
    onError: (error) => {
      toastCL("error", "Failed to delete backup", error);
    },
  });
  const { restoreBackup } = useRestoreBackup(variant, {
    onSuccess: () => {
      toastCL("success", "Backup restored successfully");
    },
    onError: (error) => {
      toastCL("error", "Failed to restore backup", error);
    },
  });

  const [deleteDialogOpen, setDeleteDialogOpen] = useState(false);
  const [restoreDialogOpen, setRestoreDialogOpen] = useState(false);
  const [selectedBackup, setSelectedBackup] = useState<BackupEntry | null>(
    null
  );

  const handleDelete = () => {
    if (selectedBackup) {
      deleteBackup(Number(selectedBackup.id));
    }
  };

  const handleRestore = () => {
    if (selectedBackup) {
      restoreBackup(Number(selectedBackup.id));
    }
  };

  const openDeleteDialog = (backup: BackupEntry) => {
    setSelectedBackup(backup);
    setDeleteDialogOpen(true);
  };

  const openRestoreDialog = (backup: BackupEntry) => {
    setSelectedBackup(backup);
    setRestoreDialogOpen(true);
  };

  if (backupsLoading) {
    return <p>Loading...</p>;
  }

  return (
    <>
      <DataTable
        columns={columns({ openDeleteDialog, openRestoreDialog })}
        data={backups}
        initialSort={[{ id: "timestamp", desc: true }]}
      />
      <DeleteBackupDialog
        open={deleteDialogOpen}
        onOpenChange={setDeleteDialogOpen}
        onDelete={handleDelete}
      />
      <RestoreBackupDialog
        open={restoreDialogOpen}
        onOpenChange={setRestoreDialogOpen}
        onRestore={handleRestore}
      />
    </>
  );
}
