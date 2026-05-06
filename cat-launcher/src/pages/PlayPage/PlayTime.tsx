import { HTMLAttributes } from "react";

import type { GameVariant } from "@/generated-types/GameVariant";
import { cn } from "@/lib/utils";
import { usePlayTime } from "./hooks";

/**
 * Props for the {@link PlayTime} component.
 */
interface PlayTimeProps extends HTMLAttributes<HTMLDivElement> {
  /** The game variant to show playtime for. */
  variant: GameVariant;
  /** The specific release version ID to show playtime for. */
  releaseId?: string;
}

/**
 * Formats a duration in seconds into a human-readable string (e.g., "2h 15m").
 *
 * @param totalSeconds - The duration in seconds.
 * @returns A formatted duration string.
 */
function formatPlayTime(totalSeconds: number): string {
  if (totalSeconds === 0) {
    return "0h";
  }

  if (totalSeconds < 60) {
    return "< 1m";
  }

  const hours = Math.floor(totalSeconds / 3600);
  const minutes = Math.floor((totalSeconds % 3600) / 60);
  return `${hours}h ${minutes}m`;
}

/**
 * Displays the total and version-specific playtime for a given game variant and release.
 *
 * @param props - The component props.
 * @returns A React element showing playtime statistics.
 */
export function PlayTime({
  variant,
  releaseId,
  className,
  ...props
}: PlayTimeProps) {
  const { totalPlayTime, versionPlayTime } = usePlayTime(
    variant,
    releaseId,
  );

  const formattedVersionPlayTime = formatPlayTime(versionPlayTime);
  const formattedTotalPlayTime = formatPlayTime(totalPlayTime);

  return (
    <div
      className={cn(
        "text-sm text-muted-foreground flex flex-col gap-2",
        className,
      )}
      {...props}
    >
      <div className="flex justify-between">
        <div>Version playtime</div>
        <div>{formattedVersionPlayTime}</div>
      </div>
      <div className="flex justify-between">
        <div>Total playtime</div>
        <div>{formattedTotalPlayTime}</div>
      </div>
    </div>
  );
}
