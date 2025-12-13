import { Channel, invoke } from "@tauri-apps/api/core";
import { emit, listen } from "@tauri-apps/api/event";

import type { BackupEntry } from "@/generated-types/BackupEntry";
import type { DownloadProgress } from "@/generated-types/DownloadProgress";
import type { GameEvent } from "@/generated-types/GameEvent";
import type { GameRelease } from "@/generated-types/GameRelease";
import type { GameReleaseStatus } from "@/generated-types/GameReleaseStatus";
import type { GameVariant } from "@/generated-types/GameVariant";
import type { GameVariantInfo } from "@/generated-types/GameVariantInfo";
import type { InstallationProgressPayload } from "@/generated-types/InstallationProgressPayload";
import type { InstallationProgressStatus } from "@/generated-types/InstallationProgressStatus";
import type { ManualBackupEntry } from "@/generated-types/ManualBackupEntry";
import type { Mod } from "@/generated-types/Mod";
import type { ReleasesUpdatePayload } from "@/generated-types/ReleasesUpdatePayload";
import type { Theme } from "@/generated-types/Theme";
import type { ThemePreference } from "@/generated-types/ThemePreference";
import type { UpdateStatus } from "@/generated-types/UpdateStatus";

export async function listenToReleasesUpdate(
  onUpdate: (payload: ReleasesUpdatePayload) => void,
) {
  return await listen<ReleasesUpdatePayload>("releases-update", (event) => {
    onUpdate(event.payload);
  });
}

export async function listenToAutoupdateStatus(
  onUpdate: (payload: UpdateStatus) => void,
) {
  return await listen<UpdateStatus>("autoupdate-status", (event) => {
    onUpdate(event.payload);
  });
}

export async function listenToGameEvent(onEvent: (payload: GameEvent) => void) {
  return await listen<GameEvent>("game-event", (event) => {
    onEvent(event.payload);
  });
}

export async function listenToInstallationStatusUpdate(
  selectedReleaseId: string,
  onUpdate: (payload: InstallationProgressStatus) => void,
) {
  return await listen<InstallationProgressPayload>(
    "installation-status-update",
    (event) => {
      if (event.payload.release_id === selectedReleaseId) {
        onUpdate(event.payload.status);
      }
    },
  );
}

export async function onFrontendReady(): Promise<void> {
  await emit("frontend-ready");
}

export async function triggerFetchReleasesForVariant(
  variant: GameVariant,
): Promise<void> {
  await invoke("fetch_releases_for_variant", {
    variant,
  });
}

export async function fetchGameVariantsInfo(): Promise<GameVariantInfo[]> {
  const response = await invoke<GameVariantInfo[]>("get_game_variants_info");
  return response;
}

export async function deleteBackupById(id: bigint): Promise<void> {
  await invoke("delete_backup_by_id", {
    id,
  });
}

export async function restoreBackupById(id: bigint): Promise<void> {
  await invoke("restore_backup_by_id", {
    id,
  });
}

export async function getTips(variant: GameVariant): Promise<string[]> {
  const response = await invoke<string[]>("get_tips", {
    variant,
  });

  return response;
}

export async function listManualBackupsForVariant(
  variant: GameVariant,
): Promise<ManualBackupEntry[]> {
  const response = await invoke<ManualBackupEntry[]>(
    "list_manual_backups_for_variant",
    {
      variant,
    },
  );

  return response;
}

export async function createManualBackupForVariant(
  variant: GameVariant,
  name: string,
  notes?: string,
): Promise<void> {
  await invoke("create_manual_backup_for_variant", {
    variant,
    name,
    notes,
  });
}

export async function deleteManualBackupById(id: bigint): Promise<void> {
  await invoke("delete_manual_backup_by_id", {
    id,
  });
}

export async function restoreManualBackupById(id: bigint): Promise<void> {
  await invoke("restore_manual_backup_by_id", {
    id,
  });
}

export async function listBackupsForVariant(
  variant: GameVariant,
): Promise<BackupEntry[]> {
  const response = await invoke<BackupEntry[]>("list_backups_for_variant", {
    variant,
  });

  return response;
}

export async function updateGameVariantOrder(
  variants: GameVariant[],
): Promise<void> {
  await invoke("update_game_variant_order", {
    variants,
  });
}

export async function getPlayTimeForVariant(
  variant: GameVariant,
): Promise<number> {
  const response = await invoke<number>("get_play_time_for_variant", {
    variant,
  });

  return response;
}

export async function getPlayTimeForVersion(
  variant: GameVariant,
  version: string,
): Promise<number> {
  const response = await invoke<number>("get_play_time_for_version", {
    variant,
    version,
  });

  return response;
}

export async function getActiveRelease(variant: GameVariant): Promise<string> {
  const response = await invoke<string | null>("get_active_release", {
    variant,
  });

  // useQuery doesn't work with null/undefined query data. That's why "" is returned.
  return response ?? "";
}

export async function installReleaseForVariant(
  variant: GameVariant,
  releaseId: string,
  onDownloadProgress: (progress: DownloadProgress) => void,
): Promise<GameRelease> {
  const channel = new Channel();
  channel.onmessage = (progress) => {
    onDownloadProgress(progress as DownloadProgress);
  };

  const response = await invoke<GameRelease>("install_release", {
    variant,
    releaseId,
    onDownloadProgress: channel,
  });

  return response;
}

export async function launchGame(
  variant: GameVariant,
  releaseId: string,
): Promise<void> {
  await invoke("launch_game", {
    variant,
    releaseId,
  });
}

export async function getInstallationStatus(
  variant: GameVariant,
  releaseId: string,
): Promise<GameReleaseStatus> {
  const response = await invoke<GameReleaseStatus>("get_installation_status", {
    variant,
    releaseId,
  });

  return response;
}

export async function getPreferredTheme(): Promise<ThemePreference> {
  const response = await invoke<ThemePreference>("get_preferred_theme");
  return response;
}

export async function setPreferredTheme(theme: Theme): Promise<void> {
  await invoke("set_preferred_theme", {
    theme,
  });
}

export async function getUserId(): Promise<string> {
  const response = await invoke<string>("get_user_id");
  return response;
}

export async function listAllMods(variant: GameVariant): Promise<Mod[]> {
  const response = await invoke<Mod[]>("list_all_mods_command", {
    variant,
  });
  return response;
}

export async function installThirdPartyMod(
  modId: string,
  variant: GameVariant,
): Promise<void> {
  await invoke("install_third_party_mod_command", {
    id: modId,
    variant,
  });
}
