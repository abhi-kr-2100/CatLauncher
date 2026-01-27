import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { GameVariant } from "@/generated-types/GameVariant";
import { listBackupsForVariant } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useBackups(
  variant: GameVariant,
  onBackupLoadError?: (error: Error) => void,
) {
  const onBackupLoadErrorRef = useRef(onBackupLoadError);

  useEffect(() => {
    onBackupLoadErrorRef.current = onBackupLoadError;
  }, [onBackupLoadError]);

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
    if (error && onBackupLoadErrorRef.current) {
      onBackupLoadErrorRef.current(error);
    }
  }, [error]);

  return {
    backups,
    isLoading,
    isError,
    error,
  };
}
