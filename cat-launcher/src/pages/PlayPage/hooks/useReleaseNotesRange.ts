import { useQueries, useQuery } from "@tanstack/react-query";
import { useMemo, useState, useEffect } from "react";
import type { GameVariant } from "@/generated-types/GameVariant";
import { fetchReleaseNotes, getActiveRelease } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { useReleases } from "./useReleases";

export type QuickSelectKey =
  | "Active"
  | "Latest"
  | "Stable"
  | "ReleaseCandidate"
  | "Experimental";

export function useReleaseNotesRange(
  variant: GameVariant,
  initialTo?: string,
) {
  const { releases } = useReleases(variant);

  const { data: activeRelease } = useQuery({
    queryKey: queryKeys.activeRelease(variant),
    queryFn: () => getActiveRelease(variant),
  });

  const [fromId, setFromId] = useState<string | undefined>();
  const [toId, setToId] = useState<string | undefined>();

  useEffect(() => {
    if (!fromId && activeRelease) {
      setFromId(activeRelease);
    }
    if (!toId && initialTo) {
      setToId(initialTo);
    } else if (!toId && releases.length > 0) {
      setToId(releases[0].version);
    }
  }, [activeRelease, initialTo, releases, fromId, toId]);

  const fromIndex = useMemo(
    () => releases.findIndex((r) => r.version === fromId),
    [releases, fromId],
  );
  const toIndex = useMemo(
    () => releases.findIndex((r) => r.version === toId),
    [releases, toId],
  );

  const isReversed =
    fromId &&
    toId &&
    fromIndex !== -1 &&
    toIndex !== -1 &&
    toIndex > fromIndex;

  const versionsToShow = useMemo(() => {
    if (!fromId || !toId || fromIndex === -1 || toIndex === -1)
      return [];

    if (fromId === toId) {
      const release = releases[fromIndex];
      return release ? [release] : [];
    }

    if (isReversed) return [];

    return releases.slice(toIndex, fromIndex);
  }, [releases, fromId, toId, fromIndex, toIndex, isReversed]);

  const notesQueries = useQueries({
    queries: versionsToShow.map((r) => ({
      queryKey: queryKeys.releaseNotes(variant, r.version),
      queryFn: () => fetchReleaseNotes(variant, r.version),
      staleTime: Infinity,
    })),
  });

  const isLoading = notesQueries.some((q) => q.isLoading);

  const combinedNotes = useMemo(() => {
    if (isReversed) return null;
    if (fromId && toId && fromId === toId) {
      const release = releases[fromIndex];
      const note = notesQueries[0]?.data;
      const body = note ?? release?.body ?? "No notes available.";
      return `# Version ${fromId}\n\n${body}`;
    }
    if (versionsToShow.length === 0) {
      return "No changes between selected versions.";
    }
    return versionsToShow
      .map((r, i) => {
        const note = notesQueries[i].data;
        const body = note ?? r.body ?? "No notes available.";
        return `# Version ${r.version}\n\n${body}`;
      })
      .join("\n\n---\n\n");
  }, [
    versionsToShow,
    notesQueries,
    isReversed,
    fromId,
    toId,
    releases,
    fromIndex,
  ]);

  const targetVersions = useMemo(() => {
    return {
      Active: activeRelease,
      Latest: releases[0]?.version,
      Stable: releases.find((r) => r.release_type === "Stable")
        ?.version,
      ReleaseCandidate: releases.find(
        (r) => r.release_type === "ReleaseCandidate",
      )?.version,
      Experimental: releases.find(
        (r) => r.release_type === "Experimental",
      )?.version,
    };
  }, [releases, activeRelease]);

  const handleSwap = () => {
    const temp = fromId;
    setFromId(toId);
    setToId(temp);
  };

  return {
    fromId,
    setFromId,
    toId,
    setToId,
    combinedNotes,
    isLoading,
    isReversed,
    handleSwap,
    targetVersions,
    releases,
  };
}
