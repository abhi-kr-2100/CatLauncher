import { Button } from "@/components/ui/button";
import type { GameVariant } from "@/generated-types/GameVariant";
import { QUICK_SELECT_BUTTONS } from "./constants";
import type { QuickSelectKey } from "./hooks/useReleaseNotesRange";

interface QuickSelectButtonsProps {
  variant: GameVariant;
  targetVersions: Partial<Record<QuickSelectKey, string>>;
  onSelect: (version: string) => void;
}

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
