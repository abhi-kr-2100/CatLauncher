import {
  QueryClient,
  QueryClientProvider,
} from "@tanstack/react-query";
import { ReactNode } from "react";
import { Provider } from "react-redux";

import AutoUpdateNotifier from "@/components/AutoUpdateNotifier";
import GameSessionMonitor from "@/components/GameSessionMonitor";
import PlayTimeMonitor from "@/components/PlayTimeMonitor";
import { Toaster } from "@/components/ui/sonner";
import { TooltipProvider } from "@/components/ui/tooltip";
import { store } from "@/store/store";
import CatLauncherVersionTracker from "./CatLauncherVersionTracker";
import { useFrontendReady } from "./hooks";
import PostHogProviderWithIdentifiedUser from "./PostHogProviderWithIdentifiedUser";
import ThemeBootstrapper from "@/theme/ThemeBootstrapper";
import QuitConfirmationProvider from "./QuitConfirmationProvider";

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
                <QuitConfirmationProvider>
                  {children}
                  <Toaster />
                  <AutoUpdateNotifier />
                  <GameSessionMonitor />
                  <PlayTimeMonitor />
                </QuitConfirmationProvider>
              </TooltipProvider>
            </Provider>
          </CatLauncherVersionTracker>
        </PostHogProviderWithIdentifiedUser>
      </ThemeBootstrapper>
    </QueryClientProvider>
  );
}
