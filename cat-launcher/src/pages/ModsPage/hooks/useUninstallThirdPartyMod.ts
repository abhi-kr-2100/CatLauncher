import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import type { GameVariant } from "@/generated-types/GameVariant";
import { uninstallThirdPartyMod } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useUninstallThirdPartyMod(
  variant: GameVariant,
  onSuccess?: () => void,
  onError?: (error: unknown) => void,
) {
  const queryClient = useQueryClient();
  const onSuccessRef = useRef(onSuccess);
  const onErrorRef = useRef(onError);

  useEffect(() => {
    onSuccessRef.current = onSuccess;
    onErrorRef.current = onError;
  }, [onSuccess, onError]);

  const mutation = useMutation({
    mutationFn: (modId: string) =>
      uninstallThirdPartyMod(modId, variant),
    onSuccess: (_data, modId) => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.mods.installationStatus(variant, modId),
      });
      onSuccessRef.current?.();
    },
  });

  useEffect(() => {
    if (mutation.error && onErrorRef.current) {
      onErrorRef.current(mutation.error);
    }
  }, [mutation.error]);

  return {
    isUninstalling: mutation.isPending,
    uninstall: (modId: string) => mutation.mutate(modId),
  };
}
