import type { QuestionStat } from '../-lib/models';
import type { GameMode, NLevel, PartOfSpeechCategory } from '@/lib/models';
import { useCallback, useState } from 'react';

export type UseQuizValue<T> =
  | QuizPreState<T>
  | QuizInState<T>
  | QuizPostState<T>;

export interface QuizMeta<T> {
  questions: T[];
  mode: GameMode;
  levels: NLevel[];
  categories: PartOfSpeechCategory[];
}

interface QuizStateShell {
  question: undefined;
  currentIndex: undefined;
  meta: undefined;
  stats: undefined;
  initQuiz: undefined;
  submitAnswer: undefined;
  reset: undefined;
}

interface QuizPreStateShape<T> {
  state: 'pre';
  initQuiz(data: QuizMeta<T>): void;
}
export type QuizPreState<T> = QuizPreStateShape<T> & {
  [K in Exclude<keyof QuizStateShell, keyof QuizPreStateShape<T>>]?: undefined;
};

interface QuizInStateShape<T> {
  state: 'in';
  question: Readonly<T>;
  currentIndex: number;
  meta: Readonly<QuizMeta<T>>;
  stats: QuestionStat<T>[];
  submitAnswer(stat: QuestionStat<T>): void;
}
export type QuizInState<T> = QuizInStateShape<T> & {
  [K in Exclude<keyof QuizStateShell, keyof QuizInStateShape<T>>]?: undefined;
};

interface QuizPostStateShape<T> {
  state: 'post';
  question: Readonly<T>;
  currentIndex: number;
  meta: Readonly<QuizMeta<T>>;
  stats: QuestionStat<T>[];
  initQuiz(data: QuizMeta<T>): void;
  reset: () => void;
}
export type QuizPostState<T> = QuizPostStateShape<T> & {
  [K in Exclude<keyof QuizStateShell, keyof QuizPostStateShape<T>>]?: undefined;
};

export function useQuiz<T>(): UseQuizValue<T> {
  const [meta, setMeta] = useState<QuizMeta<T>>();
  const [currentIndex, setCurrentIndex] = useState<number>(0);

  const [stats, setStats] = useState<QuestionStat<T>[]>([]);

  const isComplete =
    meta !== undefined && currentIndex >= meta.questions.length;

  const initQuiz = useCallback((meta: QuizMeta<T>) => {
    setMeta(meta);
    setCurrentIndex(0);
    setStats([]);
  }, []);

  const submitAnswer = useCallback(
    (stat: Parameters<QuizInState<T>['submitAnswer']>[0]) => {
      if (isComplete) return;
      setStats((prev) => [...prev, stat]);
      setCurrentIndex((prev) => prev + 1);
    },
    [isComplete]
  );

  const reset = useCallback(() => {
    setMeta(undefined);
    setCurrentIndex(0);
  }, []);

  if (meta === undefined) {
    return {
      state: 'pre',
      initQuiz,
    };
  }

  if (!isComplete) {
    return {
      state: 'in',
      question: meta.questions[currentIndex]!,
      meta: meta,
      currentIndex,
      stats,
      submitAnswer,
    };
  }

  return {
    state: 'post',
    question: meta.questions[currentIndex]!,
    meta: meta,
    currentIndex,
    stats,
    initQuiz,
    reset,
  };
}
