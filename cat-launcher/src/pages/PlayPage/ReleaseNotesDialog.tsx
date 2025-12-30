import { useQuery } from "@tanstack/react-query";

import { ConfirmationDialog } from "@/components/ui/ConfirmationDialog";
import { GameVariant } from "@/generated-types/GameVariant";
import { fetchReleaseNotes } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

interface ReleaseNotesDialogProps {
  variant: GameVariant;
  releaseTagName: string;
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onConfirm: () => void;
  confirmText: string;
}

export function ReleaseNotesDialog({
  variant,
  releaseTagName,
  open,
  onOpenChange,
  onConfirm,
  confirmText,
}: ReleaseNotesDialogProps) {
  const { data: releaseNotes, isLoading } = useQuery({
    queryKey: queryKeys.releaseNotes(variant, releaseTagName),
    queryFn: () => fetchReleaseNotes(variant, releaseTagName),
    enabled: open,
  });

  return (
    <ConfirmationDialog
      open={open}
      onOpenChange={onOpenChange}
      onConfirm={onConfirm}
      title={`Release Notes for ${releaseTagName}`}
      description={
        isLoading
          ? "Loading..."
          : releaseNotes ?? "No release notes available."
      }
      confirmText={confirmText}
      cancelText="Close"
    />
  );
}
