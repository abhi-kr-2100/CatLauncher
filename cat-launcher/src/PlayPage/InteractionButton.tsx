import { useQuery, useQueryClient } from "@tanstack/react-query";
import { Loader2 } from "lucide-react";
import { useEffect, useState } from "react";
import { useDispatch, useSelector } from "react-redux";

import { Button } from "@/components/ui/button";
import type { GameRelease } from "@/generated-types/GameRelease";
import type { GameReleaseStatus } from "@/generated-types/GameReleaseStatus";
import type { GameVariant } from "@/generated-types/GameVariant";
import {
  getInstallationStatus,
  installReleaseForVariant,
  launchGame,
} from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { toastCL } from "@/lib/utils";
import {
  selectCurrentlyPlaying,
  setCurrentlyPlaying,
} from "@/store/gameSessionSlice";

export default function InteractionButton({
  variant,
  selectedReleaseId,
}: InteractionButtonProps) {
  const queryClient = useQueryClient();
  const dispatch = useDispatch();

  const currentlyPlaying = useSelector(selectCurrentlyPlaying);
  const isThisVariantRunning = currentlyPlaying === variant;
  const isAnyVariantRunning = currentlyPlaying !== null;

  const {
    data: installationStatus,
    isLoading: isInstallationStatusLoading,
    error: installationStatusError,
  } = useQuery<GameReleaseStatus>({
    queryKey: queryKeys.installationStatus(variant, selectedReleaseId),
    queryFn: () => getInstallationStatus(variant, selectedReleaseId!),
    enabled: Boolean(selectedReleaseId),
  });

  useEffect(() => {
    if (!installationStatusError) {
      return;
    }

    toastCL(
      "error",
      `Failed to get installation status of ${variant} ${selectedReleaseId}.`,
      installationStatusError,
    );
  }, [installationStatusError, variant, selectedReleaseId]);

  const [installing, setInstalling] = useState(false);

  const actionButtonLabel = isThisVariantRunning ? (
    "Running..."
  ) : installing || isInstallationStatusLoading ? (
    <Loader2 className="animate-spin" />
  ) : installationStatus === "ReadyToPlay" ? (
    "Play"
  ) : (
    "Install"
  );

  async function handleInstall() {
    if (!selectedReleaseId || installationStatusError || installing) {
      return;
    }

    if (installationStatus === "ReadyToPlay") {
      return;
    }

    setInstalling(true);
    try {
      const updatedRelease = await installReleaseForVariant(
        variant,
        selectedReleaseId,
      );
      queryClient.setQueryData(
        queryKeys.releases(variant),
        (old: GameRelease[] | undefined) =>
          old?.map((o) => {
            if (o.version !== selectedReleaseId) {
              return o;
            }
            return updatedRelease;
          }),
      );
      queryClient.setQueryData(
        queryKeys.installationStatus(variant, selectedReleaseId),
        (): GameReleaseStatus => "ReadyToPlay",
      );
    } catch (e) {
      toastCL("error", "Failed to install release.", e);
    } finally {
      setInstalling(false);
    }
  }

  async function handlePlay() {
    if (!selectedReleaseId || installationStatus !== "ReadyToPlay") {
      return;
    }

    try {
      await launchGame(variant, selectedReleaseId);
      dispatch(setCurrentlyPlaying({ variant }));
      queryClient.setQueryData(
        queryKeys.lastPlayedVersion(variant),
        () => selectedReleaseId,
      );
    } catch (e) {
      toastCL("error", "Failed to launch game.", e);
    }
  }

  const isActionButtonDisabled =
    !selectedReleaseId ||
    installing ||
    isInstallationStatusLoading ||
    Boolean(installationStatusError) ||
    installationStatus === "Unknown" ||
    installationStatus === "NotAvailable" ||
    isAnyVariantRunning; // only one variant can be running at a time

  return (
    <Button
      className="w-full"
      onClick={
        installationStatus === "ReadyToPlay" ? handlePlay : handleInstall
      }
      disabled={isActionButtonDisabled}
    >
      {actionButtonLabel}
    </Button>
  );
}

interface InteractionButtonProps {
  variant: GameVariant;
  selectedReleaseId: string | undefined;
}
