import { Badge } from "@/components/ui/badge";

import type { GameVariant } from "@/generated-types/GameVariant";
import { useMemo } from "react";

/**
 * Props for the {@link ReleaseLabel} component.
 */
interface ReleaseLabelProps {
  /** The game variant the release belongs to. */
  variant: GameVariant;
  /** The version string of the release. */
  version: string;
  /** Whether this release is currently active. */
  isActive: boolean;
}

/**
 * Formats a full version string into a shorter, more readable format for display.
 *
 * @param variant - The game variant.
 * @param version - The full version string.
 * @returns A shortened version string.
 */
function getShortReleaseName(
  variant: GameVariant,
  version: string,
): string {
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

/**
 * Displays a release version name with an optional "Active" badge.
 *
 * @param props - The component props.
 * @returns A React element representing the release label.
 */
export default function ReleaseLabel({
  variant,
  version,
  isActive,
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
      <div className="flex items-center gap-1">
        {isActive && <Badge>Active</Badge>}
      </div>
    </div>
  );
}
