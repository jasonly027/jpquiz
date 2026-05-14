import { InGameLayout } from '../-components/InGameLayout';
import { useQuiz } from '../-hooks/useQuiz';
import { CreateMultiChoiceGame } from './-components/CreateMultiChoiceGame';
import { MultiChoiceGame } from './-components/MultiChoiceGame';
import { MultiChoiceStats } from './-components/MultiChoiceStats';
import type { MultiChoiceQuestion } from '@/lib/models';
import { createFileRoute } from '@tanstack/react-router';
import { useEffect } from 'react';

export const Route = createFileRoute('/game/multiple-choice/')({
  component: RouteComponent,
});

function RouteComponent() {
  const { state, initQuiz, question, currentIndex, meta, stats, submitAnswer } =
    useQuiz<MultiChoiceQuestion>();

  // TODO: Remove me
  useEffect(() => {
    const q1: MultiChoiceQuestion = {
      prompt:
        'This is a very very long prompt that takes many words and spans a long line of text',
      choices: [
        'Choice A',
        'This is a very very long choice that takes many characters',
        'Choice C',
        'Choice D',
      ],
      answer_idx: 1,
      word_pair: {
        id: 'id',
        kana: 'Kana',
        kanji: 'Kanji',
        level: 'N1',
        senses: [
          {
            glossary: ['gloss1', 'gloss2'],
            partsOfSpeech: ['noun'],
          },
        ],
      },
    };
    initQuiz?.({
      questions: [q1],
      mode: 'kanatoeng',
      levels: ['N1'],
      categories: ['nouns', 'verbs'],
    });
    submitAnswer?.({
      elapsed: 120,
      guesses: Infinity,
      source: q1,
    });
  }, [initQuiz]);

  const content = (() => {
    switch (state) {
      case 'pre':
        return <CreateMultiChoiceGame initQuiz={initQuiz} />;
      case 'in':
        return (
          <MultiChoiceGame
            key={currentIndex}
            question={question}
            currentIndex={currentIndex}
            meta={meta}
            submitAnswer={submitAnswer}
          />
        );
      case 'post':
        return <MultiChoiceStats stats={stats} meta={meta} />;
    }
  })();

  return (
    <InGameLayout title="Multiple Choice">
      <div className="mx-4 flex justify-center">{content}</div>
    </InGameLayout>
  );
}
