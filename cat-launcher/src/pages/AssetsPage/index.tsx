import { useState } from "react";

import VariantSelector from "@/components/VariantSelector";
import type { GameVariant } from "@/generated-types/GameVariant";
import { useGameVariants } from "@/hooks/useGameVariants";
import AssetTypeSelector from "./AssetTypeSelector";
import { type AssetType, ASSET_COMPONENTS } from "./types";

function AssetsPage() {
  const { gameVariants, isLoading: gameVariantsLoading } =
    useGameVariants();
  const [selectedVariant, setSelectedVariant] =
    useState<GameVariant | null>(null);
  const [assetType, setAssetType] = useState<AssetType>("mods");

  return (
    <div className="flex flex-col gap-2">
      <div className="flex flex-row gap-2">
        <VariantSelector
          gameVariants={gameVariants}
          selectedVariant={selectedVariant}
          onVariantChange={setSelectedVariant}
          isLoading={gameVariantsLoading}
        />
        {selectedVariant && (
          <AssetTypeSelector
            selectedAssetType={assetType}
            onAssetTypeChange={setAssetType}
          />
        )}
      </div>
      {selectedVariant &&
        (() => {
          const Component = ASSET_COMPONENTS[assetType];
          return <Component variant={selectedVariant} />;
        })()}
    </div>
  );
}

export default AssetsPage;
