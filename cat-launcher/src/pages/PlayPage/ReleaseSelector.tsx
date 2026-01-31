import { useMemo, useState } from "react";

import type { GameRelease } from "@/generated-types/GameRelease";
import type { GameVariant } from "@/generated-types/GameVariant";
import { useReleases } from "./hooks";
import ReleaseDropdown from "./ReleaseDropdown";
import ReleaseFilter, { FilterFn } from "./ReleaseFilter";
import ReleaseNotesButton from "./ReleaseNotesButton";

export default function ReleaseSelector({
  variant,
  selectedReleaseId,
  setSelectedReleaseId,
}: ReleaseSelectorProps) {
  const { releases } = useReleases(variant);

  const selectedRelease = useMemo(() => {
    return releases.find((r) => r.version === selectedReleaseId);
  }, [releases, selectedReleaseId]);

  const [appliedFilter, setAppliedFilter] = useState<FilterFn>(
    () => (_r: GameRelease) => true,
  );

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
          <ReleaseDropdown
            variant={variant}
            selectedReleaseId={selectedReleaseId}
            setSelectedReleaseId={setSelectedReleaseId}
            appliedFilter={appliedFilter}
          />
        </div>
        {selectedRelease && <ReleaseNotesButton variant={variant} />}
      </div>
    </div>
  );
}

interface ReleaseSelectorProps {
  variant: GameVariant;
  selectedReleaseId: string | undefined;
  setSelectedReleaseId: (value: string | undefined) => void;
}
