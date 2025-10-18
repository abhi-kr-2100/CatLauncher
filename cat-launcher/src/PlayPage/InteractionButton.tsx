import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useEffect } from "react";

import { Button } from "@/components/ui/button";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import type { GameRelease } from "@/generated-types/GameRelease";
import type { GameReleaseStatus } from "@/generated-types/GameReleaseStatus";
import type { GameVariant } from "@/generated-types/GameVariant";
import type { InstallationProgressStatus } from "@/generated-types/InstallationProgressStatus";
import {
  getInstallationStatus,
  installReleaseForVariant,
  launchGame,
} from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { toastCL } from "@/lib/utils";
import { setCurrentlyPlaying } from "@/store/gameSessionSlice";
import { useAppDispatch, useAppSelector } from "@/store/hooks";
import { useInstallationProgressStatus } from "./hooks";

export default function InteractionButton({
  variant,
  selectedReleaseId,
}: InteractionButtonProps) {
  const queryClient = useQueryClient();
  const dispatch = useAppDispatch();

  const currentlyPlaying = useAppSelector(
    (state) => state.gameSession.currentlyPlaying,
  );
  const isThisVariantRunning = currentlyPlaying === variant;
  const isAnyVariantRunning = currentlyPlaying !== null;

  const installationProgressStatus =
    useInstallationProgressStatus(selectedReleaseId);

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

  const { mutate: install, isPending: isInstalling } = useMutation({
    mutationFn: (releaseId: string | undefined) => {
      if (!releaseId) {
        throw new Error("No release selected");
      }
      return installReleaseForVariant(variant, releaseId);
    },
    onSuccess: (updatedRelease, releaseId) => {
      queryClient.setQueryData(
        queryKeys.releases(variant),
        (old: GameRelease[] | undefined) =>
          old?.map((o) => {
            if (o.version !== releaseId) {
              return o;
            }
            return updatedRelease;
          }),
      );
      queryClient.setQueryData(
        queryKeys.installationStatus(variant, releaseId),
        (): GameReleaseStatus => "ReadyToPlay",
      );
    },
    onError: (e) => {
      toastCL("error", "Failed to install release.", e);
    },
  });

  const { mutate: play } = useMutation({
    mutationFn: (releaseId: string | undefined) => {
      if (!releaseId) {
        throw new Error("No release selected");
      }
      return launchGame(variant, releaseId);
    },
    onSuccess: (_, releaseId) => {
      dispatch(setCurrentlyPlaying({ variant }));
      queryClient.setQueryData(
        queryKeys.lastPlayedVersion(variant),
        () => releaseId!,
      );
    },
    onError: (e) => {
      toastCL("error", "Failed to launch game.", e);
    },
  });

  const actionButtonLabel = getActionButtonLabel({
    isThisVariantRunning,
    isInstallationStatusLoading,
    installationStatus,
    isInstalling,
    installationProgressStatus,
  });

  const isActionButtonDisabled =
    !selectedReleaseId ||
    isInstalling ||
    isInstallationStatusLoading ||
    Boolean(installationStatusError) ||
    installationStatus === "Unknown" ||
    installationStatus === "NotAvailable" ||
    // Only one variant should be running at a time.
    // Disable button if any variant is already running.
    isAnyVariantRunning;

  const button = (
    <Button
      className="w-full"
      onClick={() =>
        installationStatus === "ReadyToPlay"
          ? play(selectedReleaseId)
          : install(selectedReleaseId)
      }
      disabled={isActionButtonDisabled}
    >
      {actionButtonLabel}
    </Button>
  );

  if (installationStatus === "NotAvailable") {
    return (
      <Tooltip>
        <TooltipTrigger asChild>
          <span className="w-full">{button}</span>
        </TooltipTrigger>
        <TooltipContent>
          <p>
            This release is not yet available. Try again in a couple of hours.
          </p>
        </TooltipContent>
      </Tooltip>
    );
  }

  return button;
}

interface InteractionButtonProps {
  variant: GameVariant;
  selectedReleaseId: string | undefined;
}

function getActionButtonLabel({
  isThisVariantRunning,
  isInstallationStatusLoading,
  installationStatus,
  isInstalling,
  installationProgressStatus,
}: GetActionButtonLabelParams) {
  if (isThisVariantRunning) {
    return "Running...";
  }

  if (isInstallationStatusLoading) {
    return "Loading...";
  }

  if (installationStatus === "ReadyToPlay") {
    return "Play";
  }

  if (!isInstalling) {
    return "Install";
  }

  if (installationProgressStatus === "Downloading") {
    return "Downloading...";
  }

  if (installationProgressStatus === "Installing") {
    return "Installing...";
  }

  return "Play";
}

interface GetActionButtonLabelParams {
  isThisVariantRunning: boolean;
  isInstallationStatusLoading: boolean;
  installationStatus?: GameReleaseStatus;
  isInstalling: boolean;
  installationProgressStatus?: InstallationProgressStatus | null;
}
