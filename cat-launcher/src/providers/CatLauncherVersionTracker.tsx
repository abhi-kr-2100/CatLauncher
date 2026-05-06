import { usePostHog } from "posthog-js/react";
import { ReactNode, useEffect, useRef } from "react";

import pkg from "../../package.json";

/**
 * Props for the {@link CatLauncherVersionTracker} component.
 */
export interface CatLauncherVersionTrackerProps {
  /**
   * The child components to be rendered.
   */
  children: ReactNode;
}

/**
 * A component that tracks the application version and captures a "launch" event in PostHog.
 * This ensures that every time the application is started, its version is reported.
 *
 * @param props - The component props.
 * @returns A React component that wraps its children.
 */
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
