import { useCallback, useMemo } from "react";
import { Control, Controller } from "react-hook-form";

import {
  Field,
  FieldContent,
  FieldLabel,
} from "@/components/ui/field";
import { VirtualizedCombobox } from "@/components/virtualized-combobox";
import { ColorTheme } from "@/generated-types/ColorTheme";
import { Settings } from "@/generated-types/Settings";
import { toastCL } from "@/lib/utils";
import { useColorThemes } from "../hooks";

interface ColorSettingsProps {
  control: Control<Settings>;
}

function ColorSelector({
  control,
  themes,
  isLoading,
}: {
  control: Control<Settings>;
  themes: ColorTheme[];
  isLoading: boolean;
}) {
  const themeOptions = useMemo(() => {
    return themes.map((theme) => ({
      value: theme.path,
      label: <span className="truncate">{theme.name}</span>,
    }));
  }, [themes]);

  return (
    <Field>
      <div className="mb-3">
        <FieldLabel className="text-base">Color Theme</FieldLabel>
        <p className="text-sm text-muted-foreground mt-1">
          Select a color theme to apply to all games.
        </p>
      </div>
      <FieldContent>
        <Controller
          name="color_theme"
          control={control}
          render={({ field }) => (
            <VirtualizedCombobox
              items={themeOptions}
              value={field.value?.path ?? ""}
              onChange={(value) => {
                const selectedTheme = themes.find(
                  (t) => t.path === value,
                );
                field.onChange(selectedTheme ?? null);
              }}
              placeholder={
                isLoading
                  ? "Loading themes..."
                  : "Select a color theme..."
              }
              disabled={isLoading}
            />
          )}
        />
      </FieldContent>
    </Field>
  );
}

export function ColorSettings({ control }: ColorSettingsProps) {
  const onThemesError = useCallback(
    (e: Error) => toastCL("error", "Failed to load color themes.", e),
    [],
  );
  const { themes, isLoading } = useColorThemes(onThemesError);

  return (
    <ColorSelector
      control={control}
      themes={themes}
      isLoading={isLoading}
    />
  );
}
