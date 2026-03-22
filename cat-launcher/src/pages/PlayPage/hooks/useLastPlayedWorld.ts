import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

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
  const onErrorRef = useRef(onError);

  useEffect(() => {
    onErrorRef.current = onError;
  }, [onError]);

  const { data: lastPlayedWorld, error: lastPlayedWorldError } =
    useQuery<string | null>({
      queryKey: queryKeys.lastPlayedWorld(variant),
      queryFn: () => getLastPlayedWorld(variant),
      refetchInterval: 5000,
    });

  useEffect(() => {
    if (lastPlayedWorldError && onErrorRef.current) {
      onErrorRef.current(lastPlayedWorldError as Error);
    }
  }, [lastPlayedWorldError]);

  return { lastPlayedWorld };
}
