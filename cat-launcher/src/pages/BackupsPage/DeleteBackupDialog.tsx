import { ConfirmationDialog } from "@/components/ui/ConfirmationDialog";

interface DeleteBackupDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onDelete: () => void;
}

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
