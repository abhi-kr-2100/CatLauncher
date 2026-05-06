import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import type { GameVariant } from "@/generated-types/GameVariant";
import { getActiveRelease } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

/**
 * Custom hook to fetch the currently active (installed and selected) release for a game variant.
 *
 * @param variant - The game variant.
 * @param onActiveReleaseError - Optional callback triggered if the active release fails to load.
 * @returns An object containing the active release ID, loading state, and error state.
 */
export function useActiveRelease(
  variant: GameVariant,
  onActiveReleaseError?: (error: unknown) => void,
) {
  const onActiveReleaseErrorRef = useRef(onActiveReleaseError);

  useEffect(() => {
    onActiveReleaseErrorRef.current = onActiveReleaseError;
  }, [onActiveReleaseError]);

  const {
    data: activeRelease,
    isLoading: isActiveReleaseLoading,
    error: activeReleaseError,
  } = useQuery<string | undefined>({
    queryKey: queryKeys.activeRelease(variant),
    queryFn: () => getActiveRelease(variant),
  });

  useEffect(() => {
    if (activeReleaseError && onActiveReleaseErrorRef.current) {
      onActiveReleaseErrorRef.current(activeReleaseError);
    }
  }, [activeReleaseError]);

  return {
    activeRelease,
    isActiveReleaseLoading,
    activeReleaseError,
  };
}
