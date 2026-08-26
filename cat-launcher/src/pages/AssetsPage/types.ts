import ModsList from "../ModsPage/ModsList";
import SoundpacksList from "../SoundpacksPage/SoundpacksList";
import TilesetsList from "../TilesetsPage/TilesetsList";

export type AssetType = "mods" | "soundpacks" | "tilesets";

export type AssetListComponent = typeof ModsList;

export const ASSET_COMPONENTS: Record<AssetType, AssetListComponent> =
  {
    mods: ModsList,
    soundpacks: SoundpacksList,
    tilesets: TilesetsList,
  };

export const getAssetComponent = (
  assetType: AssetType,
): AssetListComponent => ASSET_COMPONENTS[assetType];
