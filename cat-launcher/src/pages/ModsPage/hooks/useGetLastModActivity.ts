import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import type { GameVariant } from "@/generated-types/GameVariant";
import { getLastModActivity } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useGetLastModActivity(
  enabled: boolean,
  modId: string,
  variant: GameVariant,
  onError?: (error: unknown) => void,
) {
  const onErrorRef = useRef(onError);

  useEffect(() => {
    onErrorRef.current = onError;
  }, [onError]);

  const query = useQuery({
    queryKey: queryKeys.mods.lastActivity(variant, modId),
    queryFn: () => getLastModActivity(modId, variant),
    enabled,
  });

  useEffect(() => {
    if (query.error && onErrorRef.current) {
      onErrorRef.current(query.error);
    }
  }, [query.error]);

  return {
    lastActivity: query.data,
    isLoading: query.isLoading,
  };
}
