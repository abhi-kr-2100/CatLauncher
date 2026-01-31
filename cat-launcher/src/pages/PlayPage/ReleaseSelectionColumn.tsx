import type { GameVariant } from "@/generated-types/GameVariant";
import type { QuickSelectKey } from "./hooks/useReleaseNotesRange";
import QuickSelectButtons from "./QuickSelectButtons";
import ReleaseDropdown from "./ReleaseDropdown";

interface ReleaseSelectionColumnProps {
  label: string;
  variant: GameVariant;
  selectedReleaseId: string | undefined;
  onSelect: (version: string | undefined) => void;
  targetVersions: Partial<Record<QuickSelectKey, string>>;
}

export default function ReleaseSelectionColumn({
  label,
  variant,
  selectedReleaseId,
  onSelect,
  targetVersions,
}: ReleaseSelectionColumnProps) {
  return (
    <div className="flex-1 flex flex-col gap-1">
      <span className="text-sm font-medium">{label}</span>
      <ReleaseDropdown
        variant={variant}
        selectedReleaseId={selectedReleaseId}
        setSelectedReleaseId={onSelect}
        hideActiveLabel
      />
      <QuickSelectButtons
        variant={variant}
        targetVersions={targetVersions}
        onSelect={(v) => onSelect(v)}
      />
    </div>
  );
}
