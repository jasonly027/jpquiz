import {
  RetireButton,
  SkipButton,
  WordPairCard,
  WordPairCardContent,
  WordPairCardActions,
  WordPairStatBar,
} from '../../-components/WordPairGame';
import type { QuizInState } from '../../-hooks/useQuiz';
import { useTimer } from '../../-hooks/useTimer';
import { getGamePromptFont, getGameChoicesFont } from '../../-lib/utils';
import { Button } from '@/components/ui/button';
import type { MultiChoiceQuestion } from '@/lib/models';
import { cn } from '@/lib/utils';
import { useState, type ComponentProps } from 'react';

export type MultiChoiceGameProps = QuizInState<MultiChoiceQuestion>;

export function MultiChoiceGame({
  question,
  currentIndex,
  meta,
  submitAnswer,
  endQuiz,
}: MultiChoiceGameProps) {
  const [guesses, setGuesses] = useState(0);
  const elapsedSecs = useTimer();

  const onGuess = (idx: number) => {
    const isCorrect = idx === question.answer_idx;
    if (isCorrect) {
      submitAnswer({
        guesses: guesses + 1,
        elapsed: elapsedSecs,
        source: question,
      });
      return;
    }

    setGuesses((prev) => prev + 1);
  };

  const onSkip = () => {
    submitAnswer({
      guesses: Infinity,
      elapsed: elapsedSecs,
      source: question,
    });
  };

  return (
    <div className="flex w-full max-w-6xl flex-col gap-1 font-semibold">
      <WordPairStatBar guesses={guesses} seconds={elapsedSecs} />

      <WordPairCard>
        <WordPairCardActions>
          <div className="flex-1"></div>
          <RetireButton onClick={endQuiz} className="pr-0!" />
        </WordPairCardActions>

        <WordPairCardContent>
          <span
            className={`${getGamePromptFont(meta.mode)} m-2 line-clamp-4 max-w-[40ch] text-center select-none sm:text-3xl`}
          >
            {question.prompt}
          </span>
        </WordPairCardContent>

        <WordPairCardActions>
          <span className="flex items-center text-muted-foreground">
            Question {currentIndex + 1} of {meta.questions.length}
          </span>

          <div className="flex-1"></div>

          <SkipButton onClick={onSkip} />
        </WordPairCardActions>
      </WordPairCard>

      <div className="mt-1.5 grid grid-cols-2 gap-2 sm:grid-cols-4">
        {question.choices.map((choice, idx) => (
          <OnceButton
            key={idx}
            onClick={() => onGuess(idx)}
            className={getGameChoicesFont(meta.mode)}
          >
            {choice}
          </OnceButton>
        ))}
      </div>
    </div>
  );
}

type OnceButtonProps = {} & ComponentProps<'button'>;

function OnceButton({ onClick, className, ...props }: OnceButtonProps) {
  const [disabled, setDisabled] = useState(false);

  return (
    <Button
      onClick={(e) => {
        setDisabled(true);
        return onClick?.(e);
      }}
      disabled={disabled}
      size="lg"
      className={cn(
        'line-clamp-4 h-[4.5lh] text-lg whitespace-normal',
        className
      )}
      {...props}
    />
  );
}
