import { cn } from '@/lib/utils';
import type { ComponentProps } from 'react';

export type TitleProps = ComponentProps<'h1'>;

export function Title({ className, ...props }: TitleProps) {
  return (
    <h1
      className={cn('m-6 text-center text-3xl font-semibold', className)}
      {...props}
    />
  );
}
