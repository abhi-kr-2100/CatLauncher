import { useQuery } from "@tanstack/react-query";
import { useEffect } from "react";

import type { GameVariant } from "@/generated-types/GameVariant";
import { getActiveRelease } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { toastCL } from "@/lib/utils";

export function useActiveRelease(variant: GameVariant) {
  const {
    data: activeRelease,
    isLoading: isActiveReleaseLoading,
    error: activeReleaseError,
  } = useQuery<string | undefined>({
    queryKey: queryKeys.activeRelease(variant),
    queryFn: () => getActiveRelease(variant),
  });

  useEffect(() => {
    if (!activeReleaseError) {
      return;
    }

    toastCL(
      "warning",
      `Failed to get active release of ${variant}.`,
      activeReleaseError,
    );
  }, [activeReleaseError, variant]);

  return {
    activeRelease,
    isActiveReleaseLoading,
    activeReleaseError,
  };
}
