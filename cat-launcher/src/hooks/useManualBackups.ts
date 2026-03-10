import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { GameVariant } from "@/generated-types/GameVariant";
import { listManualBackupsForVariant } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useManualBackups(
  variant: GameVariant,
  onFetchError?: (error: Error) => void,
) {
  const onFetchErrorRef = useRef(onFetchError);

  useEffect(() => {
    onFetchErrorRef.current = onFetchError;
  }, [onFetchError]);

  const {
    data: manualBackups,
    isLoading,
    error,
  } = useQuery({
    queryKey: queryKeys.manualBackups(variant),
    queryFn: () => listManualBackupsForVariant(variant),
  });

  useEffect(() => {
    if (error && onFetchErrorRef.current) {
      onFetchErrorRef.current(error as Error);
    }
  }, [error]);

  return { manualBackups: manualBackups ?? [], isLoading, error };
}
