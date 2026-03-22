import {
  useQuery,
  useQueryClient,
  useMutation,
} from "@tanstack/react-query";

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
import { useEffect, useRef } from "react";

export function useInstallAndMonitorThirdPartyTileset(
  variant: GameVariant,
  tilesetId: string | undefined,
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
    "tileset",
    variant,
    tilesetId,
    installThirdPartyTileset,
    (id: string) => {
      queryClient.setQueryData<TilesetInstallationStatus>(
        queryKeys.tilesets.installationStatus(variant, id),
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

export function useGetThirdPartyTilesetInstallationStatus(
  tilesetId: string,
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
    queryKey: queryKeys.tilesets.installationStatus(
      variant,
      tilesetId,
    ),
    queryFn: () =>
      getThirdPartyTilesetInstallationStatus(tilesetId, variant),
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

export function useUninstallThirdPartyTileset(
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
      onSuccessRef.current?.();
    },
    onError: (error) => {
      onErrorRef.current?.(error);
    },
  });

  return {
    isUninstalling: mutation.isPending,
    uninstall: (tilesetId: string) => mutation.mutate(tilesetId),
  };
}

export function useListAllTilesets(
  variant: GameVariant,
  onTilesetsLoadError?: (error: unknown) => void,
) {
  const onTilesetsLoadErrorRef = useRef(onTilesetsLoadError);

  useEffect(() => {
    onTilesetsLoadErrorRef.current = onTilesetsLoadError;
  }, [onTilesetsLoadError]);

  const query = useQuery({
    queryKey: queryKeys.tilesets.listAll(variant),
    queryFn: () => listAllTilesets(variant),
  });

  useEffect(() => {
    if (query.error && onTilesetsLoadErrorRef.current) {
      onTilesetsLoadErrorRef.current(query.error);
    }
  }, [query.error]);

  return {
    tilesets: query.data,
    isLoading: query.isLoading,
    error: query.error,
  };
}
