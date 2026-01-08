import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useEffect } from "react";

import type { GameEvent } from "@/generated-types/GameEvent";
import type { GameVariant } from "@/generated-types/GameVariant";
import {
  getPlayTimeForVariant,
  getPlayTimeForVersion,
  listenToGameEvent,
} from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { setupEventListener, toastCL } from "@/lib/utils";

export function usePlayTime(
  variant: GameVariant,
  releaseId?: string,
) {
  const queryClient = useQueryClient();
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
    if (totalPlayTimeError) {
      toastCL(
        "error",
        `Failed to get total play time for ${variant}.`,
        totalPlayTimeError,
      );
    }
  }, [totalPlayTimeError, variant]);

  useEffect(() => {
    if (versionPlayTimeError) {
      toastCL(
        "error",
        `Failed to get version play time for ${variant}.`,
        versionPlayTimeError,
      );
    }
  }, [versionPlayTimeError, variant]);

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
