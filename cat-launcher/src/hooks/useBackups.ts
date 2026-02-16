import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { GameVariant } from "@/generated-types/GameVariant";
import { listBackupsForVariant } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useBackups(
  variant: GameVariant,
  onBackupsLoadError?: (error: unknown) => void,
) {
  const onBackupsLoadErrorRef = useRef(onBackupsLoadError);

  useEffect(() => {
    onBackupsLoadErrorRef.current = onBackupsLoadError;
  }, [onBackupsLoadError]);

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
    if (isError && onBackupsLoadErrorRef.current) {
      onBackupsLoadErrorRef.current(error);
    }
  }, [isError, error]);

  return {
    backups,
    isLoading,
    isError,
    error,
  };
}
