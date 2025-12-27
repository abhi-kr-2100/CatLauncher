/**
 * Error messages for Tilesets Page functionality
 */

export const tilesetsPageErrorMap: Record<string, string> = {
  // Tilesets errors
  ListAllTilesetsCommandError_AppDataDir:
    "Unable to find app data directory. Please restart the launcher.",
  ListAllTilesetsCommandError_OSInfo:
    "Failed to detect your operating system.",
  ListAllTilesetsCommandError_ListTilesets:
    "Failed to retrieve tilesets. Please check your internet connection.",
  InstallThirdPartyTilesetCommandError_AppDataDir:
    "Unable to find app data directory. Please restart the launcher.",
  InstallThirdPartyTilesetCommandError_OSInfo:
    "Failed to detect your operating system.",
  InstallThirdPartyTilesetCommandError_Install:
    "Failed to install tileset. Please check your disk space and internet connection.",
  UninstallThirdPartyTilesetCommandError_AppDataDir:
    "Unable to find app data directory. Please restart the launcher.",
  UninstallThirdPartyTilesetCommandError_Uninstall:
    "Failed to uninstall tileset. Please try again.",
  GetThirdPartyTilesetInstallationStatusCommandError_GetStatus:
    "Failed to get tileset installation status. Please try again.",
};
