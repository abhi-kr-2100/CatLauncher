import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { GameVariant } from "@/generated-types/GameVariant";
import { listBackupsForVariant } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useBackups(
  variant: GameVariant,
  options?: {
    onBackupsLoadError?: (error: Error) => void;
  },
) {
  const onBackupsLoadErrorRef = useRef(options?.onBackupsLoadError);
  useEffect(() => {
    onBackupsLoadErrorRef.current = options?.onBackupsLoadError;
  }, [options?.onBackupsLoadError]);
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
    if (error && onBackupsLoadErrorRef.current) {
      onBackupsLoadErrorRef.current(error);
    }
  }, [error]);

  return {
    backups,
    isLoading,
    isError,
    error,
  };
}
