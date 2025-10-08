import { invoke } from "@tauri-apps/api/core";
import type { GameVariantInfo } from "@/generated-types/GameVariantInfo";
import type { GameRelease } from "@/generated-types/GameRelease";
import clsx, { type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";
import type { GameReleaseStatus } from "@/generated-types/GameReleaseStatus";
import type { GameVariant } from "@/generated-types/GameVariant";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export async function fetchReleasesForVariant(
  variant: GameVariant
): Promise<GameRelease[]> {
  const response = await invoke<GameRelease[]>("fetch_releases_for_variant", {
    variant,
  });
  return response;
}

export async function fetchGameVariantsInfo(): Promise<GameVariantInfo[]> {
  const response = await invoke<GameVariantInfo[]>("get_game_variants_info");
  return response;
}

export async function getLastPlayedVersion(
  variant: GameVariant
): Promise<string> {
  const response = await invoke<string | null>("get_last_played_version", {
    variant,
  });

  // useQuery doesn't work with null/undefined query data. That's why "" is returned.
  return response ?? "";
}

export async function installReleaseForVariant(
  variant: GameVariant,
  release_id: string
): Promise<GameRelease> {
  const response = await invoke<GameRelease>("install_release", {
    variant,
    release_id,
  });

  return response;
}

export async function launchGame(
  variant: GameVariant,
  release_id: string
): Promise<void> {
  const response = await invoke<void>("launch_game", {
    variant,
    release_id,
  });

  return response;
}

export async function getInstallationStatus(
  variant: GameVariant,
  release_id: string
): Promise<GameReleaseStatus> {
  const response = await invoke<GameReleaseStatus>("get_installation_status", {
    variant,
    release_id,
  });

  return response;
}
