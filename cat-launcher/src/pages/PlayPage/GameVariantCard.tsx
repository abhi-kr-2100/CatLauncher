import { GripVertical } from "lucide-react";
import { useState } from "react";

import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import type { GameVariantInfo } from "@/generated-types/GameVariantInfo";
import { useSortableItem } from "@/hooks/useSortableItem";
import InteractionButton from "./InteractionButton";
import { PlayTime } from "./PlayTime";
import ReleaseSelector from "./ReleaseSelector";
import { TipOfTheDay } from "../game-tips/TipOfTheDay";

export interface GameVariantProps {
  variantInfo: GameVariantInfo;
}

export default function GameVariantCard({
  variantInfo,
}: GameVariantProps) {
  const [selectedReleaseId, setSelectedReleaseId] = useState<
    string | undefined
  >();
  const { attributes, listeners, setNodeRef, style } =
    useSortableItem(variantInfo.id);

  return (
    <Card ref={setNodeRef} style={style} {...attributes}>
      <CardHeader>
        <div className="flex justify-between items-start">
          <CardTitle>{variantInfo.name}</CardTitle>
          <div {...listeners} className="cursor-grab">
            <GripVertical />
          </div>
        </div>
      </CardHeader>
      <CardContent className="flex flex-col gap-4">
        <TipOfTheDay variant={variantInfo.id} />
        <ReleaseSelector
          variant={variantInfo.id}
          selectedReleaseId={selectedReleaseId}
          setSelectedReleaseId={setSelectedReleaseId}
        />
      </CardContent>
      <CardFooter className="flex flex-col gap-4 items-stretch">
        <InteractionButton
          variant={variantInfo.id}
          selectedReleaseId={selectedReleaseId}
          setSelectedReleaseId={setSelectedReleaseId}
        />
        <PlayTime
          variant={variantInfo.id}
          releaseId={selectedReleaseId}
        />
      </CardFooter>
    </Card>
  );
}
