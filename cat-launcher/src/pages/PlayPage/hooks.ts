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
import type { InstallationProgressStatus } from "@/generated-types/InstallationProgressStatus";
import type { ReleasesUpdatePayload } from "@/generated-types/ReleasesUpdatePayload";
import {
  getInstallationStatus,
  getPlayTimeForVariant,
  getPlayTimeForVersion,
  installReleaseForVariant,
  launchGame,
  listenToGameEvent,
  listenToInstallationStatusUpdate,
  listenToReleasesUpdate,
} from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { setupEventListener, toastCL } from "@/lib/utils";
import { useAppDispatch, useAppSelector } from "@/store/hooks";
import { setCurrentlyPlaying } from "@/store/gameSessionSlice";
import {
  clearInstallationProgress,
  setDownloadProgress,
  setInstallationProgress,
} from "@/store/installationProgressSlice";
import { updateReleasesForVariant } from "@/store/releasesSlice";

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

export function useAllReleasesInstallationStatuses(variant: GameVariant) {
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
          acc[releases[index].version] = result.data as GameReleaseStatus;
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
  const dispatch = useAppDispatch();
  const queryClient = useQueryClient();

  const installationProgressStatus = useAppSelector((state) => {
    if (!selectedReleaseId) {
      return null;
    }

    return state.installationProgress.statusByVariant[variant][
      selectedReleaseId
    ];
  });

  const downloadProgress = useAppSelector((state) => {
    if (!selectedReleaseId) {
      return null;
    }

    return state.installationProgress.progressByVariant[variant][
      selectedReleaseId
    ];
  });

  useEffect(() => {
    if (!selectedReleaseId) {
      return;
    }

    const installationProgressStatusUpdate = (
      status: InstallationProgressStatus,
    ) => {
      dispatch(
        setInstallationProgress({
          variant,
          releaseId: selectedReleaseId,
          status,
        }),
      );
    };

    const cleanup = setupEventListener(
      (payload) => listenToInstallationStatusUpdate(selectedReleaseId, payload),
      installationProgressStatusUpdate,
      "Failed to subscribe to installation progress.",
    );

    return () => {
      cleanup();
    };
  }, [dispatch, variant, selectedReleaseId]);

  const { mutate, isPending, reset } = useMutation({
    mutationFn: (releaseId: string | undefined) => {
      if (!releaseId) {
        throw new Error("No release selected");
      }
      return installReleaseForVariant(variant, releaseId, (progress) => {
        dispatch(
          setDownloadProgress({
            variant,
            releaseId,
            progress,
          }),
        );
      });
    },

    onSuccess: (updatedRelease, releaseId) => {
      queryClient.setQueryData(
        queryKeys.activeRelease(variant),
        () => releaseId!,
      );
      queryClient.setQueryData(
        queryKeys.releases(variant),
        (old: GameRelease[] | undefined) =>
          old?.map((o) => {
            if (o.version !== releaseId) {
              return o;
            }
            return updatedRelease;
          }),
      );

      queryClient.setQueryData(
        queryKeys.installationStatus(variant, releaseId),
        (): GameReleaseStatus => "ReadyToPlay",
      );

      queryClient.invalidateQueries({
        queryKey: queryKeys.tips(variant),
      });

      if (!releaseId) {
        console.error("😵 Release should not be undefined here. 😵");
        return;
      }

      dispatch(
        setInstallationProgress({ variant, releaseId, status: "Success" }),
      );
    },

    onError: (e, releaseId) => {
      toastCL("error", "Failed to install release.", e);
      if (releaseId) {
        dispatch(clearInstallationProgress({ variant, releaseId }));
      }
    },
  });

  useEffect(() => {
    reset();
  }, [reset, selectedReleaseId]);

  return {
    install: mutate,
    isInstalling: isPending,
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
      queryKey: queryKeys.installationStatus(variant, selectedReleaseId),
      queryFn: () => getInstallationStatus(variant, selectedReleaseId!),
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
    );
  }, [installationStatusError, variant, selectedReleaseId]);

  return {
    installationStatus,
    installationStatusError,
  };
}

export function usePlayGame(variant: GameVariant) {
  const queryClient = useQueryClient();
  const dispatch = useAppDispatch();
  const { mutate: play, isPending: isStartingGame } = useMutation({
    mutationFn: (releaseId: string | undefined) => {
      if (!releaseId) {
        throw new Error("No release selected");
      }
      return launchGame(variant, releaseId);
    },
    onSuccess: (_, releaseId) => {
      dispatch(setCurrentlyPlaying({ variant }));
      queryClient.setQueryData(
        queryKeys.activeRelease(variant),
        () => releaseId!,
      );
    },
    onError: (e) => {
      toastCL("error", "Failed to launch game.", e);
    },
  });

  return { play, isStartingGame };
}

export function usePlayTime(variant: GameVariant, releaseId?: string) {
  const queryClient = useQueryClient();
  const { data: totalPlayTime, error: totalPlayTimeError } = useQuery({
    queryKey: queryKeys.playTimeForVariant(variant),
    queryFn: () => getPlayTimeForVariant(variant),
    initialData: 0,
  });

  const { data: versionPlayTime, error: versionPlayTimeError } = useQuery({
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
            queryKey: queryKeys.playTimeForVersion(variant, releaseId),
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
