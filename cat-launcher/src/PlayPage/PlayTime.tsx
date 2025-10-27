import { useEffect, useState } from "react";

import type { GameVariantId } from "@/generated-types/GameVariantId";
import {
  getPlayTimeForVariant,
  getPlayTimeForVersion,
} from "@/lib/commands";
import { cn } from "@/lib/utils";

interface PlayTimeProps extends React.HTMLAttributes<HTMLDivElement> {
  variant: GameVariantId;
  releaseId?: string;
}

function formatPlayTime(totalSeconds: number): string {
  if (totalSeconds < 60) {
    return "";
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
  const [versionPlayTime, setVersionPlayTime] = useState(0);
  const [totalPlayTime, setTotalPlayTime] = useState(0);

  useEffect(() => {
    async function fetchPlayTime() {
      if (releaseId) {
        const versionPlayTime = await getPlayTimeForVersion(
          variant,
          releaseId,
        );
        setVersionPlayTime(versionPlayTime);
      } else {
        setVersionPlayTime(0);
      }

      const totalPlayTime = await getPlayTimeForVariant(variant);
      setTotalPlayTime(totalPlayTime);
    }
    fetchPlayTime();
  }, [variant, releaseId]);

  const formattedVersionPlayTime = formatPlayTime(versionPlayTime);
  const formattedTotalPlayTime = formatPlayTime(totalPlayTime);

  if (!formattedTotalPlayTime) return null;

  return (
    <div
      className={cn(
        "text-sm text-muted-foreground flex flex-col gap-2",
        className
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
