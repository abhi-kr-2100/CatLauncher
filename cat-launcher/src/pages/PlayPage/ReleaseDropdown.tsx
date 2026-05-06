import { useCallback, useEffect, useMemo } from "react";

import {
  VirtualizedCombobox,
  type ComboboxItem,
} from "@/components/virtualized-combobox";
import type { GameRelease } from "@/generated-types/GameRelease";
import type { GameVariant } from "@/generated-types/GameVariant";
import { toastCL } from "@/lib/utils";
import { useAppSelector } from "@/store/hooks";
import { useActiveRelease, useReleases } from "./hooks";
import type { FilterFn } from "./ReleaseFilter";
import ReleaseLabel from "./ReleaseLabel";

/**
 * Props for the {@link ReleaseDropdown} component.
 */
export interface ReleaseDropdownProps {
  /** The game variant to list releases for. */
  variant: GameVariant;
  /** The currently selected release ID (version string). */
  selectedReleaseId: string | undefined;
  /** Callback to update the selected release ID. */
  setSelectedReleaseId: (value: string | undefined) => void;
  /** Optional filter function to restrict the list of visible releases. */
  appliedFilter?: FilterFn;
  /** Whether to hide the "Active" label next to the currently active version. */
  hideActiveLabel?: boolean;
}

/**
 * A searchable, virtualized dropdown component for selecting a specific game release.
 * It automatically handles loading releases, identifying the active release,
 * and managing installation statuses.
 *
 * @param props - The component props.
 * @returns A React element containing the release selection dropdown.
 */
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
    activeRelease,
    isActiveReleaseLoading,
    activeReleaseError,
  } = useActiveRelease(variant, (error) => {
    toastCL(
      "warning",
      `Failed to get active release of ${variant}.`,
      error,
    );
  });

  const latestRelease = useMemo(() => {
    return releases?.[0];
  }, [releases]);

  const comboboxItems = useMemo<ComboboxItem[]>(() => {
    return releases.filter(appliedFilter).map((r) => {
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
    });
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
