import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import type { GameEvent } from "@/generated-types/GameEvent";
import type { GameVariant } from "@/generated-types/GameVariant";
import {
  getPlayTimeForVariant,
  getPlayTimeForVersion,
  listenToGameEvent,
} from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { setupEventListener } from "@/lib/utils";

export function usePlayTime(
  variant: GameVariant,
  releaseId?: string,
  onTotalPlayTimeError?: (error: unknown) => void,
  onVersionPlayTimeError?: (error: unknown) => void,
) {
  const queryClient = useQueryClient();
  const onTotalPlayTimeErrorRef = useRef(onTotalPlayTimeError);
  const onVersionPlayTimeErrorRef = useRef(onVersionPlayTimeError);

  useEffect(() => {
    onTotalPlayTimeErrorRef.current = onTotalPlayTimeError;
  }, [onTotalPlayTimeError]);

  useEffect(() => {
    onVersionPlayTimeErrorRef.current = onVersionPlayTimeError;
  }, [onVersionPlayTimeError]);

  const { data: totalPlayTime, error: totalPlayTimeError } = useQuery(
    {
      queryKey: queryKeys.playTimeForVariant(variant),
      queryFn: () => getPlayTimeForVariant(variant),
      initialData: 0,
    },
  );

  const { data: versionPlayTime, error: versionPlayTimeError } =
    useQuery({
      queryKey: queryKeys.playTimeForVersion(variant, releaseId),
      queryFn: () => {
        if (!releaseId) {
          return Promise.resolve(0);
        }
        return getPlayTimeForVersion(variant, releaseId);
      },
      enabled: !!releaseId,
      initialData: 0,
    });

  useEffect(() => {
    if (totalPlayTimeError && onTotalPlayTimeErrorRef.current) {
      onTotalPlayTimeErrorRef.current(totalPlayTimeError);
    }
  }, [totalPlayTimeError]);

  useEffect(() => {
    if (versionPlayTimeError && onVersionPlayTimeErrorRef.current) {
      onVersionPlayTimeErrorRef.current(versionPlayTimeError);
    }
  }, [versionPlayTimeError]);

  useEffect(() => {
    const gameEventHandler = (event: GameEvent) => {
      if (event.type === "Exit") {
        queryClient.invalidateQueries({
          queryKey: queryKeys.playTimeForVariant(variant),
        });
        if (releaseId) {
          queryClient.invalidateQueries({
            queryKey: queryKeys.playTimeForVersion(
              variant,
              releaseId,
            ),
          });
        }
      }
    };

    const cleanup = setupEventListener(
      listenToGameEvent,
      gameEventHandler,
      "Error listening to game events in PlayTime.",
    );

    return cleanup;
  }, [queryClient, variant, releaseId]);

  return { totalPlayTime, versionPlayTime };
}
