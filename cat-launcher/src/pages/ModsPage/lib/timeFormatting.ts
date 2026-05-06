/**
 * Represents a relative duration of time with a value and a unit.
 */
export interface RelativeTime {
  /**
   * The numerical value of the duration.
   */
  value: number;
  /**
   * The unit of time.
   */
  unit:
    | "seconds"
    | "minutes"
    | "hours"
    | "days"
    | "weeks"
    | "months"
    | "years";
}

/**
 * Calculates the relative time from a given timestamp to now.
 *
 * @param timestamp - The timestamp as a bigint.
 * @returns A {@link RelativeTime} object.
 */
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

/**
 * Returns the appropriate singular or plural label for a time unit.
 *
 * @param unit - The time unit.
 * @param value - The value to determine plurality.
 * @returns The unit label string.
 */
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

/**
 * Formats a {@link RelativeTime} object into a human-readable string.
 *
 * @param relativeTime - The relative time object.
 * @returns A formatted string (e.g., "5 days ago").
 */
export const formatRelativeTime = (
  relativeTime: RelativeTime,
): string => {
  if (relativeTime.unit === "seconds") {
    return "just now";
  }
  return `${relativeTime.value} ${getUnitLabel(relativeTime.unit, relativeTime.value)} ago`;
};

/**
 * Gets a human-readable relative time display string for a given timestamp.
 *
 * @param timestamp - The timestamp as a bigint.
 * @returns The relative time display string.
 */
export const getRelativeTimeDisplay = (timestamp: bigint): string => {
  const relativeTime = calculateRelativeTime(timestamp);
  return formatRelativeTime(relativeTime);
};
