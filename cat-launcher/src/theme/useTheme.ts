import { useEffect, useMemo } from "react";
import {
  useMutation,
  useQuery,
  useQueryClient,
} from "@tanstack/react-query";

import type { Theme } from "@/generated-types/Theme";
import { getPreferredTheme, setPreferredTheme } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

/**
 * The key used for storing the theme preference in local storage.
 */
export const THEME_STORAGE_KEY = "cat-launcher-theme";

/**
 * Retrieves the theme currently stored in local storage.
 *
 * @returns The stored theme, or null if none is set or if it's invalid.
 */
export function getStoredTheme(): Theme | null {
  const stored = localStorage.getItem(THEME_STORAGE_KEY);
  return stored === "Dark" || stored === "Light" ? stored : null;
}

/**
 * Persists the provided theme to local storage.
 *
 * @param theme - The theme to store.
 */
export function setStoredTheme(theme: Theme): void {
  localStorage.setItem(THEME_STORAGE_KEY, theme);
}

/**
 * Applies the specified theme to the DOM by toggling classes and setting CSS styles.
 *
 * @param theme - The theme to apply.
 */
export function applyThemeToDom(theme: Theme): void {
  const root = document.documentElement;
  root.classList.toggle("dark", theme !== "Light");
  root.style.colorScheme = theme === "Light" ? "light" : "dark";
}

/**
 * A custom hook that manages the application's color theme.
 * It handles fetching from the backend, persisting to local storage, and applying it to the DOM.
 *
 * @param onError - Optional callback for handling errors during theme operations.
 * @returns An object containing the current theme, a toggle function, and the update state.
 */
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
    () => themePreference?.theme ?? "Dark",
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
    const nextTheme = currentTheme === "Light" ? "Dark" : "Light";
    toggleTheme(nextTheme);
  };

  return { currentTheme, toggleTheme: handleToggle, isUpdating };
}
