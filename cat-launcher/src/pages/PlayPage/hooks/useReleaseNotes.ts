import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import type { GameRelease } from "@/generated-types/GameRelease";
import { fetchReleaseNotes } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

/**
 * Custom hook to fetch and provide release notes for a specific game release.
 *
 * @param release - The game release object to fetch notes for.
 * @param onReleaseNotesError - Optional callback triggered if the notes fail to load.
 * @returns An object containing the release notes, loading state, and error state.
 */
export default function useReleaseNotes(
  release: GameRelease,
  onReleaseNotesError?: (error: Error) => void,
) {
  const onReleaseNotesErrorRef = useRef(onReleaseNotesError);

  useEffect(() => {
    onReleaseNotesErrorRef.current = onReleaseNotesError;
  }, [onReleaseNotesError]);

  const {
    data: notes,
    isLoading,
    error,
  } = useQuery({
    queryKey: queryKeys.releaseNotes(
      release.variant,
      release.version,
    ),
    queryFn: async () => {
      return await fetchReleaseNotes(
        release.variant,
        release.version,
      );
    },
    placeholderData: release.body,
  });

  useEffect(() => {
    if (error && onReleaseNotesErrorRef.current) {
      onReleaseNotesErrorRef.current(error as Error);
    }
  }, [error]);

  return { notes, isLoading, error };
}
