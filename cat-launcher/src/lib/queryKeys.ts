import type { GameVariant } from "@/generated-types/GameVariant";

export const queryKeys = {
  activeRelease: (variant: GameVariant) =>
    ["active_release", variant] as const,

  installationStatus: (
    variant: GameVariant,
    releaseId: string | undefined,
  ) => ["installation_status", variant, releaseId] as const,

  releases: (variant: GameVariant) => ["releases", variant] as const,

  releaseNotes: (variant: GameVariant, version: string) =>
    ["release_notes", variant, version] as const,

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
    installationStatus: (variant: GameVariant, modId: string) =>
      ["mods", "installation_status", variant, modId] as const,
    lastActivity: (variant: GameVariant, modId: string) =>
      ["mods", "last_activity", variant, modId] as const,
  },

  tilesets: {
    listAll: (variant: GameVariant) => ["tilesets", variant] as const,
    installationStatus: (variant: GameVariant, tilesetId: string) =>
      [
        "tilesets",
        "installation_status",
        variant,
        tilesetId,
      ] as const,
  },

  soundpacks: {
    listAll: (variant: GameVariant) =>
      ["soundpacks", variant] as const,
    installationStatus: (variant: GameVariant, soundpackId: string) =>
      [
        "soundpacks",
        "installation_status",
        variant,
        soundpackId,
      ] as const,
  },

  lastPlayedWorld: (variant: GameVariant) =>
    ["last_played_world", variant] as const,

  fonts: () => ["fonts"] as const,
  colorThemes: () => ["color_themes"] as const,
  settings: () => ["settings"] as const,

  defaultSettings: () => ["default_settings"] as const,
};
