import { useQuery } from "@tanstack/react-query";
import { Lightbulb } from "lucide-react";
import { useCallback, useEffect, useMemo, useState } from "react";

import {
  Alert,
  AlertDescription,
  AlertTitle,
} from "@/components/ui/alert";
import type { GameVariant } from "@/generated-types/GameVariant";
import { getTips } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { randomInt, setImmediateInterval } from "@/lib/utils";
import { TIP_OF_THE_DAY_AUTOSHUFFLE_INTERVAL_MS } from "@/lib/constants";

interface TipOfTheDayContentProps {
  tip: string;
}

function TipOfTheDayContent({ tip }: TipOfTheDayContentProps) {
  return (
    <Alert className="flex flex-col bg-secondary text-secondary-foreground">
      <AlertTitle className="flex items-center gap-2">
        <Lightbulb />
        Tip of the Day
      </AlertTitle>
      <AlertDescription className="h-20 overflow-y-auto flex-grow items-center text-secondary-foreground">
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

  const shuffleTips = useCallback(() => {
    if (tips.length === 0) {
      return;
    }

    setRandomIndex(randomInt(tips.length));
  }, [tips]);

  useEffect(() => {
    // auto shuffle every 10 seconds
    const timerId = setImmediateInterval(() => {
      shuffleTips();
    }, TIP_OF_THE_DAY_AUTOSHUFFLE_INTERVAL_MS);

    return () => {
      clearInterval(timerId);
    };
  }, [shuffleTips]);

  return (
    <TipOfTheDayContent
      tip={
        tips.length === 0
          ? "Install a game to start getting tips and hints."
          : tips[randomIndex]
      }
    />
  );
}
