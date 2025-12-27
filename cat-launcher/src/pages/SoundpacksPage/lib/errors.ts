/**
 * Error messages for Soundpacks Page functionality
 */

export const soundpacksPageErrorMap: Record<string, string> = {
  // Soundpacks errors
  ListAllSoundpacksCommandError_AppDataDir:
    "Unable to find app data directory. Please restart the launcher.",
  ListAllSoundpacksCommandError_OSInfo:
    "Failed to detect your operating system.",
  ListAllSoundpacksCommandError_ListSoundpacks:
    "Failed to retrieve soundpacks. Please check your internet connection.",
  InstallThirdPartySoundpackCommandError_AppDataDir:
    "Unable to find app data directory. Please restart the launcher.",
  InstallThirdPartySoundpackCommandError_OSInfo:
    "Failed to detect your operating system.",
  InstallThirdPartySoundpackCommandError_Install:
    "Failed to install soundpack. Please check your disk space and internet connection.",
  UninstallThirdPartySoundpackCommandError_AppDataDir:
    "Unable to find app data directory. Please restart the launcher.",
  UninstallThirdPartySoundpackCommandError_Uninstall:
    "Failed to uninstall soundpack. Please try again.",
  GetThirdPartySoundpackInstallationStatusCommandError_GetStatus:
    "Failed to get soundpack installation status. Please try again.",
};
