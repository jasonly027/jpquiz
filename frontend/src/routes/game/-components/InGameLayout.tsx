import type { ReactNode } from 'react';

export interface InGameLayoutProps {
  title: string;
  children?: ReactNode;
}

export function InGameLayout({ title, children }: InGameLayoutProps) {
  return (
    <div className="flex h-screen flex-col pb-15 font-game-eng">
      <div className="m-6 flex flex-row justify-center">
        <h1 className="text-center font-game-title text-3xl tracking-tight">
          {title}
        </h1>
      </div>
      {children}
    </div>
  );
}
