export interface RelativeTime {
  value: number;
  unit:
    | "seconds"
    | "minutes"
    | "hours"
    | "days"
    | "weeks"
    | "months"
    | "years";
}

export const calculateRelativeTime = (
  timestamp: bigint,
): RelativeTime => {
  const lastUpdateTime = Number(timestamp);
  const now = Date.now();
  const diffMs = now - lastUpdateTime;

  const seconds = Math.floor(diffMs / 1000);
  const minutes = Math.floor(seconds / 60);
  const hours = Math.floor(minutes / 60);
  const days = Math.floor(hours / 24);
  const weeks = Math.floor(days / 7);
  const months = Math.floor(days / 30);
  const years = Math.floor(days / 365);

  if (seconds < 60) return { value: seconds, unit: "seconds" };
  if (minutes < 60) return { value: minutes, unit: "minutes" };
  if (hours < 24) return { value: hours, unit: "hours" };
  if (days < 7) return { value: days, unit: "days" };
  if (weeks < 4) return { value: weeks, unit: "weeks" };
  if (months < 12) return { value: months, unit: "months" };
  return { value: years, unit: "years" };
};

const getUnitLabel = (
  unit: RelativeTime["unit"],
  value: number,
): string => {
  const labels: Record<
    RelativeTime["unit"],
    Record<"singular" | "plural", string>
  > = {
    seconds: { singular: "second", plural: "seconds" },
    minutes: { singular: "minute", plural: "minutes" },
    hours: { singular: "hour", plural: "hours" },
    days: { singular: "day", plural: "days" },
    weeks: { singular: "week", plural: "weeks" },
    months: { singular: "month", plural: "months" },
    years: { singular: "year", plural: "years" },
  };

  return value === 1 ? labels[unit].singular : labels[unit].plural;
};

export const formatRelativeTime = (
  relativeTime: RelativeTime,
): string => {
  if (relativeTime.unit === "seconds") {
    return "just now";
  }
  return `${relativeTime.value} ${getUnitLabel(relativeTime.unit, relativeTime.value)} ago`;
};

export const getRelativeTimeDisplay = (timestamp: bigint): string => {
  const relativeTime = calculateRelativeTime(timestamp);
  return formatRelativeTime(relativeTime);
};
