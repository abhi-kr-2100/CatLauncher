import { DataTable } from "@/components/DataTable";
import { columns } from "./columns";
import { CombinedBackup } from "../lib/types/backups";

interface BackupsTableProps {
  rows: CombinedBackup[];
  onDeleteClick: (backup: CombinedBackup) => void;
  onRestoreClick: (backup: CombinedBackup) => void;
}

export function BackupsTable({
  rows,
  onDeleteClick,
  onRestoreClick,
}: BackupsTableProps) {
  return (
    <DataTable
      columns={columns({
        openDeleteDialog: onDeleteClick,
        openRestoreDialog: onRestoreClick,
      })}
      data={rows}
      initialSort={[{ id: "timestamp", desc: true }]}
    />
  );
}
