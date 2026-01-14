import { useQueries } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import type { GameVariantInfo } from "@/generated-types/GameVariantInfo";
import { getPlayTimeForVariant } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { toastCL } from "@/lib/utils";

interface PlayTimeDataPoint {
  name: string;
  playTime: number;
}

export function usePlayTimeData(
  variants: GameVariantInfo[],
  onError?: (error: unknown) => void,
) {
  const onErrorRef = useRef(onError);

  useEffect(() => {
    onErrorRef.current = onError;
  }, [onError]);

  const queries = useQueries({
    queries: variants.map((variant) => ({
      queryKey: queryKeys.playTimeForVariant(variant.id),
      queryFn: () => getPlayTimeForVariant(variant.id),
      initialData: 0,
    })),
  });

  // Track previously reported errors by index to avoid duplicate toasts
  const reportedErrorIndicesRef = useRef<Set<number>>(new Set());

  // Check for errors
  useEffect(() => {
    queries.forEach((query, index) => {
      if (query.error) {
        if (!reportedErrorIndicesRef.current.has(index)) {
          reportedErrorIndicesRef.current.add(index);
          toastCL(
            "error",
            "Failed to load play time data",
            query.error as Error,
          );
          if (onErrorRef.current) {
            onErrorRef.current(query.error);
          }
        }
      }
    });
  }, [variants.length]);

  const isLoading = queries.some((query) => query.isLoading);

  const playTimeData: PlayTimeDataPoint[] = variants
    .map((variant, index) => {
      const playTimeSeconds = queries[index]?.data ?? 0;
      const playTimeHours =
        Math.round((playTimeSeconds / 3600) * 10) / 10;
      return {
        name: variant.name,
        playTime: playTimeHours,
      };
    })
    .filter((item) => item.playTime > 0)
    .sort((a, b) => b.playTime - a.playTime);

  return { playTimeData, isLoading };
}
