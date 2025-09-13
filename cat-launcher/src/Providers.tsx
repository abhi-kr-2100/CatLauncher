import { ReactNode } from "react";
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

export interface ProvidersProps {
  children: ReactNode;
}

const queryClient = new QueryClient();

export default function Providers({ children }: ProvidersProps) {
  return (
    <QueryClientProvider client={queryClient}>
      {children}
    </QueryClientProvider>
  );
}
