import { InGameLayout } from '../-components/InGameLayout';
import { useQuiz } from '../-hooks/useQuiz';
import { CreateFreeResponseGame } from './-components/CreateFreeResponseGame';
import { FreeResponseGame } from './-components/FreeResponseGame';
import { FreeResponseStats } from './-components/FreeResponseStats';
import type { FreeResponseQuestion } from '@/lib/models';
import { createFileRoute } from '@tanstack/react-router';
import { useEffect } from 'react';

export const Route = createFileRoute('/game/free-response/')({
  component: RouteComponent,
});

function RouteComponent() {
  const quizState = useQuiz<FreeResponseQuestion>();

  // TODO: Remove me
  useEffect(() => {
    if (quizState.state !== 'pre') return;

    const q1: FreeResponseQuestion = {
      prompt:
        'This is a very very long prompt that takes many words and spans a long line of text',
      answers: ['a1', 'answer 2', 'answer 3'],
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
  }, []);

  const content = (() => {
    switch (quizState.state) {
      case 'pre':
        return <CreateFreeResponseGame {...quizState} />;
      case 'in':
        return <FreeResponseGame key={quizState.currentIndex} {...quizState} />;
      case 'post':
        return <FreeResponseStats {...quizState} />;
    }
  })();

  return <InGameLayout title="Free Response">{content}</InGameLayout>;
}
