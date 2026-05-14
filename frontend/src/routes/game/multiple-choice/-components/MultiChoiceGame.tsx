import type { QuizInState, QuizMeta } from '../../-hooks/useQuiz';
import { useTimer } from '../../-hooks/useTimer';
import {
  formatTime,
  formatAccuracy,
  getGamePromptFont,
  getGameChoicesFont,
} from '../../-lib/utils';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardFooter } from '@/components/ui/card';
import type { MultiChoiceQuestion } from '@/lib/models';
import { ArrowRightDoubleIcon } from '@hugeicons/core-free-icons';
import { HugeiconsIcon } from '@hugeicons/react';
import { useState } from 'react';

export interface MultiChoiceGameProps {
  question: MultiChoiceQuestion;
  currentIndex: number;
  meta: QuizMeta<MultiChoiceQuestion>;
  submitAnswer: QuizInState<MultiChoiceQuestion>['submitAnswer'];
}

export function MultiChoiceGame({
  question,
  currentIndex,
  meta,
  submitAnswer,
}: MultiChoiceGameProps) {
  const [guesses, setGuesses] = useState(0);

  const [didGuessChoice, setDidGuessChoice] = useState<boolean[]>(
    question.choices.map(() => false)
  );

  const time = useTimer();
  const timeStr = formatTime(time);

  const acc = 1 / guesses;
  const accStr = formatAccuracy(acc);

  const onGuess = (idx: number) => {
    const isCorrect = idx === question.answer_idx;
    if (isCorrect) {
      submitAnswer({
        guesses: guesses + 1,
        elapsed: 0,
        source: question,
      });
      return;
    }

    setGuesses((prev) => prev + 1);
    setDidGuessChoice((prev) => {
      prev[idx] = true;
      return [...prev];
    });
  };

  const onSkip = () => {
    submitAnswer({
      guesses: Infinity,
      elapsed: time,
      source: question,
    });
  };

  return (
    <div className="flex w-full max-w-4xl flex-col gap-1 font-semibold">
      <div className="flex w-full flex-row flex-wrap justify-between px-4 *:grow *:text-nowrap">
        <span className="text-left">{guesses} Guesses</span>
        <span className="text-center">{timeStr}</span>
        <span className="text-right">{accStr} Acc</span>
      </div>

      {/* Prompt */}
      <Card className="gap-0 pb-2">
        <CardContent className="flex flex-col items-center gap-6 text-xl">
          <div
            className={`${getGamePromptFont(meta.mode)} m-2 line-clamp-4 max-w-[30ch] text-center`}
          >
            {question.prompt}
          </div>
        </CardContent>

        <CardFooter className="justify-between pr-3 pl-5">
          <span className="text-muted-foreground">
            Question {currentIndex + 1} of {meta.questions.length}
          </span>
          <Button
            onClick={onSkip}
            variant="outline"
            className="gap-1 px-2 py-0 font-semibold text-muted-foreground"
          >
            Skip
            <HugeiconsIcon
              icon={ArrowRightDoubleIcon}
              size={24}
              color="currentColor"
              strokeWidth={1.5}
            />
          </Button>
        </CardFooter>
      </Card>

      {/* Choices */}
      <div className="mt-1.5 grid grid-cols-2 gap-2 self-center sm:grid-cols-4">
        {question.choices.map((choice, idx) => (
          <Button
            key={choice}
            onClick={() => onGuess(idx)}
            disabled={didGuessChoice[idx]}
            size="lg"
            className={`${getGameChoicesFont(meta.mode)} line-clamp-4 h-[4.5lh] whitespace-normal`}
          >
            {choice}
          </Button>
        ))}
      </div>
    </div>
  );
}
