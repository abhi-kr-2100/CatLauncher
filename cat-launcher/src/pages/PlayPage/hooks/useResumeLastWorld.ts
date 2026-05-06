import type { GameVariant } from "@/generated-types/GameVariant";
import { useLastPlayedWorld } from "./useLastPlayedWorld";
import { useLaunchGame } from "./useLaunchGame";

/**
 * Custom hook to resume the last played world for a given game variant.
 * It identifies the last played world and provides a function to launch it.
 *
 * @param variant - The game variant.
 * @param options - Configuration options.
 * @param options.onError - Callback triggered if an error occurs during world retrieval or launch.
 * @returns An object containing the resume function, starting state, and last played world name.
 */
export function useResumeLastWorld(
  variant: GameVariant,
  {
    onError,
  }: {
    onError: (error: Error) => void;
  },
) {
  const { lastPlayedWorld } = useLastPlayedWorld(variant, {
    onError,
  });

  const { launch, isStartingGame } = useLaunchGame(variant, {
    worldName: lastPlayedWorld ?? undefined,
    onError,
  });

  return { resume: launch, isStartingGame, lastPlayedWorld };
}
