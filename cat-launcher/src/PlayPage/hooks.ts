import { useEffect, useState } from "react";

import type { InstallationProgressStatus } from "@/generated-types/InstallationProgressStatus";
import type { ReleasesUpdatePayload } from "@/generated-types/ReleasesUpdatePayload";
import {
  listenToInstallationStatusUpdate,
  listenToReleasesUpdate,
} from "@/lib/commands";
import { setupEventListener } from "@/lib/utils";
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
