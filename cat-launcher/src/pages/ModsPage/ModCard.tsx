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
import type { Mod } from "@/generated-types/Mod";
import { getHumanFriendlyText } from "@/lib/utils";

interface ModCardProps {
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

export default function ModCard({ mod }: ModCardProps) {
  const name = getModName(mod);
  const description = getModDescription(mod);
  const modType = getModType(mod);
  const category = getModCategory(mod);

  const isThirdParty = mod.type !== "Stock";

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
      <CardContent className="flex flex-col gap-4 flex-grow">
        <Alert className="flex flex-col bg-secondary text-secondary-foreground h-full">
          <AlertDescription className="h-20 overflow-y-auto flex-grow items-center text-secondary-foreground">
            {description}
          </AlertDescription>
        </Alert>
      </CardContent>
      {isThirdParty && (
        <CardFooter className="flex flex-col gap-4 items-stretch">
          <Button className="w-full">Install</Button>
        </CardFooter>
      )}
    </Card>
  );
}
