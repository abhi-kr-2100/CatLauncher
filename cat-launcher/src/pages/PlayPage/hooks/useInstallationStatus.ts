import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import type { GameReleaseStatus } from "@/generated-types/GameReleaseStatus";
import type { GameVariant } from "@/generated-types/GameVariant";
import { getInstallationStatus } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useInstallationStatus(
  variant: GameVariant,
  selectedReleaseId: string | undefined,
  onInstallationStatusError?: (error: unknown) => void,
) {
  const onInstallationStatusErrorRef = useRef(
    onInstallationStatusError,
  );

  useEffect(() => {
    onInstallationStatusErrorRef.current = onInstallationStatusError;
  }, [onInstallationStatusError]);

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
    if (
      installationStatusError &&
      onInstallationStatusErrorRef.current
    ) {
      onInstallationStatusErrorRef.current(installationStatusError);
    }
  }, [installationStatusError]);

  return {
    installationStatus,
    installationStatusError,
  };
}
