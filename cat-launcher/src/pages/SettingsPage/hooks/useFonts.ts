import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { getFonts } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

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
