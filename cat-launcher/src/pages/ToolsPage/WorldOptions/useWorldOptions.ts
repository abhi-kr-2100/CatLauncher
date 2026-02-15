import {
  useMutation,
  useQuery,
  useQueryClient,
} from "@tanstack/react-query";
import { useEffect, useMemo, useRef } from "react";

import type { GameVariant } from "@/generated-types/GameVariant";
import type { WorldOption } from "@/generated-types/WorldOption";
import type { WorldOptionMetadata } from "@/generated-types/WorldOptionMetadata";
import {
  getWorldOptions,
  getWorldOptionsMetadata,
  getWorlds,
  updateWorldOptions,
} from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

function useErrorCallback(
  isError: boolean,
  error: unknown,
  onError?: (e: unknown) => void,
) {
  const onErrorRef = useRef(onError);
  useEffect(() => {
    onErrorRef.current = onError;
  }, [onError]);

  useEffect(() => {
    if (isError) {
      onErrorRef.current?.(error);
    }
  }, [isError, error]);
}

export function useWorlds(
  variant: GameVariant | null,
  onError?: (error: unknown) => void,
) {
  const { data, isLoading, isFetching, error, isError, refetch } =
    useQuery({
      queryKey: queryKeys.worlds(variant),
      queryFn: () => getWorlds(variant!),
      enabled: !!variant,
      staleTime: 0,
    });

  useErrorCallback(isError, error, onError);

  return {
    data: data ?? [],
    isLoading,
    isFetching,
    error,
    refetch,
  };
}

export function useWorldOptionsMetadata(
  variant: GameVariant | null,
  onError?: (error: unknown) => void,
) {
  const { data, isLoading, isError, error } = useQuery({
    queryKey: queryKeys.worldOptionsMetadata(variant),
    queryFn: () => getWorldOptionsMetadata(variant!),
    enabled: !!variant,
    staleTime: Infinity,
  });

  useErrorCallback(isError, error, onError);

  return { data, isLoading, isError, error };
}

export function useWorldOptions(
  variant: GameVariant,
  world: string | null,
  onError?: (error: unknown) => void,
) {
  const { data, isLoading, isError, error } = useQuery({
    queryKey: queryKeys.worldOptions(variant, world),
    queryFn: () => getWorldOptions(variant, world!),
    enabled: !!world,
  });

  useErrorCallback(isError, error, onError);

  return { data, isLoading, isError, error };
}

export function useHasMappedOptions(
  initialOptions: WorldOption[] | undefined,
  metadataMap: Record<string, WorldOptionMetadata> | undefined,
) {
  return useMemo(() => {
    if (!initialOptions || !metadataMap) return false;

    const optionsNames = new Set(initialOptions.map((o) => o.name));

    const checkMetadata = (
      map: Record<string, WorldOptionMetadata>,
    ): boolean => {
      return Object.values(map).some((meta) => {
        if (meta.type === "group" && meta.children) {
          return checkMetadata(meta.children);
        }
        return optionsNames.has(meta.id);
      });
    };

    return checkMetadata(metadataMap);
  }, [initialOptions, metadataMap]);
}

export function useUpdateWorldOptions(
  onError?: (error: unknown) => void,
) {
  const queryClient = useQueryClient();

  const mutation = useMutation({
    mutationFn: async ({
      variant,
      world,
      options,
    }: {
      variant: GameVariant;
      world: string;
      options: WorldOption[];
    }) => {
      await updateWorldOptions(variant, world, options);
    },
    onSuccess: (_, { variant, world }) => {
      void queryClient.invalidateQueries({
        queryKey: queryKeys.worldOptions(variant, world),
      });
    },
  });

  useErrorCallback(mutation.isError, mutation.error, onError);

  return mutation;
}
