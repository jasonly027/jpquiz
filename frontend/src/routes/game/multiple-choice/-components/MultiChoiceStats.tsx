import {
  Stats,
  StatsContent,
  StatsHeader,
  StatsRow,
  StatsRowContainer,
} from '../../-components/Stats';
import type { QuizMeta } from '../../-hooks/useQuiz';
import type { QuestionStat } from '../../-lib/models';
import { getGameChoicesFont, getGamePromptFont } from '../../-lib/utils';
import type { MultiChoiceQuestion } from '@/lib/models';
import { ArrowDataTransferVerticalIcon } from '@hugeicons/core-free-icons';
import { HugeiconsIcon } from '@hugeicons/react';

export interface MutliChoiceStatsProps {
  stats: QuestionStat<MultiChoiceQuestion>[];
  meta: QuizMeta<MultiChoiceQuestion>;
}

export function MultiChoiceStats({ stats, meta }: MutliChoiceStatsProps) {
  return (
    <Stats stats={stats}>
      <StatsContent>
        <StatsHeader />
        <StatsRowContainer>
          {stats.map((stats, idx) => (
            <StatsRow key={idx} stats={stats} idx={idx}>
              {({ choices, answer_idx, prompt }) => (
                <div className="flex flex-col gap-3 py-3 text-center text-sm font-semibold">
                  <div className={`${getGamePromptFont(meta.mode)} px-2`}>
                    {prompt}
                  </div>
                  <div className="flex justify-center bg-background/50 p-3">
                    <HugeiconsIcon
                      icon={ArrowDataTransferVerticalIcon}
                      size={20}
                      strokeWidth={1.5}
                      className="text-muted-foreground"
                    />
                  </div>
                  <div className={`${getGameChoicesFont(meta.mode)} px-2`}>
                    {choices[answer_idx]}
                  </div>
                </div>
              )}
            </StatsRow>
          ))}
        </StatsRowContainer>
      </StatsContent>
    </Stats>
  );
}
