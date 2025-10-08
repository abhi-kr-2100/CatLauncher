import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import type { GameVariantInfo } from "@/generated-types/GameVariantInfo";
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
          <p className="text-sm text-muted-foreground line-clamp-3">
            {variantInfo.description}
          </p>
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
