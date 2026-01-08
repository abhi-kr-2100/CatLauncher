import type { GameVariant } from "@/generated-types/GameVariant";
import { useLaunchGame } from "./useLaunchGame";

export function usePlayGame(variant: GameVariant) {
  const { launch, isStartingGame } = useLaunchGame(variant);
  return { play: launch, isStartingGame };
}
