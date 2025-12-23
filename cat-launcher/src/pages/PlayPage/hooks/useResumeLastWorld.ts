import { useLastPlayedWorld } from "./useLastPlayedWorld";
import { useLaunchGame } from "./useLaunchGame";
import type { GameVariant } from "@/generated-types/GameVariant";

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
