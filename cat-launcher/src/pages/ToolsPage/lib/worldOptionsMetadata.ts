import type { GameVariant } from "@/generated-types/GameVariant";

export interface WorldOptionMetadata {
  id: string;

  name: string;

  description: string;

  type: "boolean" | "number" | "enum" | "string";

  options?: string[];

  min?: number;

  max?: number;

  validation?: (value: string) => string | boolean;
}

const COMMON_METADATA: Record<
  string,
  Partial<WorldOptionMetadata>
> = {
  EVOLUTION_INVERSE_MULTIPLIER: {
    name: "Monster Evolution Scaling",

    description:
      "Multiplier for the time between monster upgrades. Higher values mean slower evolution. 0.00 disables upgrades.",

    type: "number",

    min: 0,

    max: 100,
  },

  ETERNAL_TIME_OF_DAY: {
    name: "Eternal Time of Day",

    description:
      "Determines if the game world has a normal day/night cycle or is stuck in one state.",

    type: "enum",

    options: ["normal", "day", "night"],
  },

  WANDER_SPAWNS: {
    name: "Wandering Hordes",

    description:
      "If enabled, zombies group into hordes that wander and move towards noise.",

    type: "boolean",
  },

  NPC_SPAWNTIME: {
    name: "NPC Spawn Rate",

    description:
      "Average days between random NPC spawns. Higher values mean fewer NPCs. 0 disables random NPCs.",

    type: "number",

    min: 0,

    max: 100,
  },

  WORLD_END: {
    name: "World End Behavior",

    description:
      "What happens to the world when the last character dies.",

    type: "enum",

    options: ["reset", "delete", "query", "keep"],
  },

  CITY_SIZE: {
    name: "City Size",

    description:
      "Determines how large cities are. 0 disables cities.",

    type: "number",

    min: 0,

    max: 16,
  },

  SPAWN_DENSITY: {
    name: "Monster Spawn Density",

    description:
      "Scaling factor for monster spawn density. Higher values mean more monsters.",

    type: "number",

    min: 0,

    max: 50,
  },

  ETERNAL_SEASON: {
    name: "Eternal Season",

    description: "If enabled, the initial season lasts forever.",

    type: "boolean",
  },

  DEFAULT_REGION: {
    name: "Default Region",

    description: "Determines terrain, shops, plants, and more.",

    type: "enum",

    options: ["default"],
  },

  CITY_SPACING: {
    name: "City Spacing",

    description: "Determines how far apart cities are.",

    type: "number",

    min: 0,

    max: 8,
  },

  BLACK_ROAD: {
    name: "Black Road",

    description:
      "If enabled, zombies spawn at shelters, making the start harder.",

    type: "boolean",
  },
};

export const WORLD_OPTIONS_METADATA: Record<
  GameVariant,
  Record<string, WorldOptionMetadata>
> = {
  DarkDaysAhead: Object.entries(COMMON_METADATA).reduce(
    (acc, [id, meta]) => {
      acc[id] = { id, ...meta } as WorldOptionMetadata;
      return acc;
    },
    {} as Record<string, WorldOptionMetadata>,
  ),
  BrightNights: Object.entries(COMMON_METADATA).reduce(
    (acc, [id, meta]) => {
      acc[id] = { id, ...meta } as WorldOptionMetadata;
      return acc;
    },
    {} as Record<string, WorldOptionMetadata>,
  ),
  TheLastGeneration: Object.entries(COMMON_METADATA).reduce(
    (acc, [id, meta]) => {
      acc[id] = { id, ...meta } as WorldOptionMetadata;
      return acc;
    },
    {} as Record<string, WorldOptionMetadata>,
  ),
};

export function getOptionMetadata(
  variant: GameVariant,
  optionId: string,
): WorldOptionMetadata | undefined {
  return WORLD_OPTIONS_METADATA[variant]?.[optionId];
}
