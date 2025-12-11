import { ExternalLink } from "lucide-react";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import type { ThirdPartyMod } from "@/generated-types/ThirdPartyMod";
import { openLink, toastCL } from "@/lib/utils";

interface ThirdPartyModCardProps {
  mod: ThirdPartyMod;
  onInstall: () => void;
  onUninstall: () => void;
  isInstalling: boolean;
  isUninstalling: boolean;
}

function formatDate(value: string) {
  const date = new Date(value);

  if (Number.isNaN(date.getTime())) {
    return value;
  }

  return date.toLocaleString();
}

export default function ThirdPartyModCard({
  mod,
  onInstall,
  onUninstall,
  isInstalling,
  isUninstalling,
}: ThirdPartyModCardProps) {
  const installed = Boolean(mod.status);
  const installedAt = mod.status?.installed_at
    ? formatDate(mod.status.installed_at)
    : null;
  const lastUpdated = mod.status?.last_updated_time
    ? formatDate(mod.status.last_updated_time)
    : null;

  const handleOpenRepository = () => {
    openLink(mod.repository).catch((error) => {
      toastCL("error", "Failed to open repository", error);
    });
  };

  return (
    <Card className="flex h-full flex-col">
      <CardHeader className="space-y-2">
        <div className="flex items-center justify-between gap-2">
          <CardTitle className="text-lg font-semibold">{mod.name}</CardTitle>
          <Badge variant="secondary">{mod.category}</Badge>
        </div>
        <CardDescription>{mod.description}</CardDescription>
      </CardHeader>
      <CardContent className="flex flex-col gap-2 text-sm text-muted-foreground">
        <div>
          <span className="font-semibold text-foreground">Authors:</span>{" "}
          {mod.authors.join(", ")}
        </div>
        {mod.maintainers.length > 0 && (
          <div>
            <span className="font-semibold text-foreground">Maintainers:</span>{" "}
            {mod.maintainers.join(", ")}
          </div>
        )}
        {installedAt && (
          <div className="text-xs text-foreground">
            Installed at {installedAt}
          </div>
        )}
        {lastUpdated && (
          <div className="text-xs text-foreground">
            Last updated {lastUpdated}
          </div>
        )}
      </CardContent>
      <CardFooter className="mt-auto flex flex-wrap gap-2">
        <Button variant="outline" size="sm" onClick={handleOpenRepository}>
          <ExternalLink className="mr-2 h-4 w-4" /> Repository
        </Button>
        {installed ? (
          <Button
            variant="ghost"
            size="sm"
            onClick={onUninstall}
            disabled={isUninstalling}
          >
            {isUninstalling ? "Removing..." : "Mark Uninstalled"}
          </Button>
        ) : (
          <Button size="sm" onClick={onInstall} disabled={isInstalling}>
            {isInstalling ? "Saving..." : "Mark Installed"}
          </Button>
        )}
      </CardFooter>
    </Card>
  );
}
