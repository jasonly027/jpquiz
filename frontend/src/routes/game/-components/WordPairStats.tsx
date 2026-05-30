import type { QuizMeta } from '../-hooks/useQuiz';
import { getGameChoicesFont, getGamePromptFont } from '../-lib/utils';
import { cn } from '@/lib/utils';
import {
  ArrowDataTransferHorizontalIcon,
  ArrowDataTransferVerticalIcon,
} from '@hugeicons/core-free-icons';
import { HugeiconsIcon } from '@hugeicons/react';
import type { ComponentProps } from 'react';

interface StatsRowSolutionProps {
  prompt: string;
  answer: string;
  meta: QuizMeta<unknown>;
}

export function WordPairSolution({
  prompt,
  answer,
  meta,
}: StatsRowSolutionProps) {
  return (
    <div className="flex grow flex-col items-center justify-center gap-3 p-3 text-center text-lg lg:flex-row">
      <TextCard className={getGamePromptFont(meta.mode)}>{prompt}</TextCard>
      <div className="flex justify-center rounded-full bg-background/50 p-1.5 ring ring-foreground/15">
        <HugeiconsIcon
          icon={ArrowDataTransferHorizontalIcon}
          size={20}
          strokeWidth={1.5}
          className="hidden text-muted-foreground lg:block"
        />
        <HugeiconsIcon
          icon={ArrowDataTransferVerticalIcon}
          size={20}
          strokeWidth={1.5}
          className="text-muted-foreground lg:hidden"
        />
      </div>
      <TextCard className={getGameChoicesFont(meta.mode)}>{answer}</TextCard>
    </div>
  );
}

type TextCardProps = ComponentProps<'div'>;

function TextCard({ className, ...props }: TextCardProps) {
  return (
    <div
      className={cn(
        'm-2 flex h-full w-full items-center justify-center rounded-lg bg-background/50 p-3 ring ring-foreground/15',
        className
      )}
      {...props}
    ></div>
  );
}
