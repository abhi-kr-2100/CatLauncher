import { useQuery } from "@tanstack/react-query";
import { useEffect } from "react";

import type { GameReleaseStatus } from "@/generated-types/GameReleaseStatus";
import type { GameVariant } from "@/generated-types/GameVariant";
import { getInstallationStatus } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { toastCL } from "@/lib/utils";

/**
 * Custom hook to retrieve and monitor the installation status of a specific game release.
 *
 * @param variant - The game variant.
 * @param selectedReleaseId - The specific release version ID.
 * @returns An object containing the installation status and any error that occurred.
 */
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
    );
  }, [installationStatusError, variant, selectedReleaseId]);

  return {
    installationStatus,
    installationStatusError,
  };
}
