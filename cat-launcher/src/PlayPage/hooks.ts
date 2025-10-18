import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useEffect, useState } from "react";

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
import { setCurrentlyPlaying } from "@/store/gameSessionSlice";
import { useAppDispatch } from "@/store/hooks";
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

export function useInstallationProgressStatus(
  selectedReleaseId: string | undefined,
) {
  const [installStatus, setInstallStatus] =
    useState<InstallationProgressStatus | null>(null);

  useEffect(() => {
    setInstallStatus(null);

    if (!selectedReleaseId) {
      return;
    }

    const installationProgressStatusUpdate = (
      status: InstallationProgressStatus,
    ) => {
      setInstallStatus(status);
    };

    const cleanup = setupEventListener(
      (payload) => listenToInstallationStatusUpdate(selectedReleaseId, payload),
      installationProgressStatusUpdate,
      "Failed to subscribe to installation progress.",
    );

    return () => {
      cleanup();
      setInstallStatus(null);
    };
  }, [selectedReleaseId]);

  return installStatus;
}

export function useInstallationStatus(
  variant: GameVariant,
  selectedReleaseId: string | undefined,
) {
  const {
    data: installationStatus,
    isLoading: isInstallationStatusLoading,
    error: installationStatusError,
  } = useQuery<GameReleaseStatus>({
    queryKey: queryKeys.installationStatus(variant, selectedReleaseId),
    queryFn: () => getInstallationStatus(variant, selectedReleaseId!),
    enabled: Boolean(selectedReleaseId),
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

  return { installationStatus, isInstallationStatusLoading, installationStatusError };
}

export function useInstallRelease(
  variant: GameVariant,
  selectedReleaseId: string | undefined,
) {
  const queryClient = useQueryClient();
  const {
    mutate: install,
    isPending: isInstalling,
    reset: resetInstall,
  } = useMutation({
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
    },
    onError: (e) => {
      toastCL("error", "Failed to install release.", e);
    },
  });

  useEffect(() => {
    resetInstall();
  }, [resetInstall, selectedReleaseId]);

  return { install, isInstalling };
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