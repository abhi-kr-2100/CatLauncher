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
  useInstallAndMonitorRelease,
  useInstallationStatus,
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

  const { install, installationProgressStatus } = useInstallAndMonitorRelease(
    variant,
    selectedReleaseId,
  );

  const { installationStatus, installationStatusError } = useInstallationStatus(
    variant,
    selectedReleaseId,
  );

  const { play } = usePlayGame(variant);

  const actionButtonLabel = getActionButtonLabel(
    isThisVariantRunning,
    installationStatus,
    installationProgressStatus,
  );

  const isActionButtonDisabled =
    !selectedReleaseId ||
    Boolean(installationStatusError) ||
    installationStatus === "Unknown" ||
    installationStatus === "NotAvailable" ||
    installationProgressStatus === "Downloading" ||
    installationProgressStatus === "Installing" ||
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

function getActionButtonLabel(
  isThisVariantRunning: boolean,
  installationStatus: GameReleaseStatus,
  installationProgressStatus: InstallationProgressStatus | null,
) {
  if (isThisVariantRunning) {
    return "Running...";
  }

  if (installationProgressStatus === "Downloading") {
    return "Downloading...";
  }

  if (installationProgressStatus === "Installing") {
    return "Installing...";
  }

  if (installationStatus === "Unknown") {
    return "Loading...";
  }

  if (installationStatus === "ReadyToPlay") {
    return "Play";
  }

  if (installationStatus === "NotAvailable") {
    return "Not Available";
  }

  return "Install";
}
