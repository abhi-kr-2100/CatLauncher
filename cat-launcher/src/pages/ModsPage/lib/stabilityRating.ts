export interface StabilityRating {
  score: number;

  level: "low" | "medium" | "high";
}

const calculateScore = (ageInDays: number): number => {
  const maxAge = 365;
  return Math.max(0, Math.round(100 * (1 - ageInDays / maxAge)));
};

const getLevel = (score: number): "low" | "medium" | "high" => {
  if (score >= 70) return "high";
  if (score >= 40) return "medium";
  return "low";
};

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
