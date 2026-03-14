import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { GameVariant } from "@/generated-types/GameVariant";
import { listBackupsForVariant } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useBackups(
  variant: GameVariant,
  onBackupsError?: (error: Error) => void,
) {
  const onBackupsErrorRef = useRef(onBackupsError);

  useEffect(() => {
    onBackupsErrorRef.current = onBackupsError;
  }, [onBackupsError]);

  const {
    data: backups = [],
    isLoading,
    isError,
    error,
  } = useQuery({
    queryKey: queryKeys.backups(variant),
    queryFn: () => listBackupsForVariant(variant),
  });

  useEffect(() => {
    if (error && onBackupsErrorRef.current) {
      onBackupsErrorRef.current(error as Error);
    }
  }, [error]);

  return {
    backups,
    isLoading,
    isError,
    error,
  };
}
