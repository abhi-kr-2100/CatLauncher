import { useState } from "react";

import VariantSelector from "@/components/VariantSelector";
import { useGameVariants } from "@/hooks/useGameVariants";
import { useAppDispatch, useAppSelector } from "@/store/hooks";
import { setSelectedVariant } from "@/store/selectedVariantSlice";
import AssetTypeSelector from "./AssetTypeSelector";
import { type AssetType, ASSET_COMPONENTS } from "./types";

/**
 * The main page component for managing game assets like mods, soundpacks, and tilesets.
 * It allows users to select a game variant and the type of asset they want to view or manage.
 *
 * @returns A React element representing the assets page.
 */
function AssetsPage() {
  const { gameVariants, isLoading: gameVariantsLoading } =
    useGameVariants();
  const selectedVariant = useAppSelector(
    (state) => state.selectedVariant.variant,
  );
  const dispatch = useAppDispatch();
  const [assetType, setAssetType] = useState<AssetType>("mods");

  return (
    <div className="flex flex-col gap-2">
      <div className="flex flex-row gap-2">
        <VariantSelector
          gameVariants={gameVariants}
          selectedVariant={selectedVariant}
          onVariantChange={(variant) =>
            dispatch(setSelectedVariant(variant))
          }
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
