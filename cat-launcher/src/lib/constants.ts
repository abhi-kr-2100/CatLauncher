import type { ReleaseType } from "@/generated-types/ReleaseType";

export const UPDATE_LINK =
  "https://github.com/abhi-kr-2100/CatLauncher/releases/";

export const TIP_OF_THE_DAY_AUTOSHUFFLE_INTERVAL_MS = 10 * 1000; // 10 seconds

export const RELEASE_TYPE_LABELS: Record<ReleaseType, string> = {
  Stable: "Stable",
  ReleaseCandidate: "Release Candidate",
  Experimental: "Experimental",
};
