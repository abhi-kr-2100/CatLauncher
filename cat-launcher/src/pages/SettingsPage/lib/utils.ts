import type { ThemeColors } from "@/generated-types/ThemeColors";

export function isThemeEqual(
  themeColors: ThemeColors,
  settingsColors: ThemeColors,
): boolean {
  const themeKeys = Object.keys(themeColors);
  const settingsKeys = Object.keys(settingsColors);

  // Check if both objects have the same number of keys
  if (themeKeys.length !== settingsKeys.length) {
    return false;
  }

  // Check if every key in themeColors exists in settingsColors and has equal values
  return themeKeys.every((key) => {
    const tc = themeColors[key as keyof ThemeColors];
    const sc = settingsColors[key as keyof ThemeColors];
    return (
      sc !== undefined &&
      tc[0] === sc[0] &&
      tc[1] === sc[1] &&
      tc[2] === sc[2]
    );
  });
}
