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
  const quizState = useQuiz<MultiChoiceQuestion>();

  // TODO: Remove me
  useEffect(() => {
    return;
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
        kana: 'はんのう',
        kanji: '反応',
        level: 'N1',
        senses: [
          {
            glossary: ['gloss1', 'gloss2'],
            partsOfSpeech: ['noun'],
          },
        ],
      },
    };
    quizState.initQuiz?.({
      questions: [q1],
      mode: 'kanatoeng',
      levels: ['N1'],
      categories: ['nouns', 'verbs'],
    });
    quizState.submitAnswer?.({
      elapsed: 120,
      guesses: Infinity,
      source: q1,
    });
  }, []);

  const content = (() => {
    switch (quizState.state) {
      case 'pre':
        return <CreateMultiChoiceGame {...quizState} />;
      case 'in':
        return <MultiChoiceGame key={quizState.currentIndex} {...quizState} />;
      case 'post':
        return <MultiChoiceStats {...quizState} />;
    }
  })();

  return (
    <InGameLayout title="Multiple Choice">
      <div className="mx-4 flex justify-center">{content}</div>
    </InGameLayout>
  );
}
