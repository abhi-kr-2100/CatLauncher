import { Button } from "@/components/ui/button";
import type { GameRelease } from "@/generated-types/GameRelease";
import type { GameReleaseStatus } from "@/generated-types/GameReleaseStatus";
import type { GameVariant } from "@/generated-types/GameVariant";
import {
  getInstallationStatus,
  installReleaseForVariant,
  launchGame,
  toastCL,
} from "@/lib/utils";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { Loader2 } from "lucide-react";
import { useEffect, useState } from "react";

export default function InteractionButton({
  variant,
  selectedReleaseId,
}: InteractionButtonProps) {
  const queryClient = useQueryClient();

  const {
    data: installationStatus,
    isLoading: isInstallationStatusLoading,
    error: installationStatusError,
  } = useQuery<GameReleaseStatus>({
    queryKey: ["installation_status", variant, selectedReleaseId],
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
      installationStatusError
    );
  }, [installationStatusError, variant, selectedReleaseId]);

  const [installing, setInstalling] = useState(false);

  const actionButtonLabel =
    installing || isInstallationStatusLoading ? (
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
        selectedReleaseId
      );
      queryClient.setQueryData(
        ["releases", variant],
        (old: GameRelease[] | undefined) =>
          old?.map((o) => {
            if (o.version !== selectedReleaseId) {
              return o;
            }
            return updatedRelease;
          })
      );
      queryClient.setQueryData(
        ["installation_status", variant, selectedReleaseId],
        (): GameReleaseStatus => "ReadyToPlay"
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
      queryClient.setQueryData(
        ["last_played_version", variant],
        () => selectedReleaseId
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
    installationStatus === "NotAvailable";

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
