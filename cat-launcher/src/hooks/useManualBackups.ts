import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { GameVariant } from "@/generated-types/GameVariant";
import { listManualBackupsForVariant } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useManualBackups(
  variant: GameVariant,
  onError?: (error: Error) => void,
) {
  const onErrorRef = useRef(onError);

  useEffect(() => {
    onErrorRef.current = onError;
  }, [onError]);

  const {
    data: manualBackups,
    isLoading,
    error,
  } = useQuery({
    queryKey: queryKeys.manualBackups(variant),
    queryFn: () => listManualBackupsForVariant(variant),
  });

  useEffect(() => {
    if (error && onErrorRef.current) {
      onErrorRef.current(error);
    }
  }, [error]);

  return { manualBackups: manualBackups ?? [], isLoading };
}
