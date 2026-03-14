import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { queryKeys } from "@/lib/queryKeys";
import { getTips } from "@/lib/commands";
import type { GameVariant } from "@/generated-types/GameVariant";

export function useGetTips(
  variant: GameVariant,
  onTipsError?: (error: Error) => void,
) {
  const onTipsErrorRef = useRef(onTipsError);

  useEffect(() => {
    onTipsErrorRef.current = onTipsError;
  }, [onTipsError]);

  const { data, isLoading, error } = useQuery({
    queryKey: queryKeys.tips(variant),
    queryFn: async () => getTips(variant),
  });

  useEffect(() => {
    if (error && onTipsErrorRef.current) {
      onTipsErrorRef.current(error as Error);
    }
  }, [error]);

  return { tips: data, isLoading, error };
}
