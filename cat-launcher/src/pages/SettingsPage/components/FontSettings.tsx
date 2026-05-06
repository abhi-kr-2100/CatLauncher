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

/**
 * Props for the {@link FontSettings} component.
 */
interface FontSettingsProps {
  /** The form control for settings. */
  control: Control<Settings>;
}

/**
 * A label component that previews a font in its own typeface.
 *
 * @param props - The component props.
 * @param props.font - The font to preview.
 * @returns A span displaying the font name in the corresponding typeface.
 */
function FontPreviewLabel({ font }: { font: Font }) {
  const onFontLoadError = useCallback(
    (e: unknown) => {
      console.warn(
        `Failed to load font preview for: ${font.name}`,
        e,
      );
    },
    [font.name],
  );

  const fontFamily = useFontFamily(font, onFontLoadError);

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

/**
 * A component that allows selecting a font from a list of available fonts.
 *
 * @param props - The component props.
 * @param props.control - The form control for settings.
 * @param props.fonts - The list of available fonts.
 * @param props.isLoading - Whether the fonts are still loading.
 * @returns A field containing a font selector.
 */
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

/**
 * A component that displays a larger preview of a selected font with sample text.
 *
 * @param props - The component props.
 * @param props.selectedFont - The font to preview.
 * @returns A preview section showing sample text in the selected font.
 */
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

/**
 * The font settings section, allowing users to choose and preview fonts for the game.
 *
 * @param props - The component props.
 * @returns The font settings UI.
 */
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
