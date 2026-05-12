import { QueryClient, QueryClientProvider as QCP } from '@tanstack/react-query';
import type { ReactNode } from 'react';

export interface QueryProviderProps {
  children?: ReactNode;
}

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 20_000,
    },
  },
});

export function QueryClientProvider({ children }: QueryProviderProps) {
  return <QCP client={queryClient}>{children}</QCP>;
}
