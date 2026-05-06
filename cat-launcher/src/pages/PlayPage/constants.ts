import type { GameVariant } from "@/generated-types/GameVariant";
import type { QuickSelectKey } from "./hooks/useReleaseNotesRange";

/**
 * Defines the quick select buttons available for each game variant.
 * Each button has a label for display and a key used to identify the target version.
 */
export const QUICK_SELECT_BUTTONS: Record<
  GameVariant,
  { label: string; key: QuickSelectKey }[]
> = {
  DarkDaysAhead: [
    { label: "Active", key: "Active" },
    { label: "Latest Stable", key: "Stable" },
    { label: "Latest Release Candidate", key: "ReleaseCandidate" },
    { label: "Latest Experimental", key: "Experimental" },
  ],
  BrightNights: [
    { label: "Active", key: "Active" },
    { label: "Latest Stable", key: "Stable" },
    { label: "Latest Experimental", key: "Experimental" },
  ],
  TheLastGeneration: [
    { label: "Active", key: "Active" },
    { label: "Latest", key: "Latest" },
  ],
};
