import {
  useQuery,
  useQueryClient,
  useMutation,
} from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { useInstallAndMonitor } from "@/hooks/useInstallAndMonitor";
import type { GameVariant } from "@/generated-types/GameVariant";
import {
  getThirdPartyTilesetInstallationStatus,
  installThirdPartyTileset,
  listAllTilesets,
  uninstallThirdPartyTileset,
} from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { TilesetInstallationStatus } from "@/generated-types/TilesetInstallationStatus";

export function useInstallAndMonitorThirdPartyTileset(
  variant: GameVariant,
  tilesetId: string | undefined,
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
    "tileset",
    variant,
    tilesetId,
    installThirdPartyTileset,
    (id: string) => {
      queryClient.setQueryData<TilesetInstallationStatus>(
        queryKeys.tilesets.installationStatus(variant, id),
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

export function useGetThirdPartyTilesetInstallationStatus(
  tilesetId: string,
  variant: GameVariant,
  onLoadError?: (error: Error) => void,
) {
  const onLoadErrorRef = useRef(onLoadError);

  useEffect(() => {
    onLoadErrorRef.current = onLoadError;
  }, [onLoadError]);

  const query = useQuery({
    queryKey: queryKeys.tilesets.installationStatus(
      variant,
      tilesetId,
    ),
    queryFn: () =>
      getThirdPartyTilesetInstallationStatus(tilesetId, variant),
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

export function useUninstallThirdPartyTileset(
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
    mutationFn: (tilesetId: string) =>
      uninstallThirdPartyTileset(tilesetId, variant),
    onSuccess: (_data, tilesetId) => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.tilesets.listAll(variant),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.tilesets.installationStatus(
          variant,
          tilesetId,
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
    uninstall: (tilesetId: string) => mutation.mutate(tilesetId),
  };
}

export function useListAllTilesets(
  variant: GameVariant,
  onLoadError?: (error: Error) => void,
) {
  const onLoadErrorRef = useRef(onLoadError);

  useEffect(() => {
    onLoadErrorRef.current = onLoadError;
  }, [onLoadError]);

  const query = useQuery({
    queryKey: queryKeys.tilesets.listAll(variant),
    queryFn: () => listAllTilesets(variant),
  });

  useEffect(() => {
    if (query.error && onLoadErrorRef.current) {
      onLoadErrorRef.current(query.error as Error);
    }
  }, [query.error]);

  return {
    ...query,
    tilesets: query.data,
  };
}
