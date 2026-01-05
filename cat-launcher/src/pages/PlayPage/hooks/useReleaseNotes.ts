import { useQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

import type { GameRelease } from "@/generated-types/GameRelease";
import { fetchReleaseNotes } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";

export default function useReleaseNotes(
  release: GameRelease | undefined,
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
    queryKey: release
      ? queryKeys.releaseNotes(release.variant, release.version)
      : ["release_notes", "disabled"],
    queryFn: async () => {
      if (!release) return null;
      return await fetchReleaseNotes(
        release.variant,
        release.version,
      );
    },
    enabled: !!release,
    placeholderData: release?.body,
  });

  useEffect(() => {
    if (error && onReleaseNotesErrorRef.current) {
      onReleaseNotesErrorRef.current(error as Error);
    }
  }, [error]);

  return { notes, isLoading, error };
}
