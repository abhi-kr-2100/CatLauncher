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
import type { Tileset } from "@/generated-types/Tileset";
import { toastCL } from "@/lib/utils";
import {
  useGetThirdPartyTilesetInstallationStatus,
  useInstallAndMonitorThirdPartyTileset,
  useUninstallThirdPartyTileset,
} from "./hooks";
import { PreInstalledButton } from "@/components/PreInstalledButton";

/**
 * Props for the {@link TilesetCard} component.
 */
interface TilesetCardProps {
  /**
   * The game variant associated with the tileset.
   */
  variant: GameVariant;
  /**
   * The tileset data to display.
   */
  tileset: Tileset;
}

/**
 * Extracts the display name from a tileset.
 *
 * @param tileset - The tileset object.
 * @returns The name of the tileset.
 */
function getTilesetName(tileset: Tileset): string {
  return tileset.content.name;
}

/**
 * Determines the display type of a tileset (Pre-Installed or Third-Party).
 *
 * @param tileset - The tileset object.
 * @returns A string representing the tileset type.
 */
function getTilesetType(tileset: Tileset): string {
  return tileset.type === "Stock" ? "Pre-Installed" : "Third-Party";
}

/**
 * A card component that displays information about a single tileset and provides installation/uninstallation controls.
 *
 * @param props - The component props.
 * @returns A React element representing the tileset card.
 */
export default function TilesetCard({
  variant,
  tileset,
}: TilesetCardProps) {
  const name = getTilesetName(tileset);
  const tilesetType = getTilesetType(tileset);

  const isThirdParty = tileset.type === "ThirdParty";
  const tilesetId = tileset.content.id;

  const { installationStatus } =
    useGetThirdPartyTilesetInstallationStatus(tilesetId, variant);

  const isInstalled = installationStatus === "Installed";

  const {
    isInstalling,
    install,
    installationProgressStatus: tilesetInstallationProgress,
    downloadProgress: tilesetDownloadProgress,
  } = useInstallAndMonitorThirdPartyTileset(
    variant,
    tilesetId,
    () => toastCL("success", "Tileset installed successfully."),
    (error) => toastCL("error", "Failed to install tileset.", error),
  );

  const { isUninstalling, uninstall } = useUninstallThirdPartyTileset(
    variant,
    () => toastCL("success", "Tileset uninstalled successfully."),
    (error) =>
      toastCL("error", "Failed to uninstall tileset.", error),
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
      <CardFooter className="flex flex-col gap-4 items-stretch">
        {isThirdParty ? (
          tilesetInstallationProgress === "Downloading" &&
          tilesetDownloadProgress ? (
            <DownloadProgress
              downloaded={tilesetDownloadProgress.bytes_downloaded}
              total={tilesetDownloadProgress.total_bytes}
            />
          ) : isInstalled ? (
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
              disabled={isInstalling || !!tilesetInstallationProgress}
            >
              {tilesetInstallationProgress === "Installing"
                ? "Installing..."
                : tilesetInstallationProgress === "Downloading"
                  ? "Downloading..."
                  : "Install"}
            </Button>
          )
        ) : (
          <PreInstalledButton />
        )}
      </CardFooter>
    </Card>
  );
}
