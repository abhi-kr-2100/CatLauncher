import { useQuery } from "@tanstack/react-query";
import { PostHogProvider } from "posthog-js/react";
import { ReactNode } from "react";

import { queryKeys } from "@/lib/queryKeys";
import { getUserId } from "@/lib/commands";

const posthogOptions = {
  api_host: import.meta.env.VITE_PUBLIC_POSTHOG_HOST,
  defaults: "2025-11-30",
  persistence: "localStorage",
} as const;

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

  if (!userId) {
    return null;
  }

  return (
    <PostHogProvider
      apiKey={import.meta.env.VITE_PUBLIC_POSTHOG_KEY}
      options={{
        ...posthogOptions,
        bootstrap: {
          distinctID: userId,
          isIdentifiedID: true,
        },
      }}
    >
      {children}
    </PostHogProvider>
  );
}
