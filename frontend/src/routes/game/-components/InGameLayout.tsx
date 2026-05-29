import { Title } from '@/components/Title';
import { GenericLayout } from '@/components/layouts/GenericLayout';
import type { ReactNode } from 'react';

export interface InGameLayoutProps {
  title: string;
  children?: ReactNode;
}

export function InGameLayout({ title, children }: InGameLayoutProps) {
  return (
    <GenericLayout>
      <Title className="font-game-title font-medium tracking-tight">
        {title}
      </Title>
      <div className="mx-4 flex justify-center">{children}</div>
    </GenericLayout>
  );
}
