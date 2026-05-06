import { BackupEntry } from "@/generated-types/BackupEntry";
import { ManualBackupEntry } from "@/generated-types/ManualBackupEntry";

/**
 * Represents a backup entry that can be either an automatic backup or a manual backup.
 * It combines data from {@link BackupEntry} and {@link ManualBackupEntry} with a discriminator `type`.
 */
export type CombinedBackup =
  | (BackupEntry & { type: "automatic"; name: string; notes: string })
  | (ManualBackupEntry & { type: "manual" });
