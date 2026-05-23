import {
  Stats,
  StatsContent,
  StatsHeader,
  StatsRow,
  StatsRowActions,
  StatsRowContainer,
  StatsRowDetails,
  StatsRowGenericStats,
  StatsRowJishoButton,
  StatsRowQuestionCounter,
} from '../../-components/Stats';
import type { QuizMeta, QuizPostState } from '../../-hooks/useQuiz';
import type { QuestionStat } from '../../-lib/models';
import { getGameChoicesFont, getGamePromptFont } from '../../-lib/utils';
import { useGetFreeResponse } from '@/api/server';
import { Button } from '@/components/ui/button';
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from '@/components/ui/collapsible';
import { Spinner } from '@/components/ui/spinner';
import type { FreeResponseQuestion } from '@/lib/models';
import {
  ArrowDataTransferVerticalIcon,
  ArrowDownIcon,
} from '@hugeicons/core-free-icons';
import { HugeiconsIcon } from '@hugeicons/react';
import { isAxiosError } from 'axios';
import { useState } from 'react';

export type MutliChoiceStatsProps = QuizPostState<FreeResponseQuestion>;

export function FreeResponseStats({
  stats,
  meta,
  initQuiz,
  reset,
}: MutliChoiceStatsProps) {
  const getGame = useGetFreeResponse({
    mutation: {
      onSuccess(
        { data: questions },
        { params: { mode, levels, pos: categories } }
      ) {
        initQuiz({
          questions,
          mode,
          levels,
          categories,
        });
      },
      onError(error) {
        console.error(error);
      },
    },
  });

  let getGameError: string | undefined;
  if (getGame.error) {
    if (isAxiosError(getGame.error) && getGame.error.status === 422) {
      getGameError =
        'Word pool is too small to create a game. Please try different settings.';
    } else {
      getGameError = 'Something went wrong. Please try again.';
    }
  }

  const onPlayAgain = () => {
    getGame.mutate({
      params: {
        mode: meta.mode,
        levels: meta.levels,
        pos: meta.categories,
      },
    });
  };

  return (
    <Stats stats={stats}>
      <StatsContent>
        <StatsHeader />

        <div className="flex items-center justify-center gap-3">
          <Button variant="outline" onClick={reset}>
            New Game
          </Button>
          <Button
            variant="outline"
            disabled={getGame.isPending}
            onClick={onPlayAgain}
          >
            {getGame.isPending && <Spinner data-icon="inline-start" />}
            Play Again
          </Button>
        </div>

        {getGame.error && (
          <div className="-mt-4 text-center text-destructive">
            {getGameError}
          </div>
        )}

        <StatsRowContainer>
          {stats.map((stats, idx) => (
            <FreeResponseStatsRow
              key={idx}
              idx={idx}
              stats={stats}
              meta={meta}
            />
          ))}
        </StatsRowContainer>
      </StatsContent>
    </Stats>
  );
}

export interface FreeResponseRowProps {
  idx: number;
  stats: QuestionStat<FreeResponseQuestion>;
  meta: QuizMeta<FreeResponseQuestion>;
}

function FreeResponseStatsRow({ idx, stats, meta }: FreeResponseRowProps) {
  const [open, setOpen] = useState(false);

  return (
    <Collapsible
      open={open}
      onOpenChange={setOpen}
      className="flex flex-col gap-6"
    >
      <StatsRow>
        <StatsRowQuestionCounter count={idx + 1} />

        <StatsRowSolution stats={stats} meta={meta} />

        <StatsRowGenericStats stats={stats} />

        <StatsRowActions>
          <StatsRowJishoButton
            search={stats.source.word_pair.kanji ?? stats.source.word_pair.kana}
          />

          <div className="flex flex-1 items-end justify-end">
            <CollapsibleTrigger asChild>
              <Button variant="ghost" size="icon">
                <HugeiconsIcon
                  icon={ArrowDownIcon}
                  color="currentColor"
                  strokeWidth={1.5}
                  data-open={open}
                  className="size-3/5 transition-transform duration-150 data-open:rotate-180"
                />
                <span className="sr-only">Open Details</span>
              </Button>
            </CollapsibleTrigger>
          </div>
        </StatsRowActions>
      </StatsRow>

      <CollapsibleContent>
        <StatsRowDetails wordPair={stats.source.word_pair} />
      </CollapsibleContent>
    </Collapsible>
  );
}

interface StatsRowSolutionProps {
  stats: QuestionStat<FreeResponseQuestion>;
  meta: QuizMeta<FreeResponseQuestion>;
}

function StatsRowSolution({ stats, meta }: StatsRowSolutionProps) {
  return (
    <div className="flex grow flex-col gap-4 py-3 text-center text-sm font-semibold">
      <div className={`${getGamePromptFont(meta.mode)} px-3`}>
        {stats.source.prompt}
      </div>
      <div className="flex justify-center bg-background/50 p-3">
        <HugeiconsIcon
          icon={ArrowDataTransferVerticalIcon}
          size={20}
          strokeWidth={1.5}
          className="text-muted-foreground"
        />
      </div>
      <div className={`${getGameChoicesFont(meta.mode)} px-3`}>
        {stats.source.answers.join('; ')}
      </div>
    </div>
  );
}
