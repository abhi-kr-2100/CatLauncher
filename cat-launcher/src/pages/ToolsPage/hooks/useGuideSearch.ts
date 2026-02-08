import { useEffect, useRef } from "react";
import { useQuery } from "@tanstack/react-query";

import { searchGuide } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import type { GameVariant } from "@/generated-types/GameVariant";

export default function useGuideSearch(
  variant: GameVariant,
  query: string,
  onSearchError?: (error: Error) => void,
) {
  const onSearchErrorRef = useRef(onSearchError);

  useEffect(() => {
    onSearchErrorRef.current = onSearchError;
  }, [onSearchError]);

  const { data, isLoading, error } = useQuery({
    queryKey: queryKeys.guide.search(variant, query),
    queryFn: () => searchGuide(query, variant),
    enabled: query.length >= 2,
  });

  useEffect(() => {
    if (error && onSearchErrorRef.current) {
      onSearchErrorRef.current(
        error instanceof Error ? error : new Error(String(error)),
      );
    }
  }, [error]);

  return { data, isLoading, error };
}
