import { Badge } from "@/components/ui/badge";
import Combobox, { ComboboxItem } from "@/components/ui/combobox";
import type { GameRelease } from "@/generated-types/GameRelease";
import type { GameVariant } from "@/generated-types/GameVariant";
import { fetchReleasesForVariant, getLastPlayedVersion } from "@/lib/utils";
import { useQuery } from "@tanstack/react-query";
import { useCallback, useMemo } from "react";

export default function ReleaseSelector({
  variant,
  selectedReleaseId,
  setSelectedReleaseId,
}: ReleaseSelectorProps) {
  const {
    data: releases,
    isLoading: isReleasesLoading,
    error: releasesError,
  } = useQuery<GameRelease[]>({
    queryKey: ["releases", variant],
    queryFn: () => fetchReleasesForVariant(variant),
  });

  const {
    data: lastPlayedVersion,
    isLoading: isLastPlayedVersionLoading,
    error: lastPlayedVersionError,
  } = useQuery<string | undefined>({
    queryKey: ["last_played_version", variant],
    queryFn: () => getLastPlayedVersion(variant),
  });

  const comboboxItems = useMemo<ComboboxItem[]>(() => {
    const latestVersionName = releases?.[0]?.version;

    return (
      releases?.map((r) => {
        const shortReleaseName = get_short_release_name(variant, r.version);
        const isLastPlayed = r.version === lastPlayedVersion;
        const isLatest = r.version === latestVersionName;

        return {
          value: r.version,
          label:
            isLastPlayed || isLatest ? (
              <div className="flex items-center gap-2 w-full">
                <span>{shortReleaseName}</span>
                {isLatest && <Badge>Latest</Badge>}
                {isLastPlayed && <Badge>Last Played</Badge>}
              </div>
            ) : (
              shortReleaseName
            ),
        };
      }) ?? []
    );
  }, [releases, lastPlayedVersion, variant]);

  const autoselect = useCallback(
    (items: ComboboxItem[]) => {
      if (isLastPlayedVersionLoading) {
        return;
      }

      if (lastPlayedVersionError || lastPlayedVersion === "") {
        return items[0];
      }

      return items.find((i) => i.value === lastPlayedVersion) ?? items[0];
    },
    [lastPlayedVersion, isLastPlayedVersionLoading, lastPlayedVersionError]
  );

  const placeholderText = isReleasesLoading
    ? "Loading..."
    : releasesError
    ? "Error loading releases."
    : comboboxItems.length === 0
    ? "No releases available."
    : "Select a release";

  return (
    <Combobox
      label="Version"
      items={comboboxItems}
      value={selectedReleaseId}
      onChange={setSelectedReleaseId}
      autoselect={autoselect}
      placeholder={placeholderText}
      disabled={Boolean(releasesError)}
    />
  );
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

interface ReleaseSelectorProps {
  variant: GameVariant;
  selectedReleaseId: string | undefined;
  setSelectedReleaseId: (value: string) => void;
}
