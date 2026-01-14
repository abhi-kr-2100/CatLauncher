import { useQuery } from "@tanstack/react-query";
import { useEffect, useMemo, useRef } from "react";

import type { GameRelease } from "@/generated-types/GameRelease";
import type { GameVariant } from "@/generated-types/GameVariant";
import type { ReleaseType } from "@/generated-types/ReleaseType";
import { getActiveRelease } from "@/lib/commands";
import { RELEASE_TYPE_LABELS } from "@/lib/constants";
import { queryKeys } from "@/lib/queryKeys";
import { useReleases } from "./useReleases";

const RELEASE_TYPE_ORDER: ReleaseType[] = [
  "Stable",
  "ReleaseCandidate",
  "Experimental",
];

export function useUpgradeInfo(
  variant: GameVariant,
  selectedReleaseId: string | undefined,
  setSelectedReleaseId: (id: string) => void,
  install: (id: string) => void,
  onActiveReleaseError?: (error: unknown) => void,
) {
  const onActiveReleaseErrorRef = useRef(onActiveReleaseError);

  useEffect(() => {
    onActiveReleaseErrorRef.current = onActiveReleaseError;
  }, [onActiveReleaseError]);

  const { releases } = useReleases(variant);
  const latestReleaseId = releases?.[0]?.version;

  const latestByReleaseType = useMemo(() => {
    // The Last Generation has Stable and Experimental releases, but Experimental releases are deprecated.
    if (variant === "TheLastGeneration" && releases.length > 0) {
      return {
        Stable: releases[0],
      };
    }

    const latest: Partial<Record<ReleaseType, GameRelease>> = {};
    for (const r of releases) {
      if (!latest[r.release_type]) {
        latest[r.release_type] = r;
      }
    }
    return latest;
  }, [releases, variant]);

  const supportedTypes = useMemo<ReleaseType[]>(() => {
    const presentTypes = Object.keys(
      latestByReleaseType,
    ) as ReleaseType[];
    return presentTypes.sort(
      (a, b) =>
        RELEASE_TYPE_ORDER.indexOf(a) - RELEASE_TYPE_ORDER.indexOf(b),
    );
  }, [latestByReleaseType]);

  const upgradeOptions = useMemo(() => {
    return supportedTypes
      .map((type) => {
        const latest = latestByReleaseType[type];
        if (!latest) return null;

        const isCurrent = latest.version === selectedReleaseId;

        return {
          id: type,
          label: `Latest ${RELEASE_TYPE_LABELS[type]}`,
          onClick: () => {
            setSelectedReleaseId(latest.version);
            install(latest.version);
          },
          disabled: isCurrent,
          tooltip: isCurrent ? "Already installed" : undefined,
        };
      })
      .filter((opt): opt is NonNullable<typeof opt> => opt !== null);
  }, [
    supportedTypes,
    latestByReleaseType,
    selectedReleaseId,
    setSelectedReleaseId,
    install,
  ]);

  const { data: activeRelease, error: activeReleaseError } = useQuery<
    string | undefined
  >({
    queryKey: queryKeys.activeRelease(variant),
    queryFn: () => getActiveRelease(variant),
  });

  useEffect(() => {
    if (activeReleaseError && onActiveReleaseErrorRef.current) {
      onActiveReleaseErrorRef.current(activeReleaseError);
    }
  }, [activeReleaseError]);

  const shouldAllowUpgrading = useMemo(() => {
    if (
      !selectedReleaseId ||
      !activeRelease ||
      selectedReleaseId !== activeRelease
    ) {
      return false;
    }
    // Allow if there is any supported release type that has a different version from current
    return supportedTypes.some((type) => {
      const latest = latestByReleaseType[type];
      return latest && latest.version !== selectedReleaseId;
    });
  }, [
    selectedReleaseId,
    activeRelease,
    supportedTypes,
    latestByReleaseType,
  ]);

  return {
    latestReleaseId,
    upgradeOptions,
    shouldAllowUpgrading,
  };
}
