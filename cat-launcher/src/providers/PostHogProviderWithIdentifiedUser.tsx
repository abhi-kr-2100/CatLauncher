import { useQuery } from "@tanstack/react-query";
import { PostHogProvider } from "posthog-js/react";
import { ReactNode } from "react";

import { queryKeys } from "@/lib/queryKeys";
import { getUserId } from "@/lib/commands";

/**
 * Configuration options for the PostHog client.
 */
const posthogOptions = {
  api_host: import.meta.env.VITE_PUBLIC_POSTHOG_HOST,
  defaults: "2025-11-30",
  persistence: "localStorage",
} as const;

/**
 * Props for the {@link PostHogProviderWithIdentifiedUser} component.
 */
export interface PostHogProviderWithIdentifiedUserProps {
  /**
   * The child components to be rendered.
   */
  children: ReactNode;
}

/**
 * A component that initializes PostHog and identifies the user using a unique ID retrieved from the backend.
 * It waits for the user ID to be available before rendering its children within the {@link PostHogProvider}.
 *
 * @param props - The component props.
 * @returns A React component that wraps its children with PostHog context, or null while loading.
 */
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
