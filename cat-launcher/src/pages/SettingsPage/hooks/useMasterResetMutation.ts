import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import { GameVariant } from "@/generated-types/GameVariant";
import { masterReset } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useMasterResetMutation(
  onSuccess?: () => void,
  onError?: (error: Error) => void,
) {
  const queryClient = useQueryClient();
  const onSuccessRef = useRef(onSuccess);
  const onErrorRef = useRef(onError);

  useEffect(() => {
    onSuccessRef.current = onSuccess;
    onErrorRef.current = onError;
  }, [onSuccess, onError]);

  const mutation = useMutation({
    mutationFn: (variant: GameVariant) => masterReset(variant),
    onSuccess: (_, variant) => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.mods.listAll(variant),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.tilesets.listAll(variant),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.soundpacks.listAll(variant),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.settings(),
      });

      onSuccessRef.current?.();
    },
    onError: (error) => {
      onErrorRef.current?.(error as Error);
    },
  });

  return {
    reset: (variant: GameVariant) => mutation.mutate(variant),
    isResetting: mutation.isPending,
  };
}
