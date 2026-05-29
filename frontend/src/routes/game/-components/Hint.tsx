import { useShowHint } from '../-hooks/useShowHint';
import { getGameChoicesFont } from '../-lib/utils';
import { Button } from '@/components/ui/button';
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover';
import type { GameMode } from '@/lib/models';
import { cn } from '@/lib/utils';
import {
  InformationCircleIcon,
  ViewIcon,
  ViewOffSlashIcon,
} from '@hugeicons/core-free-icons';
import { HugeiconsIcon } from '@hugeicons/react';
import type { ComponentProps } from 'react';

export interface HintBoxProps {
  hint: string;
  mode: GameMode;
}

export function HintBox({ hint, mode }: HintBoxProps) {
  return (
    <div className="flex w-full flex-col items-center justify-center rounded-lg bg-background px-3 pt-2 pb-4 ring ring-foreground/15">
      <div className="flex items-baseline gap-0.5">
        <span className="ml-2 text-muted-foreground">Hint</span>

        <Popover>
          <PopoverTrigger asChild>
            <Button variant="ghost" size="icon-xs">
              <HugeiconsIcon
                icon={InformationCircleIcon}
                color="currentColor"
                strokeWidth={1.5}
              />
            </Button>
          </PopoverTrigger>
          <PopoverContent side="top" className="rounded-sm p-3">
            There may be acceptable answers other than this hint for English
            input modes.
          </PopoverContent>
        </Popover>
      </div>

      <div
        className={`${getGameChoicesFont(mode)} flex gap-2 text-xl select-none`}
      >
        {hint.split('').map((c, idx) => (
          <span key={idx}>{c}</span>
        ))}
      </div>
    </div>
  );
}

export type HintToggleButtonProps = ComponentProps<'button'>;

export function HintToggleButton({
  className,
  ...props
}: HintToggleButtonProps) {
  const [showHint, setShowHint] = useShowHint();

  return (
    <Button
      onClick={() => setShowHint((show) => !show)}
      variant="ghost"
      className={cn('font-semibold text-muted-foreground', className)}
      {...props}
    >
      Hint
      <HugeiconsIcon
        icon={showHint ? ViewIcon : ViewOffSlashIcon}
        color="currentColor"
        strokeWidth={1.5}
      />
    </Button>
  );
}
