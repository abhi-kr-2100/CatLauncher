import { useMutation } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import type { DownloadProgress } from "@/generated-types/DownloadProgress";
import type { GameVariant } from "@/generated-types/GameVariant";
import { useThrottleWithCancel } from "@/hooks/useThrottleWithCancel";
import { toSerializableDownloadProgress } from "@/lib/types";
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

  const {
    throttledFunc: throttledOnProgress,
    cancel: cancelThrottle,
  } = useThrottleWithCancel(
    (itemId: string, progress: DownloadProgress) => {
      const serializableProgress =
        toSerializableDownloadProgress(progress);

      dispatch(
        setDownloadProgress({
          type,
          variant,
          id: itemId,
          progress: serializableProgress,
        }),
      );
    },
    1000,
  );

  const onErrorRef = useRef(onError);

  useEffect(() => {
    onErrorRef.current = onError;
  }, [onError]);

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
            bytes_downloaded: 0,
            total_bytes: 0,
          },
        }),
      );
    },
    onSuccess: (_data, itemId) => {
      onSuccess?.(itemId);
    },
    onSettled: (_data, _error, itemId) => {
      // Cancel any pending throttled dispatches to prevent setting progress
      // after installation has completed
      cancelThrottle();

      dispatch(
        clearInstallationProgress({
          variant,
          id: itemId,
          type,
        }),
      );
    },
    onError: (e) => {
      if (onErrorRef.current) {
        onErrorRef.current(e);
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
