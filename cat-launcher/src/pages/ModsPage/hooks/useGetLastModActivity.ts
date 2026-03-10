import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import type { GameVariant } from "@/generated-types/GameVariant";
import { getLastModActivity } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useGetLastModActivity(
  enabled: boolean,
  modId: string,
  variant: GameVariant,
  onActivityError?: (error: Error) => void,
) {
  const onActivityErrorRef = useRef(onActivityError);

  useEffect(() => {
    onActivityErrorRef.current = onActivityError;
  }, [onActivityError]);

  const { data, isLoading, error } = useQuery({
    queryKey: queryKeys.mods.lastActivity(variant, modId),
    queryFn: () => getLastModActivity(modId, variant),
    enabled,
  });

  useEffect(() => {
    if (error && onActivityErrorRef.current) {
      onActivityErrorRef.current(error as Error);
    }
  }, [error]);

  return {
    lastActivity: data,
    isLoading,
    error,
  };
}
