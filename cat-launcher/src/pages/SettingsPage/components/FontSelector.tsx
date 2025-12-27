import {
  Field,
  FieldContent,
  FieldLabel,
} from "@/components/ui/field";
import { VirtualizedCombobox } from "@/components/virtualized-combobox";
import { toastCL } from "@/lib/utils";
import { useFonts } from "../hooks/useFonts";

interface FontSelectorProps {
  selectedFont: string | null;
  onFontChange: (font: string | null) => void;
}

export default function FontSelector({
  selectedFont,
  onFontChange,
}: FontSelectorProps) {
  const { fonts, isLoading } = useFonts((err) =>
    toastCL("error", "Failed to resolve fonts", err),
  );

  return (
    <div className="max-w-md">
      <Field>
        <FieldLabel htmlFor="font-selector">Font</FieldLabel>
        <FieldContent>
          <VirtualizedCombobox
            items={fonts.map((font) => ({
              value: font.location,
              label: font.name,
            }))}
            value={selectedFont ?? "Unknown"}
            onChange={(value) => onFontChange(value)}
            placeholder={
              isLoading ? "Loading fonts..." : "Select a font"
            }
            className="w-full"
            disabled={isLoading}
          />
        </FieldContent>
      </Field>
    </div>
  );
}
