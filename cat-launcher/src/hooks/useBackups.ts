import { useQuery } from "@tanstack/react-query";

import { GameVariant } from "@/generated-types/GameVariant";
import { listBackupsForVariant } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

/**
 * A custom hook that fetches the list of automatic backups for a specific game variant.
 *
 * @param variant - The game variant to fetch backups for.
 * @returns An object containing the list of backups, loading state, and error information.
 */
export function useBackups(variant: GameVariant) {
  const {
    data: backups = [],
    isLoading,
    isError,
    error,
  } = useQuery({
    queryKey: queryKeys.backups(variant),
    queryFn: () => listBackupsForVariant(variant),
  });

  return {
    backups,
    isLoading,
    isError,
    error,
  };
}
