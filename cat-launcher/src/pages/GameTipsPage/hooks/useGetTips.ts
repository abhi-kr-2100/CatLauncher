import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { getTips } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import type { GameVariant } from "@/generated-types/GameVariant";

export function useGetTips(
  variant: GameVariant,
  onGetTipsError?: (error: Error) => void,
) {
  const onGetTipsErrorRef = useRef(onGetTipsError);

  useEffect(() => {
    onGetTipsErrorRef.current = onGetTipsError;
  }, [onGetTipsError]);

  const { data, isLoading, error } = useQuery({
    queryKey: queryKeys.tips(variant),
    queryFn: async () => getTips(variant),
  });

  useEffect(() => {
    if (error && onGetTipsErrorRef.current) {
      onGetTipsErrorRef.current(error);
    }
  }, [error]);

  return { data, isLoading, error };
}
