import { useQuery } from "@tanstack/react-query";
import { useEffect } from "react";

import type { GameVariant } from "@/generated-types/GameVariant";
import { getLastPlayedWorld } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useLastPlayedWorld(
  variant: GameVariant,
  {
    onError,
  }: {
    onError: (error: Error) => void;
  },
) {
  const { data: lastPlayedWorld, error: lastPlayedWorldError } =
    useQuery<string | null>({
      queryKey: queryKeys.lastPlayedWorld(variant),
      queryFn: () => getLastPlayedWorld(variant),
      refetchInterval: 5000,
    });

  useEffect(() => {
    if (lastPlayedWorldError) {
      onError(lastPlayedWorldError as Error);
    }
  }, [lastPlayedWorldError, onError]);

  return { lastPlayedWorld };
}
