import { Button } from "@/components/ui/button";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import type { GameReleaseStatus } from "@/generated-types/GameReleaseStatus";
import type { GameVariant } from "@/generated-types/GameVariant";
import { useAppSelector } from "@/store/hooks";
import { DownloadProgress } from "@/components/DownloadProgress";
import {
  useInstallAndMonitorRelease,
  useInstallationStatus,
  usePlayGame,
  useResumeLastWorld,
} from "./hooks";
import { toastCL } from "@/lib/utils";
import { InstallationProgressStatus } from "@/store/installationProgressSlice";
import { playPageErrorMap } from "./lib/errors";

export default function InteractionButton({
  variant,
  selectedReleaseId,
}: InteractionButtonProps) {
  const currentlyPlaying = useAppSelector(
    (state) => state.gameSession.currentlyPlaying,
  );
  const isThisVariantRunning = currentlyPlaying === variant;
  const isAnyVariantRunning = currentlyPlaying !== null;

  const { install, installationProgressStatus, downloadProgress } =
    useInstallAndMonitorRelease(variant, selectedReleaseId);

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
      toastCL(
        "error",
        "Failed to get last played world.",
        e,
        playPageErrorMap,
      );
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
    <div className="flex gap-2 w-full">
      <Button
        className="grow w-[50%]"
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
          className="grow w-[50%]"
          onClick={() => resume(selectedReleaseId)}
          disabled={isActionButtonDisabled || !lastPlayedWorld}
        >
          Resume Last World
        </Button>
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

interface InteractionButtonProps {
  variant: GameVariant;
  selectedReleaseId: string | undefined;
}

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
