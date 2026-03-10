import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { GameVariant } from "@/generated-types/GameVariant";
import { listBackupsForVariant } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useBackups(
  variant: GameVariant,
  onFetchError?: (error: Error) => void,
) {
  const onFetchErrorRef = useRef(onFetchError);

  useEffect(() => {
    onFetchErrorRef.current = onFetchError;
  }, [onFetchError]);

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
    if (error && onFetchErrorRef.current) {
      onFetchErrorRef.current(error as Error);
    }
  }, [error]);

  return {
    backups,
    isLoading,
    isError,
    error,
  };
}
