import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
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

export default function SoundpackCard({ variant, soundpack }: SoundpackCardProps) {
  const name = getSoundpackName(soundpack);
  const soundpackType = getSoundpackType(soundpack);

  const isThirdParty = soundpack.type !== "Stock";
  const soundpackId = soundpack.content.id;

  const { installationStatus } = useGetThirdPartySoundpackInstallationStatus(
    soundpackId,
    variant,
  );

  const isInstalled = installationStatus === "Installed";

  const { isInstalling, install } = useInstallThirdPartySoundpack(
    variant,
    () => toastCL("success", "Soundpack installed successfully."),
    (error) => toastCL("error", "Failed to install soundpack.", error),
  );

  const { isUninstalling, uninstall } = useUninstallThirdPartySoundpack(
    variant,
    () => toastCL("success", "Soundpack uninstalled successfully."),
    (error) => toastCL("error", "Failed to uninstall soundpack.", error),
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
          {isInstalled ? (
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
              disabled={isInstalling}
            >
              {isInstalling ? "Installing..." : "Install"}
            </Button>
          )}
        </CardFooter>
      )}
    </Card>
  );
}