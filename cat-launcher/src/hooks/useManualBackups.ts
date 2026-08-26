import { useQuery } from "@tanstack/react-query";

import { GameVariant } from "@/generated-types/GameVariant";
import { listManualBackupsForVariant } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export function useManualBackups(variant: GameVariant) {
  const { data: manualBackups, isLoading } = useQuery({
    queryKey: queryKeys.manualBackups(variant),
    queryFn: () => listManualBackupsForVariant(variant),
  });

  return { manualBackups: manualBackups ?? [], isLoading };
}
