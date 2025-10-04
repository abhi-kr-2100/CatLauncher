import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useMemo, useState } from "react";
import { installReleaseForVariant, launchGame } from "@/lib/utils";
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

export default function GameVariant(props: GameVariantProps) {
  const { variant } = props;
  const {
    data: releases,
    isLoading,
    error,
  } = useQuery<GameRelease[]>({
    queryKey: ["releases", variant.name],
    queryFn: () => fetchReleasesForVariant(variant),
  });
  const queryClient = useQueryClient();

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

  const isReleaseSelectionDisabled =
    isLoading || Boolean(error) || comboboxItems.length === 0 || downloading;
  const isActionButtonDisabled =
    isReleaseSelectionDisabled || !selectedReleaseId;

  const placeholderText = isLoading
    ? "Loading..."
    : error
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
          autoselect
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
