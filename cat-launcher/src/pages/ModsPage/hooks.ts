import {
  useMutation,
  useQuery,
  useQueryClient,
} from "@tanstack/react-query";

import type { GameVariant } from "@/generated-types/GameVariant";
import {
  getThirdPartyModInstallationStatus,
  installThirdPartyMod,
  uninstallThirdPartyMod,
} from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useInstallThirdPartyMod(
  variant: GameVariant,
  onSuccess?: () => void,
  onError?: (error: unknown) => void,
) {
  const queryClient = useQueryClient();

  const mutation = useMutation({
    mutationFn: (modId: string) =>
      installThirdPartyMod(modId, variant),
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
    isInstalling: mutation.isPending,
    install: (modId: string) => mutation.mutate(modId),
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
