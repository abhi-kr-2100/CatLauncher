import { invoke } from "@tauri-apps/api/core";
import { emit, listen } from "@tauri-apps/api/event";

import type { GameEvent } from "@/generated-types/GameEvent";
import type { GameRelease } from "@/generated-types/GameRelease";
import type { GameReleaseStatus } from "@/generated-types/GameReleaseStatus";
import type { GameVariant } from "@/generated-types/GameVariant";
import type { GameVariantInfo } from "@/generated-types/GameVariantInfo";
import type { InstallationProgressPayload } from "@/generated-types/InstallationProgressPayload";
import type { InstallationProgressStatus } from "@/generated-types/InstallationProgressStatus";
import type { ReleasesUpdatePayload } from "@/generated-types/ReleasesUpdatePayload";
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

export async function getTips(variant: GameVariant): Promise<string[]> {
  const response = await invoke<string[]>("get_tips", {
    variant,
  });

  return response;
}

export async function getLastPlayedVersion(
  variant: GameVariant,
): Promise<string> {
  const response = await invoke<string | null>("get_last_played_version", {
    variant,
  });

  // useQuery doesn't work with null/undefined query data. That's why "" is returned.
  return response ?? "";
}

export async function installReleaseForVariant(
  variant: GameVariant,
  releaseId: string,
): Promise<GameRelease> {
  const response = await invoke<GameRelease>("install_release", {
    variant,
    releaseId,
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
