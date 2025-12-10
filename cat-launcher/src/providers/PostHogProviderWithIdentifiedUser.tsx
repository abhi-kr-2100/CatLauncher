import { useQuery } from "@tanstack/react-query";
import { PostHogProvider } from "@posthog/react";
import posthog from "posthog-js";
import { ReactNode, useEffect } from "react";

import { queryKeys } from "@/lib/queryKeys";
import { getUserId } from "@/lib/commands";

if (import.meta.env.VITE_PUBLIC_POSTHOG_KEY) {
  posthog.init(import.meta.env.VITE_PUBLIC_POSTHOG_KEY, {
    api_host: import.meta.env.VITE_PUBLIC_POSTHOG_HOST,
    defaults: "2025-11-30",
  });
}

export interface PostHogProviderWithIdentifiedUserProps {
  children: ReactNode;
}

export default function PostHogProviderWithIdentifiedUser({
  children,
}: PostHogProviderWithIdentifiedUserProps) {
  const { data: userId } = useQuery({
    queryKey: queryKeys.userId(),
    queryFn: getUserId,
  });

  useEffect(() => {
    if (userId) {
      posthog.identify(userId);
    }
  }, [userId]);

  return <PostHogProvider client={posthog}>{children}</PostHogProvider>;
}
