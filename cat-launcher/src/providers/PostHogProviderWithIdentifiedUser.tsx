import { PostHogProvider } from "posthog-js/react";
import { ReactNode } from "react";

import { useUserId } from "@/hooks/useUserId";

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
  const { userId } = useUserId();

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
