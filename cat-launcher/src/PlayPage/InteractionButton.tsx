import { Button } from "@/components/ui/button";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import type { GameReleaseStatus } from "@/generated-types/GameReleaseStatus";
import type { GameVariant } from "@/generated-types/GameVariant";
import type { InstallationProgressStatus } from "@/generated-types/InstallationProgressStatus";
import { useAppSelector } from "@/store/hooks";
import {
  useInstallationProgressStatus,
  useInstallationStatus,
  useInstallRelease,
  usePlayGame,
} from "./hooks";

export default function InteractionButton({
  variant,
  selectedReleaseId,
}: InteractionButtonProps) {
  const currentlyPlaying = useAppSelector(
    (state) => state.gameSession.currentlyPlaying,
  );
  const isThisVariantRunning = currentlyPlaying === variant;
  const isAnyVariantRunning = currentlyPlaying !== null;

  const installationProgressStatus =
    useInstallationProgressStatus(selectedReleaseId);

  const {
    installationStatus,
    isInstallationStatusLoading,
    installationStatusError,
  } = useInstallationStatus(variant, selectedReleaseId);

  const { install, isInstalling } = useInstallRelease(
    variant,
    selectedReleaseId,
  );

  const { play } = usePlayGame(variant);

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
