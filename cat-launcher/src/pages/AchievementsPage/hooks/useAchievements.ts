import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import type { GameVariant } from "@/generated-types/GameVariant";
import { getAchievementsForVariant } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

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
