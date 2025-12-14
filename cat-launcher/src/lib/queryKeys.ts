import type { GameVariant } from "@/generated-types/GameVariant";

export const queryKeys = {
  activeRelease: (variant: GameVariant) =>
    ["active_release", variant] as const,

  installationStatus: (
    variant: GameVariant,
    releaseId: string | undefined,
  ) => ["installation_status", variant, releaseId] as const,

  releases: (variant: GameVariant) => ["releases", variant] as const,

  tips: (variant: GameVariant) => ["tips", variant] as const,

  playTimeForVariant: (variant: GameVariant) =>
    ["play_time_for_variant", variant] as const,

  playTimeForVersion: (
    variant: GameVariant,
    releaseId: string | undefined,
  ) => ["play_time_for_version", variant, releaseId] as const,

  gameVariantsInfo: () => ["gameVariantsInfo"] as const,

  userId: () => ["userId"] as const,

  backups: (variant: GameVariant) => ["backups", variant] as const,

  manualBackups: (variant: GameVariant) =>
    ["manual-backups", variant] as const,

  themePreference: () => ["theme_preference"] as const,

  mods: {
    listAll: (variant: GameVariant) => ["mods", variant] as const,
    installationStatus: (modId: string, variant: GameVariant) =>
      ["mods", "installation_status", modId, variant] as const,
  },

  tilesets: {
    listAll: (variant: GameVariant) => ["tilesets", variant] as const,
    installationStatus: (tilesetId: string, variant: GameVariant) =>
      [
        "tilesets",
        "installation_status",
        tilesetId,
        variant,
      ] as const,
  },

  soundpacks: {
    listAll: (variant: GameVariant) =>
      ["soundpacks", variant] as const,
    installationStatus: (soundpackId: string, variant: GameVariant) =>
      [
        "soundpacks",
        "installation_status",
        soundpackId,
        variant,
      ] as const,
  },

  lastPlayedWorld: (variant: GameVariant) =>
    ["last_played_world", variant] as const,
};
