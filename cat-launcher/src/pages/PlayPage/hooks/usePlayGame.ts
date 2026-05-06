import type { GameVariant } from "@/generated-types/GameVariant";
import { useLaunchGame } from "./useLaunchGame";

/**
 * Custom hook to handle starting the game for a specific variant.
 * It's a convenience wrapper around {@link useLaunchGame}.
 *
 * @param variant - The game variant to play.
 * @returns An object containing the play function and a loading state.
 */
export function usePlayGame(variant: GameVariant) {
  const { launch, isStartingGame } = useLaunchGame(variant);
  return { play: launch, isStartingGame };
}
