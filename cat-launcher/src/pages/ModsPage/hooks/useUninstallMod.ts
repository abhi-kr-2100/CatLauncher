import { useMutation, useQueryClient } from "@tanstack/react-query";

import { queryKeys } from "@/lib/queryKeys";
import { uninstallModForVariant } from "@/lib/commands";
import type { GameVariant } from "@/generated-types/GameVariant";

interface UseUninstallModOptions {
  onError?: (error: unknown) => void;
  onSuccess?: () => void;
}

export function useUninstallMod({
  onError,
  onSuccess,
}: UseUninstallModOptions = {}) {
  const queryClient = useQueryClient();

  const { mutate: uninstallModMutation, isPending: isUninstalling } =
    useMutation({
      mutationFn: async ({
        variant,
        modId,
      }: {
        variant: GameVariant;
        modId: string;
      }) => {
        await uninstallModForVariant(variant, modId);
      },
      onSuccess: (_, { variant, modId }) => {
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

  const uninstallModFn = (variant: GameVariant, modId: string) => {
    uninstallModMutation({ variant, modId });
  };

  return {
    isUninstalling,
    uninstallMod: uninstallModFn,
  };
}
