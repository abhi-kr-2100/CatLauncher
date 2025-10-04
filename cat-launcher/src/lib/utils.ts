import { invoke } from "@tauri-apps/api/core";
import type { GameVariantInfo } from "@/generated-types/GameVariantInfo";
import type { GameRelease } from "@/generated-types/GameRelease";
import clsx, { type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export async function fetchReleasesForVariant(
  variant: GameVariantInfo
): Promise<GameRelease[]> {
  const response = await invoke<GameRelease[]>("fetch_releases_for_variant", {
    variant: variant.id,
  });
  return response;
}

export async function fetchGameVariantsInfo(): Promise<GameVariantInfo[]> {
  const response = await invoke<GameVariantInfo[]>("get_game_variants_info");
  return response;
}

export async function getLastPlayedVersion(
  variant: GameVariantInfo
): Promise<string> {
  const response = await invoke<string | null>("get_last_played_version", {
    variant: variant.id,
  });

  // useQuery doesn't work with null/undefined query data. That's why "" is returned.
  return response ?? "";
}

export async function installReleaseForVariant(
  release: GameRelease
): Promise<void> {
  const response = await invoke<void>("install_release", {
    release,
  });

  return response;
}

export async function launchGame(release: GameRelease): Promise<void> {
  const response = await invoke<void>("launch_game", {
    release,
  });

  return response;
}
