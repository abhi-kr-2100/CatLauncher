import { useCallback, useMemo } from "react";
import { Control, Controller, useWatch } from "react-hook-form";

import {
  Field,
  FieldContent,
  FieldLabel,
} from "@/components/ui/field";
import { VirtualizedCombobox } from "@/components/virtualized-combobox";
import { Font } from "@/generated-types/Font";
import { Settings } from "@/generated-types/Settings";
import { toastCL } from "@/lib/utils";
import { useFontFamily, useFonts } from "../hooks";

interface FontSettingsProps {
  control: Control<Settings>;
}

function FontPreviewLabel({ font }: { font: Font }) {
  const fontFamily = useFontFamily(font);

  return (
    <span
      style={{
        fontFamily: fontFamily
          ? `'${fontFamily}', monospace`
          : "monospace",
      }}
      className="truncate"
    >
      {font.name}
    </span>
  );
}

function FontSelector({
  control,
  fonts,
  isLoading,
}: {
  control: Control<Settings>;
  fonts: Font[];
  isLoading: boolean;
}) {
  const fontOptions = useMemo(() => {
    return fonts.map((font) => ({
      value: font.path,
      label: <FontPreviewLabel font={font} />,
    }));
  }, [fonts]);

  return (
    <Field>
      <div className="mb-3">
        <FieldLabel className="text-base">Font</FieldLabel>
        <p className="text-sm text-muted-foreground mt-1">
          This will change the font used in all Cataclysm games. To
          see more font options, install fonts for your operating
          system.
        </p>
      </div>
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
      </FieldContent>
    </Field>
  );
}

function FontPreview({ selectedFont }: { selectedFont: Font }) {
  const onFontLoadError = useCallback(
    (e: unknown) => {
      toastCL(
        "error",
        `Failed to load font: ${selectedFont.name}`,
        e,
      );
    },
    [selectedFont.name],
  );

  const fontFamily = useFontFamily(selectedFont, onFontLoadError);

  return (
    <div className="mt-6 rounded-lg border bg-muted/30 p-6">
      <div
        className="space-y-3"
        style={{
          fontFamily: fontFamily
            ? `'${fontFamily}', monospace`
            : "monospace",
        }}
      >
        <p className="text-xs font-semibold uppercase tracking-wider text-muted-foreground">
          Preview: {selectedFont.name}
        </p>
        <p className="text-xl leading-snug">
          A quick brown fox jumps over the lazy dog.
        </p>
        <p className="break-all text-sm opacity-80">
          ABCDEFGHIJKLMNOPQRSTUVWXYZ
          <br />
          abcdefghijklmnopqrstuvwxyz
          <br />
          0123456789 !@#$%^&*()_+
        </p>
      </div>
    </div>
  );
}

export function FontSettings({ control }: FontSettingsProps) {
  const onFontsError = useCallback(
    (e: Error) => toastCL("error", "Failed to load fonts.", e),
    [],
  );
  const { fonts, isLoading } = useFonts(onFontsError);

  const selectedFont = useWatch({
    control,
    name: "font",
  });

  return (
    <>
      <FontSelector
        control={control}
        fonts={fonts}
        isLoading={isLoading}
      />
      {selectedFont && <FontPreview selectedFont={selectedFont} />}
    </>
  );
}
