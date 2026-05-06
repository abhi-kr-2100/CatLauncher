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
import { SoundpackInstallationStatus } from "@/generated-types/SoundpackInstallationStatus";

/**
 * Hook to install a third-party soundpack for a specific game variant.
 *
 * @param variant - The game variant to install the soundpack for.
 * @param soundpackId - The unique identifier of the soundpack.
 * @param onSuccess - Optional callback triggered on successful installation.
 * @param onError - Optional callback triggered when an error occurs during installation.
 * @returns An object containing the install function and installation status/progress.
 */
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
      queryClient.setQueryData<SoundpackInstallationStatus>(
        queryKeys.soundpacks.installationStatus(variant, id),
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

/**
 * Hook to fetch the installation status of a third-party soundpack.
 *
 * @param soundpackId - The unique identifier of the soundpack.
 * @param variant - The game variant the soundpack belongs to.
 * @returns An object containing the installation status and loading state.
 */
export function useGetThirdPartySoundpackInstallationStatus(
  soundpackId: string,
  variant: GameVariant,
) {
  const query = useQuery({
    queryKey: queryKeys.soundpacks.installationStatus(
      variant,
      soundpackId,
    ),
    queryFn: () =>
      getThirdPartySoundpackInstallationStatus(soundpackId, variant),
  });

  return {
    installationStatus: query.data,
    isLoading: query.isLoading,
  };
}

/**
 * Hook to uninstall a third-party soundpack.
 *
 * @param variant - The game variant to uninstall the soundpack from.
 * @param onSuccess - Optional callback triggered on successful uninstallation.
 * @param onError - Optional callback triggered when an error occurs during uninstallation.
 * @returns An object containing the uninstall function and uninstallation state.
 */
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
          variant,
          soundpackId,
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

/**
 * Hook to list all available soundpacks for a specific game variant.
 *
 * @param variant - The game variant to list soundpacks for.
 * @returns An object containing the list of soundpacks, loading state, and any potential error.
 */
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
