import { RefreshCw } from "lucide-react";
import { useCallback, useEffect, useMemo, useState } from "react";

import { Button } from "@/components/ui/button";
import {
  VirtualizedCombobox,
  type ComboboxItem,
} from "@/components/virtualized-combobox";
import type { GameRelease } from "@/generated-types/GameRelease";
import type { GameVariant } from "@/generated-types/GameVariant";
import { cn } from "@/lib/utils";
import { useAppSelector } from "@/store/hooks";
import { FetchStatus } from "@/store/releasesSlice";
import { useActiveRelease } from "../hooks/useActiveRelease";
import { useReleaseEvents } from "../hooks/useReleaseEvents";
import { useTriggerFetchReleases } from "../hooks/useTriggerFetchReleases";
import ReleaseFilter, { FilterFn } from "./ReleaseFilter";
import ReleaseLabel from "./ReleaseLabel";

export default function ReleaseSelector({
  variant,
  selectedReleaseId,
  setSelectedReleaseId,
}: ReleaseSelectorProps) {
  useReleaseEvents();

  const releases = useAppSelector(
    (state) => state.releases.releasesByVariant[variant],
  );
  const releasesFetchStatus = useAppSelector(
    (state) => state.releases.fetchStatusByVariant[variant],
  );

  const { triggerFetchReleases, isReleasesTriggerLoading } =
    useTriggerFetchReleases();

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
    activeRelease,
    isActiveReleaseLoading,
    activeReleaseError,
  } = useActiveRelease(variant);

  const [appliedFilter, setAppliedFilter] = useState<FilterFn>(
    () => (_r: GameRelease) => true,
  );

  const comboboxItems = useMemo<ComboboxItem[]>(() => {
    const latestRelease = releases.reduce(
      (prev: undefined | GameRelease, curr) => {
        if (!prev) {
          return curr;
        }

        if (curr.created_at > prev.created_at) {
          return curr;
        }

        return prev;
      },
      undefined,
    );

    return (
      releases.filter(appliedFilter).map((r) => {
        const isActive = r.version === activeRelease;
        const isLatest = r.version === latestRelease?.version;

        return {
          value: r.version,
          label: (
            <ReleaseLabel
              variant={variant}
              version={r.version}
              isActive={isActive}
              isLatest={isLatest}
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
        return items[0];
      }

      return items.find((i) => i.value === activeRelease) ?? items[0];
    },
    [
      activeRelease,
      isActiveReleaseLoading,
      activeReleaseError,
      installationStatusByVersion,
    ],
  );

  // Consider isReleaseFetchingComplete even if it completes due to an error
  const isReleaseFetchingComplete =
    !isReleasesTriggerLoading &&
    releasesFetchStatus !== FetchStatus.Fetching;

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
        <Button
          variant="outline"
          size="icon"
          onClick={() => triggerFetchReleases(variant)}
          disabled={!isReleaseFetchingComplete || isInstalling}
        >
          <RefreshCw
            className={cn(
              !isReleaseFetchingComplete && "animate-spin",
            )}
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
