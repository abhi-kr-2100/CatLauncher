import { DataTable } from "@/components/DataTable";
import { columns } from "./columns";
import { CombinedBackup } from "./types/backups";

/**
 * Props for the {@link BackupsTable} component.
 */
interface BackupsTableProps {
  /** The list of backup entries to display in the table. */
  rows: CombinedBackup[];
  /** Callback triggered when the delete button is clicked for a backup. */
  onDeleteClick: (backup: CombinedBackup) => void;
  /** Callback triggered when the restore button is clicked for a backup. */
  onRestoreClick: (backup: CombinedBackup) => void;
}

/**
 * A table component that displays game backups.
 * Utilizes the {@link DataTable} component and predefined columns.
 *
 * @param props - Component properties.
 * @returns A React element rendering the backups data table.
 */
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
