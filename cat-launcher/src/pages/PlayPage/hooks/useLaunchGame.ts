import { useMutation, useQueryClient } from "@tanstack/react-query";

import type { GameVariant } from "@/generated-types/GameVariant";
import { launchGame } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { toastCL } from "@/lib/utils";
import { useAppDispatch } from "@/store/hooks";
import { setCurrentlyPlaying } from "@/store/gameSessionSlice";

/**
 * Custom hook to handle launching a specific game release.
 *
 * @param variant - The game variant to launch.
 * @param options - Launch configuration options.
 * @param options.worldName - Optional name of the world to load upon launch.
 * @param options.onError - Optional callback triggered if the game fails to launch.
 * @returns An object containing the launch function and a loading state.
 */
export function useLaunchGame(
  variant: GameVariant,
  {
    worldName,
    onError,
  }: {
    worldName?: string | null;
    onError?: (error: Error) => void;
  } = {},
) {
  const queryClient = useQueryClient();
  const dispatch = useAppDispatch();

  const { mutate: launch, isPending: isStartingGame } = useMutation({
    mutationFn: (releaseId: string | undefined) => {
      if (!releaseId) {
        throw new Error("No release selected");
      }
      return launchGame(variant, releaseId, worldName ?? null);
    },
    onSuccess: (_, releaseId) => {
      dispatch(
        setCurrentlyPlaying({
          variant,
          version: releaseId!,
        }),
      );
      queryClient.setQueryData(
        queryKeys.activeRelease(variant),
        () => releaseId!,
      );
    },
    onError: (e) => {
      if (onError) {
        onError(e as Error);
      } else {
        toastCL("error", "Failed to launch game.", e);
      }
    },
  });

  return { launch, isStartingGame };
}
