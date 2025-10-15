import { TanStackDevtools } from "@tanstack/react-devtools";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ReactQueryDevtoolsPanel } from "@tanstack/react-query-devtools";
import { ReactNode } from "react";
import { Provider } from "react-redux";

import AutoUpdateNotifier from "@/components/AutoUpdateNotifier";
import { Toaster } from "@/components/ui/sonner";
import { store } from "@/store/store";
import { useFrontendReady } from "./hooks";
import GameSessionMonitor from "@/components/GameSessionMonitor";

export interface ProvidersProps {
  children: ReactNode;
}

const queryClient = new QueryClient();

export default function Providers({ children }: ProvidersProps) {
  useFrontendReady();

  return (
    <Provider store={store}>
      <QueryClientProvider client={queryClient}>
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
      </QueryClientProvider>
    </Provider>
  );
}
