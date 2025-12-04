import { useMutation, useQuery } from "@tanstack/react-query";
import { RefreshCw } from "lucide-react";
import { useCallback, useEffect, useMemo, useState } from "react";

import { Button } from "@/components/ui/button";
import {
  VirtualizedCombobox,
  type ComboboxItem,
} from "@/components/virtualized-combobox";
import type { GameRelease } from "@/generated-types/GameRelease";
import type { GameVariant } from "@/generated-types/GameVariant";
import {
  getInstallationStatus,
  getLastPlayedVersion,
  triggerFetchReleasesForVariant,
} from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { cn, toastCL } from "@/lib/utils";
import { useAppDispatch, useAppSelector } from "@/store/hooks";
import {
  FetchStatus,
  onFetchingReleasesFailed,
  setReleaseStatus,
  startFetchingReleases,
} from "@/store/releasesSlice";
import { useReleaseEvents } from "./hooks";
import ReleaseFilter, { FilterFn } from "./ReleaseFilter";
import ReleaseLabel from "./ReleaseLabel";

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
  const releasesFetchStatus = useAppSelector(
    (state) => state.releases.fetchStatusByVariant[variant],
  );

  const { mutate: triggerFetchReleases, isPending: isReleasesTriggerLoading } =
    useMutation({
      mutationFn: triggerFetchReleasesForVariant,
      onMutate: (variant: GameVariant) => {
        dispatch(startFetchingReleases({ variant }));
      },
      onError: (error: unknown, variant) => {
        dispatch(onFetchingReleasesFailed({ variant }));
        toastCL("error", `Failed to fetch releases for ${variant}.`, error);
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
    data: lastPlayedVersion,
    isLoading: isLastPlayedVersionLoading,
    error: lastPlayedVersionError,
  } = useQuery<string | undefined>({
    queryKey: queryKeys.lastPlayedVersion(variant),
    queryFn: () => getLastPlayedVersion(variant),
  });

  useQuery({
    queryKey: ["installationStatus", variant, selectedReleaseId],
    queryFn: () => {
      if (!selectedReleaseId) {
        return;
      }

      return getInstallationStatus(variant, selectedReleaseId);
    },
    enabled: !!selectedReleaseId,
    onSuccess: (status) => {
      if (status && selectedReleaseId) {
        dispatch(
          setReleaseStatus({
            variant,
            version: selectedReleaseId,
            status,
          }),
        );
      }
    },
  });

  useEffect(() => {
    if (!lastPlayedVersionError) {
      return;
    }

    toastCL(
      "warning",
      `Failed to get last played version of ${variant}.`,
      lastPlayedVersionError,
    );
  }, [lastPlayedVersionError, variant]);

  const [appliedFilter, setAppliedFilter] = useState<FilterFn>(
    () => (_r: GameRelease) => true,
  );

  const comboboxItems = useMemo<ComboboxItem[]>(() => {
    return (
      releases.filter(appliedFilter).map((r) => {
        const isLastPlayed = r.version === lastPlayedVersion;

        return {
          value: r.version,
          label: (
            <ReleaseLabel
              variant={variant}
              version={r.version}
              isLastPlayed={isLastPlayed}
            />
          ),
        };
      }) ?? []
    );
  }, [releases, lastPlayedVersion, variant, appliedFilter]);

  useEffect(() => {
    // Selected release may become unavailable after filtering
    if (
      selectedReleaseId &&
      !comboboxItems.find((item) => item.value === selectedReleaseId)
    ) {
      setSelectedReleaseId(undefined);
    }
  }, [comboboxItems, selectedReleaseId, setSelectedReleaseId]);

  const autoselect = useCallback(
    (items: ComboboxItem[]) => {
      if (isLastPlayedVersionLoading) {
        return;
      }

      if (lastPlayedVersionError || lastPlayedVersion === "") {
        return items[0];
      }

      return items.find((i) => i.value === lastPlayedVersion) ?? items[0];
    },
    [lastPlayedVersion, isLastPlayedVersionLoading, lastPlayedVersionError],
  );

  // Consider isReleaseFetchingComplete even if it completes due to an error
  const isReleaseFetchingComplete =
    !isReleasesTriggerLoading && releasesFetchStatus !== FetchStatus.Fetching;

  // Even if isReleasesTriggerLoading is true, releases from previous release
  // events might be available. Consider isReleaseFetchingLoading only when
  // there are no items as well.
  const isReleaseFetchingLoading =
    isReleasesTriggerLoading && comboboxItems.length === 0;

  const placeholderText = isReleaseFetchingLoading
    ? "Loading..."
    : comboboxItems.length === 0
      ? "No releases available."
      : "Select a release";

  return (
    <div className="flex flex-col gap-2">
      <ReleaseFilter
        variant={variant}
        onChange={(filter) => setAppliedFilter((_prev: FilterFn) => filter)}
      />
      <div className="flex items-end gap-2">
        <div className="flex-grow">
          <VirtualizedCombobox
            label="Version"
            items={comboboxItems}
            value={selectedReleaseId}
            onChange={setSelectedReleaseId}
            autoselect={autoselect}
            placeholder={placeholderText}
            disabled={isReleaseFetchingLoading}
          />
        </div>
        <Button
          variant="outline"
          size="icon"
          onClick={() => triggerFetchReleases(variant)}
          disabled={!isReleaseFetchingComplete}
        >
          <RefreshCw
            className={cn(!isReleaseFetchingComplete && "animate-spin")}
            size={16}
          />
        </Button>
      </div>
    </div>
  );
}

interface ReleaseSelectorProps {
  variant: GameVariant;
  selectedReleaseId: string | undefined;
  setSelectedReleaseId: (value: string | undefined) => void;
}
