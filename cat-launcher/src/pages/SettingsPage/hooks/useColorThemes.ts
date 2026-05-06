import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { getColorThemes } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

/**
 * A custom hook that fetches available color themes for the application.
 *
 * @param onThemesError - Optional callback fired if there's an error fetching color themes.
 * @returns An object containing the list of color themes and loading status.
 */
export function useColorThemes(
  onThemesError?: (error: Error) => void,
) {
  const onThemesErrorRef = useRef(onThemesError);

  useEffect(() => {
    onThemesErrorRef.current = onThemesError;
  }, [onThemesError]);

  const {
    data: themes = [],
    isLoading,
    error,
  } = useQuery({
    queryKey: queryKeys.colorThemes(),
    queryFn: getColorThemes,
  });

  useEffect(() => {
    if (error && onThemesErrorRef.current) {
      onThemesErrorRef.current(error);
    }
  }, [error]);

  return { themes, isLoading };
}
