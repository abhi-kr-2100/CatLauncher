import {
  useMutation,
  useQuery,
  useQueryClient,
} from "@tanstack/react-query";
import { useEffect, useRef, useState } from "react";

import type { GameVariant } from "@/generated-types/GameVariant";
import type { Mod } from "@/generated-types/Mod";
import type { ModsUpdatePayload } from "@/generated-types/ModsUpdatePayload";
import {
  listenToModsUpdate,
  triggerFetchModsForVariant,
} from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { setupEventListener } from "@/lib/utils";

export type ModFetchStatus = "idle" | "loading" | "success" | "error";

export function useMods(
  variant: GameVariant,
  onModsLoadError?: (error: unknown) => void,
  onModsTriggerError?: (error: unknown) => void,
) {
  const queryClient = useQueryClient();
  const onModsLoadErrorRef = useRef(onModsLoadError);
  const onModsTriggerErrorRef = useRef(onModsTriggerError);
  const [fetchStatus, setFetchStatus] =
    useState<ModFetchStatus>("idle");

  useEffect(() => {
    onModsLoadErrorRef.current = onModsLoadError;
  }, [onModsLoadError]);

  useEffect(() => {
    onModsTriggerErrorRef.current = onModsTriggerError;
  }, [onModsTriggerError]);

  useEffect(() => {
    const modsUpdateHandler = (payload: ModsUpdatePayload) => {
      queryClient.setQueryData(
        queryKeys.mods.listAll(variant),
        payload.mods,
      );

      if (payload.variant === variant) {
        if (payload.status === "Success") {
          setFetchStatus("success");
        } else if (payload.status === "Error") {
          setFetchStatus("error");
        } else if (payload.status === "Fetching") {
          setFetchStatus("loading");
        }
      }
    };

    const cleanup = setupEventListener(
      listenToModsUpdate,
      modsUpdateHandler,
      "Failed to subscribe to mods updates.",
    );

    return () => {
      setFetchStatus("idle");
      cleanup();
    };
  }, [variant, queryClient]);

  const { data: mods = [], error: modsError } = useQuery<Mod[]>({
    queryKey: queryKeys.mods.listAll(variant),
    queryFn: () => {
      // The actual data will come via events and update the cache.
      // We return an empty array if nothing is in the cache.
      return [];
    },
    staleTime: Infinity,
    initialData: [],
  });

  useEffect(() => {
    if (modsError && onModsLoadErrorRef.current) {
      onModsLoadErrorRef.current(modsError);
    }
  }, [modsError]);

  const {
    mutate: triggerFetchMods,
    isPending: isModsTriggerLoading,
  } = useMutation({
    mutationFn: triggerFetchModsForVariant,
    onMutate: () => {
      setFetchStatus("loading");
    },
    onError: (error: unknown) => {
      setFetchStatus("error");
      if (onModsTriggerErrorRef.current) {
        onModsTriggerErrorRef.current(error);
      }
    },
  });

  useEffect(() => {
    const shouldFetch = fetchStatus === "idle" && mods.length === 0;

    if (shouldFetch) {
      triggerFetchMods(variant);
    }
  }, [variant, triggerFetchMods, mods.length, fetchStatus]);

  return {
    mods,
    isLoading:
      (fetchStatus === "loading" || isModsTriggerLoading) &&
      mods.length === 0,
  };
}
