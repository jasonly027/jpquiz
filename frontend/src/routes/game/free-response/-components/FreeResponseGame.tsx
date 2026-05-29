import { HintBox, HintToggleButton } from '../../-components/Hint';
import {
  RetireButton,
  SkipButton,
  WordPairCard,
  WordPairCardContent,
  WordPairCardActions,
  WordPairStatBar,
} from '../../-components/WordPairGame';
import { useHint } from '../../-hooks/useHint';
import type { QuizInState } from '../../-hooks/useQuiz';
import { useShowHint } from '../../-hooks/useShowHint';
import { useTimer } from '../../-hooks/useTimer';
import { getGameChoicesFont, getGamePromptFont } from '../../-lib/utils';
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

  const isKanjiResponse =
    meta.mode === 'engtokanji' || meta.mode === 'kanatokanji';
  const { hint, revealOne } = useHint(question.answers[0]!, {
    minRemaining: 1,
    showHiragana: isKanjiResponse,
  });
  const [showHint] = useShowHint();

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
    revealOne();
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
          <HintToggleButton className="pl-0!" />
          <div className="flex-1"></div>
          <RetireButton onClick={endQuiz} className="pr-0!" />
        </WordPairCardActions>

        <WordPairCardContent>
          <div className="flex flex-col items-center gap-4">
            <div
              className={`${getGamePromptFont(meta.mode)} m-2 line-clamp-4 max-w-[30ch] text-center select-none sm:text-xl`}
            >
              {question.prompt}
            </div>
            {showHint && <HintBox hint={hint} mode={meta.mode} />}
          </div>
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
            className={`${getGameChoicesFont(meta.mode)} h-[2lh] text-center text-sm sm:text-xl md:text-xl`}
          />
        </form>
      </div>
    </div>
  );
}
