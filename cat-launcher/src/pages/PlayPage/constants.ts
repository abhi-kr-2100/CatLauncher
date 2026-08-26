import type { GameVariant } from "@/generated-types/GameVariant";
import type { QuickSelectKey } from "./hooks/useReleaseNotesRange";

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
