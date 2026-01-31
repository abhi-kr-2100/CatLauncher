import { useQueries } from "@tanstack/react-query";
import { useEffect, useMemo, useState } from "react";

import type { GameVariant } from "@/generated-types/GameVariant";
import { fetchReleaseNotes } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { useActiveRelease } from "./useActiveRelease";
import { useReleases } from "./useReleases";

export type QuickSelectKey =
  | "Active"
  | "Latest"
  | "Stable"
  | "ReleaseCandidate"
  | "Experimental";

export function useReleaseNotesRange(variant: GameVariant) {
  const { releases } = useReleases(variant);
  const { activeRelease } = useActiveRelease(variant);

  const [fromId, setFromId] = useState<string | undefined>();
  const [toId, setToId] = useState<string | undefined>();

  // Initialize selection
  useEffect(() => {
    if (activeRelease) {
      setFromId((fromId) => fromId ?? activeRelease);
    }

    if (releases.length > 0) {
      setToId((toId) => toId ?? releases[0].version); // latest
    }
  }, [activeRelease, releases]);

  const { versionsToShow, isReversed } = useMemo(() => {
    if (!fromId || !toId) {
      return { versionsToShow: [], isReversed: false };
    }

    const fromIndex = releases.findIndex((r) => r.version === fromId);
    const toIndex = releases.findIndex((r) => r.version === toId);

    if (fromIndex === -1 || toIndex === -1) {
      return { versionsToShow: [], isReversed: false };
    }

    if (toIndex > fromIndex) {
      return { versionsToShow: [], isReversed: true };
    }

    if (fromIndex === toIndex) {
      const release = releases[fromIndex];
      return {
        versionsToShow: release ? [release] : [],
        isReversed: false,
      };
    }

    return {
      versionsToShow: releases.slice(toIndex, fromIndex),
      isReversed: false,
    };
  }, [releases, fromId, toId]);

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
    if (versionsToShow.length === 0)
      return "No changes between selected versions.";

    // Single version case
    if (versionsToShow.length === 1 && fromId === toId) {
      const release = versionsToShow[0];
      const note = notesQueries[0]?.data;
      const body = note ?? release?.body ?? "No notes available.";
      return `# Version ${release.version}\n\n${body}`;
    }

    return versionsToShow
      .map((r, i) => {
        const note = notesQueries[i].data;
        const body = note ?? r.body ?? "No notes available.";
        return `# Version ${r.version}\n\n${body}`;
      })
      .join("\n\n---\n\n");
  }, [versionsToShow, notesQueries, isReversed, fromId, toId]);

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
    setFromId(toId);
    setToId(fromId);
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
  };
}
