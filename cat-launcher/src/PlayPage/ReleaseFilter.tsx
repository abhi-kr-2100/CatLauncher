import { useMemo, useState } from "react";

import { Checkbox } from "@/components/ui/checkbox";
import { Label } from "@/components/ui/label";
import type { GameVariant } from "@/generated-types/GameVariant";
import type { ReleaseType } from "@/generated-types/ReleaseType";
import { GameRelease } from "@/generated-types/GameRelease";

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
  const defaultFilters: Filter[] = [
    {
      id: "Stable",
      label: "Stable",
      apply: (r) => r.release_type === "Stable",
    },

    {
      id: "Experimental",
      label: "Experimental",
      apply: (r) => r.release_type === "Experimental",
    },
  ];

  switch (variant) {
    case "DarkDaysAhead":
      return [
        ...defaultFilters,
        {
          id: "ReleaseCandidate",
          label: "Release Candidate",
          apply: (r) => r.release_type === "ReleaseCandidate",
        },
      ];
    default:
      return defaultFilters;
  }
}

export default function ReleaseFilter({
  variant,
  onChange,
}: ReleaseFilterProps) {
  const filters = useMemo<Filter[]>(() => getFilters(variant), [variant]);

  const [selectedFilterIds, setSelectedFilterIds] = useState<ReleaseType[]>(
    filters.map((f) => f.id), // default to all filters selected
  );

  function handleCheckedChange(checked: boolean, filterId: ReleaseType) {
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
      {filters.map((filter) => (
        <div key={filter.id} className="flex items-center space-x-2">
          <Checkbox
            id={filter.id}
            checked={selectedFilterIds.includes(filter.id)}
            onCheckedChange={(checked: boolean) =>
              handleCheckedChange(checked, filter.id)
            }
          />
          <Label htmlFor={filter.id} className="text-sm font-medium">
            {filter.label}
          </Label>
        </div>
      ))}
    </div>
  );
}
