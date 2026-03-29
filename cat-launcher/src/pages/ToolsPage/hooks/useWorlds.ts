import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import type { GameVariant } from "@/generated-types/GameVariant";
import { listWorlds } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useWorlds(
  variant: GameVariant,
  onWorldsLoadError?: (error: Error) => void,
) {
  const onWorldsLoadErrorRef = useRef(onWorldsLoadError);

  useEffect(() => {
    onWorldsLoadErrorRef.current = onWorldsLoadError;
  }, [onWorldsLoadError]);

  const { data, isLoading, error } = useQuery({
    queryKey: queryKeys.worlds(variant),
    queryFn: () => listWorlds(variant),
  });

  useEffect(() => {
    if (error && onWorldsLoadErrorRef.current) {
      onWorldsLoadErrorRef.current(error as Error);
    }
  }, [error]);

  return { data, isLoading, error };
}
