import { useState } from "react";

import VariantSelector from "@/components/VariantSelector";
import { useGameVariants } from "@/hooks/useGameVariants";
import { useAppDispatch, useAppSelector } from "@/store/hooks";
import { setSelectedVariant } from "@/store/selectedVariantSlice";
import AssetTypeSelector from "./components/AssetTypeSelector";
import { AssetType, ASSET_COMPONENTS } from "./lib/types";

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
