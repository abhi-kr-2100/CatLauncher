import { TanStackDevtools } from "@tanstack/react-devtools";
import {
  QueryClient,
  QueryClientProvider,
} from "@tanstack/react-query";
import { ReactQueryDevtoolsPanel } from "@tanstack/react-query-devtools";
import { ReactNode } from "react";
import { Provider } from "react-redux";

import AutoUpdateNotifier from "@/components/AutoUpdateNotifier";
import GameSessionMonitor from "@/components/GameSessionMonitor";
import { Toaster } from "@/components/ui/sonner";
import { TooltipProvider } from "@/components/ui/tooltip";
import { store } from "@/store/store";
import CatLauncherVersionTracker from "./CatLauncherVersionTracker";
import { useFrontendReady } from "./hooks";
import PostHogProviderWithIdentifiedUser from "./PostHogProviderWithIdentifiedUser";
import ThemeBootstrapper from "@/theme/ThemeBootstrapper";

export interface ProvidersProps {
  children: ReactNode;
}

const queryClient = new QueryClient();

export default function Providers({ children }: ProvidersProps) {
  useFrontendReady();

  return (
    <QueryClientProvider client={queryClient}>
      <ThemeBootstrapper>
        <PostHogProviderWithIdentifiedUser>
          <CatLauncherVersionTracker>
            <Provider store={store}>
              <TooltipProvider>
                {children}
                <Toaster />
                <AutoUpdateNotifier />
                <GameSessionMonitor />
                {import.meta.env.DEV && (
                  <TanStackDevtools
                    plugins={[
                      {
                        name: "TanStack Query",
                        render: <ReactQueryDevtoolsPanel />,
                      },
                    ]}
                  />
                )}
              </TooltipProvider>
            </Provider>
          </CatLauncherVersionTracker>
        </PostHogProviderWithIdentifiedUser>
      </ThemeBootstrapper>
    </QueryClientProvider>
  );
}
