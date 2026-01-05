import { useMutation, useQuery } from "@tanstack/react-query";
import { useCallback, useEffect, useMemo, useState } from "react";

import {
  VirtualizedCombobox,
  type ComboboxItem,
} from "@/components/virtualized-combobox";
import type { GameRelease } from "@/generated-types/GameRelease";
import type { GameVariant } from "@/generated-types/GameVariant";
import {
  getActiveRelease,
  triggerFetchReleasesForVariant,
} from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { toastCL } from "@/lib/utils";
import { useAppDispatch, useAppSelector } from "@/store/hooks";
import {
  onFetchingReleasesFailed,
  startFetchingReleases,
} from "@/store/releasesSlice";
import { useReleaseEvents } from "./hooks";
import ReleaseFilter, { FilterFn } from "./ReleaseFilter";
import ReleaseLabel from "./ReleaseLabel";
import ReleaseNotesButton from "./ReleaseNotesButton";

export default function ReleaseSelector({
  variant,
  selectedReleaseId,
  setSelectedReleaseId,
}: ReleaseSelectorProps) {
  useReleaseEvents();

  const dispatch = useAppDispatch();

  const releases = useAppSelector(
    (state) => state.releases.releasesByVariant[variant],
  );

  const selectedRelease = useMemo(() => {
    return releases.find((r) => r.version === selectedReleaseId);
  }, [releases, selectedReleaseId]);

  const {
    mutate: triggerFetchReleases,
    isPending: isReleasesTriggerLoading,
  } = useMutation({
    mutationFn: triggerFetchReleasesForVariant,
    onMutate: (variant: GameVariant) => {
      dispatch(startFetchingReleases({ variant }));
    },
    onError: (error: unknown, variant) => {
      dispatch(onFetchingReleasesFailed({ variant }));
      toastCL(
        "error",
        `Failed to fetch releases for ${variant}.`,
        error,
      );
    },
  });

  useEffect(() => {
    // Fetch only if we don't have releases yet. Don't trigger fetch even if
    // the fetch failed previously as long as we have releases.
    // Fetching on failed previous fetches can lead to an infinite fetching cycle.
    const shouldFetch = releases.length === 0;

    if (shouldFetch) {
      triggerFetchReleases(variant);
    }
  }, [variant, triggerFetchReleases, releases.length]);

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

  const [appliedFilter, setAppliedFilter] = useState<FilterFn>(
    () => (_r: GameRelease) => true,
  );

  const latestRelease = useMemo(() => {
    return releases?.[0];
  }, [releases]);

  const comboboxItems = useMemo<ComboboxItem[]>(() => {
    return (
      releases.filter(appliedFilter).map((r) => {
        const isActive = r.version === activeRelease;

        return {
          value: r.version,
          label: (
            <ReleaseLabel
              variant={variant}
              version={r.version}
              isActive={isActive}
            />
          ),
        };
      }) ?? []
    );
  }, [releases, activeRelease, variant, appliedFilter]);

  useEffect(() => {
    // Selected release may become unavailable after filtering
    if (
      selectedReleaseId &&
      !comboboxItems.find((item) => item.value === selectedReleaseId)
    ) {
      setSelectedReleaseId(undefined);
    }
  }, [comboboxItems, selectedReleaseId, setSelectedReleaseId]);

  const installationStatusByVersion = useAppSelector(
    (state) =>
      state.installationProgress.installationStatusByVariant.release[
        variant
      ],
  );

  const autoselect = useCallback(
    (items: ComboboxItem[]) => {
      const installingOrDownloadingRelease = items.find((item) => {
        const status = installationStatusByVersion?.[item.value];
        return status === "Downloading" || status === "Installing";
      });

      if (installingOrDownloadingRelease) {
        return installingOrDownloadingRelease;
      }

      if (isActiveReleaseLoading) {
        return;
      }

      if (activeReleaseError || activeRelease === "") {
        return (
          items.find((i) => i.value === latestRelease?.version) ??
          items?.[0]
        );
      }

      return (
        items.find((i) => i.value === activeRelease) ?? items?.[0]
      );
    },
    [
      activeRelease,
      isActiveReleaseLoading,
      activeReleaseError,
      installationStatusByVersion,
      latestRelease,
    ],
  );

  // Even if isReleasesTriggerLoading is true, releases from previous release
  // events might be available. Consider isReleaseFetchingLoading only when
  // there are no items as well.
  const isReleaseFetchingLoading =
    isReleasesTriggerLoading && comboboxItems.length === 0;

  const isInstalling = Object.values(
    installationStatusByVersion ?? {},
  ).some(
    (status) => status === "Downloading" || status === "Installing",
  );

  const placeholderText = isReleaseFetchingLoading
    ? "Loading..."
    : comboboxItems.length === 0
      ? "No releases available."
      : "Select a release";

  return (
    <div className="flex flex-col gap-2">
      <ReleaseFilter
        variant={variant}
        onChange={(filter) =>
          setAppliedFilter((_prev: FilterFn) => filter)
        }
      />
      <div className="flex items-end gap-2">
        <div className="grow">
          <VirtualizedCombobox
            label="Version"
            items={comboboxItems}
            value={selectedReleaseId}
            onChange={setSelectedReleaseId}
            autoselect={autoselect}
            placeholder={placeholderText}
            disabled={isReleaseFetchingLoading || isInstalling}
          />
        </div>
        {selectedRelease && (
          <ReleaseNotesButton release={selectedRelease} />
        )}
      </div>
    </div>
  );
}

interface ReleaseSelectorProps {
  variant: GameVariant;
  selectedReleaseId: string | undefined;
  setSelectedReleaseId: (value: string | undefined) => void;
}
