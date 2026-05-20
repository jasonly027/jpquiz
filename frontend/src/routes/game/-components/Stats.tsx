import type { QuestionStat } from '../-lib/models';
import { formatAccuracy, formatTime } from '../-lib/utils';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader } from '@/components/ui/card';
import { cn } from '@/lib/utils';
import { BookOpenIcon } from '@hugeicons/core-free-icons';
import { HugeiconsIcon } from '@hugeicons/react';
import React, { useContext } from 'react';
import { createContext, useMemo } from 'react';

const StatsRootContext = createContext<StatsProviderState<unknown> | undefined>(
  undefined
);

interface StatsProviderState<T> {
  guessAvg: string;
  accAvg: string;
  totalTime: string;
  stats: QuestionStat<T>[];
}

function useStatsRootContext<T>() {
  const context = useContext(StatsRootContext) as
    | StatsProviderState<T>
    | undefined;
  if (!context) {
    throw new Error('StatsRootContext is missing');
  }
  return context;
}

export interface StatsProps {
  stats: QuestionStat<unknown>[];
  children?: React.ReactNode;
}

function StatsRoot({ stats, children }: StatsProps) {
  const { guessAvg, accAvg, totalTime } = useMemo(
    () => calculateQuizStats(stats),
    [stats]
  );

  const state = useMemo(
    () => ({
      guessAvg,
      accAvg,
      totalTime,
      stats,
    }),
    [guessAvg, accAvg, totalTime, stats]
  );

  return (
    <StatsRootContext.Provider value={state}>
      {children}
    </StatsRootContext.Provider>
  );
}
export { StatsRoot as Stats };

export interface StatsContentProps {
  children?: React.ReactNode;
}

export function StatsContent({ children }: StatsContentProps) {
  return (
    <Card variant="outline" className="flex w-full max-w-4xl flex-col">
      <CardHeader>
        <h2 className="text-center text-base font-semibold">Quiz Results</h2>
      </CardHeader>

      <CardContent className="flex flex-col gap-6">{children}</CardContent>
    </Card>
  );
}

export function StatsHeader() {
  const { guessAvg, accAvg, totalTime } = useStatsRootContext();

  return (
    <div className="flex flex-col gap-1 font-semibold *:flex-1 *:px-1.5 *:py-1 *:text-center *:ring *:ring-foreground/15 *:max-sm:w-full *:max-sm:rounded-lg sm:flex-row">
      <div className="rounded-l-lg bg-primary/60">{guessAvg} Guess Average</div>
      <div className="bg-primary/60">{accAvg} Accuracy</div>
      <div className="rounded-r-lg bg-secondary/45">{totalTime} Elapsed</div>
    </div>
  );
}

export interface StatsRowContainerProps {
  children?: React.ReactNode;
}

export function StatsRowContainer({ children }: StatsRowContainerProps) {
  return <div className="flex flex-col gap-6">{children}</div>;
}

export interface StatsRowProps {
  children?: React.ReactNode;
}

export function StatsRow({ children }: StatsRowProps) {
  return (
    <Card className="flex flex-col gap-0 divide-card-foreground/10 border-primary p-0 max-sm:divide-y-2 max-sm:border-t-4 sm:flex-row sm:divide-x-2 sm:border-l-4">
      {children}
    </Card>
  );
}

export interface StatsRowQuestionCounterProps {
  count: number;
}

export function StatsRowQuestionCounter({
  count,
}: StatsRowQuestionCounterProps) {
  return (
    <h3 className="flex items-center justify-center bg-primary/15 p-6 text-lg font-semibold">
      Q{count}
    </h3>
  );
}

export interface StatsRowGenericStatsProps {
  stats: Omit<QuestionStat<unknown>, 'source'>;
}

export function StatsRowGenericStats({ stats }: StatsRowGenericStatsProps) {
  return (
    <div className="flex flex-row divide-card-foreground/10 *:flex *:flex-1 *:items-center *:justify-center *:px-4 *:py-1.5 *:text-nowrap max-sm:divide-x-2 sm:flex-col sm:divide-y-2">
      <div
        data-skipped={isFinite(stats.guesses)}
        className="min-w-20 data-skipped:italic"
      >
        {isFinite(stats.guesses) ? `${stats.guesses} Guesses` : 'Skipped'}
      </div>
      <div>{stats.elapsed}s</div>
    </div>
  );
}

export interface StatsRowActionsProps {
  children?: React.ReactNode;
}

export function StatsRowActions({ children }: StatsRowActionsProps) {
  return (
    <div className="flex flex-row items-center justify-center gap-3 p-1 sm:flex-col">
      {children}
    </div>
  );
}

export interface StatsRowJishoButtonProps {
  search: string;
  className?: string;
}

export function StatsRowJishoButton({
  search,
  className,
}: StatsRowJishoButtonProps) {
  const href = `http://jisho.org/search/${search}`;

  return (
    <Button
      asChild
      variant="outline"
      size="icon-sm"
      className={cn(`rounded-full`, className)}
    >
      <a href={href} target="_blank" rel="noreferrer">
        <HugeiconsIcon
          icon={BookOpenIcon}
          size={24}
          color="currentColor"
          strokeWidth={1.5}
        />
      </a>
    </Button>
  );
}

function calculateQuizStats<T>(stats: QuestionStat<T>[]) {
  const guesses = stats
    .map(({ guesses }) => guesses)
    .filter((g) => isFinite(g));
  const guessAvg =
    guesses.length > 0
      ? (guesses.reduce((sum, g) => g + sum, 0) / guesses.length).toFixed(2)
      : 'N/A';

  const accs = stats.map(({ guesses }) => 1 / guesses);
  const accAvg = formatAccuracy(
    accs.reduce((sum, a) => a + sum, 0) / accs.length
  );

  const totalTime = formatTime(
    stats.map(({ elapsed }) => elapsed).reduce((sum, t) => t + sum, 0)
  );

  return { guessAvg, accAvg, totalTime };
}
