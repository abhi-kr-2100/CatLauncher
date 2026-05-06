import { useState } from "react";

import VariantSelector from "@/components/VariantSelector";
import { GameVariant } from "@/generated-types/GameVariant";
import { useGameVariants } from "@/hooks/useGameVariants";
import TilesetsList from "./TilesetsList";

/**
 * The Tilesets page component, which allows users to select a game variant
 * and view/manage tilesets for that variant.
 *
 * @returns The tilesets page UI.
 */
function TilesetsPage() {
  const { gameVariants, isLoading: gameVariantsLoading } =
    useGameVariants();
  const [selectedVariant, setSelectedVariant] =
    useState<GameVariant | null>(null);

  return (
    <div className="flex flex-col gap-2">
      <VariantSelector
        gameVariants={gameVariants}
        selectedVariant={selectedVariant}
        onVariantChange={setSelectedVariant}
        isLoading={gameVariantsLoading}
      />
      {selectedVariant && <TilesetsList variant={selectedVariant} />}
    </div>
  );
}

export default TilesetsPage;
