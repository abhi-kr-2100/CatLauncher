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
import { useEffect, useRef } from "react";

export function useInstallThirdPartySoundpack(
  variant: GameVariant,
  soundpackId: string | undefined,
  onSuccess?: () => void,
  onError?: (error: Error) => void,
) {
  const queryClient = useQueryClient();
  const onSuccessRef = useRef(onSuccess);
  const onErrorRef = useRef(onError);

  useEffect(() => {
    onSuccessRef.current = onSuccess;
  }, [onSuccess]);

  useEffect(() => {
    onErrorRef.current = onError;
  }, [onError]);

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
      onSuccessRef.current?.();
    },
    (error: Error) => {
      onErrorRef.current?.(error);
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
  onInstallationStatusError?: (error: unknown) => void,
) {
  const onInstallationStatusErrorRef = useRef(
    onInstallationStatusError,
  );

  useEffect(() => {
    onInstallationStatusErrorRef.current = onInstallationStatusError;
  }, [onInstallationStatusError]);

  const query = useQuery({
    queryKey: queryKeys.soundpacks.installationStatus(
      variant,
      soundpackId,
    ),
    queryFn: () =>
      getThirdPartySoundpackInstallationStatus(soundpackId, variant),
  });

  useEffect(() => {
    if (query.error && onInstallationStatusErrorRef.current) {
      onInstallationStatusErrorRef.current(query.error);
    }
  }, [query.error]);

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
  const onSuccessRef = useRef(onSuccess);
  const onErrorRef = useRef(onError);

  useEffect(() => {
    onSuccessRef.current = onSuccess;
  }, [onSuccess]);

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
      onSuccessRef.current?.();
    },
    onError: (error) => {
      onErrorRef.current?.(error);
    },
  });

  return {
    isUninstalling: mutation.isPending,
    uninstall: (soundpackId: string) => mutation.mutate(soundpackId),
  };
}

export function useListAllSoundpacks(
  variant: GameVariant,
  onSoundpacksLoadError?: (error: unknown) => void,
) {
  const onSoundpacksLoadErrorRef = useRef(onSoundpacksLoadError);

  useEffect(() => {
    onSoundpacksLoadErrorRef.current = onSoundpacksLoadError;
  }, [onSoundpacksLoadError]);

  const query = useQuery({
    queryKey: queryKeys.soundpacks.listAll(variant),
    queryFn: () => listAllSoundpacks(variant),
  });

  useEffect(() => {
    if (query.error && onSoundpacksLoadErrorRef.current) {
      onSoundpacksLoadErrorRef.current(query.error);
    }
  }, [query.error]);

  return {
    soundpacks: query.data,
    isLoading: query.isLoading,
    error: query.error,
  };
}
