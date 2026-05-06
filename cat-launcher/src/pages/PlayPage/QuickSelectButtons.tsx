import { Button } from "@/components/ui/button";
import type { GameVariant } from "@/generated-types/GameVariant";
import { QUICK_SELECT_BUTTONS } from "./constants";
import type { QuickSelectKey } from "./hooks/useReleaseNotesRange";

/**
 * Props for the {@link QuickSelectButtons} component.
 */
interface QuickSelectButtonsProps {
  /** The game variant for which to show quick select buttons. */
  variant: GameVariant;
  /** A record of version strings mapped to quick select keys (e.g., 'latest', 'active'). */
  targetVersions: Partial<Record<QuickSelectKey, string>>;
  /** Callback triggered when a version is selected via a button click. */
  onSelect: (version: string) => void;
}

/**
 * Renders a grid of buttons for quickly selecting specific game versions
 * (e.g., Latest Stable, Latest Experimental, Active Version) based on the game variant.
 *
 * @param props - The component props.
 * @returns A React element containing the quick select buttons.
 */
export default function QuickSelectButtons({
  variant,
  targetVersions,
  onSelect,
}: QuickSelectButtonsProps) {
  return (
    <div className="grid grid-cols-2 gap-2 mt-1">
      {QUICK_SELECT_BUTTONS[variant].map((btn) => {
        const version = targetVersions[btn.key];
        return (
          <Button
            key={btn.key}
            variant="secondary"
            size="sm"
            className="h-7 text-xs px-2"
            disabled={!version}
            onClick={() => version && onSelect(version)}
          >
            {btn.label}
          </Button>
        );
      })}
    </div>
  );
}
