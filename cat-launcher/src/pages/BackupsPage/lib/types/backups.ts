import { BackupEntry } from "@/generated-types/BackupEntry";
import { ManualBackupEntry } from "@/generated-types/ManualBackupEntry";

export type CombinedBackup =
  | (BackupEntry & { type: "automatic"; name: string; notes: string })
  | (ManualBackupEntry & { type: "manual" });
