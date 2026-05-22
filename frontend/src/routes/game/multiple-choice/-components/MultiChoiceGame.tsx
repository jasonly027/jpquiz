import type { QuizInState } from '../../-hooks/useQuiz';
import { useTimer } from '../../-hooks/useTimer';
import {
  formatTime,
  formatAccuracy,
  getGamePromptFont,
  getGameChoicesFont,
} from '../../-lib/utils';
import { Button } from '@/components/ui/button';
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
} from '@/components/ui/card';
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog';
import type { MultiChoiceQuestion } from '@/lib/models';
import {
  ArrowRightDoubleIcon,
  StopWatchIcon,
} from '@hugeicons/core-free-icons';
import { HugeiconsIcon } from '@hugeicons/react';
import { useState } from 'react';

export type MultiChoiceGameProps = QuizInState<MultiChoiceQuestion>;

export function MultiChoiceGame({
  question,
  currentIndex,
  meta,
  submitAnswer,
  endQuiz,
}: MultiChoiceGameProps) {
  const [guesses, setGuesses] = useState(0);

  const [didGuessChoice, setDidGuessChoice] = useState<boolean[]>(
    question.choices.map(() => false)
  );

  const time = useTimer();
  const timeStr = formatTime(time);

  const acc = 1 / (guesses + 1);
  const accStr = formatAccuracy(acc);

  const onGuess = (idx: number) => {
    const isCorrect = idx === question.answer_idx;
    if (isCorrect) {
      submitAnswer({
        guesses: guesses + 1,
        elapsed: time,
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
      <div className="flex flex-row flex-wrap justify-between gap-1 px-4 text-center *:grow *:text-nowrap max-[375px]:flex-col max-[375px]:*:text-center">
        <span className="text-left">{guesses} Guesses</span>
        <span className="text-center">{timeStr}</span>
        <span className="text-right">{accStr} Acc</span>
      </div>

      {/* Prompt */}
      <Card className="min-h-50 justify-center gap-0 py-2">
        <CardHeader className="flex flex-1 justify-end pr-2.5">
          <RetireButton endQuiz={endQuiz} />
        </CardHeader>

        <CardContent className="flex flex-col items-center gap-6">
          <div
            className={`${getGamePromptFont(meta.mode)} m-2 line-clamp-4 max-w-[30ch] text-center text-xl`}
          >
            {question.prompt}
          </div>
        </CardContent>

        <CardFooter className="flex flex-1 flex-col items-stretch justify-end pr-2.5 pl-5">
          <div className="flex items-baseline justify-between">
            <div className="text-muted-foreground">
              Question {currentIndex + 1} of {meta.questions.length}
            </div>
            <Button
              onClick={onSkip}
              variant="ghost"
              className="font-semibold text-muted-foreground"
            >
              Skip
              <HugeiconsIcon
                icon={ArrowRightDoubleIcon}
                size={24}
                color="currentColor"
                strokeWidth={1.5}
                data-icon="inline-end"
              />
            </Button>
          </div>
        </CardFooter>
      </Card>

      {/* Choices */}
      <div className="mt-1.5 grid grid-cols-2 gap-2 sm:grid-cols-4">
        {question.choices.map((choice, idx) => (
          <Button
            key={choice}
            onClick={() => onGuess(idx)}
            disabled={didGuessChoice[idx]}
            size="lg"
            className={`${getGameChoicesFont(meta.mode)} line-clamp-4 h-[4.5lh] text-lg whitespace-normal`}
          >
            {choice}
          </Button>
        ))}
      </div>
    </div>
  );
}

interface RetireButtonProps {
  endQuiz: () => void;
}

function RetireButton({ endQuiz }: RetireButtonProps) {
  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="ghost" className="font-semibold text-muted-foreground">
          Retire
          <HugeiconsIcon
            icon={StopWatchIcon}
            size={24}
            color="currentColor"
            strokeWidth={1.5}
            data-icon="inline-end"
          />
        </Button>
      </DialogTrigger>

      <DialogContent>
        <DialogHeader>
          <DialogTitle>Are you sure you want to retire?</DialogTitle>

          <DialogDescription>
            Retiring is as if you skipped all remaining questions. You will be
            able to review the questions you have answered up to retirement.
          </DialogDescription>
        </DialogHeader>

        <DialogFooter>
          <DialogClose asChild>
            <Button variant="outline">Cancel</Button>
          </DialogClose>
          <Button onClick={endQuiz}>Retire</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
