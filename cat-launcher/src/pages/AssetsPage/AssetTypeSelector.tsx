import {
  VirtualizedCombobox,
  type ComboboxItem,
} from "@/components/virtualized-combobox";
import type { AssetType } from "./types";

const ASSET_TYPE_LABELS: Record<AssetType, string> = {
  mods: "Mods",
  soundpacks: "Soundpacks",
  tilesets: "Tilesets",
};

/**
 * Props for the {@link AssetTypeSelector} component.
 */
interface AssetTypeSelectorProps {
  /** The currently selected asset type. */
  selectedAssetType: AssetType;
  /** Callback triggered when the asset type selection changes. */
  onAssetTypeChange: (assetType: AssetType) => void;
}

/**
 * A dropdown component that allows the user to select the type of game assets to display.
 *
 * @param props - The component props.
 * @returns A React element representing the asset type selector.
 */
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
