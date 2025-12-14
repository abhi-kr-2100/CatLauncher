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
} from "./hooks";

interface ModCardProps {
  variant: GameVariant;
  mod: Mod;
  activity?: string | null;
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

export default function ModCard({
  variant,
  mod,
  activity,
}: ModCardProps) {
  const name = getModName(mod);
  const description = getModDescription(mod);
  const modType = getModType(mod);
  const category = getModCategory(mod);

  const isThirdParty = mod.type !== "Stock";
  const modId = mod.content.id;

  const { installationStatus } =
    useGetThirdPartyModInstallationStatus(modId, variant);

  const isInstalled = installationStatus === "Installed";

  const { isInstalling, install } = useInstallThirdPartyMod(
    variant,
    () => toastCL("success", "Mod installed successfully."),
    (error) => toastCL("error", "Failed to install mod.", error),
  );

  const { isUninstalling, uninstall } = useUninstallThirdPartyMod(
    variant,
    () => toastCL("success", "Mod uninstalled successfully."),
    (error) => toastCL("error", "Failed to uninstall mod.", error),
  );

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
        {activity && (
          <div className="text-sm text-muted-foreground">
            Last updated: {new Date(activity).toLocaleDateString()}
          </div>
        )}
      </CardContent>
      {isThirdParty && (
        <CardFooter className="flex flex-col gap-4 items-stretch">
          {isInstalled ? (
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
              onClick={() => install(modId)}
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
