import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useEffect } from "react";

import type { GameRelease } from "@/generated-types/GameRelease";
import type { GameReleaseStatus } from "@/generated-types/GameReleaseStatus";
import type { GameVariant } from "@/generated-types/GameVariant";
import type { InstallationProgressStatus } from "@/generated-types/InstallationProgressStatus";
import type { ReleasesUpdatePayload } from "@/generated-types/ReleasesUpdatePayload";
import {
  getInstallationStatus,
  installReleaseForVariant,
  launchGame,
  listenToInstallationStatusUpdate,
  listenToReleasesUpdate,
} from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { setupEventListener, toastCL } from "@/lib/utils";
import { useAppDispatch, useAppSelector } from "@/store/hooks";
import {
  clearInstallationProgress,
  setInstallationProgress,
} from "@/store/installationProgressSlice";
import { updateReleasesForVariant } from "@/store/releasesSlice";
import { setCurrentlyPlaying } from "@/store/gameSessionSlice";

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
      return installReleaseForVariant(variant, releaseId);
    },

    onSuccess: (updatedRelease, releaseId) => {
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
  const { mutate: play } = useMutation({
    mutationFn: (releaseId: string | undefined) => {
      if (!releaseId) {
        throw new Error("No release selected");
      }
      return launchGame(variant, releaseId);
    },
    onSuccess: (_, releaseId) => {
      dispatch(setCurrentlyPlaying({ variant }));
      queryClient.setQueryData(
        queryKeys.lastPlayedVersion(variant),
        () => releaseId!,
      );
    },
    onError: (e) => {
      toastCL("error", "Failed to launch game.", e);
    },
  });

  return { play };
}
