import { useQuery } from "@tanstack/react-query";
import { useCallback, useEffect, useMemo } from "react";

import {
  VirtualizedCombobox,
  type ComboboxItem,
} from "@/components/virtualized-combobox";
import type { GameRelease } from "@/generated-types/GameRelease";
import type { GameVariant } from "@/generated-types/GameVariant";
import { getActiveRelease } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { toastCL } from "@/lib/utils";
import { useAppSelector } from "@/store/hooks";
import { useReleases } from "./hooks";
import ReleaseLabel from "./ReleaseLabel";
import type { FilterFn } from "./ReleaseFilter";

export interface ReleaseDropdownProps {
  variant: GameVariant;
  selectedReleaseId: string | undefined;
  setSelectedReleaseId: (value: string | undefined) => void;
  appliedFilter?: FilterFn;
  hideActiveLabel?: boolean;
}

export default function ReleaseDropdown({
  variant,
  selectedReleaseId,
  setSelectedReleaseId,
  appliedFilter = (_r: GameRelease) => true,
  hideActiveLabel = false,
}: ReleaseDropdownProps) {
  const { releases, isLoading: isReleasesLoading } = useReleases(
    variant,
    (error) =>
      toastCL(
        "error",
        `Failed to load releases for ${variant}.`,
        error,
      ),
    (error) =>
      toastCL(
        "error",
        `Failed to fetch releases for ${variant}.`,
        error,
      ),
  );

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

  const latestRelease = useMemo(() => {
    return releases?.[0];
  }, [releases]);

  const comboboxItems = useMemo<ComboboxItem[]>(() => {
    return (
      releases.filter(appliedFilter).map((r) => {
        const isActive =
          !hideActiveLabel && r.version === activeRelease;

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
  }, [
    releases,
    activeRelease,
    variant,
    appliedFilter,
    hideActiveLabel,
  ]);

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

  const isInstalling = Object.values(
    installationStatusByVersion ?? {},
  ).some(
    (status) => status === "Downloading" || status === "Installing",
  );

  const placeholderText = isReleasesLoading
    ? "Loading..."
    : comboboxItems.length === 0
      ? "No releases available."
      : "Select a release";

  return (
    <VirtualizedCombobox
      label="Version"
      items={comboboxItems}
      value={selectedReleaseId}
      onChange={setSelectedReleaseId}
      autoselect={autoselect}
      placeholder={placeholderText}
      disabled={isReleasesLoading || isInstalling}
    />
  );
}
