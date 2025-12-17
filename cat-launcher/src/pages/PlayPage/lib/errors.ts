/**
 * Error messages for Play Page functionality
 */

export const playPageErrorMap: Record<string, string> = {
  // Installation errors
  InstallReleaseCommandError_SystemDir:
    "Unable to find system directory. Please restart the launcher and try again.",
  InstallReleaseCommandError_Install:
    "Installation failed. Please check your internet connection and disk space.",
  InstallReleaseCommandError_Release:
    "Failed to load release information. Please try again.",
  InstallReleaseCommandError_Os:
    "Your operating system is not supported for this release.",
  InstallReleaseCommandError_Arch:
    "Your system architecture is not supported for this release.",

  // Launch game errors
  LaunchGameCommandError_LaunchGame:
    "Failed to launch the game. Please ensure it's installed and not already running.",
  LaunchGameCommandError_SystemDirectoryNotFound:
    "Unable to find system directory. Please restart the launcher.",
  LaunchGameCommandError_SystemTime:
    "System time error. Please check your system clock.",
  LaunchGameCommandError_Os:
    "Your operating system is not supported.",

  // Fetch releases errors
  FetchReleasesCommandError_SystemDir:
    "Unable to find system directory. Please restart the launcher.",
  FetchReleasesCommandError_Fetch:
    "Failed to fetch releases. Please check your internet connection.",

  // Active release errors
  ActiveReleaseCommandError_GetActiveRelease:
    "Failed to get active release. Please try again.",
  ActiveReleaseCommandError_SystemDirectory:
    "Unable to find system directory. Please restart the launcher.",

  // Game tips errors
  GetTipsCommandError_DataDir:
    "Unable to find data directory. Please restart the launcher.",
  GetTipsCommandError_UnsupportedOS:
    "Your operating system is not supported.",
  GetTipsCommandError_GetForVariant:
    "Failed to retrieve tips. Please try again.",

  // Installation status errors
  GetInstallationStatusCommandError_SystemDir:
    "Unable to find system directory. Please restart the launcher.",
  GetInstallationStatusCommandError_Release:
    "Failed to load release information. Please try again.",
  GetInstallationStatusCommandError_Os:
    "Your operating system is not supported for this release.",

  // Play time errors
  GetPlayTimeCommandError_Repository:
    "Failed to retrieve play time. Please try again.",

  // Last played world errors
  GetLastPlayedWorldCommandError_GetLastPlayedWorld:
    "Failed to retrieve last played world. Please try again.",
  GetLastPlayedWorldCommandError_AppLocalDataDir:
    "Unable to find app data directory. Please restart the launcher.",
};
