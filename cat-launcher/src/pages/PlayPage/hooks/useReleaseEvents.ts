import { useEffect } from "react";

import type { ReleasesUpdatePayload } from "@/generated-types/ReleasesUpdatePayload";
import { listenToReleasesUpdate } from "@/lib/commands";
import { setupEventListener } from "@/lib/utils";
import { useAppDispatch } from "@/store/hooks";
import { updateReleasesForVariant } from "@/store/releasesSlice";

export function useReleaseEvents() {
  const dispatch = useAppDispatch();

  useEffect(() => {
    const releaseUpdateHandler = (payload: ReleasesUpdatePayload) => {
      dispatch(updateReleasesForVariant(payload));
    };

    const cleanup = setupEventListener(
      listenToReleasesUpdate,
      releaseUpdateHandler,
      "Failed to subscribe to releases.",
    );

    return cleanup;
  }, [dispatch]);
}
