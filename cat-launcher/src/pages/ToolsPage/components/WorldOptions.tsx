import { useMemo, useState } from "react";
import { Controller, useFieldArray } from "react-hook-form";
import { AlertTriangle } from "lucide-react";

import { Card, CardContent } from "@/components/ui/card";
import { Checkbox } from "@/components/ui/checkbox";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { VirtualizedCombobox } from "@/components/virtualized-combobox";
import { GameVariant } from "@/generated-types/GameVariant";
import { toastCL } from "@/lib/utils";
import { useGameVariants } from "@/hooks/useGameVariants";
import { useWorlds } from "../hooks/useWorlds";
import { useWorldOptionsForm } from "../hooks/useWorldOptionsForm";
import { WorldOptionsFooter } from "./WorldOptionsFooter";

type OptionType = "Boolean" | "Integer" | "Float" | "String" | "Enum";

interface OptionSchema {
  name: string;
  type: OptionType;
  min?: number;
  max?: number;
  values?: string[];
}

const SCHEMA: OptionSchema[] = [
  { name: "BLACK_ROAD", type: "Boolean" },
  { name: "ETERNAL_SEASON", type: "Boolean" },
  {
    name: "CONSTRUCTION_SCALING",
    type: "Integer",
    min: 0,
    max: 1000,
  },
  { name: "SEASON_LENGTH", type: "Integer", min: 14, max: 127 },
  {
    name: "WORLD_END",
    type: "Enum",
    values: ["reset", "delete", "query", "keep"],
  },
  { name: "ITEM_SPAWNRATE", type: "Float", min: 0.01, max: 10.0 },
  { name: "SPAWN_DENSITY", type: "Float", min: 0.0, max: 50.0 },
  {
    name: "EVOLUTION_INVERSE_MULTIPLIER",
    type: "Float",
    min: 0.0,
    max: 100.0,
  },
  {
    name: "ETERNAL_TIME_OF_DAY",
    type: "Enum",
    values: ["normal", "day", "night"],
  },
  { name: "NPC_SPAWNTIME", type: "Float", min: 0.0, max: 100.0 },
  {
    name: "MONSTER_RESILIENCE",
    type: "Integer",
    min: 1,
    max: 1000,
  },
  { name: "META_PROGRESS", type: "Boolean" },
  { name: "MONSTER_SPEED", type: "Integer", min: 1, max: 1000 },
  { name: "INITIAL_DAY", type: "Integer", min: -1, max: 999 },
  { name: "VEHICLE_SPAWNRATE", type: "Float", min: 0.0, max: 5.0 },
  { name: "CARRION_SPAWNRATE", type: "Float", min: 0.0, max: 10.0 },
  { name: "SPECIALS_DENSITY", type: "Float", min: 0.01, max: 10.0 },
  { name: "SPAWN_DELAY", type: "Integer", min: 0, max: 9999 },
  {
    name: "SPAWN_ANIMAL_DENSITY",
    type: "Float",
    min: 0.0,
    max: 50.0,
  },
  { name: "CITY_SIZE", type: "Integer", min: 0, max: 16 },
  {
    name: "MONSTER_UPGRADE_FACTOR",
    type: "Float",
    min: 0.0,
    max: 100.0,
  },
  {
    name: "STARTING_NPC",
    type: "Enum",
    values: ["never", "always", "scenario"],
  },
  { name: "SPECIALS_SPACING", type: "Integer", min: -1, max: 72 },
  { name: "CITY_SPACING", type: "Integer", min: 0, max: 8 },
  { name: "WANDER_SPAWNS", type: "Boolean" },
  { name: "VEHICLE_DAMAGE", type: "Float", min: 0.0, max: 10.0 },
  {
    name: "CRAFTING_SPEED_MULT",
    type: "Integer",
    min: 0,
    max: 1000,
  },
  { name: "GROWTH_SCALING", type: "Integer", min: 0, max: 1000 },
  { name: "DEFAULT_REGION", type: "Enum", values: ["default"] },
  { name: "RANDOM_NPC", type: "Boolean" },
  { name: "INITIAL_TIME", type: "Integer", min: 0, max: 23 },
  { name: "VEHICLE_LOCKS", type: "Boolean" },
  { name: "RAD_MUTATION", type: "Boolean" },
  { name: "NPC_DENSITY", type: "Float", min: 0.0, max: 100.0 },
  {
    name: "CHARACTER_POINT_POOLS",
    type: "Enum",
    values: ["any", "multi_pool", "no_freeform"],
  },
  { name: "RESTOCK_DELAY_MULT", type: "Float", min: 0.01, max: 10.0 },
];

const OPTION_NAME_MAPPING: Record<string, string> = {
  BLACK_ROAD: "Black Road",
  ETERNAL_SEASON: "Eternal Season",
  CONSTRUCTION_SCALING: "Construction Scaling",
  SEASON_LENGTH: "Season Length",
  WORLD_END: "World End Handling",
  ITEM_SPAWNRATE: "Item Spawn Rate",
  SPAWN_DENSITY: "Monster Spawn Density",
  EVOLUTION_INVERSE_MULTIPLIER: "Monster Evolution Multiplier",
  ETERNAL_TIME_OF_DAY: "Eternal Time of Day",
  NPC_SPAWNTIME: "NPC Spawn Delay",
  MONSTER_RESILIENCE: "Monster Resilience",
  META_PROGRESS: "Meta Progression",
  MONSTER_SPEED: "Monster Speed",
  INITIAL_DAY: "Initial Day",
  VEHICLE_SPAWNRATE: "Vehicle Spawn Density",
  CARRION_SPAWNRATE: "Carrion Spawn Density",
  SPECIALS_DENSITY: "Overmap Specials Density",
  SPAWN_DELAY: "Spawn Delay",
  SPAWN_ANIMAL_DENSITY: "Animal Spawn Density",
  CITY_SIZE: "City Size",
  MONSTER_UPGRADE_FACTOR: "Monster Upgrade Factor",
  STARTING_NPC: "Starting NPC Spawn",
  SPECIALS_SPACING: "Overmap Specials Spacing",
  CITY_SPACING: "City Spacing",
  WANDER_SPAWNS: "Wander Spawns (Hordes)",
  VEHICLE_DAMAGE: "Vehicle Damage Multiplier",
  CRAFTING_SPEED_MULT: "Crafting Speed Multiplier",
  GROWTH_SCALING: "Crop Growth Scaling",
  DEFAULT_REGION: "Default Region",
  RANDOM_NPC: "Random NPC Spawns",
  INITIAL_TIME: "Initial Time of Day",
  VEHICLE_LOCKS: "Vehicle Door Locks",
  RAD_MUTATION: "Radiation Mutations",
  NPC_DENSITY: "Dynamic NPC Density",
  CHARACTER_POINT_POOLS: "Character Point Pools",
  RESTOCK_DELAY_MULT: "Merchant Restock Delay",
};

const ENUM_VALUE_MAPPING: Record<string, string> = {
  reset: "Reset World",
  delete: "Delete World",
  query: "Query User",
  keep: "Keep World",
  never: "Never",
  always: "Always",
  scenario: "Based on Scenario",
  normal: "Normal Cycle",
  day: "Eternal Day",
  night: "Eternal Night",
  any: "Any Pool",
  multi_pool: "Multiple Pools",
  no_freeform: "No Freeform",
  default: "Default Region",
};

export default function WorldOptions() {
  const [selectedVariant, setSelectedVariant] =
    useState<string>("DarkDaysAhead");
  const [selectedWorldName, setSelectedWorldName] =
    useState<string>("");

  const { gameVariants } = useGameVariants();
  const { data: worlds } = useWorlds(selectedVariant as GameVariant);

  const { form, isLoading, isUpdating, apply, cancel } =
    useWorldOptionsForm({
      variant: selectedVariant as GameVariant,
      worldName: selectedWorldName,
      onWorldOptionsError: (error) =>
        toastCL("error", "Failed to load world options.", error),
      onUpdateError: (error) =>
        toastCL("error", "Failed to update world options.", error),
      onUpdateSuccess: () =>
        toastCL("success", "World options updated successfully."),
    });

  const { fields } = useFieldArray({
    control: form.control,
    name: "options",
  });

  const variantOptions = useMemo(() => {
    return (gameVariants ?? []).map((v) => ({
      value: v.id,
      label: v.name,
    }));
  }, [gameVariants]);

  const worldOptions = useMemo(() => {
    return (worlds ?? []).map((w) => ({
      value: w.name,
      label: w.name,
    }));
  }, [worlds]);

  const isDirty = form.formState.isDirty;

  const getFriendlyName = (name: string) => {
    if (OPTION_NAME_MAPPING[name]) {
      return OPTION_NAME_MAPPING[name];
    }
    if (name.startsWith("SPAWN_RATE_")) {
      const category = name
        .replace("SPAWN_RATE_", "")
        .replace(/_/g, " ");
      return `Spawn Rate: ${category.charAt(0).toUpperCase() + category.slice(1)}`;
    }
    return name;
  };

  const getFriendlyEnumValue = (value: string) => {
    return ENUM_VALUE_MAPPING[value] || value;
  };

  return (
    <div className="space-y-6 pb-24">
      <div className="flex flex-col gap-4 md:flex-row md:items-end">
        <div className="w-full md:w-64">
          <Label className="mb-2 block text-sm font-medium">
            Variant
          </Label>
          <VirtualizedCombobox
            items={variantOptions}
            value={selectedVariant}
            onChange={(v: string) => {
              setSelectedVariant(v);
              setSelectedWorldName("");
            }}
            placeholder="Select a variant"
          />
        </div>
        <div className="w-full md:w-64">
          <Label className="mb-2 block text-sm font-medium">
            World
          </Label>
          <VirtualizedCombobox
            items={worldOptions}
            value={selectedWorldName}
            onChange={setSelectedWorldName}
            placeholder={
              worlds?.length ? "Select a world" : "No worlds found"
            }
            disabled={!worlds?.length}
          />
        </div>
      </div>

      {selectedWorldName ? (
        <Card>
          <CardContent className="pt-6">
            {isLoading ? (
              <div className="py-8 text-center text-muted-foreground">
                Loading options...
              </div>
            ) : fields.length === 0 ? (
              <div className="py-8 text-center text-muted-foreground">
                No options found in worldoptions.json
              </div>
            ) : (
              <div className="grid gap-6 md:grid-cols-2">
                {fields.map((field, index) => {
                  const option = field;
                  const schema = SCHEMA.find(
                    (s) => s.name === option.name,
                  );
                  const isSpawnRate =
                    option.name.startsWith("SPAWN_RATE_");
                  const isKnown = !!schema || isSpawnRate;

                  const type = schema
                    ? schema.type
                    : isSpawnRate
                      ? "Float"
                      : option.value === "true" ||
                          option.value === "false"
                        ? "Boolean"
                        : "String";

                  return (
                    <div
                      key={field.id}
                      className="space-y-2 rounded-lg border p-4 shadow-sm"
                    >
                      <div className="flex items-start justify-between">
                        <div className="flex flex-col">
                          <Label className="text-base font-semibold">
                            {getFriendlyName(option.name)}
                          </Label>
                          {isKnown && (
                            <span className="text-[10px] text-muted-foreground font-mono uppercase">
                              {option.name}
                            </span>
                          )}
                        </div>
                        {!isKnown && (
                          <div
                            className="flex items-center gap-1 text-xs text-amber-500"
                            title="Unknown option"
                          >
                            <AlertTriangle className="h-3 w-3" />
                            <span>Unknown</span>
                          </div>
                        )}
                      </div>

                      {option.info && (
                        <p className="text-sm text-muted-foreground">
                          {option.info}
                        </p>
                      )}

                      <div className="pt-2">
                        {type === "Boolean" ? (
                          <div className="flex items-center space-x-2">
                            <Controller
                              control={form.control}
                              name={`options.${index}.value`}
                              render={({ field }) => (
                                <Checkbox
                                  id={`option-${option.name}`}
                                  checked={field.value === "true"}
                                  onCheckedChange={(checked) =>
                                    field.onChange(
                                      checked ? "true" : "false",
                                    )
                                  }
                                />
                              )}
                            />
                            <Label
                              htmlFor={`option-${option.name}`}
                              className="text-sm font-normal cursor-pointer"
                            >
                              {form.getValues(
                                `options.${index}.value`,
                              ) === "true"
                                ? "Enabled"
                                : "Disabled"}
                            </Label>
                          </div>
                        ) : type === "Enum" ? (
                          <Controller
                            control={form.control}
                            name={`options.${index}.value`}
                            render={({ field }) => (
                              <VirtualizedCombobox
                                items={
                                  schema?.values?.map((v) => ({
                                    value: v,
                                    label: getFriendlyEnumValue(v),
                                  })) ?? []
                                }
                                value={field.value}
                                onChange={field.onChange}
                              />
                            )}
                          />
                        ) : (
                          <Input
                            {...form.register(
                              `options.${index}.value`,
                              {
                                validate: (value) => {
                                  if (type === "Integer") {
                                    const cleanValue = value.endsWith(
                                      "%",
                                    )
                                      ? value.slice(0, -1)
                                      : value;
                                    const parsed =
                                      parseInt(cleanValue);
                                    if (isNaN(parsed))
                                      return "Must be an integer";
                                    if (
                                      schema?.min !== undefined &&
                                      parsed < schema.min
                                    )
                                      return `Must be at least ${schema.min}`;
                                    if (
                                      schema?.max !== undefined &&
                                      parsed > schema.max
                                    )
                                      return `Must be at most ${schema.max}`;
                                  }
                                  if (type === "Float") {
                                    const parsed = parseFloat(value);
                                    if (isNaN(parsed))
                                      return "Must be a number";
                                    const min =
                                      schema?.min ??
                                      (isSpawnRate ? 0 : undefined);
                                    const max =
                                      schema?.max ??
                                      (isSpawnRate ? 20 : undefined);
                                    if (
                                      min !== undefined &&
                                      parsed < min
                                    )
                                      return `Must be at least ${min}`;
                                    if (
                                      max !== undefined &&
                                      parsed > max
                                    )
                                      return `Must be at most ${max}`;
                                  }
                                  return true;
                                },
                              },
                            )}
                            type="text"
                            placeholder="Value"
                          />
                        )}
                        {form.formState.errors.options?.[index]
                          ?.value && (
                          <p className="mt-1 text-xs text-destructive">
                            {
                              form.formState.errors.options[index]
                                .value.message
                            }
                          </p>
                        )}
                      </div>

                      {option.default && (
                        <p className="mt-1 text-xs text-muted-foreground italic">
                          {option.default}
                        </p>
                      )}
                    </div>
                  );
                })}
              </div>
            )}
          </CardContent>
        </Card>
      ) : (
        <div className="rounded-lg border border-dashed p-12 text-center text-muted-foreground">
          Select a variant and a world to manage its options.
        </div>
      )}

      {selectedWorldName && (
        <WorldOptionsFooter
          isDirty={isDirty}
          isUpdating={isUpdating}
          apply={apply}
          cancel={cancel}
        />
      )}
    </div>
  );
}
