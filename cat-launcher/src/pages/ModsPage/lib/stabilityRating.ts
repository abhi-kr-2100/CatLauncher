/**
 * Represents the stability rating of a mod.
 */
export interface StabilityRating {
  /**
   * The numerical stability score from 0 to 100.
   */
  score: number;
  /**
   * The qualitative stability level.
   */
  level: "low" | "medium" | "high";
}

/**
 * Calculates a stability score based on the age of the last update.
 *
 * @param ageInDays - The number of days since the last update.
 * @returns A score between 0 and 100.
 */
const calculateScore = (ageInDays: number): number => {
  // Linear decay from 100 (0 days) to 0 (365+ days)
  const maxAge = 365;
  return Math.max(0, Math.round(100 * (1 - ageInDays / maxAge)));
};

/**
 * Maps a numerical score to a stability level.
 *
 * @param score - The stability score.
 * @returns The stability level.
 */
const getLevel = (score: number): "low" | "medium" | "high" => {
  if (score >= 70) return "high";
  if (score >= 40) return "medium";
  return "low";
};

/**
 * Computes the stability rating for a given timestamp.
 *
 * @param timestamp - The last update timestamp as a bigint.
 * @returns The calculated stability rating.
 */
export const getStabilityRating = (
  timestamp: bigint,
): StabilityRating => {
  const lastUpdateTime = Number(timestamp);
  const now = Date.now();
  const ageInDays = (now - lastUpdateTime) / (1000 * 60 * 60 * 24);

  const score = calculateScore(ageInDays);
  const level = getLevel(score);

  return { score, level };
};

/**
 * Returns a human-readable label for a stability level.
 *
 * @param level - The stability level.
 * @returns A capitalized label string.
 */
export const getStabilityLevelLabel = (
  level: "low" | "medium" | "high",
): string => {
  const labels: Record<"low" | "medium" | "high", string> = {
    high: "High",
    medium: "Medium",
    low: "Low",
  };
  return labels[level];
};
