import {
  useQuery,
  useMutation,
  useQueryClient,
} from "@tanstack/react-query";
import { useEffect, useRef } from "react";

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

export function useGetThirdPartySoundpackInstallationStatus(
  soundpackId: string,
  variant: GameVariant,
  onLoadError?: (error: Error) => void,
) {
  const onLoadErrorRef = useRef(onLoadError);

  useEffect(() => {
    onLoadErrorRef.current = onLoadError;
  }, [onLoadError]);

  const query = useQuery({
    queryKey: queryKeys.soundpacks.installationStatus(
      variant,
      soundpackId,
    ),
    queryFn: () =>
      getThirdPartySoundpackInstallationStatus(soundpackId, variant),
  });

  useEffect(() => {
    if (query.error && onLoadErrorRef.current) {
      onLoadErrorRef.current(query.error as Error);
    }
  }, [query.error]);

  return {
    ...query,
    installationStatus: query.data,
  };
}

export function useUninstallThirdPartySoundpack(
  variant: GameVariant,
  onSuccess?: () => void,
  onError?: (error: Error) => void,
) {
  const queryClient = useQueryClient();
  const onErrorRef = useRef(onError);

  useEffect(() => {
    onErrorRef.current = onError;
  }, [onError]);

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
    onError: (error) => {
      if (onErrorRef.current) {
        onErrorRef.current(error as Error);
      }
    },
  });

  return {
    isUninstalling: mutation.isPending,
    uninstall: (soundpackId: string) => mutation.mutate(soundpackId),
  };
}

export function useListAllSoundpacks(
  variant: GameVariant,
  onLoadError?: (error: Error) => void,
) {
  const onLoadErrorRef = useRef(onLoadError);

  useEffect(() => {
    onLoadErrorRef.current = onLoadError;
  }, [onLoadError]);

  const query = useQuery({
    queryKey: queryKeys.soundpacks.listAll(variant),
    queryFn: () => listAllSoundpacks(variant),
  });

  useEffect(() => {
    if (query.error && onLoadErrorRef.current) {
      onLoadErrorRef.current(query.error as Error);
    }
  }, [query.error]);

  return {
    ...query,
    soundpacks: query.data,
  };
}
