import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import type { GameVariantInfo } from "@/generated-types/GameVariantInfo";
import { openLink } from "@/lib/utils";
import { useState } from "react";
import InteractionButton from "./InteractionButton";
import ReleaseSelector from "./ReleaseSelector";

export interface GameVariantProps {
  variantInfo: GameVariantInfo;
}

export default function GameVariantCard({ variantInfo }: GameVariantProps) {
  const [selectedReleaseId, setSelectedReleaseId] = useState<
    string | undefined
  >();

  return (
    <Card>
      <CardHeader>
        <CardTitle>{variantInfo.name}</CardTitle>
        <CardDescription>
          <div className="flex space-x-2">
            {variantInfo.links.map((link) => (
              <Button
                key={link.href}
                variant="link"
                onClick={() => openLink(link.href)}
                className="p-0"
              >
                {link.label}
              </Button>
            ))}
          </div>
        </CardDescription>
      </CardHeader>
      <CardContent>
        <ReleaseSelector
          variant={variantInfo.id}
          selectedReleaseId={selectedReleaseId}
          setSelectedReleaseId={setSelectedReleaseId}
        />
      </CardContent>
      <CardFooter>
        <InteractionButton
          variant={variantInfo.id}
          selectedReleaseId={selectedReleaseId}
        />
      </CardFooter>
    </Card>
  );
}
