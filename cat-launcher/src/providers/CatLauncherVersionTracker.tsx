import { usePostHog } from "posthog-js/react";
import { ReactNode, useEffect } from "react";

import pkg from "../../package.json";

export interface CatLauncherVersionTrackerProps {
  children: ReactNode;
}

export default function CatLauncherVersionTracker({
  children,
}: CatLauncherVersionTrackerProps) {
  const posthog = usePostHog();

  useEffect(() => {
    if (posthog) {
      posthog.capture("launch", {
        version: pkg.version,
      });
    }
  }, [posthog]);

  return <>{children}</>;
}
