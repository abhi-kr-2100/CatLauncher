import {
  useQuery,
  useMutation,
  useQueryClient,
} from "@tanstack/react-query";

import { useInstallAndMonitor } from "@/hooks/useInstallAndMonitor";
import type { GameVariant } from "@/generated-types/GameVariant";
import {
  getThirdPartySoundpackInstallationStatus,
  installThirdPartySoundpack,
  listAllSoundpacks,
  uninstallThirdPartySoundpack,
} from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useInstallThirdPartySoundpack(
  variant: GameVariant,
  soundpackId: string | undefined,
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
    "soundpack",
    variant,
    soundpackId,
    installThirdPartySoundpack,
    (id: string) => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.soundpacks.listAll(variant),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.soundpacks.installationStatus(
          id,
          variant,
        ),
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

export function useGetThirdPartySoundpackInstallationStatus(
  soundpackId: string,
  variant: GameVariant,
) {
  const query = useQuery({
    queryKey: queryKeys.soundpacks.installationStatus(
      soundpackId,
      variant,
    ),
    queryFn: () =>
      getThirdPartySoundpackInstallationStatus(soundpackId, variant),
  });

  return {
    installationStatus: query.data,
    isLoading: query.isLoading,
  };
}

export function useUninstallThirdPartySoundpack(
  variant: GameVariant,
  onSuccess?: () => void,
  onError?: (error: unknown) => void,
) {
  const queryClient = useQueryClient();

  const mutation = useMutation({
    mutationFn: (soundpackId: string) =>
      uninstallThirdPartySoundpack(soundpackId, variant),
    onSuccess: (_data, soundpackId) => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.soundpacks.listAll(variant),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.soundpacks.installationStatus(
          soundpackId,
          variant,
        ),
      });
      onSuccess?.();
    },
    onError,
  });

  return {
    isUninstalling: mutation.isPending,
    uninstall: (soundpackId: string) => mutation.mutate(soundpackId),
  };
}

export function useListAllSoundpacks(variant: GameVariant) {
  const query = useQuery({
    queryKey: queryKeys.soundpacks.listAll(variant),
    queryFn: () => listAllSoundpacks(variant),
  });

  return {
    soundpacks: query.data,
    isLoading: query.isLoading,
    error: query.error,
  };
}
