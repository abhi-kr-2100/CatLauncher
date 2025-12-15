import { useQuery, useQueryClient } from "@tanstack/react-query";

import { useInstallAndMonitor } from "@/hooks/useInstallAndMonitor";
import type { GameVariant } from "@/generated-types/GameVariant";
import {
  getThirdPartyModInstallationStatus,
  installThirdPartyMod,
  uninstallThirdPartyMod,
} from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { useMutation } from "@tanstack/react-query";

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
      queryClient.invalidateQueries({
        queryKey: queryKeys.mods.listAll(variant),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.mods.installationStatus(id, variant),
      });
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

export function useGetThirdPartyModInstallationStatus(
  modId: string,
  variant: GameVariant,
) {
  const query = useQuery({
    queryKey: queryKeys.mods.installationStatus(modId, variant),
    queryFn: () => getThirdPartyModInstallationStatus(modId, variant),
  });

  return {
    installationStatus: query.data,
    isLoading: query.isLoading,
  };
}

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
      queryClient.invalidateQueries({
        queryKey: queryKeys.mods.listAll(variant),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.mods.installationStatus(modId, variant),
      });
      onSuccess?.();
    },
    onError,
  });

  return {
    isUninstalling: mutation.isPending,
    uninstall: (modId: string) => mutation.mutate(modId),
  };
}
