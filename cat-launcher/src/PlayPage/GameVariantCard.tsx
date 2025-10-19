import { useState } from "react";

import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { ExternalLink } from "@/components/ui/ExternalLink";
import type { GameVariantInfo } from "@/generated-types/GameVariantInfo";
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
          <div className="flex gap-5">
            {variantInfo.links.map((link) => (
              <ExternalLink key={link.href} href={link.href}>
                {link.label}
              </ExternalLink>
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
