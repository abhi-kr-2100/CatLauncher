import { useMutation } from "@tanstack/react-query";
import { useEffect } from "react";

import type { DownloadProgress } from "@/generated-types/DownloadProgress";
import type { GameVariant } from "@/generated-types/GameVariant";
import { useThrottle } from "@/hooks/useThrottle";
import { useAppDispatch, useAppSelector } from "@/store/hooks";
import {
  clearInstallationProgress,
  setDownloadProgress,
} from "@/store/installationProgressSlice";

type InstallationType = "release" | "mod" | "soundpack" | "tileset";

type InstallationFunction<T> = (
  id: string,
  variant: GameVariant,
  onProgress: (progress: DownloadProgress) => void,
) => Promise<T>;

export function useInstallAndMonitor<T>(
  type: InstallationType,
  variant: GameVariant,
  id: string | undefined,
  installationFunction: InstallationFunction<T>,
  onSuccess?: (id: string) => void,
  onError?: (error: Error) => void,
) {
  const dispatch = useAppDispatch();

  const downloadProgress = useAppSelector((state) => {
    if (!id) {
      return null;
    }

    return state.installationProgress.downloadProgressByVariant[type][
      variant
    ][id];
  });

  const installationProgressStatus = useAppSelector((state) => {
    if (!id) {
      return null;
    }

    return state.installationProgress.installationStatusByVariant[
      type
    ][variant][id];
  });

  const throttledOnProgress = useThrottle(
    (itemId: string, progress: DownloadProgress) => {
      if (progress.total_bytes === 0n) {
        return;
      }

      dispatch(
        setDownloadProgress({
          type,
          variant,
          id: itemId,
          progress,
        }),
      );
    },
    1000,
  );

  const {
    mutate,
    isPending: isInstalling,
    reset,
  } = useMutation({
    mutationFn: (itemId: string) => {
      return installationFunction(itemId, variant, (progress) => {
        throttledOnProgress(itemId, progress);
      });
    },
    onMutate: (itemId) => {
      dispatch(
        setDownloadProgress({
          type,
          variant,
          id: itemId,
          progress: {
            bytes_downloaded: 0n,
            total_bytes: 0n,
          },
        }),
      );
    },
    onSuccess: (_data, itemId) => {
      onSuccess?.(itemId);
    },
    onSettled: (_data, _error, itemId) => {
      dispatch(
        clearInstallationProgress({
          variant,
          id: itemId,
          type,
        }),
      );
    },
    onError: (e) => {
      if (onError) {
        onError(e);
      }
    },
  });

  useEffect(() => {
    reset();
  }, [reset, id]);

  return {
    install: mutate,
    isInstalling,
    installationProgressStatus,
    downloadProgress,
  };
}
