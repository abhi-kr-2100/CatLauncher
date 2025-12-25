import type { ThemeColors } from "@/generated-types/ThemeColors";

export function isThemeEqual(
  themeColors: ThemeColors,
  settingsColors: ThemeColors,
): boolean {
  const keys = Object.keys(themeColors) as Array<keyof ThemeColors>;
  return keys.every((key) => {
    const tc = themeColors[key];
    const sc = settingsColors[key];
    return tc[0] === sc[0] && tc[1] === sc[1] && tc[2] === sc[2];
  });
}
