import { TanStackDevtools } from "@tanstack/react-devtools";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ReactQueryDevtoolsPanel } from "@tanstack/react-query-devtools";
import { ReactNode } from "react";
import { Provider } from "react-redux";

import { Toaster } from "@/components/ui/sonner";
import { store } from "@/store/store";

export interface ProvidersProps {
  children: ReactNode;
}

const queryClient = new QueryClient();

export default function Providers({ children }: ProvidersProps) {
  return (
    <Provider store={store}>
      <QueryClientProvider client={queryClient}>
        {children}
        <Toaster />
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
