import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";

import type { GameVariant } from "@/generated-types/GameVariant";
import { getLastModActivity } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useGetLastModActivity(
  enabled: boolean,
  modId: string,
  variant: GameVariant,
  onError?: (error: unknown) => void,
) {
  const query = useQuery({
    queryKey: queryKeys.mods.lastActivity(variant, modId),
    queryFn: () => getLastModActivity(modId, variant),
    enabled,
  });

  useEffect(() => {
    if (query.error) {
      onError?.(query.error);
    }
  }, [query.error, onError]);

  return {
    lastActivity: query.data,
    isLoading: query.isLoading,
  };
}
