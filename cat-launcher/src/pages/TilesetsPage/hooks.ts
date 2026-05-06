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

/**
 * A custom hook that handles the installation and monitoring of a third-party tileset.
 *
 * @param variant - The game variant to install the tileset for.
 * @param tilesetId - The unique identifier of the tileset.
 * @param onSuccess - Optional callback fired when installation is successful.
 * @param onError - Optional callback fired if installation fails.
 * @returns An object containing installation methods and status.
 */
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

/**
 * A custom hook that fetches the installation status of a third-party tileset.
 *
 * @param tilesetId - The unique identifier of the tileset.
 * @param variant - The game variant.
 * @returns An object containing the installation status and loading state.
 */
export function useGetThirdPartyTilesetInstallationStatus(
  tilesetId: string,
  variant: GameVariant,
) {
  const query = useQuery({
    queryKey: queryKeys.tilesets.installationStatus(
      variant,
      tilesetId,
    ),
    queryFn: () =>
      getThirdPartyTilesetInstallationStatus(tilesetId, variant),
  });

  return {
    installationStatus: query.data,
    isLoading: query.isLoading,
  };
}

/**
 * A custom hook that provides a mutation for uninstalling a third-party tileset.
 *
 * @param variant - The game variant to uninstall the tileset from.
 * @param onSuccess - Optional callback fired when uninstallation is successful.
 * @param onError - Optional callback fired if uninstallation fails.
 * @returns An object containing the `uninstall` function and `isUninstalling` status.
 */
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

/**
 * A custom hook that fetches all available tilesets for a game variant.
 *
 * @param variant - The game variant to list tilesets for.
 * @returns An object containing the list of tilesets, loading status, and any error.
 */
export function useListAllTilesets(variant: GameVariant) {
  const query = useQuery({
    queryKey: queryKeys.tilesets.listAll(variant),
    queryFn: () => listAllTilesets(variant),
  });

  return {
    tilesets: query.data,
    isLoading: query.isLoading,
    error: query.error,
  };
}
