import { useEffect, useState } from "react";
import {
  useMutation,
  useQuery,
  useQueryClient,
} from "@tanstack/react-query";

import type { Theme } from "@/generated-types/Theme";
import { getPreferredTheme, setPreferredTheme } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useTheme(onError?: (error: unknown) => void) {
  const queryClient = useQueryClient();
  const [currentTheme, setCurrentTheme] = useState<Theme>("Light");

  const { data: themePreference, error: fetchError } = useQuery({
    queryKey: queryKeys.themePreference(),
    queryFn: getPreferredTheme,
  });

  useEffect(() => {
    if (themePreference) {
      setCurrentTheme(themePreference.theme);
    }
  }, [themePreference]);

  useEffect(() => {
    if (fetchError && onError) {
      onError(fetchError);
    }
  }, [fetchError, onError]);

  useEffect(() => {
    const root = document.documentElement;

    if (currentTheme === "Dark") {
      root.classList.add("dark");
    } else {
      root.classList.remove("dark");
    }

    root.style.colorScheme =
      currentTheme === "Dark" ? "dark" : "light";
  }, [currentTheme]);

  const { mutate: toggleTheme, isPending: isUpdating } = useMutation({
    mutationFn: async (newTheme: Theme) => {
      await setPreferredTheme(newTheme);
      return newTheme;
    },
    onMutate: () => {
      const nextTheme = currentTheme === "Dark" ? "Light" : "Dark";
      setCurrentTheme(nextTheme);

      queryClient.setQueryData(queryKeys.themePreference(), {
        theme: nextTheme,
      });

      return nextTheme;
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
