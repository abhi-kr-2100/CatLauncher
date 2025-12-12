import { Mod } from "@/generated-types/Mod";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  CardDescription,
} from "@/components/ui/card";
import { Alert, AlertDescription } from "@/components/ui/alert";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { useInstallMod } from "./hooks/useInstallMod";
import { useUninstallMod } from "./hooks/useUninstallMod";
import { GameVariant } from "@/generated-types/GameVariant";
import { getHumanFriendlyText, toastCL } from "@/lib/utils";
import { useState } from "react";

interface ModCardProps {
  mod: Mod;
  variant: GameVariant;
}

const DONT_SHOW_MOD_WARNING_KEY = "dontShowModInstallWarning";

export default function ModCard({ mod, variant }: ModCardProps) {
  const [showInstallConfirmation, setShowInstallConfirmation] = useState(false);
  const [dontShowAgain, setDontShowAgain] = useState(false);
  const [isOperationInitiated, setIsOperationInitiated] = useState(false);

  const { installMod, isInstalling } = useInstallMod({
    onError: (error) => {
      setIsOperationInitiated(false);
      toastCL("error", "Failed to install mod", error);
    },
    onSuccess: () => {
      setIsOperationInitiated(false);
      toastCL("success", "Mod installed successfully");
    },
  });

  const { uninstallMod, isUninstalling } = useUninstallMod({
    onError: (error) => {
      setIsOperationInitiated(false);
      toastCL("error", "Failed to uninstall mod", error);
    },
    onSuccess: () => {
      setIsOperationInitiated(false);
      toastCL("success", "Mod uninstalled successfully");
    },
  });

  const handleInstall = () => {
    if (isOperationInitiated) return;

    // Check if user has opted out of the warning
    const dontShowWarning = localStorage.getItem(DONT_SHOW_MOD_WARNING_KEY) === "true";

    if (dontShowWarning) {
      setIsOperationInitiated(true);
      installMod(variant, mod.id);
    } else {
      setShowInstallConfirmation(true);
    }
  };

  const handleConfirmInstall = () => {
    if (isOperationInitiated) return;

    // Save the "don't show again" preference if checked
    if (dontShowAgain) {
      localStorage.setItem(DONT_SHOW_MOD_WARNING_KEY, "true");
    }

    setIsOperationInitiated(true);
    installMod(variant, mod.id);
    setShowInstallConfirmation(false);
    setDontShowAgain(false); // Reset for next time
  };

  const handleUninstall = () => {
    if (isOperationInitiated) return;
    setIsOperationInitiated(true);
    uninstallMod(variant, mod.id);
  };

  const handleReinstall = () => {
    if (isOperationInitiated) return;
    setIsOperationInitiated(true);
    installMod(variant, mod.id);
  };

  const isOperationInProgress = isInstalling || isUninstalling || isOperationInitiated;

  return (
    <Card>
      <CardHeader>
        <div>
          <CardTitle>{mod.name}</CardTitle>
          <CardDescription>
            <div className="flex gap-2 mt-1">
              <Badge variant="secondary" className="capitalize">
                {getHumanFriendlyText(mod.category)}
              </Badge>
              <Badge variant="outline" className="capitalize">
                {mod.mod_type === "Stock" ? "Pre-Installed" : "Third-Party"}
              </Badge>
            </div>
          </CardDescription>
        </div>
      </CardHeader>
      <CardContent className="flex flex-col gap-4">
        <Alert className="flex flex-col bg-secondary text-secondary-foreground">
          <AlertDescription className="h-20 overflow-y-auto text-secondary-foreground">
            {mod.description}
          </AlertDescription>
        </Alert>
        {mod.mod_type === "ThirdParty" && (
          <div className="space-y-2">
            {mod.status === "NotInstalled" && (
              <Button
                onClick={handleInstall}
                disabled={isOperationInProgress}
                className="w-full"
              >
                {isInstalling ? "Installing..." : "Install"}
              </Button>
            )}
            {mod.status === "Installed" && (
              <div className="flex gap-2">
                <Button
                  onClick={handleReinstall}
                  disabled={isOperationInProgress}
                  variant="outline"
                  className="flex-1"
                >
                  {isInstalling ? "Reinstalling..." : "Reinstall"}
                </Button>
                <Button
                  onClick={handleUninstall}
                  disabled={isOperationInProgress}
                  variant="destructive"
                  className="flex-1"
                >
                  {isUninstalling ? "Uninstalling..." : "Uninstall"}
                </Button>
              </div>
            )}
          </div>
        )}
      </CardContent>
      <Dialog
        open={showInstallConfirmation}
        onOpenChange={(open) => {
          setShowInstallConfirmation(open);
          if (!open) {
            setDontShowAgain(false); // Reset checkbox when dialog closes
          }
        }}
      >
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Install Third-Party Mod</DialogTitle>
            <DialogDescription>
              Third-party mods can break your game. Your saves and backups are safe. If you encounter an error, uninstall the mod.
            </DialogDescription>
          </DialogHeader>
          <div className="flex items-center space-x-2 py-4">
            <Checkbox
              id="dont-show-again"
              checked={dontShowAgain}
              onCheckedChange={(checked) => setDontShowAgain(checked === true)}
            />
            <label
              htmlFor="dont-show-again"
              className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
            >
              Don't show this warning again
            </label>
          </div>
          <DialogFooter>
            <DialogClose asChild>
              <Button variant="outline">Cancel</Button>
            </DialogClose>
            <DialogClose asChild>
              <Button onClick={handleConfirmInstall}>Install</Button>
            </DialogClose>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </Card>
  );
}
