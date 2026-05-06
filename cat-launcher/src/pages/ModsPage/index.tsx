import { useState } from "react";

import VariantSelector from "@/components/VariantSelector";
import type { GameVariant } from "@/generated-types/GameVariant";
import { useGameVariants } from "@/hooks/useGameVariants";
import ModsList from "./ModsList";

/**
 * The main page component for the Mods section.
 * Provides a variant selector and displays a list of mods for the selected variant.
 *
 * @returns A React element representing the mods page.
 */
function ModsPage() {
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
      {selectedVariant && <ModsList variant={selectedVariant} />}
    </div>
  );
}

export default ModsPage;
