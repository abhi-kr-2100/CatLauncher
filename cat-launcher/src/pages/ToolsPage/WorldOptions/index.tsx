import { useCallback } from "react";

import { Card, CardContent } from "@/components/ui/card";
import { useGameVariants } from "@/hooks/useGameVariants";
import { toastCL } from "@/lib/utils";

import { useUserInput } from "./useUserInput";
import { WorldOptionsForm } from "./WorldOptionsForm";
import { WorldSelector } from "./WorldSelector";

export default function WorldOptions() {
  const onVariantsFetchError = useCallback((error: unknown) => {
    toastCL("error", "Failed to fetch game variants.", error);
  }, []);

  const { gameVariants, isLoading: isLoadingVariants } =
    useGameVariants({
      onFetchError: onVariantsFetchError,
    });

  const { selectedVariant, selectedWorld, setVariant, setWorld } =
    useUserInput();

  return (
    <div className="space-y-6">
      <WorldSelector
        gameVariants={gameVariants}
        selectedVariant={selectedVariant}
        onVariantChange={setVariant}
        selectedWorld={selectedWorld}
        onWorldChange={setWorld}
        isLoadingVariants={isLoadingVariants}
      />

      {!selectedVariant && (
        <Card className="border-dashed">
          <CardContent className="py-10 text-center text-muted-foreground">
            Please select a game variant to see available worlds.
          </CardContent>
        </Card>
      )}

      {selectedVariant && !selectedWorld && (
        <Card className="border-dashed">
          <CardContent className="py-10 text-center text-muted-foreground">
            Select a world to configure its options.
          </CardContent>
        </Card>
      )}

      {selectedVariant && selectedWorld && (
        <WorldOptionsForm
          key={`${selectedVariant}-${selectedWorld}`}
          variant={selectedVariant}
          world={selectedWorld}
        />
      )}
    </div>
  );
}
