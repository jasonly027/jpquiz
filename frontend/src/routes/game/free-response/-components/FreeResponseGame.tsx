import { HintBox, HintToggleButton } from '../../-components/Hint';
import {
  RetireButton,
  SkipButton,
  WordPairCard,
  WordPairCardContent,
  WordPairCardActions,
  WordPairStatBar,
} from '../../-components/WordPairGame';
import {
  isHiraganaChar,
  useHint,
  type UseHintOptions,
  type UseHintValue,
} from '../../-hooks/useHint';
import type { QuizInState } from '../../-hooks/useQuiz';
import { useShowHint } from '../../-hooks/useShowHint';
import { useTimer } from '../../-hooks/useTimer';
import {
  getGameChoicesFont,
  getGamePromptFont,
  isKanjiChoices,
} from '../../-lib/utils';
import { Input } from '@/components/ui/input';
import {
  getCommentedMask,
  type FreeResponseQuestion,
  type GameMode,
} from '@/lib/models';
import { useMemo, useRef, useState } from 'react';

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

  const { hint, revealOne, showHint, hintCommentMask } = useFreeResponseHint(
    question.answers[0]!,
    meta.mode
  );

  const inputRef = useRef<HTMLInputElement>(null);
  const onGuess = () => {
    if (!inputRef.current?.value) return;

    if (question.isAnswer(inputRef.current.value.trim())) {
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
    <div className="flex w-full max-w-6xl flex-col gap-1 font-semibold">
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
              className={`${getGamePromptFont(meta.mode)} m-2 line-clamp-4 max-w-[40ch] text-center select-none sm:text-3xl`}
            >
              {question.prompt}
            </div>
            {showHint && (
              <HintBox
                hint={hint}
                mode={meta.mode}
                mutedMask={hintCommentMask}
              />
            )}
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

interface UseFreeResponseHintValue extends UseHintValue {
  showHint: boolean;
  hintCommentMask: boolean[];
}

function useFreeResponseHint(
  hintTarget: string,
  mode: GameMode
): UseFreeResponseHintValue {
  const hintCommentMask = useMemo(
    () => getCommentedMask(hintTarget),
    [hintTarget]
  );

  const autoRevealFilters: UseHintOptions['autoRevealFilters'] = [];
  if (isKanjiChoices(mode)) {
    autoRevealFilters.push(isHiraganaChar);
  }
  autoRevealFilters.push((_, idx) => !!hintCommentMask[idx]);

  const { hint, revealOne } = useHint(hintTarget, {
    minRemaining: 1,
    autoRevealFilters,
  });

  const [showHint] = useShowHint();

  return {
    hint,
    revealOne,
    showHint,
    hintCommentMask,
  };
}
