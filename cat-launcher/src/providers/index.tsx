import { TanStackDevtools } from "@tanstack/react-devtools";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ReactQueryDevtoolsPanel } from "@tanstack/react-query-devtools";
import { ReactNode } from "react";
import { Provider } from "react-redux";

import AutoUpdateNotifier from "@/components/AutoUpdateNotifier";
import GameSessionMonitor from "@/components/GameSessionMonitor";
import { Toaster } from "@/components/ui/sonner";
import { TooltipProvider } from "@/components/ui/tooltip";
import { store } from "@/store/store";
import { useFrontendReady } from "./hooks";

export interface ProvidersProps {
  children: ReactNode;
}

const queryClient = new QueryClient();

export default function Providers({ children }: ProvidersProps) {
  useFrontendReady();

  return (
    <Provider store={store}>
      <QueryClientProvider client={queryClient}>
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
      </QueryClientProvider>
    </Provider>
  );
}
