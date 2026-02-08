import {
  useMutation,
  useQuery,
  useQueryClient,
} from "@tanstack/react-query";

import type { GameVariant } from "@/generated-types/GameVariant";
import type { WorldOption } from "@/generated-types/WorldOption";
import {
  getWorldOptions,
  getWorlds,
  updateWorldOptions,
} from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useWorlds(variant: GameVariant) {
  return useQuery({
    queryKey: queryKeys.worlds(variant),
    queryFn: () => getWorlds(variant),
  });
}

export function useWorldOptions(
  variant: GameVariant,
  world: string | null,
) {
  return useQuery({
    queryKey: queryKeys.worldOptions(variant, world),
    queryFn: () => getWorldOptions(variant, world!),
    enabled: !!world,
  });
}

export function useUpdateWorldOptions() {
  const queryClient = useQueryClient();

  return useMutation({
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
}
