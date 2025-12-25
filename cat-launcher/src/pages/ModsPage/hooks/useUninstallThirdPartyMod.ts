import { useMutation, useQueryClient } from "@tanstack/react-query";

import type { GameVariant } from "@/generated-types/GameVariant";
import { uninstallThirdPartyMod } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { ModInstallationStatus } from "@/generated-types/ModInstallationStatus";

export function useUninstallThirdPartyMod(
  variant: GameVariant,
  onSuccess?: () => void,
  onError?: (error: unknown) => void,
) {
  const queryClient = useQueryClient();

  const mutation = useMutation({
    mutationFn: (modId: string) =>
      uninstallThirdPartyMod(modId, variant),
    onSuccess: (_data, modId) => {
      queryClient.setQueryData<ModInstallationStatus>(
        queryKeys.mods.installationStatus(variant, modId),
        "NotInstalled",
      );
      onSuccess?.();
    },
    onError,
  });

  return {
    isUninstalling: mutation.isPending,
    uninstall: (modId: string) => mutation.mutate(modId),
  };
}
