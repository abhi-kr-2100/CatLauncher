import { useQueryClient } from "@tanstack/react-query";

import type { GameVariant } from "@/generated-types/GameVariant";
import { ModInstallationStatus } from "@/generated-types/ModInstallationStatus";
import { useInstallAndMonitor } from "@/hooks/useInstallAndMonitor";
import { installThirdPartyMod } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useInstallThirdPartyMod(
  variant: GameVariant,
  modId: string | undefined,
  onSuccess?: () => void,
  onError?: (error: Error) => void,
) {
  const queryClient = useQueryClient();

  const {
    install,
    isInstalling,
    downloadProgress,
    installationProgressStatus,
  } = useInstallAndMonitor(
    "mod",
    variant,
    modId,
    installThirdPartyMod,
    (id: string) => {
      queryClient.setQueryData<ModInstallationStatus>(
        queryKeys.mods.installationStatus(variant, id),
        "Installed",
      );
      onSuccess?.();
    },
    (error: Error) => {
      onError?.(error);
    },
  );

  return {
    install,
    isInstalling,
    downloadProgress,
    installationProgressStatus,
  };
}
