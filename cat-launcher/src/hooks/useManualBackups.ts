import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { GameVariant } from "@/generated-types/GameVariant";
import { listManualBackupsForVariant } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useManualBackups(
  variant: GameVariant,
  onManualBackupsError?: (error: Error) => void,
) {
  const onManualBackupsErrorRef = useRef(onManualBackupsError);

  useEffect(() => {
    onManualBackupsErrorRef.current = onManualBackupsError;
  }, [onManualBackupsError]);

  const {
    data: manualBackups,
    isLoading,
    error,
  } = useQuery({
    queryKey: queryKeys.manualBackups(variant),
    queryFn: () => listManualBackupsForVariant(variant),
  });

  useEffect(() => {
    if (error && onManualBackupsErrorRef.current) {
      onManualBackupsErrorRef.current(error as Error);
    }
  }, [error]);

  return { manualBackups: manualBackups ?? [], isLoading, error };
}
