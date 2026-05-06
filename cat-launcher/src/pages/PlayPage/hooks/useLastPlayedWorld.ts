import { useQuery } from "@tanstack/react-query";
import { useEffect } from "react";

import type { GameVariant } from "@/generated-types/GameVariant";
import { getLastPlayedWorld } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

/**
 * Custom hook to fetch and poll for the name of the last played world for a specific game variant.
 *
 * @param variant - The game variant.
 * @param options - Hook options.
 * @param options.onError - Callback triggered if the world name fails to load.
 * @returns An object containing the name of the last played world.
 */
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
