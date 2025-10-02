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

export async function installReleaseForVariant(
  release: GameRelease
): Promise<string> {
  const response = await invoke<string>("install_release", {
    release,
  });

  return response;
}
