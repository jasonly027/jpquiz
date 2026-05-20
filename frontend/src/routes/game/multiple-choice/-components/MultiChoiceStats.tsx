import { gameMutationOptions } from '../-hooks/useGameMutation';
import {
  Stats,
  StatsContent,
  StatsHeader,
  StatsRow,
  StatsRowActions,
  StatsRowContainer,
  StatsRowGenericStats,
  StatsRowJishoButton,
  StatsRowQuestionCounter,
} from '../../-components/Stats';
import type { QuizMeta, QuizPostState } from '../../-hooks/useQuiz';
import type { QuestionStat } from '../../-lib/models';
import { getGameChoicesFont, getGamePromptFont } from '../../-lib/utils';
import { Button } from '@/components/ui/button';
import { Card, CardContent } from '@/components/ui/card';
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from '@/components/ui/collapsible';
import { Separator } from '@/components/ui/separator';
import type { MultiChoiceQuestion } from '@/lib/models';
import {
  ArrowDataTransferVerticalIcon,
  ArrowDownIcon,
} from '@hugeicons/core-free-icons';
import { HugeiconsIcon } from '@hugeicons/react';
import { useMutation } from '@tanstack/react-query';
import { useState } from 'react';

export type MutliChoiceStatsProps = QuizPostState<MultiChoiceQuestion>;

export function MultiChoiceStats({
  stats,
  meta,
  initQuiz,
  reset,
}: MutliChoiceStatsProps) {
  const gameMutation = useMutation({
    ...gameMutationOptions(),
    onSuccess({ data: questions }, { mode, levels, pos: categories }) {
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
  });

  const onPlayAgain = () => {
    gameMutation.mutate({
      mode: meta.mode,
      levels: meta.levels,
      pos: meta.categories,
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
          <Button variant="outline" onClick={onPlayAgain}>
            Play Again
          </Button>
        </div>

        <StatsRowContainer>
          {stats.map((stats, idx) => (
            <MultiChoiceStatsRow
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

export interface MultiChoiceStatsRowProps {
  idx: number;
  stats: QuestionStat<MultiChoiceQuestion>;
  meta: QuizMeta<MultiChoiceQuestion>;
}

function MultiChoiceStatsRow({ idx, stats, meta }: MultiChoiceStatsRowProps) {
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
  stats: QuestionStat<MultiChoiceQuestion>;
  meta: QuizMeta<MultiChoiceQuestion>;
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
        {stats.source.choices[stats.source.answer_idx]}
      </div>
    </div>
  );
}

interface StatsRowDetailsProps {
  wordPair: MultiChoiceQuestion['word_pair'];
}

function StatsRowDetails({ wordPair }: StatsRowDetailsProps) {
  return (
    <Card className="p-3">
      <CardContent className="flex flex-col items-start gap-6 py-3 sm:flex-row">
        <div className="flex shrink-0 flex-col gap-1.5">
          <span className="font-jp">{wordPair.kana}</span>
          {wordPair.kanji && (
            <span className="font-jp text-base">{wordPair.kanji}</span>
          )}
          <span className="mt-1 rounded-sm bg-primary/15 px-1.5">
            JLPT {wordPair.level}
          </span>
        </div>

        <Separator
          orientation="horizontal"
          className="bg-card-foreground/10 sm:hidden"
        />
        <Separator
          orientation="vertical"
          className="hidden bg-card-foreground/10 sm:block"
        />

        <div className="flex flex-col gap-3">
          {wordPair.senses.map((sense, idx) => (
            <div key={idx}>
              <div className="text-muted-foreground">
                {sense.partsOfSpeech.join(', ')}
              </div>
              <div>
                {idx + 1}. {sense.glossary.join('; ')}
              </div>
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  );
}
