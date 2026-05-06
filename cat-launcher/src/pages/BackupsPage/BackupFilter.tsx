import { useState } from "react";

import { Checkbox } from "@/components/ui/checkbox";
import { Label } from "@/components/ui/label";
import { CombinedBackup } from "./types/backups";

/**
 * A function that takes a backup and returns true if it should be included in the filtered results.
 */
export type BackupFilterFn = (backup: CombinedBackup) => boolean;

/**
 * Defines a filter type with its display label and logic.
 */
export type BackupFilterType = {
  /** Unique identifier for the filter. */
  id: "automatic" | "manual";
  /** Human-readable label for the filter. */
  label: string;
  /** The function to apply when this filter is active. */
  apply: BackupFilterFn;
};

/**
 * Props for the {@link BackupFilter} component.
 */
interface BackupFilterProps {
  /** Callback triggered when the active filter changes. */
  onChange: (filterFn: BackupFilterFn) => void;
}

/**
 * Predefined backup filters for automatic and manual backups.
 */
const FILTERS: BackupFilterType[] = [
  {
    id: "automatic",
    label: "Automatic",
    apply: (backup) => backup.type === "automatic",
  },
  {
    id: "manual",
    label: "Manual",
    apply: (backup) => backup.type === "manual",
  },
];

/**
 * Component that provides checkboxes to filter backups by their type (Automatic/Manual).
 *
 * @param props - Component properties.
 * @returns A React element containing filter controls.
 */
export default function BackupFilter({
  onChange,
}: BackupFilterProps) {
  const [selectedFilterIds, setSelectedFilterIds] = useState<
    ("automatic" | "manual")[]
  >(FILTERS.map((f) => f.id)); // default to all filters selected

  function handleCheckedChange(
    checked: boolean,
    filterId: "automatic" | "manual",
  ) {
    const appliedFilterIds = new Set(selectedFilterIds);

    if (checked) {
      appliedFilterIds.add(filterId);
    } else {
      appliedFilterIds.delete(filterId);
    }

    const appliedFilters = Array.from(appliedFilterIds).map(
      (fid) => FILTERS.find((f) => f.id === fid)!,
    );
    setSelectedFilterIds(appliedFilters.map((f) => f.id));

    const effectiveFilter: BackupFilterFn = (backup) => {
      // If no filters are selected, show nothing
      if (appliedFilters.length === 0) {
        return false;
      }
      return appliedFilters.some((f) => f.apply(backup));
    };

    onChange(effectiveFilter);
  }

  return (
    <div className="flex items-center space-x-4">
      {FILTERS.map((filter) => {
        const key = `backup-filter-${filter.id}`;

        return (
          <div key={key} className="flex items-center space-x-2">
            <Checkbox
              id={key}
              checked={selectedFilterIds.includes(filter.id)}
              onCheckedChange={(checked: boolean) =>
                handleCheckedChange(checked, filter.id)
              }
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
