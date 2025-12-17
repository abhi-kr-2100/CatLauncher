import {
  useMutation,
  useQuery,
  useQueryClient,
  useQueries,
} from "@tanstack/react-query";
import { useEffect, useMemo } from "react";

import type { GameEvent } from "@/generated-types/GameEvent";
import type { GameRelease } from "@/generated-types/GameRelease";
import type { GameReleaseStatus } from "@/generated-types/GameReleaseStatus";
import type { GameVariant } from "@/generated-types/GameVariant";
import type { ReleasesUpdatePayload } from "@/generated-types/ReleasesUpdatePayload";
import {
  getInstallationStatus,
  getPlayTimeForVariant,
  getPlayTimeForVersion,
  installReleaseForVariant,
  launchGame,
  listenToGameEvent,
  listenToReleasesUpdate,
  getLastPlayedWorld,
} from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { setupEventListener, toastCL } from "@/lib/utils";
import { useAppDispatch, useAppSelector } from "@/store/hooks";
import { setCurrentlyPlaying } from "@/store/gameSessionSlice";
import { updateReleasesForVariant } from "@/store/releasesSlice";
import { useInstallAndMonitor } from "@/hooks/useInstallAndMonitor";
import { playPageErrorMap } from "./lib/errors";

export function useReleaseEvents() {
  const dispatch = useAppDispatch();

  useEffect(() => {
    const releaseUpdateHandler = (payload: ReleasesUpdatePayload) => {
      dispatch(updateReleasesForVariant(payload));
    };

    const cleanup = setupEventListener(
      listenToReleasesUpdate,
      releaseUpdateHandler,
      "Failed to subscribe to releases.",
    );

    return cleanup;
  }, [dispatch]);
}

export function useAllReleasesInstallationStatuses(
  variant: GameVariant,
) {
  const releases = useAppSelector(
    (state) => state.releases.releasesByVariant[variant],
  );

  const results = useQueries({
    queries:
      releases?.map((r) => ({
        queryKey: queryKeys.installationStatus(variant, r.version),
        queryFn: () => getInstallationStatus(variant, r.version),
        staleTime: 5 * 60 * 1000, // 5 minutes
        initialData: "Unknown",
      })) ?? [],
  });

  const statuses = useMemo(() => {
    return results.reduce(
      (acc, result, index) => {
        if (releases && releases[index]) {
          acc[releases[index].version] =
            result.data as GameReleaseStatus;
        }
        return acc;
      },
      {} as Record<string, GameReleaseStatus | undefined>,
    );
  }, [results, releases]);

  return statuses;
}

export function useInstallAndMonitorRelease(
  variant: GameVariant,
  selectedReleaseId: string | undefined,
) {
  const queryClient = useQueryClient();

  const {
    install,
    isInstalling,
    installationProgressStatus,
    downloadProgress,
  } = useInstallAndMonitor(
    "release",
    variant,
    selectedReleaseId,
    installReleaseForVariant,
    (releaseId: string) => {
      queryClient.setQueryData(
        queryKeys.activeRelease(variant),
        () => releaseId,
      );
      queryClient.setQueryData(
        queryKeys.releases(variant),
        (old: GameRelease[] | undefined) =>
          old?.map((o) => {
            if (o.version !== releaseId) {
              return o;
            }
            // Note: We don't have the updated release here, so we return the old one
            // The actual update should be handled by the releases update listener
            return o;
          }),
      );

      queryClient.setQueryData(
        queryKeys.installationStatus(variant, releaseId),
        (): GameReleaseStatus => "ReadyToPlay",
      );

      queryClient.invalidateQueries({
        queryKey: queryKeys.tips(variant),
      });
    },
    (e) => {
      toastCL(
        "error",
        "Failed to install release.",
        e,
        playPageErrorMap,
      );
    },
  );

  return {
    install,
    isInstalling,
    installationProgressStatus,
    downloadProgress,
  };
}

export function useInstallationStatus(
  variant: GameVariant,
  selectedReleaseId: string | undefined,
) {
  const { data: installationStatus, error: installationStatusError } =
    useQuery<GameReleaseStatus>({
      queryKey: queryKeys.installationStatus(
        variant,
        selectedReleaseId,
      ),
      queryFn: () =>
        getInstallationStatus(variant, selectedReleaseId!),
      enabled: Boolean(selectedReleaseId),
      initialData: "Unknown",
    });

  useEffect(() => {
    if (!installationStatusError) {
      return;
    }

    toastCL(
      "error",
      `Failed to get installation status of ${variant} ${selectedReleaseId}.`,
      installationStatusError,
      playPageErrorMap,
    );
  }, [installationStatusError, variant, selectedReleaseId]);

  return {
    installationStatus,
    installationStatusError,
  };
}

export function usePlayGame(variant: GameVariant) {
  const { launch, isStartingGame } = useLaunchGame(variant);
  return { play: launch, isStartingGame };
}

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

export function useResumeLastWorld(
  variant: GameVariant,
  {
    onError,
  }: {
    onError: (error: Error) => void;
  },
) {
  const { lastPlayedWorld } = useLastPlayedWorld(variant, {
    onError,
  });

  const { launch, isStartingGame } = useLaunchGame(variant, {
    worldName: lastPlayedWorld ?? undefined,
    onError,
  });

  return { resume: launch, isStartingGame, lastPlayedWorld };
}

export function useLaunchGame(
  variant: GameVariant,
  {
    worldName,
    onError,
  }: {
    worldName?: string | null;
    onError?: (error: Error) => void;
  } = {},
) {
  const queryClient = useQueryClient();
  const dispatch = useAppDispatch();

  const { mutate: launch, isPending: isStartingGame } = useMutation({
    mutationFn: (releaseId: string | undefined) => {
      if (!releaseId) {
        throw new Error("No release selected");
      }
      return launchGame(variant, releaseId, worldName ?? null);
    },
    onSuccess: (_, releaseId) => {
      dispatch(setCurrentlyPlaying({ variant }));
      queryClient.setQueryData(
        queryKeys.activeRelease(variant),
        () => releaseId!,
      );
    },
    onError: (e) => {
      if (onError) {
        onError(e as Error);
      } else {
        toastCL(
          "error",
          "Failed to launch game.",
          e,
          playPageErrorMap,
        );
      }
    },
  });

  return { launch, isStartingGame };
}

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
        playPageErrorMap,
      );
    }
  }, [totalPlayTimeError, variant]);

  useEffect(() => {
    if (versionPlayTimeError) {
      toastCL(
        "error",
        `Failed to get version play time for ${variant}.`,
        versionPlayTimeError,
        playPageErrorMap,
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
