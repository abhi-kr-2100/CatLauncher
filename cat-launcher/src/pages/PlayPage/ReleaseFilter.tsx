import { useMemo, useState } from "react";

import { Checkbox } from "@/components/ui/checkbox";
import { Label } from "@/components/ui/label";
import { GameRelease } from "@/generated-types/GameRelease";
import type { GameVariant } from "@/generated-types/GameVariant";
import type { ReleaseType } from "@/generated-types/ReleaseType";
import { useAppSelector } from "@/store/hooks";

export type FilterFn = (r: GameRelease) => boolean;

export type Filter = {
  id: ReleaseType;
  label: string;
  apply: FilterFn;
};

interface ReleaseFilterProps {
  variant: GameVariant;
  onChange: (filterFn: FilterFn) => void;
}

function getFilters(variant: GameVariant): Filter[] {
  const stableFilter = {
    id: "Stable",
    label: "Stable",
    apply: (r) => r.release_type === "Stable",
  } satisfies Filter;

  const experimentalFilter = {
    id: "Experimental",
    label: "Experimental",
    apply: (r) => r.release_type === "Experimental",
  } satisfies Filter;

  const releaseCandidateFilter = {
    id: "ReleaseCandidate",
    label: "Release Candidate",
    apply: (r) => r.release_type === "ReleaseCandidate",
  } satisfies Filter;

  switch (variant) {
    case "DarkDaysAhead":
      return [
        stableFilter,
        experimentalFilter,
        releaseCandidateFilter,
      ];
    case "TheLastGeneration":
      return [stableFilter];
    case "BrightNights":
      return [stableFilter, experimentalFilter];
  }
}

export default function ReleaseFilter({
  variant,
  onChange,
}: ReleaseFilterProps) {
  const filters = useMemo<Filter[]>(
    () => getFilters(variant),
    [variant],
  );

  const [selectedFilterIds, setSelectedFilterIds] = useState<
    ReleaseType[]
  >(
    filters.map((f) => f.id), // default to all filters selected
  );

  const installationStatuses = useAppSelector(
    (state) =>
      state.installationProgress.installationStatusByVariant.release[
        variant
      ],
  );

  const isInProgress = useMemo(() => {
    if (!installationStatuses) {
      return false;
    }
    return Object.values(installationStatuses).some(
      (status) => status === "Downloading" || status === "Installing",
    );
  }, [installationStatuses]);

  function handleCheckedChange(
    checked: boolean,
    filterId: ReleaseType,
  ) {
    const appliedFilterIds = new Set(selectedFilterIds);

    if (checked) {
      appliedFilterIds.add(filterId);
    } else {
      appliedFilterIds.delete(filterId);
    }

    const appliedFilters = Array.from(appliedFilterIds).map(
      (fid) => filters.find((f) => f.id === fid)!,
    );
    setSelectedFilterIds(appliedFilters.map((f) => f.id));

    const effectiveFilter: FilterFn = (r) => {
      return appliedFilters.some((f) => f.apply(r));
    };

    onChange(effectiveFilter);
  }

  return (
    <div className="flex items-center space-x-4">
      {filters.map((filter) => {
        const key = `${variant}-${filter.id}`;

        return (
          <div key={key} className="flex items-center space-x-2">
            <Checkbox
              id={key}
              checked={selectedFilterIds.includes(filter.id)}
              onCheckedChange={(checked: boolean) =>
                handleCheckedChange(checked, filter.id)
              }
              disabled={isInProgress}
            />
            <Label htmlFor={key} className="text-sm font-medium">
              {filter.label}
            </Label>
          </div>
        );
      })}
    </div>
  );
}
