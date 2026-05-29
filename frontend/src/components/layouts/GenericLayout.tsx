import { SideBar } from './SideBar';
import { cn } from '@/lib/utils';
import type { ComponentProps } from 'react';

export type GenericLayoutProps = ComponentProps<'main'>;

export function GenericLayout({ className, ...props }: GenericLayoutProps) {
  return (
    <div className="flex flex-col sm:flex-row">
      <SideBar />
      <main
        className={cn('flex min-h-screen grow flex-col pb-15', className)}
        {...props}
      />
    </div>
  );
}
