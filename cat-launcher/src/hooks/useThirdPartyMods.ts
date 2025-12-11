import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useState } from "react";

import type { GameVariant } from "@/generated-types/GameVariant";
import type { ThirdPartyMod } from "@/generated-types/ThirdPartyMod";
import {
  listThirdPartyModsForVariant,
  markThirdPartyModInstalled,
  removeThirdPartyModInstallation,
} from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

interface UseThirdPartyModsOptions {
  onInstallSuccess?: () => void;
  onInstallError?: (error: unknown) => void;
  onUninstallSuccess?: () => void;
  onUninstallError?: (error: unknown) => void;
}

const disabledQueryKey = ["third-party-mods", "disabled"] as const;

export function useThirdPartyMods(
  variant: GameVariant | null,
  {
    onInstallError,
    onInstallSuccess,
    onUninstallError,
    onUninstallSuccess,
  }: UseThirdPartyModsOptions = {},
) {
  const queryClient = useQueryClient();
  const [installingModId, setInstallingModId] = useState<string | null>(null);
  const [uninstallingModId, setUninstallingModId] = useState<string | null>(null);

  const {
    data: mods = [],
    isLoading,
    isError,
    error,
  } = useQuery<ThirdPartyMod[]>({
    queryKey: variant ? queryKeys.thirdPartyMods(variant) : disabledQueryKey,
    queryFn: () => {
      if (!variant) {
        throw new Error("variant is required");
      }

      return listThirdPartyModsForVariant(variant);
    },
    enabled: !!variant,
  });

  const invalidateMods = () => {
    if (variant) {
      queryClient.invalidateQueries({
        queryKey: queryKeys.thirdPartyMods(variant),
      });
    }
  };

  const installMutation = useMutation({
    mutationFn: (modId: string) => markThirdPartyModInstalled(variant!, modId),
    onMutate: (modId) => {
      setInstallingModId(modId);
    },
    onError: (err) => {
      setInstallingModId(null);
      onInstallError?.(err);
    },
    onSuccess: () => {
      onInstallSuccess?.();
    },
    onSettled: () => {
      setInstallingModId(null);
      invalidateMods();
    },
  });

  const uninstallMutation = useMutation({
    mutationFn: (modId: string) => removeThirdPartyModInstallation(variant!, modId),
    onMutate: (modId) => {
      setUninstallingModId(modId);
    },
    onError: (err) => {
      setUninstallingModId(null);
      onUninstallError?.(err);
    },
    onSuccess: () => {
      onUninstallSuccess?.();
    },
    onSettled: () => {
      setUninstallingModId(null);
      invalidateMods();
    },
  });

  const markInstalled = (modId: string) => {
    if (!variant) {
      return;
    }

    installMutation.mutate(modId);
  };

  const removeInstallation = (modId: string) => {
    if (!variant) {
      return;
    }

    uninstallMutation.mutate(modId);
  };

  return {
    mods,
    isLoading,
    isError,
    error,
    markInstalled,
    removeInstallation,
    installingModId,
    uninstallingModId,
  };
}
