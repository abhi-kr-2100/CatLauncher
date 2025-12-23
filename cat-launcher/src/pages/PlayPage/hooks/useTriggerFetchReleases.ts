import { useMutation } from "@tanstack/react-query";

import type { GameVariant } from "@/generated-types/GameVariant";
import { triggerFetchReleasesForVariant } from "@/lib/commands";
import { toastCL } from "@/lib/utils";
import { useAppDispatch } from "@/store/hooks";
import {
  onFetchingReleasesFailed,
  startFetchingReleases,
} from "@/store/releasesSlice";

export function useTriggerFetchReleases() {
  const dispatch = useAppDispatch();

  const {
    mutate: triggerFetchReleases,
    isPending: isReleasesTriggerLoading,
  } = useMutation({
    mutationFn: triggerFetchReleasesForVariant,
    onMutate: (variant: GameVariant) => {
      dispatch(startFetchingReleases({ variant }));
    },
    onError: (error: unknown, variant) => {
      dispatch(onFetchingReleasesFailed({ variant }));
      toastCL(
        "error",
        `Failed to fetch releases for ${variant}.`,
        error,
      );
    },
  });

  return { triggerFetchReleases, isReleasesTriggerLoading };
}
