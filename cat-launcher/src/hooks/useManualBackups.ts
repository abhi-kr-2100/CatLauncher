import { useQuery } from "@tanstack/react-query";

import { GameVariant } from "@/generated-types/GameVariant";
import { listManualBackupsForVariant } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

/**
 * A custom hook that fetches the list of manual backups for a specific game variant.
 *
 * @param variant - The game variant to fetch manual backups for.
 * @returns An object containing the list of manual backups and the loading state.
 */
export function useManualBackups(variant: GameVariant) {
  const { data: manualBackups, isLoading } = useQuery({
    queryKey: queryKeys.manualBackups(variant),
    queryFn: () => listManualBackupsForVariant(variant),
  });

  return { manualBackups: manualBackups ?? [], isLoading };
}
