import { useState } from "react";

import VariantSelector from "@/components/VariantSelector";
import { GameVariant } from "@/generated-types/GameVariant";
import { useGameVariants } from "@/hooks/useGameVariants";
import ModsList from "./ModsList";

function ModsPage() {
  const { gameVariants, isLoading: gameVariantsLoading } = useGameVariants();
  const [selectedVariant, setSelectedVariant] = useState<GameVariant | null>(
    null,
  );

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
