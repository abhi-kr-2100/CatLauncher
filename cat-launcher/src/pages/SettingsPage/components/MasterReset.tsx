import { AlertTriangle } from "lucide-react";
import { useCallback, useState } from "react";

import { Alert, AlertDescription } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import { ConfirmationDialog } from "@/components/ui/ConfirmationDialog";
import { FieldLabel } from "@/components/ui/field";
import { Label } from "@/components/ui/label";
import VariantSelector from "@/components/VariantSelector";
import { GameVariant } from "@/generated-types/GameVariant";
import { useGameVariants } from "@/hooks/useGameVariants";
import { toastCL } from "@/lib/utils";
import { useMasterResetMutation } from "../hooks";

export function MasterReset() {
  const onVariantsFetchError = useCallback((error: unknown) => {
    toastCL("error", "Failed to fetch game variants.", error);
  }, []);

  const { gameVariants, isLoading: isLoadingGameVariants } =
    useGameVariants({
      onFetchError: onVariantsFetchError,
    });
  const [selectedVariant, setSelectedVariant] =
    useState<GameVariant | null>(null);

  const [isConfirmationDialogOpen, setIsConfirmationDialogOpen] =
    useState(false);

  const onResetSuccess = useCallback(() => {
    toastCL("success", "Reset successful.");
    setIsConfirmationDialogOpen(false);
  }, []);

  const onResetError = useCallback((error: Error) => {
    toastCL("error", "Failed to reset.", error);
  }, []);

  const { reset, isResetting } = useMasterResetMutation(
    onResetSuccess,
    onResetError,
  );

  const handleConfirm = useCallback(() => {
    if (selectedVariant) {
      reset(selectedVariant);
    }
  }, [selectedVariant, reset]);

  return (
    <div className="space-y-4">
      <FieldLabel className="text-base">Master Reset</FieldLabel>

      <Alert variant="destructive">
        <AlertTriangle className="h-4 w-4" />
        <AlertDescription>
          <p>Use this option if your game is broken.</p>
          <p>
            All custom configs, mods, soundpacks, tilesets, etc. will
            be deleted. Saves and backups will be preserved.
          </p>
          <p>
            Worlds that use custom mods, won't be playable until you
            reinstall them.
          </p>
        </AlertDescription>
      </Alert>

      <div className="flex items-end gap-4">
        <div className="space-y-2">
          <Label>Game Variant</Label>
          <VariantSelector
            gameVariants={gameVariants}
            selectedVariant={selectedVariant}
            onVariantChange={setSelectedVariant}
            isLoading={isLoadingGameVariants}
          />
        </div>

        <Button
          variant="destructive"
          disabled={!selectedVariant || isResetting}
          onClick={() => setIsConfirmationDialogOpen(true)}
          type="button"
        >
          {isResetting ? "Resetting..." : "Reset"}
        </Button>
      </div>

      <ConfirmationDialog
        open={isConfirmationDialogOpen}
        onOpenChange={setIsConfirmationDialogOpen}
        onConfirm={handleConfirm}
        title="Are you sure?"
        description="This action cannot be undone."
        confirmText="Reset"
        cancelText="Cancel"
      />
    </div>
  );
}
