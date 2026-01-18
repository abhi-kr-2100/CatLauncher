import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { GameVariant } from "@/generated-types/GameVariant";
import { listManualBackupsForVariant } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useManualBackups(
  variant: GameVariant,
  onManualBackupsLoadError?: (error: Error) => void,
) {
  const onManualBackupsLoadErrorRef = useRef(
    onManualBackupsLoadError,
  );

  useEffect(() => {
    onManualBackupsLoadErrorRef.current = onManualBackupsLoadError;
  }, [onManualBackupsLoadError]);

  const {
    data: manualBackups,
    isLoading,
    error,
  } = useQuery({
    queryKey: queryKeys.manualBackups(variant),
    queryFn: () => listManualBackupsForVariant(variant),
  });

  useEffect(() => {
    if (error && onManualBackupsLoadErrorRef.current) {
      onManualBackupsLoadErrorRef.current(error);
    }
  }, [error]);

  return { manualBackups: manualBackups ?? [], isLoading };
}
