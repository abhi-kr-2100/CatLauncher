import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { getFonts } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

/**
 * A custom hook that fetches available fonts for the application.
 *
 * @param onFontsError - Optional callback fired if there's an error fetching fonts.
 * @returns An object containing the list of fonts and loading status.
 */
export function useFonts(onFontsError?: (error: Error) => void) {
  const onFontsErrorRef = useRef(onFontsError);

  useEffect(() => {
    onFontsErrorRef.current = onFontsError;
  }, [onFontsError]);

  const {
    data: fonts = [],
    isLoading,
    error,
  } = useQuery({
    queryKey: queryKeys.fonts(),
    queryFn: getFonts,
  });

  useEffect(() => {
    if (error && onFontsErrorRef.current) {
      onFontsErrorRef.current(error);
    }
  }, [error]);

  return { fonts, isLoading };
}
