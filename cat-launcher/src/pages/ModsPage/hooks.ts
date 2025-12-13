import { useMutation, useQueryClient } from "@tanstack/react-query";

import type { GameVariant } from "@/generated-types/GameVariant";
import { installThirdPartyMod } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useInstallThirdPartyMod(
  variant: GameVariant,
  onSuccess?: () => void,
  onError?: (error: unknown) => void,
) {
  const queryClient = useQueryClient();

  const mutation = useMutation({
    mutationFn: (modId: string) => installThirdPartyMod(modId, variant),
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.mods.listAll(variant),
      });
      onSuccess?.();
    },
    onError,
  });

  return {
    isInstalling: mutation.isPending,
    install: (modId: string) => mutation.mutate(modId),
  };
}
