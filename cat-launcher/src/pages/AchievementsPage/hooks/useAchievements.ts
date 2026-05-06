import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import type { GameVariant } from "@/generated-types/GameVariant";
import { getAchievementsForVariant } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

/**
 * A custom hook to fetch achievements for a specific game variant.
 *
 * @param variant - The game variant to fetch achievements for, or null if none selected.
 * @param onAchievementsError - An optional callback function to handle errors during fetching.
 * @returns The query result from react-query.
 */
export function useAchievements(
  variant: GameVariant | null,
  onAchievementsError?: (error: Error) => void,
) {
  const onAchievementsErrorRef = useRef(onAchievementsError);

  useEffect(() => {
    onAchievementsErrorRef.current = onAchievementsError;
  }, [onAchievementsError]);

  const query = useQuery({
    queryKey: variant
      ? queryKeys.achievements(variant)
      : ["achievements", null],
    queryFn: () =>
      variant
        ? getAchievementsForVariant(variant)
        : Promise.resolve([]),
    enabled: !!variant,
  });

  useEffect(() => {
    if (query.error && onAchievementsErrorRef.current) {
      onAchievementsErrorRef.current(query.error as Error);
    }
  }, [query.error]);

  return query;
}
