import {
  VirtualizedCombobox,
  type ComboboxItem,
} from "@/components/virtualized-combobox";
import type { AssetType } from "./index";

const ASSET_TYPE_LABELS: Record<AssetType, string> = {
  mods: "Mods",
  soundpacks: "Soundpacks",
  tilesets: "Tilesets",
};

interface AssetTypeSelectorProps {
  selectedAssetType: AssetType;
  onAssetTypeChange: (assetType: AssetType) => void;
}

export default function AssetTypeSelector({
  selectedAssetType,
  onAssetTypeChange,
}: AssetTypeSelectorProps) {
  const comboboxItems: ComboboxItem[] = (
    Object.keys(ASSET_TYPE_LABELS) as AssetType[]
  ).map((assetType) => ({
    value: assetType,
    label: ASSET_TYPE_LABELS[assetType],
  }));

  return (
    <VirtualizedCombobox
      items={comboboxItems}
      value={selectedAssetType}
      onChange={(value) => onAssetTypeChange(value as AssetType)}
      placeholder="Select asset type"
      className="w-2xs"
    />
  );
}
