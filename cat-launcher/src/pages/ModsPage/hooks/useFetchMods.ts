import {
  useMutation,
  useQuery,
  useQueryClient,
} from "@tanstack/react-query";
import { useEffect, useState } from "react";

import type { GameVariant } from "@/generated-types/GameVariant";
import type { Mod } from "@/generated-types/Mod";
import type { ModsUpdatePayload } from "@/generated-types/ModsUpdatePayload";
import {
  listenToModsUpdate,
  triggerFetchModsForVariant,
} from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { setupEventListener, toastCL } from "@/lib/utils";

export function useFetchMods(variant: GameVariant) {
  const queryClient = useQueryClient();
  const [hasReceivedUpdate, setHasReceivedUpdate] = useState(false);

  const { data: mods = [] } = useQuery<Mod[]>({
    queryKey: queryKeys.mods.listAll(variant),
    queryFn: async () => [],
    enabled: false,
    initialData: [],
    staleTime: Infinity,
  });

  useEffect(() => {
    const modsUpdateHandler = (payload: ModsUpdatePayload) => {
      queryClient.setQueryData<Mod[]>(
        queryKeys.mods.listAll(payload.variant),
        payload.mods,
      );

      if (payload.variant === variant) {
        setHasReceivedUpdate(true);
      }
    };

    const cleanup = setupEventListener(
      listenToModsUpdate,
      modsUpdateHandler,
      "Failed to subscribe to mods updates.",
    );

    return cleanup;
  }, [queryClient, variant]);

  const { mutate: fetchMods, isPending: isFetchingMods } =
    useMutation({
      mutationFn: triggerFetchModsForVariant,
      onMutate: () => {
        setHasReceivedUpdate(false);
        queryClient.setQueryData<Mod[]>(
          queryKeys.mods.listAll(variant),
          [],
        );
      },
      onError: (error: unknown, variant: GameVariant) => {
        setHasReceivedUpdate(true);
        toastCL(
          "error",
          `Failed to fetch mods for ${variant}.`,
          error,
        );
      },
    });

  useEffect(() => {
    setHasReceivedUpdate(false);
  }, [variant]);

  useEffect(() => {
    const shouldFetch = !hasReceivedUpdate && !isFetchingMods;

    if (shouldFetch) {
      fetchMods(variant);
    }
  }, [variant, fetchMods, isFetchingMods, hasReceivedUpdate]);

  return {
    mods,
    isFetchingMods,
    hasReceivedUpdate,
  };
}
