import { useEffect } from "react";
import { useDispatch } from "react-redux";

import { ReleasesUpdatePayload } from "@/generated-types/ReleasesUpdatePayload";
import { listenToReleasesUpdate } from "@/lib/commands";
import { setupEventListener } from "@/lib/utils";
import { updateReleasesForVariant } from "@/store/releasesSlice";

export function useReleaseEvents() {
  const dispatch = useDispatch();

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
