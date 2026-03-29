import { useCallback, useMemo } from "react";
import { Control, Controller, useWatch } from "react-hook-form";

import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";
import {
  Field,
  FieldContent,
  FieldLabel,
} from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { VirtualizedCombobox } from "@/components/virtualized-combobox";
import { Font } from "@/generated-types/Font";
import { Settings } from "@/generated-types/Settings";
import { toastCL } from "@/lib/utils";
import { useFontFamily, useFonts } from "../hooks";

interface FontSettingsProps {
  control: Control<Settings>;
}

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

function FontSizeSettings({
  control,
}: {
  control: Control<Settings>;
}) {
  return (
    <div className="space-y-6">
      <div>
        <h3 className="text-sm font-medium mb-3">UI Font Settings</h3>
        <div className="grid grid-cols-3 gap-4">
          <Field>
            <FieldLabel className="text-sm">Font Size</FieldLabel>
            <FieldContent>
              <Controller
                name="font_size"
                control={control}
                render={({ field }) => (
                  <Input
                    type="number"
                    {...field}
                    onChange={(e) => {
                      const val = parseInt(e.target.value);
                      field.onChange(isNaN(val) ? 0 : val);
                    }}
                  />
                )}
              />
            </FieldContent>
          </Field>
          <Field>
            <FieldLabel className="text-sm">Font Width</FieldLabel>
            <FieldContent>
              <Controller
                name="font_width"
                control={control}
                render={({ field }) => (
                  <Input
                    type="number"
                    {...field}
                    onChange={(e) => {
                      const val = parseInt(e.target.value);
                      field.onChange(isNaN(val) ? 0 : val);
                    }}
                  />
                )}
              />
            </FieldContent>
          </Field>
          <Field>
            <FieldLabel className="text-sm">Font Height</FieldLabel>
            <FieldContent>
              <Controller
                name="font_height"
                control={control}
                render={({ field }) => (
                  <Input
                    type="number"
                    {...field}
                    onChange={(e) => {
                      const val = parseInt(e.target.value);
                      field.onChange(isNaN(val) ? 0 : val);
                    }}
                  />
                )}
              />
            </FieldContent>
          </Field>
        </div>
      </div>

      <div>
        <h3 className="text-sm font-medium mb-3">
          Map Font Settings
        </h3>
        <div className="grid grid-cols-3 gap-4">
          <Field>
            <FieldLabel className="text-sm">Map Font Size</FieldLabel>
            <FieldContent>
              <Controller
                name="map_font_size"
                control={control}
                render={({ field }) => (
                  <Input
                    type="number"
                    {...field}
                    onChange={(e) => {
                      const val = parseInt(e.target.value);
                      field.onChange(isNaN(val) ? 0 : val);
                    }}
                  />
                )}
              />
            </FieldContent>
          </Field>
          <Field>
            <FieldLabel className="text-sm">
              Map Font Width
            </FieldLabel>
            <FieldContent>
              <Controller
                name="map_font_width"
                control={control}
                render={({ field }) => (
                  <Input
                    type="number"
                    {...field}
                    onChange={(e) => {
                      const val = parseInt(e.target.value);
                      field.onChange(isNaN(val) ? 0 : val);
                    }}
                  />
                )}
              />
            </FieldContent>
          </Field>
          <Field>
            <FieldLabel className="text-sm">
              Map Font Height
            </FieldLabel>
            <FieldContent>
              <Controller
                name="map_font_height"
                control={control}
                render={({ field }) => (
                  <Input
                    type="number"
                    {...field}
                    onChange={(e) => {
                      const val = parseInt(e.target.value);
                      field.onChange(isNaN(val) ? 0 : val);
                    }}
                  />
                )}
              />
            </FieldContent>
          </Field>
        </div>
      </div>

      <div>
        <h3 className="text-sm font-medium mb-3">
          Overmap Font Settings
        </h3>
        <div className="grid grid-cols-3 gap-4">
          <Field>
            <FieldLabel className="text-sm">
              Overmap Font Size
            </FieldLabel>
            <FieldContent>
              <Controller
                name="overmap_font_size"
                control={control}
                render={({ field }) => (
                  <Input
                    type="number"
                    {...field}
                    onChange={(e) => {
                      const val = parseInt(e.target.value);
                      field.onChange(isNaN(val) ? 0 : val);
                    }}
                  />
                )}
              />
            </FieldContent>
          </Field>
          <Field>
            <FieldLabel className="text-sm">
              Overmap Font Width
            </FieldLabel>
            <FieldContent>
              <Controller
                name="overmap_font_width"
                control={control}
                render={({ field }) => (
                  <Input
                    type="number"
                    {...field}
                    onChange={(e) => {
                      const val = parseInt(e.target.value);
                      field.onChange(isNaN(val) ? 0 : val);
                    }}
                  />
                )}
              />
            </FieldContent>
          </Field>
          <Field>
            <FieldLabel className="text-sm">
              Overmap Font Height
            </FieldLabel>
            <FieldContent>
              <Controller
                name="overmap_font_height"
                control={control}
                render={({ field }) => (
                  <Input
                    type="number"
                    {...field}
                    onChange={(e) => {
                      const val = parseInt(e.target.value);
                      field.onChange(isNaN(val) ? 0 : val);
                    }}
                  />
                )}
              />
            </FieldContent>
          </Field>
        </div>
      </div>
    </div>
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
    <div className="space-y-6">
      <FontSelector
        control={control}
        fonts={fonts}
        isLoading={isLoading}
      />

      {selectedFont && <FontPreview selectedFont={selectedFont} />}

      <Accordion
        type="single"
        collapsible
        className="w-full"
        defaultValue="font-size"
      >
        <AccordionItem value="font-size" className="border-none">
          <AccordionTrigger className="py-2 hover:no-underline">
            <span className="text-sm font-medium">
              Font size options
            </span>
          </AccordionTrigger>
          <AccordionContent className="pt-4 pb-2">
            <FontSizeSettings control={control} />
          </AccordionContent>
        </AccordionItem>
      </Accordion>
    </div>
  );
}
