import { useState } from "react";

import { Combobox } from "@/components/ui/combobox";
import { Button } from "@/components/ui/button";
import { useGameVariants } from "@/hooks/useGameVariants";
import { useCombinedBackups } from "@/hooks/useCombinedBackups";
import { GameVariant } from "@/generated-types/GameVariant";
import { toastCL } from "@/lib/utils";
import { BackupsTable } from "./BackupsTable";
import { CombinedBackup } from "./types/backups";
import { DeleteBackupDialog } from "./DeleteBackupDialog";
import { RestoreBackupDialog } from "./RestoreBackupDialog";
import { NewBackupDialog } from "./NewBackupDialog";

function BackupsPage() {
  const { gameVariants, isLoading: gameVariantsLoading } = useGameVariants();
  const [selectedVariant, setSelectedVariant] = useState<GameVariant | null>(
    null
  );
  const [newManualDialogOpen, setNewManualDialogOpen] = useState(false);
  const [deleteDialogOpen, setDeleteDialogOpen] = useState(false);
  const [restoreDialogOpen, setRestoreDialogOpen] = useState(false);
  const [selectedBackup, setSelectedBackup] = useState<CombinedBackup | null>(
    null,
  );

  const activeVariant = (selectedVariant ?? gameVariants[0]?.id)!;

  const {
    combinedBackups,
    isLoading: backupsLoading,
    createManualBackup,
    deleteBackup,
    restoreBackup,
  } = useCombinedBackups(activeVariant, {
    onDeleteError: (error) => {
      toastCL("error", "Failed to delete backup", error);
    },
    onRestoreSuccess: () => {
      toastCL("success", "Backup restored successfully");
    },
    onRestoreError: (error) => {
      toastCL("error", "Failed to restore backup", error);
    },
    onCreateSuccess: () => {
      toastCL("success", "Manual backup created successfully");
      setNewManualDialogOpen(false);
    },
    onCreateError: (error) => {
      toastCL("error", "Failed to create manual backup", error);
    },
  });

  const handleSave = (values: { name: string; notes?: string }) => {
    createManualBackup({
      name: values.name,
      notes: values.notes,
    });
  };

  const openDeleteDialog = (backup: CombinedBackup) => {
    setSelectedBackup(backup);
    setDeleteDialogOpen(true);
  };

  const openRestoreDialog = (backup: CombinedBackup) => {
    setSelectedBackup(backup);
    setRestoreDialogOpen(true);
  };

  return (
    <div className="flex flex-col gap-4 p-2">
      <div className="flex items-center gap-4">
        <Combobox
          items={gameVariants.map((v) => ({
            value: v.id,
            label: v.name,
          }))}
          value={selectedVariant ?? undefined}
          onChange={(value) => setSelectedVariant(value as GameVariant)}
          placeholder={
            gameVariantsLoading ? "Loading..." : "Select a game variant"
          }
          disabled={gameVariantsLoading}
          autoselect={true}
          className="w-72"
        />
        <Button
          onClick={() => setNewManualDialogOpen(true)}
          disabled={!gameVariants.length || gameVariantsLoading}
        >
          New Backup
        </Button>
      </div>
      {backupsLoading ? (
        <p>Loading...</p>
      ) : (
        activeVariant && (
          <BackupsTable
            rows={combinedBackups}
            onDeleteClick={openDeleteDialog}
            onRestoreClick={openRestoreDialog}
          />
        )
      )}
      <DeleteBackupDialog
        open={deleteDialogOpen}
        onOpenChange={setDeleteDialogOpen}
        onDelete={() => {
          if (selectedBackup) {
            deleteBackup(selectedBackup);
          }
        }}
      />
      <RestoreBackupDialog
        open={restoreDialogOpen}
        onOpenChange={setRestoreDialogOpen}
        onRestore={() => {
          if (selectedBackup) {
            restoreBackup(selectedBackup);
          }
        }}
      />
      <NewBackupDialog
        open={newManualDialogOpen}
        onOpenChange={setNewManualDialogOpen}
        onSave={handleSave}
        variant={activeVariant}
      />
    </div>
  );
}

export default BackupsPage;
