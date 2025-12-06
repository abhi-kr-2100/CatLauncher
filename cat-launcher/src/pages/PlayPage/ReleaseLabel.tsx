import { Badge } from "@/components/ui/badge";

import type { GameVariant } from "@/generated-types/GameVariant";
import { useMemo } from "react";

interface ReleaseLabelProps {
  variant: GameVariant;
  version: string;
  isLastPlayed: boolean;
}

function getShortReleaseName(variant: GameVariant, version: string): string {
  switch (variant) {
    case "BrightNights": {
      return version;
    }
    case "DarkDaysAhead": {
      if (version.startsWith("cdda-experimental-")) {
        return version.slice("cdda-experimental-".length);
      }
      if (version.startsWith("cdda-")) {
        return version.slice("cdda-".length);
      }
      return version;
    }
    case "TheLastGeneration": {
      if (version.startsWith("cataclysm-tlg-")) {
        return version.slice("cataclysm-tlg-".length);
      }
      return version;
    }
  }
}

export default function ReleaseLabel({
  variant,
  version,
  isLastPlayed,
}: ReleaseLabelProps) {
  const shortReleaseName = useMemo(
    () => getShortReleaseName(variant, version),
    [variant, version],
  );

  return (
    <div className="flex items-center justify-between w-full">
      <div className="flex items-center gap-2">
        {shortReleaseName}
      </div>
      {isLastPlayed && (
        <div className="flex items-center gap-1">
          <Badge>Last Played</Badge>
        </div>
      )}
    </div>
  );
}
