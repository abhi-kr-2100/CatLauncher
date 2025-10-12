import { useEffect } from "react";
import { useDispatch } from "react-redux";

import { listenToReleasesUpdate } from "@/lib/commands";
import { toastCL } from "@/lib/utils";
import { updateReleasesForVariant } from "@/store/releasesSlice";

export function useReleaseEvents() {
  const dispatch = useDispatch();

  useEffect(() => {
    let unlisten: (() => void) | undefined;
    listenToReleasesUpdate((payload) => {
      dispatch(updateReleasesForVariant(payload));
    })
      .then((unlistenFn) => {
        unlisten = unlistenFn;
      })
      .catch((e) => {
        toastCL("error", "Failed to subscribe to releases.", e);
      });

    return () => {
      unlisten?.();
    };
  }, [dispatch]);
}
