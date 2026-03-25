import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { queryKeys } from "@/lib/queryKeys";
import { getTips } from "@/lib/commands";
import type { GameVariant } from "@/generated-types/GameVariant";

export function useGetTips(
  variant: GameVariant,
  onLoadError?: (error: Error) => void,
) {
  const onLoadErrorRef = useRef(onLoadError);

  useEffect(() => {
    onLoadErrorRef.current = onLoadError;
  }, [onLoadError]);

  const query = useQuery({
    queryKey: queryKeys.tips(variant),
    queryFn: async () => getTips(variant),
  });

  useEffect(() => {
    if (query.error && onLoadErrorRef.current) {
      onLoadErrorRef.current(query.error as Error);
    }
  }, [query.error]);

  return query;
}
