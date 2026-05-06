import { useQuery } from "@tanstack/react-query";

import type { GameVariant } from "@/generated-types/GameVariant";
import { getThirdPartyModInstallationStatus } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

/**
 * A custom hook to fetch the installation status of a third-party mod.
 *
 * @param modId - The unique identifier of the mod.
 * @param variant - The game variant the mod belongs to.
 * @returns An object containing the installation status.
 */
export function useGetThirdPartyModInstallationStatus(
  modId: string,
  variant: GameVariant,
) {
  const query = useQuery({
    queryKey: queryKeys.mods.installationStatus(variant, modId),
    queryFn: () => getThirdPartyModInstallationStatus(modId, variant),
  });

  return {
    installationStatus: query.data,
  };
}
