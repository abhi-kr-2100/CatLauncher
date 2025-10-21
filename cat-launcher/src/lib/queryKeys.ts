import type { GameVariant } from "@/generated-types/GameVariant";

export const queryKeys = {
  lastPlayedVersion: (variant: GameVariant) =>
    ["last_played_version", variant] as const,

  installationStatus: (variant: GameVariant, releaseId: string | undefined) =>
    ["installation_status", variant, releaseId] as const,

  releases: (variant: GameVariant) => ["releases", variant] as const,

  tips: (variant: GameVariant) => ["tips", variant] as const,
};
