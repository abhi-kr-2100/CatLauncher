import { useQuery } from "@tanstack/react-query";

import { queryKeys } from "@/lib/queryKeys";
import { getTips } from "@/lib/commands";
import type { GameVariant } from "@/generated-types/GameVariant";

export function useGetTips(variant: GameVariant) {
  return useQuery({
    queryKey: queryKeys.tips(variant),
    queryFn: async () => getTips(variant),
  });
}
