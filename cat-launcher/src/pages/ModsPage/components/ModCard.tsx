import { useState } from "react";
import { DownloadProgress } from "@/components/DownloadProgress";
import { Alert, AlertDescription } from "@/components/ui/alert";
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
import type { Mod } from "@/generated-types/Mod";
import { getHumanFriendlyText, toastCL } from "@/lib/utils";
import {
  useGetThirdPartyModInstallationStatus,
  useInstallThirdPartyMod,
  useUninstallThirdPartyMod,
} from "../hooks/hooks";
import { ModInstallationConfirmationDialog } from "./ModInstallationConfirmationDialog";
import { PreInstalledButton } from "@/components/PreInstalledButton";

interface ModCardProps {
  variant: GameVariant;
  mod: Mod;
}

function getModName(mod: Mod): string {
  return mod.content.name;
}

function getModDescription(mod: Mod): string {
  return mod.content.description;
}

function getModType(mod: Mod): string {
  return mod.type === "Stock" ? "Pre-Installed" : "Third-Party";
}

function getModCategory(mod: Mod): string {
  return getHumanFriendlyText(mod.content.category);
}

export default function ModCard({ variant, mod }: ModCardProps) {
  const [confirmationDialogOpen, setConfirmationDialogOpen] =
    useState(false);

  const name = getModName(mod);
  const description = getModDescription(mod);
  const modType = getModType(mod);
  const category = getModCategory(mod);

  const isThirdParty = mod.type === "ThirdParty";
  const modId = mod.content.id;

  const { installationStatus } =
    useGetThirdPartyModInstallationStatus(modId, variant);

  const isInstalled = installationStatus === "Installed";

  const {
    isInstalling,
    install,
    downloadProgress: modDownloadProgress,
    installationProgressStatus: modInstallationProgress,
  } = useInstallThirdPartyMod(
    variant,
    modId,
    () => toastCL("success", "Mod installed successfully."),
    (error: unknown) =>
      toastCL("error", "Failed to install mod.", error),
  );

  const { isUninstalling, uninstall } = useUninstallThirdPartyMod(
    variant,
    () => toastCL("success", "Mod uninstalled successfully."),
    (error: unknown) =>
      toastCL("error", "Failed to uninstall mod.", error),
  );

  const handleInstallClick = () => {
    setConfirmationDialogOpen(true);
  };

  const handleConfirmInstall = () => {
    setConfirmationDialogOpen(false);
    install(modId);
  };

  return (
    <Card className="flex flex-col">
      <CardHeader>
        <div className="flex justify-between items-start">
          <div className="flex-1">
            <CardTitle>{name}</CardTitle>
            <div className="flex gap-2 mt-2">
              <Badge variant="default" className="capitalize">
                {category}
              </Badge>
              <Badge variant="secondary">{modType}</Badge>
            </div>
          </div>
        </div>
      </CardHeader>
      <CardContent className="flex flex-col gap-4 grow">
        <Alert className="flex flex-col bg-secondary text-secondary-foreground h-full">
          <AlertDescription className="h-20 overflow-y-auto grow items-center text-secondary-foreground">
            {description}
          </AlertDescription>
        </Alert>
      </CardContent>
      <CardFooter className="flex flex-col gap-4 items-stretch">
        {isThirdParty ? (
          modInstallationProgress === "Downloading" &&
          modDownloadProgress ? (
            <DownloadProgress
              downloaded={modDownloadProgress.bytes_downloaded}
              total={modDownloadProgress.total_bytes}
            />
          ) : isInstalled ? (
            <Button
              className="w-full"
              variant="destructive"
              onClick={() => uninstall(modId)}
              disabled={isUninstalling}
            >
              {isUninstalling ? "Uninstalling..." : "Uninstall"}
            </Button>
          ) : (
            <Button
              className="w-full"
              onClick={handleInstallClick}
              disabled={isInstalling || !!modInstallationProgress}
            >
              {modInstallationProgress === "Installing"
                ? "Installing..."
                : modInstallationProgress === "Downloading"
                  ? "Downloading..."
                  : "Install"}
            </Button>
          )
        ) : (
          <PreInstalledButton />
        )}
      </CardFooter>
      <ModInstallationConfirmationDialog
        open={confirmationDialogOpen}
        onOpenChange={setConfirmationDialogOpen}
        onConfirm={handleConfirmInstall}
        modId={modId}
        variant={variant}
      />
    </Card>
  );
}
