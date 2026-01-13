import { Channel, invoke } from "@tauri-apps/api/core";
import { emit, listen } from "@tauri-apps/api/event";

import type { BackupEntry } from "@/generated-types/BackupEntry";
import type { ColorTheme } from "@/generated-types/ColorTheme";
import type { DownloadProgress } from "@/generated-types/DownloadProgress";
import type { Font } from "@/generated-types/Font";
import type { GameEvent } from "@/generated-types/GameEvent";
import type { GameRelease } from "@/generated-types/GameRelease";
import type { GameReleaseStatus } from "@/generated-types/GameReleaseStatus";
import type { GameVariant } from "@/generated-types/GameVariant";
import type { GameVariantInfo } from "@/generated-types/GameVariantInfo";
import type { LastModActivity } from "@/generated-types/LastModActivity";
import type { ManualBackupEntry } from "@/generated-types/ManualBackupEntry";
import type { ModInstallationStatus } from "@/generated-types/ModInstallationStatus";
import type { ModsUpdatePayload } from "@/generated-types/ModsUpdatePayload";
import type { ReleasesUpdatePayload } from "@/generated-types/ReleasesUpdatePayload";
import type { Settings } from "@/generated-types/Settings";
import type { Soundpack } from "@/generated-types/Soundpack";
import type { SoundpackInstallationStatus } from "@/generated-types/SoundpackInstallationStatus";
import type { Theme } from "@/generated-types/Theme";
import type { ThemePreference } from "@/generated-types/ThemePreference";
import type { Tileset } from "@/generated-types/Tileset";
import type { TilesetInstallationStatus } from "@/generated-types/TilesetInstallationStatus";
import type { UpdateStatus } from "@/generated-types/UpdateStatus";

export async function listenToQuitRequested(
  onQuitRequested: () => void,
) {
  return await listen("quit-requested", () => {
    onQuitRequested();
  });
}

export async function listenToReleasesUpdate(
  onUpdate: (payload: ReleasesUpdatePayload) => void,
) {
  return await listen<ReleasesUpdatePayload>(
    "releases-update",
    (event) => {
      onUpdate(event.payload);
    },
  );
}

export async function listenToModsUpdate(
  onUpdate: (payload: ModsUpdatePayload) => void,
) {
  return await listen<ModsUpdatePayload>("mods-update", (event) => {
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

export async function listenToGameEvent(
  onEvent: (payload: GameEvent) => void,
) {
  return await listen<GameEvent>("game-event", (event) => {
    onEvent(event.payload);
  });
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

export async function fetchGameVariantsInfo(): Promise<
  GameVariantInfo[]
> {
  const response = await invoke<GameVariantInfo[]>(
    "get_game_variants_info",
  );
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

export async function getTips(
  variant: GameVariant,
): Promise<string[]> {
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

export async function deleteManualBackupById(
  id: bigint,
): Promise<void> {
  await invoke("delete_manual_backup_by_id", {
    id,
  });
}

export async function restoreManualBackupById(
  id: bigint,
): Promise<void> {
  await invoke("restore_manual_backup_by_id", {
    id,
  });
}

export async function listBackupsForVariant(
  variant: GameVariant,
): Promise<BackupEntry[]> {
  const response = await invoke<BackupEntry[]>(
    "list_backups_for_variant",
    {
      variant,
    },
  );

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

export async function logPlayTime(
  variant: GameVariant,
  version: string,
  durationInSeconds: number,
): Promise<void> {
  await invoke("log_play_time", {
    variant,
    version,
    durationInSeconds,
  });
}

export async function getActiveRelease(
  variant: GameVariant,
): Promise<string> {
  const response = await invoke<string | null>("get_active_release", {
    variant,
  });

  // useQuery doesn't work with null/undefined query data. That's why "" is returned.
  return response ?? "";
}

export async function fetchReleaseNotes(
  variant: GameVariant,
  releaseId: string,
): Promise<string | null> {
  const response = await invoke<string | null>(
    "fetch_release_notes",
    {
      variant,
      releaseId,
    },
  );
  return response;
}

export async function installReleaseForVariant(
  releaseId: string,
  variant: GameVariant,
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
  world: string | null,
): Promise<void> {
  await invoke("launch_game", {
    variant,
    releaseId,
    world,
  });
}

export async function getInstallationStatus(
  variant: GameVariant,
  releaseId: string,
): Promise<GameReleaseStatus> {
  const response = await invoke<GameReleaseStatus>(
    "get_installation_status",
    {
      variant,
      releaseId,
    },
  );

  return response;
}

export async function getLastPlayedWorld(
  variant: GameVariant,
): Promise<string | null> {
  const response = await invoke<string | null>(
    "get_last_played_world",
    {
      variant,
    },
  );

  return response;
}

export async function getPreferredTheme(): Promise<ThemePreference> {
  const response = await invoke<ThemePreference>(
    "get_preferred_theme",
  );
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

export async function triggerFetchModsForVariant(
  variant: GameVariant,
): Promise<void> {
  await invoke("list_all_mods_command", {
    variant,
  });
}

export async function installThirdPartyMod(
  modId: string,
  variant: GameVariant,
  onDownloadProgress: (progress: DownloadProgress) => void,
): Promise<void> {
  const channel = new Channel();
  channel.onmessage = (progress) => {
    onDownloadProgress(progress as DownloadProgress);
  };

  await invoke("install_third_party_mod_command", {
    id: modId,
    variant,
    channel,
  });
}

export async function getThirdPartyModInstallationStatus(
  modId: string,
  variant: GameVariant,
): Promise<ModInstallationStatus> {
  const response = await invoke<ModInstallationStatus>(
    "get_third_party_mod_installation_status_command",
    {
      id: modId,
      variant,
    },
  );
  return response;
}

export async function uninstallThirdPartyMod(
  modId: string,
  variant: GameVariant,
): Promise<void> {
  await invoke("uninstall_third_party_mod_command", {
    id: modId,
    variant: variant,
  });
}

export async function getLastModActivity(
  modId: string,
  variant: GameVariant,
): Promise<LastModActivity> {
  const response = await invoke<LastModActivity>(
    "get_last_activity_on_third_party_mod_command",
    {
      id: modId,
      variant,
    },
  );
  return response;
}

export async function listAllTilesets(
  variant: GameVariant,
): Promise<Tileset[]> {
  const response = await invoke<Tileset[]>(
    "list_all_tilesets_command",
    {
      variant,
    },
  );
  return response;
}

export async function installThirdPartyTileset(
  tilesetId: string,
  variant: GameVariant,
  onDownloadProgress: (progress: DownloadProgress) => void,
): Promise<void> {
  const channel = new Channel();
  channel.onmessage = (progress) => {
    onDownloadProgress(progress as DownloadProgress);
  };

  await invoke("install_third_party_tileset_command", {
    id: tilesetId,
    variant,
    channel,
  });
}

export async function getThirdPartyTilesetInstallationStatus(
  tilesetId: string,
  variant: GameVariant,
): Promise<TilesetInstallationStatus> {
  const response = await invoke<TilesetInstallationStatus>(
    "get_third_party_tileset_installation_status_command",
    {
      id: tilesetId,
      variant,
    },
  );
  return response;
}

export async function uninstallThirdPartyTileset(
  tilesetId: string,
  variant: GameVariant,
): Promise<void> {
  await invoke("uninstall_third_party_tileset_command", {
    id: tilesetId,
    variant: variant,
  });
}

export async function listAllSoundpacks(
  variant: GameVariant,
): Promise<Soundpack[]> {
  const response = await invoke<Soundpack[]>(
    "list_all_soundpacks_command",
    {
      variant,
    },
  );
  return response;
}

export async function installThirdPartySoundpack(
  soundpackId: string,
  variant: GameVariant,
  onDownloadProgress: (progress: DownloadProgress) => void,
): Promise<void> {
  const channel = new Channel();
  channel.onmessage = (progress) => {
    onDownloadProgress(progress as DownloadProgress);
  };

  await invoke("install_third_party_soundpack_command", {
    id: soundpackId,
    variant,
    channel,
  });
}

export async function getThirdPartySoundpackInstallationStatus(
  soundpackId: string,
  variant: GameVariant,
): Promise<SoundpackInstallationStatus> {
  const response = await invoke<SoundpackInstallationStatus>(
    "get_third_party_soundpack_installation_status_command",
    {
      id: soundpackId,
      variant,
    },
  );
  return response;
}

export async function uninstallThirdPartySoundpack(
  soundpackId: string,
  variant: GameVariant,
): Promise<void> {
  await invoke("uninstall_third_party_soundpack_command", {
    id: soundpackId,
    variant: variant,
  });
}

export async function confirmQuit(): Promise<void> {
  await invoke("confirm_quit");
}

export async function getFonts(): Promise<Font[]> {
  const response = await invoke<Font[]>("get_fonts");
  return response;
}

export async function getColorThemes(): Promise<ColorTheme[]> {
  const response = await invoke<ColorTheme[]>("get_color_themes");
  return response;
}

export async function getSettings(): Promise<Settings> {
  const response = await invoke<Settings>("get_settings");
  return response;
}

export async function getDefaultSettings(): Promise<Settings> {
  const response = await invoke<Settings>("get_default_settings");
  return response;
}

export async function updateSettings(
  settings: Settings,
): Promise<void> {
  await invoke("update_settings", { settings });
}

export async function masterReset(
  variant: GameVariant,
): Promise<void> {
  await invoke("master_reset", { variant });
}
