import { useCallback, useMemo } from "react";
import { Control, Controller } from "react-hook-form";

import {
  Field,
  FieldContent,
  FieldDescription,
  FieldLabel,
} from "@/components/ui/field";
import { VirtualizedCombobox } from "@/components/virtualized-combobox";
import { Settings } from "@/generated-types/Settings";
import { toastCL } from "@/lib/utils";
import { useFonts } from "../hooks";

interface FontSettingsProps {
  control: Control<Settings>;
}

export function FontSettings({ control }: FontSettingsProps) {
  const onFontsError = useCallback(
    (e: Error) => toastCL("error", "Failed to load fonts.", e),
    [],
  );
  const { fonts, isLoading } = useFonts(onFontsError);

  const fontOptions = useMemo(() => {
    return fonts.map((font) => ({
      value: font.path,
      label: font.name,
    }));
  }, [fonts]);

  return (
    <Field>
      <FieldLabel>Font</FieldLabel>
      <FieldContent>
        <Controller
          name="font"
          control={control}
          render={({ field }) => (
            <VirtualizedCombobox
              items={fontOptions}
              value={field.value?.path ?? ""}
              onChange={(value) => {
                const selectedFont = fonts.find(
                  (f) => f.path === value,
                );
                field.onChange(selectedFont ?? null);
              }}
              placeholder={
                isLoading
                  ? "Loading fonts..."
                  : "Select a monospace font..."
              }
              disabled={isLoading}
            />
          )}
        />
        <FieldDescription>
          Select a monospace font to use in the game and launcher.
        </FieldDescription>
      </FieldContent>
    </Field>
  );
}
