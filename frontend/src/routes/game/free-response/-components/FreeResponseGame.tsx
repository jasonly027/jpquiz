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
import { getGamePromptFont } from '../../-lib/utils';
import { Input } from '@/components/ui/input';
import type { FreeResponseQuestion } from '@/lib/models';
import { useRef, useState } from 'react';

export type FreeResponseGameProps = QuizInState<FreeResponseQuestion>;

export function FreeResponseGame({
  question,
  currentIndex,
  meta,
  submitAnswer,
  endQuiz,
}: FreeResponseGameProps) {
  const [guesses, setGuesses] = useState(0);
  const elapsedSecs = useTimer();

  const inputRef = useRef<HTMLInputElement>(null);

  const onGuess = () => {
    if (!inputRef.current?.value) return;

    const isCorrect = question.answers.includes(inputRef.current.value);
    if (isCorrect) {
      submitAnswer({
        guesses: guesses + 1,
        elapsed: elapsedSecs,
        source: question,
      });
      return;
    }

    setGuesses((prev) => prev + 1);
    inputRef.current.value = '';
  };

  const onSkip = () => {
    submitAnswer({
      guesses: Infinity,
      elapsed: elapsedSecs,
      source: question,
    });
  };

  return (
    <div className="flex w-full max-w-4xl flex-col gap-1 font-semibold">
      <WordPairStatBar guesses={guesses} seconds={elapsedSecs} />

      <WordPairCard>
        <WordPairCardActions>
          <div className="flex-1"></div>
          <RetireButton onClick={endQuiz} className="pr-0!" />
        </WordPairCardActions>

        <WordPairCardContent>
          <span
            className={`${getGamePromptFont(meta.mode)} m-2 line-clamp-4 max-w-[30ch] text-center select-none sm:text-xl`}
          >
            {question.prompt}
          </span>
        </WordPairCardContent>

        <WordPairCardActions>
          <span className="flex items-center text-muted-foreground">
            Question {currentIndex + 1} of {meta.questions.length}
          </span>

          <div className="flex-1"></div>

          <SkipButton onClick={onSkip} className="pr-0!" />
        </WordPairCardActions>
      </WordPairCard>

      <div className="mt-1.5">
        <form
          onSubmit={(e) => {
            e.preventDefault();
            onGuess();
          }}
        >
          <Input
            ref={inputRef}
            autoFocus
            type="text"
            placeholder="Input your answer"
            className="h-[2lh] text-center text-sm sm:text-xl md:text-xl"
          />
        </form>
      </div>
    </div>
  );
}
