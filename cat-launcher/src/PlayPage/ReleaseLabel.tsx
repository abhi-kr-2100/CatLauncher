import { Badge } from "@/components/ui/badge";
import { Check, Loader2 } from "lucide-react";

import type { GameVariant } from "@/generated-types/GameVariant";
import { useAppSelector } from "@/store/hooks";
import { useInstallationStatus } from "./hooks";

interface ReleaseLabelProps {
  variant: GameVariant;
  version: string;
  isLastPlayed: boolean;
}

function get_short_release_name(variant: GameVariant, version: string): string {
  switch (variant) {
    case "BrightNights": {
      return version;
    }
    case "DarkDaysAhead": {
      if (version.startsWith("cdda-experimental-")) {
        return version.slice("cdda-experimental-".length);
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
  const shortReleaseName = get_short_release_name(variant, version);

  const { installationStatus } = useInstallationStatus(variant, version);
  const progressStatus = useAppSelector(
    (state) => state.installationProgress.statusByVariant[variant]?.[version],
  );

  let statusIcon = null;
  if (progressStatus === "Downloading" || progressStatus === "Installing") {
    statusIcon = <Loader2 className="h-4 w-4 animate-spin" />;
  } else if (
    progressStatus === "Success" ||
    installationStatus === "ReadyToPlay"
  ) {
    statusIcon = <Check className="h-4 w-4 text-green-500" />;
  }

  return (
    <div className="flex items-center justify-between w-full">
      <div className="flex items-center gap-2">
        <span>{shortReleaseName}</span>
        {statusIcon}
      </div>
      {isLastPlayed && (
        <div className="flex items-center gap-1">
          <Badge>Last Played</Badge>
        </div>
      )}
    </div>
  );
}
