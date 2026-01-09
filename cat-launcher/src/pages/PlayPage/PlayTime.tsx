import { HTMLAttributes } from "react";
import { useTranslation } from "react-i18next";

import type { GameVariant } from "@/generated-types/GameVariant";
import { cn } from "@/lib/utils";
import { usePlayTime } from "./hooks";

interface PlayTimeProps extends HTMLAttributes<HTMLDivElement> {
  variant: GameVariant;
  releaseId?: string;
}

function formatPlayTime(
  t: (
    key: string,
    options?: { [key: string]: string | number },
  ) => string,
  totalSeconds: number,
): string {
  if (totalSeconds === 0) {
    return t("played_time_in_hours", { hours: 0 });
  }

  if (totalSeconds < 60) {
    return t("played_time_in_minutes");
  }

  const hours = Math.floor(totalSeconds / 3600);
  const minutes = Math.floor((totalSeconds % 3600) / 60);
  return t("played_time_in_hours_and_minutes", { hours, minutes });
}

export function PlayTime({
  variant,
  releaseId,
  className,
  ...props
}: PlayTimeProps) {
  const { t } = useTranslation();
  const { totalPlayTime, versionPlayTime } = usePlayTime(
    variant,
    releaseId,
  );

  const formattedVersionPlayTime = formatPlayTime(t, versionPlayTime);
  const formattedTotalPlayTime = formatPlayTime(t, totalPlayTime);

  return (
    <div
      className={cn(
        "text-sm text-muted-foreground flex flex-col gap-2",
        className,
      )}
      {...props}
    >
      <div className="flex justify-between">
        <div>{t("Version playtime")}</div>
        <div>{formattedVersionPlayTime}</div>
      </div>
      <div className="flex justify-between">
        <div>{t("Total playtime")}</div>
        <div>{formattedTotalPlayTime}</div>
      </div>
    </div>
  );
}
