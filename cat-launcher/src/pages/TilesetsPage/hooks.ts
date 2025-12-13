import {
  useMutation,
  useQuery,
  useQueryClient,
} from "@tanstack/react-query";

import type { GameVariant } from "@/generated-types/GameVariant";
import {
  getThirdPartyTilesetInstallationStatus,
  installThirdPartyTileset,
  listAllTilesets,
  uninstallThirdPartyTileset,
} from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useInstallThirdPartyTileset(
  variant: GameVariant,
  onSuccess?: () => void,
  onError?: (error: unknown) => void,
) {
  const queryClient = useQueryClient();

  const mutation = useMutation({
    mutationFn: (tilesetId: string) =>
      installThirdPartyTileset(tilesetId, variant),
    onSuccess: (_data, tilesetId) => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.tilesets.listAll(variant),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.tilesets.installationStatus(
          tilesetId,
          variant,
        ),
      });
      onSuccess?.();
    },
    onError,
  });

  return {
    isInstalling: mutation.isPending,
    install: (tilesetId: string) => mutation.mutate(tilesetId),
  };
}

export function useGetThirdPartyTilesetInstallationStatus(
  tilesetId: string,
  variant: GameVariant,
) {
  const query = useQuery({
    queryKey: queryKeys.tilesets.installationStatus(
      tilesetId,
      variant,
    ),
    queryFn: () =>
      getThirdPartyTilesetInstallationStatus(tilesetId, variant),
  });

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
          tilesetId,
          variant,
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
