import { useState } from "react";

import VariantSelector from "@/components/VariantSelector";
import type { GameVariant } from "@/generated-types/GameVariant";
import { useGameVariants } from "@/hooks/useGameVariants";
import ModsList from "../ModsPage/ModsList";
import SoundpacksList from "../SoundpacksPage/SoundpacksList";
import TilesetsList from "../TilesetsPage/TilesetsList";
import AssetTypeSelector from "./AssetTypeSelector";

export type AssetType = "mods" | "soundpacks" | "tilesets";

function AssetsPage() {
  const { gameVariants, isLoading: gameVariantsLoading } =
    useGameVariants();
  const [selectedVariant, setSelectedVariant] =
    useState<GameVariant | null>(null);
  const [assetType, setAssetType] = useState<AssetType>("mods");

  const renderAssetList = () => {
    if (!selectedVariant) return null;

    switch (assetType) {
      case "mods":
        return <ModsList variant={selectedVariant} />;
      case "soundpacks":
        return <SoundpacksList variant={selectedVariant} />;
      case "tilesets":
        return <TilesetsList variant={selectedVariant} />;
    }
  };

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
      {selectedVariant && renderAssetList()}
    </div>
  );
}

export default AssetsPage;
