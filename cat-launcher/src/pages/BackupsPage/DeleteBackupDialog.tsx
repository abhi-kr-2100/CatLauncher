import { ConfirmationDialog } from "@/components/ui/ConfirmationDialog";

/**
 * Props for the {@link DeleteBackupDialog} component.
 */
interface DeleteBackupDialogProps {
  /** Whether the dialog is currently open. */
  open: boolean;
  /** Callback triggered when the dialog's open state changes. */
  onOpenChange: (open: boolean) => void;
  /** Callback triggered when the user confirms the deletion. */
  onDelete: () => void;
}

/**
 * A confirmation dialog for deleting a backup.
 * Warns the user that this action is permanent and cannot be undone.
 *
 * @param props - Component properties.
 * @returns A React element rendering the deletion dialog.
 */
export function DeleteBackupDialog({
  open,
  onOpenChange,
  onDelete,
}: DeleteBackupDialogProps) {
  return (
    <ConfirmationDialog
      open={open}
      onOpenChange={onOpenChange}
      onConfirm={onDelete}
      title="Are you sure?"
      description="This action cannot be undone. This will permanently delete the backup."
      confirmText="Delete"
    />
  );
}
