import { useQueryClient } from "@tanstack/react-query";

import type { GameVariant } from "@/generated-types/GameVariant";
import { ModInstallationStatus } from "@/generated-types/ModInstallationStatus";
import { useInstallAndMonitor } from "@/hooks/useInstallAndMonitor";
import { installThirdPartyMod } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

/**
 * A custom hook to handle the installation and progress monitoring of a third-party mod.
 *
 * @param variant - The game variant associated with the mod.
 * @param modId - The unique identifier of the mod, or undefined if not yet known.
 * @param onSuccess - An optional callback function to execute on successful installation.
 * @param onError - An optional callback function to handle errors.
 * @returns An object containing the installation function, progress status, and download details.
 */
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
