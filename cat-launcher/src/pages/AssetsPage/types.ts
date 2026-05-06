import ModsList from "../ModsPage/ModsList";
import SoundpacksList from "../SoundpacksPage/SoundpacksList";
import TilesetsList from "../TilesetsPage/TilesetsList";

/**
 * Represents the types of assets available in the launcher.
 */
export type AssetType = "mods" | "soundpacks" | "tilesets";

/**
 * Type definition for a component that displays a list of assets.
 */
export type AssetListComponent = typeof ModsList;

/**
 * A mapping from {@link AssetType} to its corresponding list component.
 */
export const ASSET_COMPONENTS: Record<AssetType, AssetListComponent> =
  {
    mods: ModsList,
    soundpacks: SoundpacksList,
    tilesets: TilesetsList,
  };

/**
 * Retrieves the component responsible for rendering a specific asset type.
 *
 * @param assetType - The type of asset.
 * @returns The list component for the given asset type.
 */
export const getAssetComponent = (
  assetType: AssetType,
): AssetListComponent => ASSET_COMPONENTS[assetType];
