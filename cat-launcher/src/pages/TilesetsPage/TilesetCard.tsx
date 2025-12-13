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
import type { Tileset } from "@/generated-types/Tileset";
import { toastCL } from "@/lib/utils";
import {
  useGetThirdPartyTilesetInstallationStatus,
  useInstallThirdPartyTileset,
  useUninstallThirdPartyTileset,
} from "./hooks";

interface TilesetCardProps {
  variant: GameVariant;
  tileset: Tileset;
}

function getTilesetName(tileset: Tileset): string {
  return tileset.content.name;
}

function getTilesetType(tileset: Tileset): string {
  return tileset.type === "Stock" ? "Pre-Installed" : "Third-Party";
}

export default function TilesetCard({ variant, tileset }: TilesetCardProps) {
  const name = getTilesetName(tileset);
  const tilesetType = getTilesetType(tileset);

  const isThirdParty = tileset.type !== "Stock";
  const tilesetId = tileset.content.id;

  const { installationStatus } = useGetThirdPartyTilesetInstallationStatus(
    tilesetId,
    variant,
  );

  const isInstalled = installationStatus === "Installed";

  const { isInstalling, install } = useInstallThirdPartyTileset(
    variant,
    () => toastCL("success", "Tileset installed successfully."),
    (error) => toastCL("error", "Failed to install tileset.", error),
  );

  const { isUninstalling, uninstall } = useUninstallThirdPartyTileset(
    variant,
    () => toastCL("success", "Tileset uninstalled successfully."),
    (error) => toastCL("error", "Failed to uninstall tileset.", error),
  );

  return (
    <Card className="flex flex-col">
      <CardHeader>
        <div className="flex justify-between items-start">
          <div className="flex-1">
            <CardTitle>{name}</CardTitle>
            <div className="flex gap-2 mt-2">
              <Badge variant="secondary">{tilesetType}</Badge>
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
              onClick={() => uninstall(tilesetId)}
              disabled={isUninstalling}
            >
              {isUninstalling ? "Uninstalling..." : "Uninstall"}
            </Button>
          ) : (
            <Button
              className="w-full"
              onClick={() => install(tilesetId)}
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
