import { formatAccuracy, formatTime } from '../-lib/utils';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader } from '@/components/ui/card';
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog';
import { cn } from '@/lib/utils';
import {
  ArrowRightDoubleIcon,
  StopWatchIcon,
} from '@hugeicons/core-free-icons';
import { HugeiconsIcon } from '@hugeicons/react';
import type { ComponentProps } from 'react';

export interface WordPairStatBarProps {
  guesses: number;
  seconds: number;
}

export function WordPairStatBar({ guesses, seconds }: WordPairStatBarProps) {
  const timeStr = formatTime(seconds);

  const acc = 1 / (guesses + 1);
  const accStr = formatAccuracy(acc);

  return (
    <div className="flex flex-row flex-wrap justify-between gap-1 px-4 text-center *:grow *:text-nowrap max-[375px]:flex-col max-[375px]:*:text-center">
      <span className="text-left">{guesses} Guesses</span>
      <span className="text-center">{timeStr}</span>
      <span className="text-right">{accStr} Acc</span>
    </div>
  );
}

export function WordPairCard({ children }: { children?: React.ReactNode }) {
  return <Card className="min-h-50 justify-center gap-1 py-2">{children}</Card>;
}

export function WordPairCardActions({
  children,
}: {
  children?: React.ReactNode;
}) {
  return <div className="flex flex-1 gap-1 px-5">{children}</div>;
}

export function WordPairCardContent({
  children,
}: {
  children?: React.ReactNode;
}) {
  return (
    <CardContent className="flex h-full items-center justify-center px-5">
      {children}
    </CardContent>
  );
}

export type SkipButtonProps = ComponentProps<'button'>;

export function SkipButton({ onClick, className, ...props }: SkipButtonProps) {
  return (
    <Button
      onClick={onClick}
      variant="ghost"
      className={cn('font-semibold text-muted-foreground', className)}
      {...props}
    >
      Skip
      <HugeiconsIcon
        icon={ArrowRightDoubleIcon}
        size={24}
        color="currentColor"
        strokeWidth={1.5}
        data-icon="inline-end"
      />
    </Button>
  );
}

export type RetireButtonProps = ComponentProps<'button'>;

export function RetireButton({
  onClick,
  className,
  ...props
}: RetireButtonProps) {
  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button
          variant="ghost"
          className={cn('font-semibold text-muted-foreground', className)}
          {...props}
        >
          Retire
          <HugeiconsIcon
            icon={StopWatchIcon}
            size={24}
            color="currentColor"
            strokeWidth={1.5}
            data-icon="inline-end"
          />
        </Button>
      </DialogTrigger>

      <DialogContent>
        <DialogHeader>
          <DialogTitle>Are you sure you want to retire?</DialogTitle>

          <DialogDescription>
            Retiring is as if you skipped all remaining questions. You will be
            able to review the questions you have answered up to retirement.
          </DialogDescription>
        </DialogHeader>

        <DialogFooter>
          <DialogClose asChild>
            <Button variant="outline">Cancel</Button>
          </DialogClose>
          <Button onClick={onClick}>Retire</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
