import { RefreshCw } from "lucide-react";
import { useMemo } from "react";

import VariantSelector from "@/components/VariantSelector";
import { VirtualizedCombobox } from "@/components/virtualized-combobox";
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import type { GameVariant } from "@/generated-types/GameVariant";
import { GameVariantInfo } from "@/generated-types/GameVariantInfo";
import { cn, toastCL } from "@/lib/utils";

import { useWorlds } from "./useWorldOptions";

interface WorldSelectorProps {
  gameVariants: GameVariantInfo[];
  selectedVariant: GameVariant | null;
  onVariantChange: (variant: GameVariant) => void;
  selectedWorld: string | null;
  onWorldChange: (world: string | null) => void;
  isLoadingVariants: boolean;
}

export function WorldSelector({
  gameVariants,
  selectedVariant,
  onVariantChange,
  selectedWorld,
  onWorldChange,
  isLoadingVariants,
}: WorldSelectorProps) {
  const {
    data: worlds = [],
    isLoading: isLoadingWorlds,
    isFetching: isFetchingWorlds,
    refetch: refetchWorlds,
  } = useWorlds(selectedVariant, (error) => {
    toastCL("error", "Failed to fetch worlds.", error);
  });

  const worldItems = useMemo(
    () =>
      worlds.map((w) => ({
        value: w.name,
        label: w.name,
      })),
    [worlds],
  );

  return (
    <div className="flex flex-col gap-4 sm:flex-row sm:items-end">
      <div className="space-y-2">
        <Label>Game Variant</Label>
        <VariantSelector
          gameVariants={gameVariants}
          selectedVariant={selectedVariant}
          onVariantChange={onVariantChange}
          isLoading={isLoadingVariants}
        />
      </div>

      <div className="space-y-2">
        <Label>World</Label>
        <div className="flex items-center gap-2">
          <VirtualizedCombobox
            items={worldItems}
            value={selectedWorld ?? undefined}
            onChange={onWorldChange}
            placeholder={
              isLoadingWorlds ? "Loading worlds..." : "Select a world"
            }
            disabled={!selectedVariant || isLoadingWorlds}
            className="w-2xs"
          />
          <Button
            variant="outline"
            size="icon"
            onClick={() => void refetchWorlds()}
            disabled={!selectedVariant || isFetchingWorlds}
            title="Refresh worlds"
          >
            <RefreshCw
              className={cn(
                "h-4 w-4",
                isFetchingWorlds && "animate-spin",
              )}
            />
          </Button>
        </div>
      </div>
    </div>
  );
}
