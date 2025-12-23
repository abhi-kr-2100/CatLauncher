import { useLaunchGame } from "./useLaunchGame";
import type { GameVariant } from "@/generated-types/GameVariant";

export function usePlayGame(variant: GameVariant) {
  const { launch, isStartingGame } = useLaunchGame(variant);
  return { play: launch, isStartingGame };
}
