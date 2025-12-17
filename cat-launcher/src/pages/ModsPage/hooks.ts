import { useQuery, useQueryClient } from "@tanstack/react-query";

import { useInstallAndMonitor } from "@/hooks/useInstallAndMonitor";
import type { GameVariant } from "@/generated-types/GameVariant";
import {
  getThirdPartyModInstallationStatus,
  installThirdPartyMod,
  uninstallThirdPartyMod,
  getLastModActivity,
} from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { useMutation } from "@tanstack/react-query";
import { useEffect } from "react";
import { ModInstallationStatus } from "@/generated-types/ModInstallationStatus";

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

export function useGetThirdPartyModInstallationStatus(
  modId: string,
  variant: GameVariant,
) {
  const query = useQuery({
    queryKey: queryKeys.mods.installationStatus(variant, modId),
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
        queryKey: queryKeys.mods.installationStatus(variant, modId),
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

export function useGetLastModActivity(
  enabled: boolean,
  modId: string,
  variant: GameVariant,
  onError?: (error: unknown) => void,
) {
  const query = useQuery({
    queryKey: queryKeys.mods.lastActivity(variant, modId),
    queryFn: () => getLastModActivity(modId, variant),
    enabled,
  });

  useEffect(() => {
    if (query.error) {
      onError?.(query.error);
    }
  }, [query.error, onError]);

  return {
    lastActivity: query.data,
    isLoading: query.isLoading,
  };
}
