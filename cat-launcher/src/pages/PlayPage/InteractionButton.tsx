import { DownloadProgress } from "@/components/DownloadProgress";
import { DropdownButton } from "@/components/DropdownButton";
import { Button } from "@/components/ui/button";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import type { GameReleaseStatus } from "@/generated-types/GameReleaseStatus";
import type { GameVariant } from "@/generated-types/GameVariant";
import { toastCL } from "@/lib/utils";
import { useAppSelector } from "@/store/hooks";
import { InstallationProgressStatus } from "@/store/installationProgressSlice";
import {
  useInstallAndMonitorRelease,
  useInstallationStatus,
  usePlayGame,
  useResumeLastWorld,
  useUpgradeInfo,
} from "./hooks";

/**
 * Component that renders the primary interaction buttons for a game variant.
 * Handles Play, Install, Resume, and Upgrade actions based on the current
 * installation and running status of the selected release.
 *
 * @param props - Component properties.
 * @returns A React element containing the interaction buttons.
 */
export default function InteractionButton({
  variant,
  selectedReleaseId,
  setSelectedReleaseId,
}: InteractionButtonProps) {
  const currentlyPlaying = useAppSelector(
    (state) => state.gameSession.currentlyPlaying,
  );
  const isThisVariantRunning = currentlyPlaying === variant;
  const isAnyVariantRunning = currentlyPlaying !== null;

  const { install, installationProgressStatus, downloadProgress } =
    useInstallAndMonitorRelease(variant, selectedReleaseId);

  const { latestReleaseId, upgradeOptions, shouldAllowUpgrading } =
    useUpgradeInfo(
      variant,
      selectedReleaseId,
      setSelectedReleaseId,
      install,
      (e) => {
        toastCL("error", "Failed to get active release.", e);
      },
    );

  const { installationStatus, installationStatusError } =
    useInstallationStatus(variant, selectedReleaseId);

  const { play, isStartingGame: isStartingGameFromPlay } =
    usePlayGame(variant);

  const {
    resume,
    isStartingGame: isStartingGameFromResume,
    lastPlayedWorld,
  } = useResumeLastWorld(variant, {
    onError: (e) => {
      toastCL("error", "Failed to get last played world.", e);
    },
  });
  const isStartingGame =
    isStartingGameFromPlay || isStartingGameFromResume;

  const actionButtonLabel = getActionButtonLabel(
    selectedReleaseId,
    isThisVariantRunning || isStartingGame,
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
    isAnyVariantRunning ||
    isStartingGame;

  if (installationProgressStatus === "Downloading") {
    return (
      <DownloadProgress
        downloaded={downloadProgress?.bytes_downloaded ?? 0}
        total={downloadProgress?.total_bytes ?? 0}
      />
    );
  }

  const button = (
    <div className="flex gap-1 w-full">
      <Button
        className="grow"
        onClick={() =>
          installationStatus === "ReadyToPlay"
            ? play(selectedReleaseId)
            : selectedReleaseId && install(selectedReleaseId)
        }
        disabled={isActionButtonDisabled}
      >
        {actionButtonLabel}
      </Button>
      {selectedReleaseId && installationStatus === "ReadyToPlay" && (
        <Button
          className="grow"
          onClick={() => resume(selectedReleaseId)}
          disabled={isActionButtonDisabled || !lastPlayedWorld}
        >
          Resume
        </Button>
      )}
      {shouldAllowUpgrading && (
        <DropdownButton
          className="grow"
          onClick={() => {
            if (latestReleaseId) {
              setSelectedReleaseId(latestReleaseId);
              install(latestReleaseId);
            }
          }}
          disabled={isAnyVariantRunning || isStartingGame}
          mainButtonDisabled={selectedReleaseId === latestReleaseId}
          options={upgradeOptions}
        >
          Upgrade
        </DropdownButton>
      )}
    </div>
  );

  if (installationStatus === "NotAvailable") {
    return (
      <Tooltip>
        <TooltipTrigger asChild>
          <span className="w-full">{button}</span>
        </TooltipTrigger>
        <TooltipContent>
          <p>
            This release is not yet available. Try again in a couple
            of hours.
          </p>
        </TooltipContent>
      </Tooltip>
    );
  }

  return button;
}

/**
 * Props for the {@link InteractionButton} component.
 */
interface InteractionButtonProps {
  /** The game variant this button is associated with. */
  variant: GameVariant;
  /** The currently selected release ID for this variant. */
  selectedReleaseId: string | undefined;
  /** Callback to update the selected release ID. */
  setSelectedReleaseId: (value: string) => void;
}

/**
 * Determines the appropriate label for the main action button based on the game's state.
 *
 * @param selectedReleaseId - The ID of the currently selected release.
 * @param isRunning - Whether the game is currently running.
 * @param installationStatus - The current installation status of the release.
 * @param installationProgressStatus - The progress status if an installation is active.
 * @returns A string label for the action button.
 */
function getActionButtonLabel(
  selectedReleaseId: string | undefined,
  isRunning: boolean,
  installationStatus: GameReleaseStatus,
  installationProgressStatus: InstallationProgressStatus | null,
) {
  if (!selectedReleaseId) {
    return "Select a Release to Play";
  }

  if (isRunning) {
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
