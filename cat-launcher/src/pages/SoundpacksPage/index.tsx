import { useState } from "react";

import VariantSelector from "@/components/VariantSelector";
import { GameVariant } from "@/generated-types/GameVariant";
import { useGameVariants } from "@/hooks/useGameVariants";
import SoundpacksList from "./SoundpacksList";

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
