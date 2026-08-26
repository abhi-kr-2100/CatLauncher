import { useQueryClient } from "@tanstack/react-query";

import type { GameRelease } from "@/generated-types/GameRelease";
import type { GameReleaseStatus } from "@/generated-types/GameReleaseStatus";
import type { GameVariant } from "@/generated-types/GameVariant";
import { useInstallAndMonitor } from "@/hooks/useInstallAndMonitor";
import { installReleaseForVariant } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { toastCL } from "@/lib/utils";

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
      toastCL("error", "Failed to install release.", e);
    },
  );

  return {
    install,
    isInstalling,
    installationProgressStatus,
    downloadProgress,
  };
}
