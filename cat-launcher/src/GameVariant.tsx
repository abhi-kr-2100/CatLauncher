import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useCallback, useMemo, useState } from "react";
import {
  installReleaseForVariant,
  launchGame,
  getLastPlayedVersion,
} from "@/lib/utils";
import { Button } from "@/components/ui/button";
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
import type { GameRelease } from "@/generated-types/GameRelease";
import type { GameVariantInfo } from "@/generated-types/GameVariantInfo";
import { fetchReleasesForVariant } from "@/lib/utils";

export interface GameVariantProps {
  variant: GameVariantInfo;
}

export default function GameVariant({ variant }: GameVariantProps) {
  const queryClient = useQueryClient();

  const {
    data: releases,
    isLoading: isReleasesLoading,
    error: releasesError,
  } = useQuery<GameRelease[]>({
    queryKey: ["releases", variant.name],
    queryFn: () => fetchReleasesForVariant(variant),
  });

  const {
    data: lastPlayedVersion,
    isLoading: isLastPlayedVersionLoading,
    error: lastPlayedVersionError,
  } = useQuery<string | undefined>({
    queryKey: ["last_played_version", variant.name],
    queryFn: () => getLastPlayedVersion(variant),
  });

  const [selectedReleaseId, setSelectedReleaseId] = useState<
    string | undefined
  >();
  const selectedRelease = useMemo<GameRelease | undefined>(
    () => releases?.find((r) => r.version === selectedReleaseId),
    [releases, selectedReleaseId]
  );

  const [downloading, setDownloading] = useState(false);

  async function handleDownload() {
    if (!releases || !selectedReleaseId) {
      return;
    }

    if (!selectedRelease || selectedRelease.status === "ReadyToPlay") {
      return;
    }

    setDownloading(true);
    try {
      await installReleaseForVariant(selectedRelease);
      queryClient.setQueryData(
        ["releases", variant.name],
        (old: GameRelease[] | undefined) =>
          old?.map((o) => {
            if (o.version !== selectedReleaseId) {
              return o;
            }
            return {
              ...o,
              status: "ReadyToPlay",
            };
          })
      );
    } catch (e) {
      console.error("install_release_for_variant failed", e);
    } finally {
      setDownloading(false);
    }
  }

  async function handlePlay() {
    if (!selectedRelease || selectedRelease.status !== "ReadyToPlay") {
      return;
    }

    try {
      await launchGame(selectedRelease);
      queryClient.setQueryData(
        ["last_played_version", variant.name],
        () => selectedReleaseId
      );
    } catch (e) {
      console.error("launch_game failed", e);
    }
  }

  const comboboxItems = useMemo<ComboboxItem[]>(
    () =>
      releases?.map((r) => ({
        value: r.version,
        label: r.version,
      })) ?? [],
    [releases]
  );

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
    downloading;
  const isActionButtonDisabled =
    isReleaseSelectionDisabled || !selectedReleaseId;

  const placeholderText = isReleasesLoading
    ? "Loading..."
    : releasesError
    ? "Error loading releases."
    : comboboxItems.length === 0
    ? "No releases available."
    : "Select a release";

  const actionButtonLabel = downloading ? (
    <Loader2 className="animate-spin" />
  ) : selectedRelease?.status === "ReadyToPlay" ? (
    "Play"
  ) : (
    "Download"
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
            selectedRelease?.status === "ReadyToPlay"
              ? handlePlay
              : handleDownload
          }
          disabled={isActionButtonDisabled}
        >
          {actionButtonLabel}
        </Button>
      </CardFooter>
    </Card>
  );
}
