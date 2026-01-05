import { useQueryClient } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import type { GameVariant } from "@/generated-types/GameVariant";
import type { ModsUpdatePayload } from "@/generated-types/ModsUpdatePayload";
import { listenToModsUpdate } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { setupEventListener } from "@/lib/utils";

export function useModEvents(
  variant: GameVariant,
  onError?: (error: unknown) => void,
) {
  const queryClient = useQueryClient();
  const onErrorRef = useRef(onError);

  useEffect(() => {
    onErrorRef.current = onError;
  }, [onError]);

  useEffect(() => {
    const modsUpdateHandler = (payload: ModsUpdatePayload) => {
      if (payload.variant === variant) {
        queryClient.setQueryData(
          queryKeys.mods.listAll(variant),
          payload.mods,
        );
      }
    };

    const cleanup = setupEventListener(
      listenToModsUpdate,
      modsUpdateHandler,
      "Failed to subscribe to mods updates.",
      onErrorRef.current,
    );

    return cleanup;
  }, [variant, queryClient]);
}
