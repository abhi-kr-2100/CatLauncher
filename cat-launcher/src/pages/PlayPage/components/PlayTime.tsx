import { HTMLAttributes } from "react";

import type { GameVariant } from "@/generated-types/GameVariant";
import { cn } from "@/lib/utils";
import { usePlayTime } from "./hooks";

interface PlayTimeProps extends HTMLAttributes<HTMLDivElement> {
  variant: GameVariant;
  releaseId?: string;
}

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
