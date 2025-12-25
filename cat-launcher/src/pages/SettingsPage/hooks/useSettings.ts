import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";

import { queryKeys } from "@/lib/queryKeys";
import { getSettings, listFonts, listThemes } from "@/lib/commands";
import { isThemeEqual } from "../lib/utils";

export function useSettings(
  onSettingsError?: (err: Error) => void,
  onFontsError?: (err: Error) => void,
  onThemesError?: (err: Error) => void,
) {
  const {
    data: settings,
    isLoading: settingsLoading,
    isError: settingsError,
    error: settingsErrorObj,
  } = useQuery({
    queryKey: queryKeys.settings(),
    queryFn: async () => {
      return await getSettings();
    },
  });

  const {
    data: fonts,
    isLoading: fontsLoading,
    isError: fontsError,
    error: fontsErrorObj,
  } = useQuery({
    queryKey: queryKeys.fonts(),
    queryFn: async () => {
      return await listFonts();
    },
  });

  const {
    data: themes,
    isLoading: themesLoading,
    isError: themesError,
    error: themesErrorObj,
  } = useQuery({
    queryKey: queryKeys.themes(),
    queryFn: async () => {
      return await listThemes();
    },
  });

  useEffect(() => {
    if (settingsError && onSettingsError) {
      onSettingsError(settingsErrorObj as Error);
    }
  }, [settingsError, settingsErrorObj, onSettingsError]);

  useEffect(() => {
    if (fontsError && onFontsError) {
      onFontsError(fontsErrorObj as Error);
    }
  }, [fontsError, fontsErrorObj, onFontsError]);

  useEffect(() => {
    if (themesError && onThemesError) {
      onThemesError(themesErrorObj as Error);
    }
  }, [themesError, themesErrorObj, onThemesError]);

  const isLoading = settingsLoading || fontsLoading || themesLoading;

  const resolvedFonts = fonts ?? [];
  const resolvedThemes = themes ?? [];

  let themeName = "Custom";
  if (settings?.theme && resolvedThemes.length > 0) {
    const currentTheme = settings.theme;
    const foundTheme = resolvedThemes.find((t) => {
      return isThemeEqual(t.colors, currentTheme);
    });
    if (foundTheme) {
      themeName = foundTheme.name;
    }
  }

  const combinedSettings = settings
    ? {
        ...settings,
        themeName,
      }
    : undefined;

  return {
    settings: combinedSettings,
    fonts: resolvedFonts,
    themes: resolvedThemes,
    isLoading,
  };
}
