import { useQuery } from "@tanstack/react-query";
import { Lightbulb, Shuffle } from "lucide-react";
import { useEffect, useMemo, useState } from "react";

import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import type { GameVariant } from "@/generated-types/GameVariant";
import { getTips } from "@/lib/commands";
import { randomInt } from "@/lib/utils";
import { queryKeys } from "@/lib/queryKeys";

interface TipOfTheDayContentProps {
  tip: string;
  onShuffle: () => void;
}

function TipOfTheDayContent({ tip, onShuffle }: TipOfTheDayContentProps) {
  return (
    <Alert>
      <AlertTitle className="flex items-center gap-2">
        <Lightbulb />
        Tip of the Day
        <Button variant="ghost" size="icon" onClick={onShuffle}>
          <Shuffle />
        </Button>
      </AlertTitle>
      <AlertDescription className="h-20 overflow-y-auto">
        {tip}
      </AlertDescription>
    </Alert>
  );
}

interface TipOfTheDayProps {
  variant: GameVariant;
}

export function TipOfTheDay({ variant }: TipOfTheDayProps) {
  const { data, status } = useQuery({
    queryKey: queryKeys.tips(variant),
    queryFn: async () => getTips(variant),
  });

  const [randomIndex, setRandomIndex] = useState(0);

  const tips = useMemo(() => {
    if (status !== "success" || data.length === 0) {
      return [];
    }
    return data;
  }, [data, status]);

  useEffect(() => {
    if (tips.length > 0) {
      setRandomIndex(randomInt(tips.length));
    }
  }, [tips]);

  const handleShuffle = () => {
    if (tips.length === 0) {
      return;
    }

    setRandomIndex(randomInt(tips.length));
  };

  return (
    <TipOfTheDayContent
      tip={
        tips.length === 0
          ? "Install and play a game to start getting tips and hints."
          : tips[randomIndex]
      }
      onShuffle={handleShuffle}
    />
  );
}
