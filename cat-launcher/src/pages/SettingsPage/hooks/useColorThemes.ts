import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { getColorThemes } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

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
