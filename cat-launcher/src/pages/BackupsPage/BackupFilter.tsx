import { useState } from "react";

import { Checkbox } from "@/components/ui/checkbox";
import { Label } from "@/components/ui/label";
import { CombinedBackup } from "./types/backups";

export type BackupFilterFn = (backup: CombinedBackup) => boolean;

export type BackupFilterType = {
  id: "automatic" | "manual";
  label: string;
  apply: BackupFilterFn;
};

interface BackupFilterProps {
  onChange: (filterFn: BackupFilterFn) => void;
}

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
