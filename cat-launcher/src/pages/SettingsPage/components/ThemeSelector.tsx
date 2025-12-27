import { VirtualizedCombobox } from "@/components/virtualized-combobox";
import type { ThemeColors } from "@/generated-types/ThemeColors";
import {
  Field,
  FieldContent,
  FieldLabel,
} from "@/components/ui/field";
import { toastCL } from "@/lib/utils";
import { useThemes } from "../hooks/useThemes";

function ThemeColorsDisplay({ colors }: { colors: ThemeColors }) {
  return (
    <div>
      <p className="text-sm text-muted-foreground mb-3 font-medium">
        Theme Colors:
      </p>
      <div className="flex flex-wrap gap-3">
        {Object.entries(colors).map(([name, rgb]) => {
          if (!rgb) return null;
          return (
            <div
              key={name}
              className="flex items-center gap-2 min-w-20"
            >
              <div
                className="h-5 w-5 rounded border shadow-sm"
                style={{
                  backgroundColor: `rgb(${rgb[0]}, ${rgb[1]}, ${rgb[2]})`,
                }}
              />
              <span className="text-[10px] truncate max-w-[60px]">
                {name}
              </span>
            </div>
          );
        })}
      </div>
    </div>
  );
}

interface ThemeSelectorProps {
  selectedTheme: string | null;
  onThemeChange: (theme: string | null) => void;
}

export default function ThemeSelector({
  selectedTheme,
  onThemeChange,
}: ThemeSelectorProps) {
  const { themes, isLoading } = useThemes((err) =>
    toastCL("error", "Failed to resolve themes", err),
  );

  const selectedThemeObj = themes.find(
    (t) => t.name === selectedTheme,
  );

  return (
    <div className="flex flex-col gap-6 max-w-2xl">
      <Field>
        <FieldLabel htmlFor="theme-selector">Color Theme</FieldLabel>
        <FieldContent>
          <VirtualizedCombobox
            items={themes.map((theme) => ({
              value: theme.name,
              label: theme.name,
            }))}
            value={selectedTheme ?? "Unknown"}
            onChange={(value) => onThemeChange(value)}
            placeholder={
              isLoading ? "Loading themes..." : "Select a theme"
            }
            disabled={isLoading}
          />
        </FieldContent>
      </Field>

      {selectedThemeObj && (
        <ThemeColorsDisplay colors={selectedThemeObj.colors} />
      )}
    </div>
  );
}
