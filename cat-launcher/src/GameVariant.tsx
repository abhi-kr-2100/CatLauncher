import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useCallback, useMemo, useState } from "react";
import {
  installReleaseForVariant,
  launchGame,
  getLastPlayedVersion,
  getInstallationStatus,
} from "@/lib/utils";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Loader2 } from "lucide-react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  CardFooter,
} from "@/components/ui/card";
import Combobox, { ComboboxItem } from "@/components/ui/combobox";
import type { GameRelease, GameReleaseStatus } from "@/generated-types/GameRelease";
import type { GameVariantInfo } from "@/generated-types/GameVariantInfo";
import { fetchReleasesForVariant } from "@/lib/utils";

export interface GameVariantProps {
  variant: GameVariantInfo;
}

function get_short_version_name(variantID: string, version: string): string {
  switch (variantID) {
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

  return version;
}

export default function GameVariant({ variant }: GameVariantProps) {
  const queryClient = useQueryClient();

  const { data: releases, isLoading: isReleasesLoading, error: releasesError, } = useQuery<GameRelease[]>({
    queryKey: ["releases", variant.name],
    queryFn: () => fetchReleasesForVariant(variant),
  });

  const { data: lastPlayedVersion, isLoading: isLastPlayedVersionLoading, error: lastPlayedVersionError, } = useQuery<string | undefined>({
    queryKey: ["last_played_version", variant.name],
    queryFn: () => getLastPlayedVersion(variant),
  });

  const [selectedReleaseId, setSelectedReleaseId] = useState<string | undefined>();
  const selectedRelease = useMemo<GameRelease | undefined>(
    () => releases?.find((r) => r.version === selectedReleaseId),
    [releases, selectedReleaseId]
  );

  const { data: installationStatus, isLoading: isInstallationStatusLoading, } = useQuery<GameReleaseStatus>({
    queryKey: ["installation_status", variant.name, selectedReleaseId],
    queryFn: () => getInstallationStatus(selectedRelease!),
    enabled: !!selectedRelease,
  });

  const [downloading, setDownloading] = useState(false);

  async function handleDownload() {
    if (!releases || !selectedReleaseId) {
      return;
    }

    if (!selectedRelease || installationStatus === "ReadyToPlay") {
      return;
    }

    setDownloading(true);
    try {
      const updatedRelease = await installReleaseForVariant(selectedRelease);
      queryClient.setQueryData(
        ["releases", variant.name],
        (old: GameRelease[] | undefined) =>
          old?.map((o) => {
            if (o.version !== selectedReleaseId) {
              return o;
            }
            return updatedRelease;
          })
      );
      queryClient.setQueryData(
        ["installation_status", variant.id, selectedReleaseId],
        (): GameReleaseStatus => "ReadyToPlay"
      );
    } catch (e) {
      console.error("install_release_for_variant failed", e);
    } finally {
      setInstalling(false);
    }
  }

  async function handlePlay() {
    if (
      !selectedRelease ||
      selectedReleaseInstallationStatus !== "ReadyToPlay"
    ) {
      return;
    }

    try {
      await launchGame(selectedRelease);
      queryClient.setQueryData(
        ["last_played_version", variant.id],
        () => selectedReleaseId
      );
    } catch (e) {
      console.error("launch_game failed", e);
    }
  }

  const comboboxItems = useMemo<ComboboxItem[]>(() => {
    const latestVersionName = releases?.[0]?.version;

    return (
      releases?.map((r) => {
        const shortVersionName = get_short_version_name(variant.id, r.version);
        const isLastPlayed = r.version === lastPlayedVersion;
        const isLatest = r.version === latestVersionName;

        return {
          value: r.version,
          label:
            isLastPlayed || isLatest ? (
              <div className="flex items-center gap-2 w-full">
                <span>{shortVersionName}</span>
                {isLatest && <Badge>Latest</Badge>}
                {isLastPlayed && <Badge>Last Played</Badge>}
              </div>
            ) : (
              shortVersionName
            ),
        };
      }) ?? []
    );
  }, [releases, lastPlayedVersion]);

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

  const isReleaseSelectionDisabled =
    isReleasesLoading ||
    Boolean(releasesError) ||
    comboboxItems.length === 0 ||
    installing;
  const isActionButtonDisabled =
    isReleaseSelectionDisabled ||
    !selectedReleaseId ||
    isSelectedReleaseInstallationStatusLoading;

  const placeholderText = isReleasesLoading
    ? "Loading..."
    : releasesError
    ? "Error loading releases."
    : comboboxItems.length === 0
    ? "No releases available."
    : "Select a release";

  const actionButtonLabel =
    installing || isSelectedReleaseInstallationStatusLoading ? (
      <Loader2 className="animate-spin" />
    ) : selectedReleaseInstallationStatus === "ReadyToPlay" ? (
      "Play"
    ) : (
      "Install"
    );

  return (
    <Card>
      <CardHeader>
        <CardTitle>{variant.name}</CardTitle>
        <CardDescription>
          <p className="text-sm text-muted-foreground line-clamp-3">
            {variant.description}
          </p>
        </CardDescription>
      </CardHeader>
      <CardContent>
        <Combobox
          label="Version"
          items={comboboxItems}
          value={selectedReleaseId}
          onChange={setSelectedReleaseId}
          autoselect={autoselect}
          placeholder={placeholderText}
          disabled={isReleaseSelectionDisabled}
        />
      </CardContent>
      <CardFooter>
        <Button
          className="w-full"
          onClick={
            selectedReleaseInstallationStatus === "ReadyToPlay"
              ? handlePlay
              : handleInstall
          }
          disabled={isActionButtonDisabled}
        >
          {actionButtonLabel}
        </Button>
      </CardFooter>
    </Card>
  );
}
