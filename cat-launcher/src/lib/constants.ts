import type { ReleaseType } from "@/generated-types/ReleaseType";

/**
 * The base URL for checking CatLauncher updates.
 */
export const UPDATE_LINK =
  "https://github.com/abhi-kr-2100/CatLauncher/releases/";

/**
 * The interval in milliseconds at which the "Tip of the Day" is automatically shuffled.
 */
export const TIP_OF_THE_DAY_AUTOSHUFFLE_INTERVAL_MS = 10 * 1000; // 10 seconds

/**
 * Human-friendly labels for different game release types.
 */
export const RELEASE_TYPE_LABELS: Record<ReleaseType, string> = {
  Stable: "Stable",
  ReleaseCandidate: "Release Candidate",
  Experimental: "Experimental",
};
