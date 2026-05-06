import { ConfirmationDialog } from "@/components/ui/ConfirmationDialog";

/**
 * Props for the {@link RestoreBackupDialog} component.
 */
interface RestoreBackupDialogProps {
  /** Whether the dialog is currently open. */
  open: boolean;
  /** Callback triggered when the dialog's open state changes. */
  onOpenChange: (open: boolean) => void;
  /** Callback triggered when the user confirms the restoration. */
  onRestore: () => void;
}

/**
 * A confirmation dialog for restoring a backup.
 * Warns the user that this action will overwrite current save files.
 *
 * @param props - Component properties.
 * @returns A React element rendering the restoration dialog.
 */
export function RestoreBackupDialog({
  open,
  onOpenChange,
  onRestore,
}: RestoreBackupDialogProps) {
  return (
    <ConfirmationDialog
      open={open}
      onOpenChange={onOpenChange}
      onConfirm={onRestore}
      title="Are you sure?"
      description="This will overwrite your current save files with the selected backup."
      confirmText="Restore"
    />
  );
}
