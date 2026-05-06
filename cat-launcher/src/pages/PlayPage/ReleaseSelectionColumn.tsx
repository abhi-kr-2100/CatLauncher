import type { GameVariant } from "@/generated-types/GameVariant";
import type { QuickSelectKey } from "./hooks/useReleaseNotesRange";
import QuickSelectButtons from "./QuickSelectButtons";
import ReleaseDropdown from "./ReleaseDropdown";

/**
 * Props for the {@link ReleaseSelectionColumn} component.
 */
interface ReleaseSelectionColumnProps {
  /** The label for the selection column (e.g., "From", "To"). */
  label: string;
  /** The game variant for which releases are being selected. */
  variant: GameVariant;
  /** The currently selected release ID. */
  selectedReleaseId: string | undefined;
  /** Callback triggered when a release is selected. */
  onSelect: (version: string | undefined) => void;
  /** A record of version strings mapped to quick select keys. */
  targetVersions: Partial<Record<QuickSelectKey, string>>;
}

/**
 * A layout component that groups a label, a release dropdown, and quick select buttons
 * into a single column for selecting a version.
 *
 * @param props - The component props.
 * @returns A React element representing the selection column.
 */
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
