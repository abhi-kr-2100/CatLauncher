import { useMutation, useQueryClient } from "@tanstack/react-query";

import type { GameVariant } from "@/generated-types/GameVariant";
import { uninstallThirdPartyMod } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

/**
 * A custom hook to handle the uninstallation of a third-party mod.
 *
 * @param variant - The game variant associated with the mod.
 * @param onSuccess - An optional callback function to execute on successful uninstallation.
 * @param onError - An optional callback function to handle errors.
 * @returns An object containing the uninstallation function and pending state.
 */
export function useUninstallThirdPartyMod(
  variant: GameVariant,
  onSuccess?: () => void,
  onError?: (error: unknown) => void,
) {
  const queryClient = useQueryClient();

  const mutation = useMutation({
    mutationFn: (modId: string) =>
      uninstallThirdPartyMod(modId, variant),
    onSuccess: (_data, modId) => {
      queryClient.invalidateQueries({
        queryKey: queryKeys.mods.installationStatus(variant, modId),
      });
      onSuccess?.();
    },
    onError,
  });

  return {
    isUninstalling: mutation.isPending,
    uninstall: (modId: string) => mutation.mutate(modId),
  };
}
