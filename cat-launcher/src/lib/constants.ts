import type { ReleaseType } from "@/generated-types/ReleaseType";

export const UPDATE_LINK =
  "https://github.com/abhi-kr-2100/CatLauncher/releases/";

export const TIP_OF_THE_DAY_AUTOSHUFFLE_INTERVAL_MS = 10 * 1000; // 10 seconds

export const RELEASE_TYPE_LABELS: Record<ReleaseType, string> = {
  Stable: "Stable",
  ReleaseCandidate: "Release Candidate",
  Experimental: "Experimental",
};

/**
 * UI strings that should be internationalized in the future.
 */
export const UI_STRINGS = {
  VARIANT_SELECTOR: {
    PLACEHOLDER: "Select a game variant",
    LOADING: "Loading...",
  },
  SEARCH_INPUT: {
    PLACEHOLDER: "Search...",
  },
} as const;
