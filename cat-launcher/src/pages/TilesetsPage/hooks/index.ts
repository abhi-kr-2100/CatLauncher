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
  onStatusError?: (error: Error) => void,
) {
  const onStatusErrorRef = useRef(onStatusError);

  useEffect(() => {
    onStatusErrorRef.current = onStatusError;
  }, [onStatusError]);

  const query = useQuery({
    queryKey: queryKeys.tilesets.installationStatus(
      variant,
      tilesetId,
    ),
    queryFn: () =>
      getThirdPartyTilesetInstallationStatus(tilesetId, variant),
  });

  useEffect(() => {
    if (query.error && onStatusErrorRef.current) {
      onStatusErrorRef.current(query.error as Error);
    }
  }, [query.error]);

  return {
    installationStatus: query.data,
    isLoading: query.isLoading,
  };
}

export function useUninstallThirdPartyTileset(
  variant: GameVariant,
  onSuccess?: () => void,
  onError?: (error: unknown) => void,
) {
  const queryClient = useQueryClient();

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
    onError,
  });

  return {
    isUninstalling: mutation.isPending,
    uninstall: (tilesetId: string) => mutation.mutate(tilesetId),
  };
}

export function useListAllTilesets(
  variant: GameVariant,
  onListError?: (error: Error) => void,
) {
  const onListErrorRef = useRef(onListError);

  useEffect(() => {
    onListErrorRef.current = onListError;
  }, [onListError]);

  const query = useQuery({
    queryKey: queryKeys.tilesets.listAll(variant),
    queryFn: () => listAllTilesets(variant),
  });

  useEffect(() => {
    if (query.error && onListErrorRef.current) {
      onListErrorRef.current(query.error as Error);
    }
  }, [query.error]);

  return {
    tilesets: query.data,
    isLoading: query.isLoading,
    error: query.error,
  };
}
