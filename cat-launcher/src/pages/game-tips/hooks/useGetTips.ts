import { useQuery } from "@tanstack/react-query";

import { queryKeys } from "@/lib/queryKeys";
import { getTips } from "@/lib/commands";
import type { GameVariant } from "@/generated-types/GameVariant";

/**
 * Custom hook to fetch game tips for a specific game variant.
 * Uses TanStack Query for data fetching and caching.
 *
 * @param variant - The game variant to fetch tips for.
 * @returns A query object containing the status and data of the tips.
 */
export function useGetTips(variant: GameVariant) {
  return useQuery({
    queryKey: queryKeys.tips(variant),
    queryFn: async () => getTips(variant),
  });
}
