import { useEffect, useMemo } from "react";
import {
  useMutation,
  useQuery,
  useQueryClient,
} from "@tanstack/react-query";

import type { Theme } from "@/generated-types/Theme";
import { getPreferredTheme, setPreferredTheme } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export const THEME_STORAGE_KEY = "cat-launcher-theme";

export function getStoredTheme(): Theme | null {
  const stored = localStorage.getItem(THEME_STORAGE_KEY);
  return stored === "Dark" || stored === "Light" ? stored : null;
}

export function setStoredTheme(theme: Theme): void {
  localStorage.setItem(THEME_STORAGE_KEY, theme);
}

export function applyThemeToDom(theme: Theme): void {
  const root = document.documentElement;
  root.classList.toggle("dark", theme === "Dark");
  root.style.colorScheme = theme === "Dark" ? "dark" : "light";
}

export function useTheme(onError?: (error: unknown) => void) {
  const queryClient = useQueryClient();

  const { data: themePreference, error: fetchError } = useQuery({
    queryKey: queryKeys.themePreference(),
    queryFn: getPreferredTheme,
    initialData: () => {
      const theme = getStoredTheme();
      return theme
        ? {
            theme,
          }
        : undefined;
    },
    staleTime: Infinity,
  });

  const currentTheme = useMemo(
    () => themePreference?.theme ?? "Light",
    [themePreference],
  );

  useEffect(() => {
    if (fetchError && onError) {
      onError(fetchError);
    }
  }, [fetchError, onError]);

  useEffect(() => {
    applyThemeToDom(currentTheme);
    setStoredTheme(currentTheme);
  }, [currentTheme]);

  const { mutate: toggleTheme, isPending: isUpdating } = useMutation({
    mutationFn: async (newTheme: Theme) => {
      await setPreferredTheme(newTheme);
      return newTheme;
    },
    onMutate: (newTheme) => {
      queryClient.setQueryData(queryKeys.themePreference(), {
        theme: newTheme,
      });
      return { newTheme };
    },
    onError: (error) => {
      // Don't revert the theme change even if the update fails.
      // Just call the error handler if provided

      if (onError) {
        onError(error);
      }
    },
  });

  const handleToggle = () => {
    const nextTheme = currentTheme === "Dark" ? "Light" : "Dark";
    toggleTheme(nextTheme);
  };

  return { currentTheme, toggleTheme: handleToggle, isUpdating };
}
