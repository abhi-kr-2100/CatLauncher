import { useCallback, useEffect, useMemo, useState } from "react";
import {
  useForm,
  Controller,
  Control,
  RegisterOptions,
} from "react-hook-form";

import VariantSelector from "@/components/VariantSelector";
import { VirtualizedCombobox } from "@/components/virtualized-combobox";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Checkbox } from "@/components/ui/checkbox";
import {
  Field,
  FieldContent,
  FieldDescription,
  FieldError,
  FieldLabel,
} from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import type { GameVariant } from "@/generated-types/GameVariant";
import { WorldOption } from "@/generated-types/WorldOption";
import { useGameVariants } from "@/hooks/useGameVariants";
import { cn } from "@/lib/utils";
import { toastCL } from "@/lib/utils";
import {
  useUpdateWorldOptions,
  useWorldOptions,
  useWorlds,
} from "../hooks/useWorldOptions";
import { getOptionMetadata } from "../lib/worldOptionsMetadata";

interface WorldOptionsFormValues {
  options: WorldOption[];
}

function WorldOptionInput({
  option,
  index,
  control,
  variant,
}: {
  option: WorldOption;
  index: number;
  control: Control<WorldOptionsFormValues>;
  variant: GameVariant;
}) {
  const metadata = useMemo(
    () => getOptionMetadata(variant, option.name),
    [variant, option.name],
  );

  // Try to detect the type of input needed from the default/info string
  // or use metadata if available
  const isBoolean = useMemo(() => {
    if (metadata?.type === "boolean") return true;
    const defaultLower = option.default.toLowerCase();
    return (
      defaultLower.includes("default: true") ||
      defaultLower.includes("default: false")
    );
  }, [option.default, metadata]);

  const enumValues = useMemo(() => {
    if (metadata?.type === "enum" && metadata.options) {
      return metadata.options;
    }
    const match = option.default.match(/Values: ([\w, ]+)/);
    if (match) {
      return match[1]
        .split(",")
        .map((v) => v.trim())
        .filter(Boolean);
    }
    return null;
  }, [option.default, metadata]);

  const displayName = metadata?.name ?? option.name;
  const description = metadata?.description ?? option.info;

  const validationRules = useMemo(() => {
    const rules: RegisterOptions<
      WorldOptionsFormValues,
      `options.${number}.value`
    > = {};

    if (
      metadata?.type === "number" ||
      (!metadata && !isBoolean && !enumValues)
    ) {
      rules.validate = (v: string) => {
        const num = parseFloat(v);

        if (isNaN(num)) return "Must be a valid number";

        if (metadata?.min !== undefined && num < metadata.min)
          return `Minimum value is ${metadata.min}`;

        if (metadata?.max !== undefined && num > metadata.max)
          return `Maximum value is ${metadata.max}`;

        return true;
      };
    }

    return rules;
  }, [metadata, isBoolean, enumValues]);

  return (
    <Controller
      name={`options.${index}.value`}
      control={control}
      rules={validationRules}
      render={({ field, fieldState: { error } }) => (
        <Field className="pb-4">
          <FieldLabel
            className={cn(
              "text-sm font-semibold",
              error && "text-destructive",
            )}
          >
            {displayName}
          </FieldLabel>
          <FieldDescription className="text-xs">
            {description}
          </FieldDescription>
          <FieldContent className="mt-2">
            {isBoolean ? (
              <div className="flex items-center space-x-2">
                <Checkbox
                  id={`option-${option.name}`}
                  checked={field.value === "true"}
                  onCheckedChange={(checked) =>
                    field.onChange(checked ? "true" : "false")
                  }
                />
                <Label
                  htmlFor={`option-${option.name}`}
                  className="text-xs text-muted-foreground"
                >
                  Enabled
                </Label>
              </div>
            ) : enumValues ? (
              <VirtualizedCombobox
                items={enumValues.map((v) => ({
                  value: v,
                  label: v,
                }))}
                value={field.value}
                onChange={field.onChange}
                placeholder="Select value..."
                className="w-full max-w-xs"
              />
            ) : (
              <Input
                {...field}
                className={cn(
                  "max-w-xs",
                  error && "border-destructive",
                )}
                placeholder={option.default}
                type={metadata?.type === "number" ? "number" : "text"}
              />
            )}
            {error && <FieldError errors={[error]} />}
            <div className="mt-1 text-[10px] text-muted-foreground italic">
              {option.default}
            </div>
          </FieldContent>
        </Field>
      )}
    />
  );
}

export default function WorldOptions() {
  const onVariantsFetchError = useCallback((error: unknown) => {
    toastCL("error", "Failed to fetch game variants.", error);
  }, []);

  const { gameVariants, isLoading: isLoadingVariants } =
    useGameVariants({
      onFetchError: onVariantsFetchError,
    });

  const [selectedVariant, setSelectedVariant] =
    useState<GameVariant | null>(null);
  const [selectedWorld, setSelectedWorld] = useState<string | null>(
    null,
  );

  const { data: worlds = [], isLoading: isLoadingWorlds } = useWorlds(
    selectedVariant as GameVariant,
  );

  useEffect(() => {
    setSelectedWorld(null);
  }, [selectedVariant]);

  const {
    data: initialOptions,
    isLoading: isLoadingOptions,
    isError: isOptionsError,
    error: optionsError,
  } = useWorldOptions(selectedVariant as GameVariant, selectedWorld);

  const { mutate: updateOptions, isPending: isUpdating } =
    useUpdateWorldOptions();

  const form = useForm<WorldOptionsFormValues>({
    defaultValues: {
      options: [],
    },
  });

  useEffect(() => {
    if (initialOptions) {
      form.reset({ options: initialOptions });
    }
  }, [initialOptions, form]);

  useEffect(() => {
    if (isOptionsError) {
      toastCL("error", "Failed to load world options.", optionsError);
    }
  }, [isOptionsError, optionsError]);

  const onSubmit = form.handleSubmit(
    (data) => {
      if (!selectedVariant || !selectedWorld) return;

      updateOptions(
        {
          variant: selectedVariant,
          world: selectedWorld,
          options: data.options,
        },
        {
          onSuccess: () => {
            toastCL("success", "World options updated successfully.");
          },
          onError: (error) => {
            toastCL(
              "error",
              "Failed to update world options.",
              error,
            );
          },
        },
      );
    },
    () => {
      toastCL("error", "Please fix the errors in the form.");
    },
  );

  const worldItems = useMemo(
    () =>
      worlds.map((w) => ({
        value: w.name,
        label: w.name,
      })),
    [worlds],
  );

  return (
    <div className="space-y-6">
      <div className="flex flex-col gap-4 sm:flex-row sm:items-end">
        <div className="space-y-2">
          <Label>Game Variant</Label>
          <VariantSelector
            gameVariants={gameVariants}
            selectedVariant={selectedVariant}
            onVariantChange={setSelectedVariant}
            isLoading={isLoadingVariants}
          />
        </div>

        <div className="space-y-2">
          <Label>World</Label>
          <VirtualizedCombobox
            items={worldItems}
            value={selectedWorld ?? undefined}
            onChange={setSelectedWorld}
            placeholder={
              isLoadingWorlds ? "Loading worlds..." : "Select a world"
            }
            disabled={!selectedVariant || isLoadingWorlds}
            className="w-2xs"
          />
        </div>
      </div>

      {!selectedVariant && (
        <Card className="border-dashed">
          <CardContent className="py-10 text-center text-muted-foreground">
            Please select a game variant to see available worlds.
          </CardContent>
        </Card>
      )}

      {selectedVariant && !selectedWorld && !isLoadingWorlds && (
        <Card className="border-dashed">
          <CardContent className="py-10 text-center text-muted-foreground">
            {worlds.length > 0
              ? "Select a world to configure its options."
              : "No worlds found for this variant."}
          </CardContent>
        </Card>
      )}

      {selectedWorld && (
        <form onSubmit={onSubmit} className="space-y-6">
          <Card>
            <CardContent className="pt-6">
              {isLoadingOptions ? (
                <div className="py-10 text-center text-muted-foreground">
                  Loading options...
                </div>
              ) : initialOptions && initialOptions.length > 0 ? (
                <div className="space-y-4 divide-y">
                  {form.watch("options").map((option, index) => (
                    <WorldOptionInput
                      key={option.name}
                      option={option}
                      index={index}
                      control={form.control}
                      variant={selectedVariant!}
                    />
                  ))}
                </div>
              ) : (
                <div className="py-10 text-center text-muted-foreground">
                  No options found for this world.
                </div>
              )}
            </CardContent>
          </Card>

          <div className="flex justify-end gap-4">
            <Button
              type="button"
              variant="ghost"
              onClick={() => form.reset()}
              disabled={!form.formState.isDirty || isUpdating}
            >
              Cancel
            </Button>
            <Button
              type="submit"
              disabled={!form.formState.isDirty || isUpdating}
            >
              {isUpdating ? "Updating..." : "Update"}
            </Button>
          </div>
        </form>
      )}
    </div>
  );
}
