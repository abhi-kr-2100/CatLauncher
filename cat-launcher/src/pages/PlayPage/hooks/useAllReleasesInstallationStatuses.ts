import { useQueries } from "@tanstack/react-query";
import { useMemo } from "react";

import type { GameReleaseStatus } from "@/generated-types/GameReleaseStatus";
import type { GameVariant } from "@/generated-types/GameVariant";
import { getInstallationStatus } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { useReleases } from "./useReleases";

export function useAllReleasesInstallationStatuses(
  variant: GameVariant,
) {
  const { releases } = useReleases(variant);

  const results = useQueries({
    queries:
      releases?.map((r) => ({
        queryKey: queryKeys.installationStatus(variant, r.version),
        queryFn: () => getInstallationStatus(variant, r.version),
        staleTime: 5 * 60 * 1000, // 5 minutes
        initialData: "Unknown",
      })) ?? [],
  });

  const statuses = useMemo(() => {
    return results.reduce(
      (acc, result, index) => {
        if (releases && releases[index]) {
          acc[releases[index].version] =
            result.data as GameReleaseStatus;
        }
        return acc;
      },
      {} as Record<string, GameReleaseStatus | undefined>,
    );
  }, [results, releases]);

  return statuses;
}
