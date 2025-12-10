import { usePostHog } from "@posthog/react";
import { useEffect } from "react";

import { trackPageView } from "@/lib/analytics";

export const useTrackPageView = (page: string) => {
  const posthog = usePostHog();

  useEffect(() => {
    if (posthog) {
      trackPageView(posthog, page);
    }
  }, [posthog, page]);
};
