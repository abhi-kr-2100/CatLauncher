/**
 * Error messages for Backups Page functionality
 */

export const backupsPageErrorMap: Record<string, string> = {
  // Backups errors
  ListBackupsCommandError_Get:
    "Failed to retrieve backups. Please try again.",
  DeleteBackupCommandError_Delete:
    "Failed to delete backup. Please try again.",
  DeleteBackupCommandError_DataDir:
    "Unable to find data directory. Please restart the launcher.",
  RestoreBackupCommandError_Restore:
    "Failed to restore backup. Please check your disk space.",
  RestoreBackupCommandError_DataDir:
    "Unable to find data directory. Please restart the launcher.",
  RestoreBackupCommandError_UnsupportedOS:
    "Your operating system is not supported for this operation.",

  // Manual backups errors
  ListManualBackupsCommandError_Get:
    "Failed to retrieve backups. Please try again.",
  CreateManualBackupCommandError_Create:
    "Failed to create backup. Please check disk space and permissions.",
  CreateManualBackupCommandError_DataDir:
    "Unable to find data directory. Please restart the launcher.",
  CreateManualBackupCommandError_UnsupportedOS:
    "Your operating system is not supported.",
  CreateManualBackupCommandError_SystemTime:
    "Failed to get system time. Please check your system clock.",
  DeleteManualBackupCommandError_Delete:
    "Failed to delete backup. Please try again.",
  DeleteManualBackupCommandError_DataDir:
    "Unable to find data directory. Please restart the launcher.",
  RestoreManualBackupCommandError_Restore:
    "Failed to restore backup. Please check your disk space.",
  RestoreManualBackupCommandError_DataDir:
    "Unable to find data directory. Please restart the launcher.",
  RestoreManualBackupCommandError_UnsupportedOS:
    "Your operating system is not supported.",
};
