import type { GameVariant } from "@/generated-types/GameVariant";

/**
 * A collection of query key factories for use with React Query.
 */
export const queryKeys = {
  /**
   * Key for the active release of a specific game variant.
   */
  activeRelease: (variant: GameVariant) =>
    ["active_release", variant] as const,

  /**
   * Key for the installation status of a specific release and variant.
   */
  installationStatus: (
    variant: GameVariant,
    releaseId: string | undefined,
  ) => ["installation_status", variant, releaseId] as const,

  /**
   * Key for the list of releases for a specific game variant.
   */
  releases: (variant: GameVariant) => ["releases", variant] as const,

  /**
   * Key for release notes of a specific version and variant.
   */
  releaseNotes: (variant: GameVariant, version: string) =>
    ["release_notes", variant, version] as const,

  /**
   * Key for game tips of a specific variant.
   */
  tips: (variant: GameVariant) => ["tips", variant] as const,

  /**
   * Key for total play time of a specific variant.
   */
  playTimeForVariant: (variant: GameVariant) =>
    ["play_time_for_variant", variant] as const,

  /**
   * Key for play time of a specific version and variant.
   */
  playTimeForVersion: (
    variant: GameVariant,
    releaseId: string | undefined,
  ) => ["play_time_for_version", variant, releaseId] as const,

  /**
   * Key for general game variants information.
   */
  gameVariantsInfo: () => ["gameVariantsInfo"] as const,

  /**
   * Key for the user's unique identifier.
   */
  userId: () => ["userId"] as const,

  /**
   * Key for the list of backups for a specific variant.
   */
  backups: (variant: GameVariant) => ["backups", variant] as const,

  /**
   * Key for the list of manual backups for a specific variant.
   */
  manualBackups: (variant: GameVariant) =>
    ["manual-backups", variant] as const,

  /**
   * Key for the user's theme preference.
   */
  themePreference: () => ["theme_preference"] as const,

  /**
   * Query keys for mod-related queries.
   */
  mods: {
    /**
     * Key for listing all mods for a specific variant.
     */
    listAll: (variant: GameVariant) => ["mods", variant] as const,
    /**
     * Key for the installation status of a specific mod and variant.
     */
    installationStatus: (variant: GameVariant, modId: string) =>
      ["mods", "installation_status", variant, modId] as const,
    /**
     * Key for the last activity on a specific mod and variant.
     */
    lastActivity: (variant: GameVariant, modId: string) =>
      ["mods", "last_activity", variant, modId] as const,
  },

  /**
   * Query keys for tileset-related queries.
   */
  tilesets: {
    /**
     * Key for listing all tilesets for a specific variant.
     */
    listAll: (variant: GameVariant) => ["tilesets", variant] as const,
    /**
     * Key for the installation status of a specific tileset and variant.
     */
    installationStatus: (variant: GameVariant, tilesetId: string) =>
      [
        "tilesets",
        "installation_status",
        variant,
        tilesetId,
      ] as const,
  },

  /**
   * Query keys for soundpack-related queries.
   */
  soundpacks: {
    /**
     * Key for listing all soundpacks for a specific variant.
     */
    listAll: (variant: GameVariant) =>
      ["soundpacks", variant] as const,
    /**
     * Key for the installation status of a specific soundpack and variant.
     */
    installationStatus: (variant: GameVariant, soundpackId: string) =>
      [
        "soundpacks",
        "installation_status",
        variant,
        soundpackId,
      ] as const,
  },

  /**
   * Key for the last played world of a specific variant.
   */
  lastPlayedWorld: (variant: GameVariant) =>
    ["last_played_world", variant] as const,

  /**
   * Key for the list of available fonts.
   */
  fonts: () => ["fonts"] as const,
  /**
   * Key for the list of available color themes.
   */
  colorThemes: () => ["color_themes"] as const,
  /**
   * Key for the application settings.
   */
  settings: () => ["settings"] as const,

  /**
   * Key for the default application settings.
   */
  defaultSettings: () => ["default_settings"] as const,

  /**
   * Key for character achievements of a specific variant.
   */
  achievements: (variant: GameVariant) =>
    ["achievements", variant] as const,
};
