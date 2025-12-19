import { useMemo, useState } from "react";

import { Button } from "@/components/ui/button";
import { SearchInput } from "@/components/SearchInput";
import VariantSelector from "@/components/VariantSelector";
import { GameVariant } from "@/generated-types/GameVariant";
import { useCombinedBackups } from "@/hooks/useCombinedBackups";
import { useGameVariants } from "@/hooks/useGameVariants";
import { useSearch } from "@/hooks/useSearch";
import { toastCL } from "@/lib/utils";
import BackupFilter, {
  BackupFilterFn,
} from "./components/BackupFilter";
import { BackupsTable } from "./components/BackupsTable";
import { DeleteBackupDialog } from "./components/DeleteBackupDialog";
import { NewBackupDialog } from "./components/NewBackupDialog";
import { RestoreBackupDialog } from "./components/RestoreBackupDialog";
import { CombinedBackup } from "./lib/types/backups";

function formatTimestampForSearch(timestamp: bigint): string {
  const date = new Date(Number(timestamp) * 1000);

  const day = date.getDate().toString().padStart(2, "0");
  const month = date.toLocaleString("default", { month: "long" });
  const year = date.getFullYear();
  const hours = date.getHours().toString().padStart(2, "0");
  const minutes = date.getMinutes().toString().padStart(2, "0");
  const seconds = date.getSeconds().toString().padStart(2, "0");

  return `${day} ${month}, ${year}, ${hours}:${minutes}:${seconds}`;
}

function BackupsPage() {
  const { gameVariants, isLoading: gameVariantsLoading } =
    useGameVariants();
  const [selectedVariant, setSelectedVariant] =
    useState<GameVariant | null>(null);
  const [newManualDialogOpen, setNewManualDialogOpen] =
    useState(false);
  const [deleteDialogOpen, setDeleteDialogOpen] = useState(false);
  const [restoreDialogOpen, setRestoreDialogOpen] = useState(false);
  const [selectedBackup, setSelectedBackup] =
    useState<CombinedBackup | null>(null);
  const [appliedFilter, setAppliedFilter] = useState<BackupFilterFn>(
    () => (_backup: CombinedBackup) => true,
  );

  const activeVariant = (selectedVariant ?? gameVariants[0]?.id)!;

  const {
    combinedBackups,
    isLoading: backupsLoading,
    createManualBackup,
    isCreatingManualBackup,
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

  const {
    searchQuery,
    setSearchQuery,
    filteredItems: searchedBackups,
  } = useSearch(combinedBackups, {
    searchFn: (backup, query) => {
      const formattedTimestamp = formatTimestampForSearch(
        backup.timestamp,
      );
      return (
        backup.name.toLowerCase().includes(query) ||
        backup.notes?.toLowerCase().includes(query) ||
        formattedTimestamp.toLowerCase().includes(query)
      );
    },
  });

  const filteredBackups = useMemo(() => {
    return searchedBackups.filter(appliedFilter);
  }, [searchedBackups, appliedFilter]);

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
    <div className="flex flex-col gap-4">
      <div className="flex items-center gap-4">
        <VariantSelector
          gameVariants={gameVariants}
          selectedVariant={selectedVariant}
          onVariantChange={setSelectedVariant}
          isLoading={gameVariantsLoading}
        />
        <Button
          onClick={() => setNewManualDialogOpen(true)}
          disabled={!gameVariants.length || gameVariantsLoading}
        >
          New Backup
        </Button>
      </div>
      <SearchInput
        value={searchQuery}
        onChange={setSearchQuery}
        placeholder="Search backups..."
        className="mb-4"
      />
      <BackupFilter
        onChange={(filter) =>
          setAppliedFilter((_prev: BackupFilterFn) => filter)
        }
      />
      {backupsLoading ? (
        <p>Loading...</p>
      ) : (
        activeVariant && (
          <BackupsTable
            rows={filteredBackups}
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
        isCreating={isCreatingManualBackup}
      />
    </div>
  );
}

export default BackupsPage;
