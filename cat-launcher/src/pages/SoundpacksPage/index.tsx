import { useState } from "react";

import VariantSelector from "@/components/VariantSelector";
import { GameVariant } from "@/generated-types/GameVariant";
import { useGameVariants } from "@/hooks/useGameVariants";
import SoundpacksList from "./SoundpacksList";

/**
 * The SoundpacksPage component provides a user interface for browsing and managing soundpacks
 * for different game variants. It allows users to select a game variant and displays
 * the corresponding list of soundpacks.
 *
 * @returns A React component that renders the soundpack selection and listing interface.
 */
function SoundpacksPage() {
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
      {selectedVariant && (
        <SoundpacksList variant={selectedVariant} />
      )}
    </div>
  );
}

export default SoundpacksPage;
