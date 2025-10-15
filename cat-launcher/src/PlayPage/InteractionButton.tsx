import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Loader2 } from "lucide-react";
import { useEffect } from "react";
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

  const { mutate: install, isPending: isInstalling } = useMutation({
    mutationFn: () => {
      if (!selectedReleaseId) {
        throw new Error("No release selected");
      }
      return installReleaseForVariant(variant, selectedReleaseId);
    },
    onSuccess: (updatedRelease) => {
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
        queryKeys.installationStatus(variant, selectedReleaseId!),
        (): GameReleaseStatus => "ReadyToPlay",
      );
    },
    onError: (e) => {
      toastCL("error", "Failed to install release.", e);
    },
  });

  const { mutate: play } = useMutation({
    mutationFn: () => {
      if (!selectedReleaseId) {
        throw new Error("No release selected");
      }
      return launchGame(variant, selectedReleaseId);
    },
    onSuccess: () => {
      dispatch(setCurrentlyPlaying({ variant }));
      queryClient.setQueryData(
        queryKeys.lastPlayedVersion(variant),
        () => selectedReleaseId,
      );
    },
    onError: (e) => {
      toastCL("error", "Failed to launch game.", e);
    },
  });

  const actionButtonLabel = isThisVariantRunning ? (
    "Running..."
  ) : isInstalling || isInstallationStatusLoading ? (
    <Loader2 className="animate-spin" />
  ) : installationStatus === "ReadyToPlay" ? (
    "Play"
  ) : (
    "Install"
  );

  const isActionButtonDisabled =
    !selectedReleaseId ||
    isInstalling ||
    isInstallationStatusLoading ||
    Boolean(installationStatusError) ||
    installationStatus === "Unknown" ||
    installationStatus === "NotAvailable" ||
    isAnyVariantRunning; // only one variant can be running at a time

  return (
    <Button
      className="w-full"
      onClick={() =>
        installationStatus === "ReadyToPlay" ? play() : install()
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
