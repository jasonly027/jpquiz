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

export interface QuizPreState<T> {
  state: 'pre';
  initQuiz(data: QuizMeta<T>): void;
}

export interface QuizInState<T> {
  state: 'in';
  question: Readonly<T>;
  currentIndex: number;
  meta: Readonly<QuizMeta<T>>;
  submitAnswer(stat: QuestionStat<T>): void;
  endQuiz(): void;
}

export interface QuizPostState<T> {
  state: 'post';
  meta: Readonly<QuizMeta<T>>;
  stats: QuestionStat<T>[];
  initQuiz(data: QuizMeta<T>): void;
  reset: () => void;
}

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

  const endQuiz = useCallback(() => {
    if (!meta) return;
    setCurrentIndex(meta.questions.length);
  }, [meta]);

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
      submitAnswer,
      endQuiz,
    };
  }

  return {
    state: 'post',
    meta,
    stats,
    initQuiz,
    reset,
  };
}
