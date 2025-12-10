import { usePostHog } from "@posthog/react";
import { useCallback } from "react";

import { trackButtonClick as trackButtonClickFn } from "@/lib/analytics";

export const useTrackButtonClick = () => {
  const posthog = usePostHog();

  const trackButtonClick = useCallback(
    (buttonName: string, properties: Record<string, any> = {}) => {
      if (posthog) {
        trackButtonClickFn(posthog, buttonName, properties);
      }
    },
    [posthog]
  );

  return { trackButtonClick };
};
