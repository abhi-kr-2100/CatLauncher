import { DownloadProgress } from "@/components/DownloadProgress";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { GameVariant } from "@/generated-types/GameVariant";
import type { Soundpack } from "@/generated-types/Soundpack";
import { toastCL } from "@/lib/utils";
import {
  useGetThirdPartySoundpackInstallationStatus,
  useInstallThirdPartySoundpack,
  useUninstallThirdPartySoundpack,
} from "./hooks";
import { soundpacksPageErrorMap } from "./lib/errors";

interface SoundpackCardProps {
  variant: GameVariant;
  soundpack: Soundpack;
}

function getSoundpackName(soundpack: Soundpack): string {
  return soundpack.content.name;
}

function getSoundpackType(soundpack: Soundpack): string {
  return soundpack.type === "Stock" ? "Pre-Installed" : "Third-Party";
}

export default function SoundpackCard({
  variant,
  soundpack,
}: SoundpackCardProps) {
  const name = getSoundpackName(soundpack);
  const soundpackType = getSoundpackType(soundpack);

  const isThirdParty = soundpack.type !== "Stock";
  const soundpackId = soundpack.content.id;

  const { installationStatus } =
    useGetThirdPartySoundpackInstallationStatus(soundpackId, variant);

  const isInstalled = installationStatus === "Installed";

  const {
    isInstalling,
    install,
    installationProgressStatus: soundpackInstallationProgress,
    downloadProgress: soundpackDownloadProgress,
  } = useInstallThirdPartySoundpack(
    variant,
    soundpackId,
    () => toastCL("success", "Soundpack installed successfully."),
    (error) =>
      toastCL(
        "error",
        "Failed to install soundpack.",
        error,
        soundpacksPageErrorMap,
      ),
  );

  const { isUninstalling, uninstall } =
    useUninstallThirdPartySoundpack(
      variant,
      () => toastCL("success", "Soundpack uninstalled successfully."),
      (error) =>
        toastCL(
          "error",
          "Failed to uninstall soundpack.",
          error,
          soundpacksPageErrorMap,
        ),
    );

  return (
    <Card className="flex flex-col">
      <CardHeader>
        <div className="flex justify-between items-start">
          <div className="flex-1">
            <CardTitle>{name}</CardTitle>
            <div className="flex gap-2 mt-2">
              <Badge variant="secondary">{soundpackType}</Badge>
            </div>
          </div>
        </div>
      </CardHeader>
      {isThirdParty && (
        <CardFooter className="flex flex-col gap-4 items-stretch">
          {soundpackInstallationProgress === "Downloading" &&
          soundpackDownloadProgress ? (
            <DownloadProgress
              downloaded={soundpackDownloadProgress.bytes_downloaded}
              total={soundpackDownloadProgress.total_bytes}
            />
          ) : isInstalled ? (
            <Button
              className="w-full"
              variant="destructive"
              onClick={() => uninstall(soundpackId)}
              disabled={isUninstalling}
            >
              {isUninstalling ? "Uninstalling..." : "Uninstall"}
            </Button>
          ) : (
            <Button
              className="w-full"
              onClick={() => install(soundpackId)}
              disabled={
                isInstalling || !!soundpackInstallationProgress
              }
            >
              {soundpackInstallationProgress === "Installing"
                ? "Installing..."
                : soundpackInstallationProgress === "Downloading"
                  ? "Downloading..."
                  : "Install"}
            </Button>
          )}
        </CardFooter>
      )}
    </Card>
  );
}
