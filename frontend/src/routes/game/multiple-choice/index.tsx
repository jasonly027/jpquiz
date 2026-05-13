import { InGameLayout } from '../-components/InGameLayout';
import { Stats } from '../-components/Stats';
import { useQuiz } from '../-hooks/useQuiz';
import { CreateGameMenu } from './-components/CreateGameMenu';
import { Game } from './-components/Game';
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
    return;
    initQuiz?.({
      questions: [q1],
      mode: 'kanatoeng',
      levels: ['N1'],
      categories: ['nouns', 'verbs'],
    });
    return;
    submitAnswer?.({
      elapsed: 120,
      guesses: 3,
      source: q1,
    });
  }, [initQuiz]);

  const content = (() => {
    switch (state) {
      case 'pre':
        return <CreateGameMenu initQuiz={initQuiz} />;
      case 'in':
        return (
          <Game
            key={currentIndex}
            question={question}
            currentIndex={currentIndex}
            meta={meta}
            submitAnswer={submitAnswer}
          />
        );
      case 'post':
        return <Stats stats={stats} />;
    }
  })();

  return (
    <InGameLayout title="Multiple Choice">
      <div className="mx-4 flex justify-center">{content}</div>
    </InGameLayout>
  );
}
