import { usePostHog } from "posthog-js/react";
import { ReactNode, useEffect, useRef } from "react";

import pkg from "../../package.json";

export interface CatLauncherVersionTrackerProps {
  children: ReactNode;
}

export default function CatLauncherVersionTracker({
  children,
}: CatLauncherVersionTrackerProps) {
  const posthog = usePostHog();
  const hasCaptured = useRef(false);

  useEffect(() => {
    if (posthog && !hasCaptured.current) {
      posthog.capture("launch", {
        version: pkg.version,
      });
      hasCaptured.current = true;
    }
  }, [posthog]);

  return <>{children}</>;
}
