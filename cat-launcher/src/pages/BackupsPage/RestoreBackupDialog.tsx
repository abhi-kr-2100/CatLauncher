import { ConfirmationDialog } from "@/components/ui/ConfirmationDialog";

interface RestoreBackupDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onRestore: () => void;
}

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
