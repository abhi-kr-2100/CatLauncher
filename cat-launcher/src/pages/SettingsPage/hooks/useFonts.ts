import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";

import { queryKeys } from "@/lib/queryKeys";
import { listFonts } from "@/lib/commands";

export function useFonts(onError?: (err: Error) => void) {
  const {
    data: fonts,
    isLoading,
    isError,
    error,
  } = useQuery({
    queryKey: queryKeys.fonts(),
    queryFn: async () => {
      return await listFonts();
    },
  });

  useEffect(() => {
    if (isError && onError) {
      onError(error as Error);
    }
  }, [isError, error, onError]);

  return { fonts: fonts ?? [], isLoading };
}
