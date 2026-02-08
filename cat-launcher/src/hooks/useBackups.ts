import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { GameVariant } from "@/generated-types/GameVariant";
import { listBackupsForVariant } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useBackups(
  variant: GameVariant,
  onError?: (error: Error) => void,
) {
  const onErrorRef = useRef(onError);

  useEffect(() => {
    onErrorRef.current = onError;
  }, [onError]);

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
    if (error && onErrorRef.current) {
      onErrorRef.current(error);
    }
  }, [error]);

  return {
    backups,
    isLoading,
    isError,
    error,
  };
}
