/**
 * Error messages for Mods Page functionality
 */

export const modsPageErrorMap: Record<string, string> = {
  // Mods errors
  ListAllModsCommandError_AppDataDir:
    "Unable to find app data directory. Please restart the launcher.",
  ListAllModsCommandError_OSInfo:
    "Failed to detect your operating system.",
  ListAllModsCommandError_ListMods:
    "Failed to retrieve mods list. Please check your internet connection.",
  InstallThirdPartyModCommandError_AppDataDir:
    "Unable to find app data directory. Please restart the launcher.",
  InstallThirdPartyModCommandError_OSInfo:
    "Failed to detect your operating system.",
  InstallThirdPartyModCommandError_Install:
    "Failed to install mod. Please check your disk space and internet connection.",
  UninstallThirdPartyModCommandError_AppDataDir:
    "Unable to find app data directory. Please restart the launcher.",
  UninstallThirdPartyModCommandError_Uninstall:
    "Failed to uninstall mod. Please try again.",
  GetThirdPartyModInstallationStatusCommandError_GetStatus:
    "Failed to get mod installation status. Please try again.",
  GetLastActivityCommandError_AppDataDir:
    "Unable to find app data directory. Please restart the launcher.",
  GetLastActivityCommandError_OSInfo:
    "Failed to detect your operating system.",
  GetLastActivityCommandError_GetActivity:
    "Failed to get last activity. Please check your internet connection.",
};
