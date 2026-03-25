import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { GameVariant } from "@/generated-types/GameVariant";
import { listManualBackupsForVariant } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useManualBackups(
  variant: GameVariant,
  onLoadError?: (error: Error) => void,
) {
  const onLoadErrorRef = useRef(onLoadError);

  useEffect(() => {
    onLoadErrorRef.current = onLoadError;
  }, [onLoadError]);

  const query = useQuery({
    queryKey: queryKeys.manualBackups(variant),
    queryFn: () => listManualBackupsForVariant(variant),
  });

  useEffect(() => {
    if (query.error && onLoadErrorRef.current) {
      onLoadErrorRef.current(query.error as Error);
    }
  }, [query.error]);

  return {
    ...query,
    manualBackups: query.data ?? [],
  };
}
