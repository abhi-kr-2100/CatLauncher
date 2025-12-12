import { useMutation, useQueryClient } from "@tanstack/react-query";

import { queryKeys } from "@/lib/queryKeys";
import { installMod } from "@/lib/commands";
import type { GameVariant } from "@/generated-types/GameVariant";

interface UseInstallModOptions {
  onError?: (error: unknown) => void;
  onSuccess?: () => void;
}

export function useInstallMod({ onError, onSuccess }: UseInstallModOptions = {}) {
  const queryClient = useQueryClient();

  const { mutate: installModMutation, isPending: isInstalling } = useMutation({
    mutationFn: async ({
      variant,
      modId,
    }: {
      variant: GameVariant;
      modId: string;
    }) => {
      await installMod(variant, modId);
    },
    onSuccess: (_, { variant, modId }) => {
      // Invalidate queries to refresh mod list and status
      queryClient.invalidateQueries({
        queryKey: queryKeys.mods(variant),
      });
      queryClient.invalidateQueries({
        queryKey: queryKeys.modInstallationStatus(variant, modId),
      });

      onSuccess?.();
    },
    onError,
  });

  const installModFn = (variant: GameVariant, modId: string) => {
    installModMutation({ variant, modId });
  };

  return {
    isInstalling,
    installMod: installModFn,
  };
}
