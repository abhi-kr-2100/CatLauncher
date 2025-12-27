import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";

import { queryKeys } from "@/lib/queryKeys";
import { listThemes } from "@/lib/commands";

export function useThemes(onError?: (err: Error) => void) {
  const {
    data: themes,
    isLoading,
    isError,
    error,
  } = useQuery({
    queryKey: queryKeys.themes(),
    queryFn: async () => {
      return await listThemes();
    },
  });

  useEffect(() => {
    if (isError && onError) {
      onError(error as Error);
    }
  }, [isError, error, onError]);

  return { themes: themes ?? [], isLoading };
}
