import { useEffect, useMemo } from "react";
import { useForm, Control } from "react-hook-form";

import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import type { GameVariant } from "@/generated-types/GameVariant";
import { WorldOption } from "@/generated-types/WorldOption";
import { WorldOptionMetadata } from "@/generated-types/WorldOptionMetadata";
import { toastCL } from "@/lib/utils";

import {
  useHasMappedOptions,
  useUpdateWorldOptions,
  useWorldOptions,
  useWorldOptionsMetadata,
} from "./useWorldOptions";
import {
  WorldOptionInput,
  WorldOptionsFormValues,
} from "./WorldOptionInput";
import { GroupOption } from "./inputs/GroupOption";

interface WorldOptionsFormProps {
  variant: GameVariant;
  world: string;
}

interface MetadataRendererProps {
  metadataMap: Record<string, WorldOptionMetadata>;
  options: WorldOption[];
  optionsIndexMap: Map<string, number>;
  control: Control<WorldOptionsFormValues>;
  level?: number;
}

function MetadataRenderer({
  metadataMap,
  options,
  optionsIndexMap,
  control,
  level = 0,
}: MetadataRendererProps) {
  return (
    <div className={level > 0 ? "pl-4 space-y-4" : "space-y-4"}>
      {Object.values(metadataMap).map((meta) => {
        if (meta.type === "group" && meta.children) {
          return (
            <GroupOption key={meta.id} metadata={meta}>
              <MetadataRenderer
                metadataMap={meta.children}
                options={options}
                optionsIndexMap={optionsIndexMap}
                control={control}
                level={level + 1}
              />
            </GroupOption>
          );
        }

        const optionIndex = optionsIndexMap.get(meta.id);
        if (optionIndex === undefined) return null;

        return (
          <div key={meta.id} className="pt-2 first:pt-0">
            <WorldOptionInput
              option={options[optionIndex]}
              index={optionIndex}
              control={control}
              metadata={meta}
            />
          </div>
        );
      })}
    </div>
  );
}

export function WorldOptionsForm({
  variant,
  world,
}: WorldOptionsFormProps) {
  const { data: metadataMap, isLoading: isLoadingMetadata } =
    useWorldOptionsMetadata(variant, (error) => {
      toastCL(
        "error",
        "Failed to load world options metadata.",
        error,
      );
    });

  const { data: initialOptions, isLoading: isLoadingOptions } =
    useWorldOptions(variant, world, (error) => {
      toastCL("error", "Failed to load world options.", error);
    });

  const hasMappedOptions = useHasMappedOptions(
    initialOptions,
    metadataMap,
  );

  const { mutate: updateOptions, isPending: isUpdating } =
    useUpdateWorldOptions((error) => {
      toastCL("error", "Failed to update world options.", error);
    });

  const form = useForm<WorldOptionsFormValues>({
    defaultValues: {
      options: [],
    },
    mode: "onChange",
  });

  const { isDirty } = form.formState;

  useEffect(() => {
    if (initialOptions && !isDirty) {
      form.reset({ options: initialOptions });
    }
  }, [initialOptions, form, isDirty]);

  const onSubmit = form.handleSubmit(
    (data) => {
      updateOptions(
        {
          variant,
          world,
          options: data.options,
        },
        {
          onSuccess: () => {
            toastCL("success", "World options updated successfully.");
            form.reset(data);
          },
        },
      );
    },
    () => {
      toastCL("error", "Please fix the errors in the form.");
    },
  );

  const optionsIndexMap = useMemo(() => {
    const map = new Map<string, number>();
    if (initialOptions) {
      initialOptions.forEach((opt, index) => {
        map.set(opt.name, index);
      });
    }
    return map;
  }, [initialOptions]);

  if (isLoadingOptions || isLoadingMetadata) {
    return (
      <Card>
        <CardContent className="py-10 text-center text-muted-foreground">
          Loading options...
        </CardContent>
      </Card>
    );
  }

  if (!hasMappedOptions || !metadataMap || !initialOptions) {
    return (
      <Card>
        <CardContent className="py-10 text-center text-muted-foreground">
          No supported options found for this world.
        </CardContent>
      </Card>
    );
  }

  return (
    <form
      onSubmit={onSubmit}
      className="space-y-6 flex flex-col min-h-full"
    >
      <div className="flex-1">
        <Card>
          <CardContent className="pt-6">
            <MetadataRenderer
              metadataMap={metadataMap}
              options={initialOptions}
              optionsIndexMap={optionsIndexMap}
              control={form.control}
            />
          </CardContent>
        </Card>
      </div>

      <div className="sticky bottom-0 border-t bg-background p-4 shadow-lg z-40 -mx-8 -mb-8">
        <div className="flex justify-end gap-4 px-8">
          <Button
            type="button"
            variant="ghost"
            onClick={() => form.reset()}
            disabled={!isDirty || isUpdating}
          >
            Cancel
          </Button>
          <Button type="submit" disabled={!isDirty || isUpdating}>
            {isUpdating ? "Updating..." : "Update"}
          </Button>
        </div>
      </div>
    </form>
  );
}
