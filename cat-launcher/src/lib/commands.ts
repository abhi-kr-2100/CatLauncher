import { Channel, invoke } from "@tauri-apps/api/core";
import { emit, listen } from "@tauri-apps/api/event";

import type { BackupEntry } from "@/generated-types/BackupEntry";
import type { CharacterAchievements } from "@/generated-types/CharacterAchievements";
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

/**
 * Listens for a request to quit the application.
 *
 * @param onQuitRequested - Callback function to be executed when a quit is requested.
 * @returns A promise that resolves to an unlisten function.
 */
export async function listenToQuitRequested(
  onQuitRequested: () => void,
) {
  return await listen("quit-requested", () => {
    onQuitRequested();
  });
}

/**
 * Listens for updates to the available game releases.
 *
 * @param onUpdate - Callback function that receives the {@link ReleasesUpdatePayload}.
 * @returns A promise that resolves to an unlisten function.
 */
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

/**
 * Listens for updates to the available mods.
 *
 * @param onUpdate - Callback function that receives the {@link ModsUpdatePayload}.
 * @returns A promise that resolves to an unlisten function.
 */
export async function listenToModsUpdate(
  onUpdate: (payload: ModsUpdatePayload) => void,
) {
  return await listen<ModsUpdatePayload>("mods-update", (event) => {
    onUpdate(event.payload);
  });
}

/**
 * Listens for status updates of the autoupdate process.
 *
 * @param onUpdate - Callback function that receives the {@link UpdateStatus}.
 * @returns A promise that resolves to an unlisten function.
 */
export async function listenToAutoupdateStatus(
  onUpdate: (payload: UpdateStatus) => void,
) {
  return await listen<UpdateStatus>("autoupdate-status", (event) => {
    onUpdate(event.payload);
  });
}

/**
 * Listens for generic game events.
 *
 * @param onEvent - Callback function that receives the {@link GameEvent}.
 * @returns A promise that resolves to an unlisten function.
 */
export async function listenToGameEvent(
  onEvent: (payload: GameEvent) => void,
) {
  return await listen<GameEvent>("game-event", (event) => {
    onEvent(event.payload);
  });
}

/**
 * Notifies the backend that the frontend is ready.
 */
export async function onFrontendReady(): Promise<void> {
  await emit("frontend-ready");
}

/**
 * Triggers a fetch for releases of a specific game variant.
 *
 * @param variant - The game variant to fetch releases for.
 */
export async function triggerFetchReleasesForVariant(
  variant: GameVariant,
): Promise<void> {
  await invoke("fetch_releases_for_variant", {
    variant,
  });
}

/**
 * Fetches information about all supported game variants.
 *
 * @returns A promise that resolves to an array of {@link GameVariantInfo}.
 */
export async function fetchGameVariantsInfo(): Promise<
  GameVariantInfo[]
> {
  const response = await invoke<GameVariantInfo[]>(
    "get_game_variants_info",
  );
  return response;
}

/**
 * Deletes a backup entry by its unique identifier.
 *
 * @param id - The unique identifier of the backup.
 */
export async function deleteBackupById(id: bigint): Promise<void> {
  await invoke("delete_backup_by_id", {
    id,
  });
}

/**
 * Restores a backup entry by its unique identifier.
 *
 * @param id - The unique identifier of the backup to restore.
 */
export async function restoreBackupById(id: bigint): Promise<void> {
  await invoke("restore_backup_by_id", {
    id,
  });
}

/**
 * Fetches game tips for a specific variant.
 *
 * @param variant - The game variant to fetch tips for.
 * @returns A promise that resolves to an array of strings representing tips.
 */
export async function getTips(
  variant: GameVariant,
): Promise<string[]> {
  const response = await invoke<string[]>("get_tips", {
    variant,
  });

  return response;
}

/**
 * Lists all manual backups available for a specific variant.
 *
 * @param variant - The game variant to list manual backups for.
 * @returns A promise that resolves to an array of {@link ManualBackupEntry}.
 */
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

/**
 * Creates a manual backup for a specific game variant.
 *
 * @param variant - The game variant to create a backup for.
 * @param name - The name of the manual backup.
 * @param notes - Optional notes associated with the backup.
 */
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

/**
 * Deletes a manual backup entry by its unique identifier.
 *
 * @param id - The unique identifier of the manual backup.
 */
export async function deleteManualBackupById(
  id: bigint,
): Promise<void> {
  await invoke("delete_manual_backup_by_id", {
    id,
  });
}

/**
 * Restores a manual backup entry by its unique identifier.
 *
 * @param id - The unique identifier of the manual backup to restore.
 */
export async function restoreManualBackupById(
  id: bigint,
): Promise<void> {
  await invoke("restore_manual_backup_by_id", {
    id,
  });
}

/**
 * Lists all automatic backups available for a specific variant.
 *
 * @param variant - The game variant to list backups for.
 * @returns A promise that resolves to an array of {@link BackupEntry}.
 */
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

/**
 * Updates the display order of game variants.
 *
 * @param variants - The new ordered list of game variants.
 */
export async function updateGameVariantOrder(
  variants: GameVariant[],
): Promise<void> {
  await invoke("update_game_variant_order", {
    variants,
  });
}

/**
 * Gets the total play time for a specific game variant.
 *
 * @param variant - The game variant to get the play time for.
 * @returns A promise that resolves to the total play time in seconds.
 */
export async function getPlayTimeForVariant(
  variant: GameVariant,
): Promise<number> {
  const response = await invoke<number>("get_play_time_for_variant", {
    variant,
  });

  return response;
}

/**
 * Gets the play time for a specific version of a game variant.
 *
 * @param variant - The game variant.
 * @param version - The specific version string.
 * @returns A promise that resolves to the play time in seconds.
 */
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

/**
 * Logs the play time for a specific version of a game variant.
 *
 * @param variant - The game variant.
 * @param version - The specific version string.
 * @param durationInSeconds - The duration of the session in seconds.
 */
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

/**
 * Gets the currently active (installed/selected) release ID for a variant.
 *
 * @param variant - The game variant.
 * @returns A promise that resolves to the active release ID, or an empty string if none.
 */
export async function getActiveRelease(
  variant: GameVariant,
): Promise<string> {
  const response = await invoke<string | null>("get_active_release", {
    variant,
  });

  // useQuery doesn't work with null/undefined query data. That's why "" is returned.
  return response ?? "";
}

/**
 * Fetches the release notes for a specific release of a game variant.
 *
 * @param variant - The game variant.
 * @param releaseId - The unique identifier of the release.
 * @returns A promise that resolves to the release notes as a string, or null if not found.
 */
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

/**
 * Installs a specific release for a game variant, with progress tracking.
 *
 * @param releaseId - The unique identifier of the release to install.
 * @param variant - The game variant.
 * @param onDownloadProgress - Callback function for download progress updates.
 * @returns A promise that resolves to the installed {@link GameRelease}.
 */
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

/**
 * Launches the game with the specified release and optional world.
 *
 * @param variant - The game variant.
 * @param releaseId - The unique identifier of the release to launch.
 * @param world - Optional world name to load directly.
 */
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

/**
 * Gets the installation status of a specific release for a game variant.
 *
 * @param variant - The game variant.
 * @param releaseId - The unique identifier of the release.
 * @returns A promise that resolves to the {@link GameReleaseStatus}.
 */
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

/**
 * Gets the name of the last played world for a specific variant.
 *
 * @param variant - The game variant.
 * @returns A promise that resolves to the name of the last played world, or null if none.
 */
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

/**
 * Gets the user's preferred theme setting.
 *
 * @returns A promise that resolves to the {@link ThemePreference}.
 */
export async function getPreferredTheme(): Promise<ThemePreference> {
  const response = await invoke<ThemePreference>(
    "get_preferred_theme",
  );
  return response;
}

/**
 * Sets the user's preferred theme.
 *
 * @param theme - The theme to set as preferred.
 */
export async function setPreferredTheme(theme: Theme): Promise<void> {
  await invoke("set_preferred_theme", {
    theme,
  });
}

/**
 * Gets the user's unique identifier.
 *
 * @returns A promise that resolves to the user ID string.
 */
export async function getUserId(): Promise<string> {
  const response = await invoke<string>("get_user_id");
  return response;
}

/**
 * Triggers a fetch for available mods for a specific variant.
 *
 * @param variant - The game variant to fetch mods for.
 */
export async function triggerFetchModsForVariant(
  variant: GameVariant,
): Promise<void> {
  await invoke("list_all_mods_command", {
    variant,
  });
}

/**
 * Installs a third-party mod for a game variant.
 *
 * @param modId - The unique identifier of the mod to install.
 * @param variant - The game variant.
 * @param onDownloadProgress - Callback function for download progress updates.
 */
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

/**
 * Gets the installation status of a third-party mod.
 *
 * @param modId - The unique identifier of the mod.
 * @param variant - The game variant.
 * @returns A promise that resolves to the {@link ModInstallationStatus}.
 */
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

/**
 * Uninstalls a third-party mod.
 *
 * @param modId - The unique identifier of the mod to uninstall.
 * @param variant - The game variant.
 */
export async function uninstallThirdPartyMod(
  modId: string,
  variant: GameVariant,
): Promise<void> {
  await invoke("uninstall_third_party_mod_command", {
    id: modId,
    variant: variant,
  });
}

/**
 * Gets the last activity recorded for a third-party mod.
 *
 * @param modId - The unique identifier of the mod.
 * @param variant - The game variant.
 * @returns A promise that resolves to the {@link LastModActivity}.
 */
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

/**
 * Lists all available tilesets for a specific variant.
 *
 * @param variant - The game variant.
 * @returns A promise that resolves to an array of {@link Tileset}.
 */
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

/**
 * Installs a third-party tileset for a game variant.
 *
 * @param tilesetId - The unique identifier of the tileset to install.
 * @param variant - The game variant.
 * @param onDownloadProgress - Callback function for download progress updates.
 */
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

/**
 * Gets the installation status of a third-party tileset.
 *
 * @param tilesetId - The unique identifier of the tileset.
 * @param variant - The game variant.
 * @returns A promise that resolves to the {@link TilesetInstallationStatus}.
 */
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

/**
 * Uninstalls a third-party tileset.
 *
 * @param tilesetId - The unique identifier of the tileset to uninstall.
 * @param variant - The game variant.
 */
export async function uninstallThirdPartyTileset(
  tilesetId: string,
  variant: GameVariant,
): Promise<void> {
  await invoke("uninstall_third_party_tileset_command", {
    id: tilesetId,
    variant: variant,
  });
}

/**
 * Lists all available soundpacks for a specific variant.
 *
 * @param variant - The game variant.
 * @returns A promise that resolves to an array of {@link Soundpack}.
 */
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

/**
 * Installs a third-party soundpack for a game variant.
 *
 * @param soundpackId - The unique identifier of the soundpack to install.
 * @param variant - The game variant.
 * @param onDownloadProgress - Callback function for download progress updates.
 */
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

/**
 * Gets the installation status of a third-party soundpack.
 *
 * @param soundpackId - The unique identifier of the soundpack.
 * @param variant - The game variant.
 * @returns A promise that resolves to the {@link SoundpackInstallationStatus}.
 */
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

/**
 * Uninstalls a third-party soundpack.
 *
 * @param soundpackId - The unique identifier of the soundpack to uninstall.
 * @param variant - The game variant.
 */
export async function uninstallThirdPartySoundpack(
  soundpackId: string,
  variant: GameVariant,
): Promise<void> {
  await invoke("uninstall_third_party_soundpack_command", {
    id: soundpackId,
    variant: variant,
  });
}

/**
 * Confirms that the application can quit.
 */
export async function confirmQuit(): Promise<void> {
  await invoke("confirm_quit");
}

/**
 * Fetches all available system/application fonts.
 *
 * @returns A promise that resolves to an array of {@link Font}.
 */
export async function getFonts(): Promise<Font[]> {
  const response = await invoke<Font[]>("get_fonts");
  return response;
}

/**
 * Fetches all available color themes.
 *
 * @returns A promise that resolves to an array of {@link ColorTheme}.
 */
export async function getColorThemes(): Promise<ColorTheme[]> {
  const response = await invoke<ColorTheme[]>("get_color_themes");
  return response;
}

/**
 * Fetches the current application settings.
 *
 * @returns A promise that resolves to the current {@link Settings}.
 */
export async function getSettings(): Promise<Settings> {
  const response = await invoke<Settings>("get_settings");
  return response;
}

/**
 * Fetches the default application settings.
 *
 * @returns A promise that resolves to the default {@link Settings}.
 */
export async function getDefaultSettings(): Promise<Settings> {
  const response = await invoke<Settings>("get_default_settings");
  return response;
}

/**
 * Updates the application settings.
 *
 * @param settings - The new settings to apply.
 */
export async function updateSettings(
  settings: Settings,
): Promise<void> {
  await invoke("update_settings", { settings });
}

/**
 * Resets a game variant to its initial state, deleting all associated data.
 *
 * @param variant - The game variant to reset.
 */
export async function masterReset(
  variant: GameVariant,
): Promise<void> {
  await invoke("master_reset", { variant });
}

/**
 * Fetches character achievements for a specific variant.
 *
 * @param variant - The game variant.
 * @returns A promise that resolves to an array of {@link CharacterAchievements}.
 */
export async function getAchievementsForVariant(
  variant: GameVariant,
): Promise<CharacterAchievements[]> {
  const response = await invoke<CharacterAchievements[]>(
    "get_achievements_for_variant",
    {
      variant,
    },
  );
  return response;
}
