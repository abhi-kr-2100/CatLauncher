import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import type { GameVariant } from "@/generated-types/GameVariant";
import { getLastModActivity } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

/**
 * A custom hook to fetch the last recorded activity for a specific mod.
 * Useful for determining mod stability and update recency.
 *
 * @param enabled - Whether the query should be active.
 * @param modId - The unique identifier of the mod.
 * @param variant - The game variant associated with the mod.
 * @param onError - An optional callback function to handle errors.
 * @returns An object containing the last activity data and loading state.
 */
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
