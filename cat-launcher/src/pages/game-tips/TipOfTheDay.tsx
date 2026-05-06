import { Lightbulb } from "lucide-react";
import { useCallback, useEffect, useMemo, useState } from "react";

import {
  Alert,
  AlertDescription,
  AlertTitle,
} from "@/components/ui/alert";
import type { GameVariant } from "@/generated-types/GameVariant";
import { randomInt, setImmediateInterval } from "@/lib/utils";
import { useGetTips } from "./hooks/useGetTips";
import { NO_TIPS_AVAILABLE } from "./lib/constants";
import { TIP_OF_THE_DAY_AUTOSHUFFLE_INTERVAL_MS } from "@/lib/constants";

/**
 * Props for the {@link TipOfTheDayContent} component.
 */
interface TipOfTheDayContentProps {
  /** The tip text to display. */
  tip: string;
}

/**
 * Renders the visual content of a game tip within an Alert component.
 *
 * @param props - Component properties.
 * @returns A React element displaying the tip.
 */
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

/**
 * Props for the {@link TipOfTheDay} component.
 */
interface TipOfTheDayProps {
  /** The game variant to fetch and display tips for. */
  variant: GameVariant;
}

/**
 * Component that displays a randomly selected game tip for a given variant.
 * The tip is automatically shuffled at regular intervals.
 *
 * @param props - Component properties.
 * @returns A React element that manages tip selection and display.
 */
export function TipOfTheDay({ variant }: TipOfTheDayProps) {
  const { data, status } = useGetTips(variant);

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
      tip={tips.length === 0 ? NO_TIPS_AVAILABLE : tips[randomIndex]}
    />
  );
}
