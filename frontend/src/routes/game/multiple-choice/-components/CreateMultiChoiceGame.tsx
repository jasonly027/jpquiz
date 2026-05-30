import {
  FieldCategory,
  FieldGameMode,
  FieldNLevel,
  FieldSubmit,
} from '../../-components/WordPairForm';
import type { QuizPreState } from '../../-hooks/useQuiz';
import { useGetMultiChoice } from '@/api/server';
import { Card, CardContent } from '@/components/ui/card';
import { FieldGroup, FieldLegend, FieldSet } from '@/components/ui/field';
import type {
  GameMode,
  MultiChoiceQuestion,
  NLevel,
  PartOfSpeechCategory,
} from '@/lib/models';
import { isAxiosError } from 'axios';
import { useState, type SubmitEventHandler } from 'react';

export type CreateMultiChoiceGameProps = QuizPreState<MultiChoiceQuestion>;

export function CreateMultiChoiceGame({
  initQuiz,
}: CreateMultiChoiceGameProps) {
  const [mode, setMode] = useState<GameMode>();
  const [levels, setLevels] = useState<NLevel[]>();
  const [categories, setCategories] = useState<PartOfSpeechCategory[]>();

  const getGame = useGetMultiChoice({
    mutation: {
      onSuccess(
        { data: questions },
        { params: { mode, levels, pos: categories } }
      ) {
        initQuiz({
          questions,
          mode,
          levels,
          categories,
        });
      },
      onError(error) {
        console.error(error);
      },
    },
  });

  let getGameError: string | undefined;
  if (getGame.error) {
    if (isAxiosError(getGame.error) && getGame.error.status === 422) {
      getGameError =
        'Word pool is too small to create a game. Please try different settings.';
    } else {
      getGameError = 'Something went wrong. Please try again.';
    }
  }

  const onSubmit: SubmitEventHandler = (e) => {
    e.preventDefault();
    if (!mode || !levels || !categories) return;
    getGame.mutate({ params: { mode, levels, pos: categories } });
  };

  return (
    <Card variant="outline" className="w-full max-w-xl">
      <CardContent>
        <form onSubmit={onSubmit}>
          <FieldSet>
            <FieldLegend className="w-full text-center font-semibold">
              Create a Game
            </FieldLegend>

            <FieldGroup>
              <FieldGameMode setMode={setMode} />
              <FieldNLevel setLevels={setLevels} />
              <FieldCategory setCategories={setCategories} />
              <FieldSubmit disabled={getGame.isPending} />
            </FieldGroup>
          </FieldSet>
        </form>

        {getGameError && (
          <div className="mt-3 text-center text-destructive">
            {getGameError}
          </div>
        )}
      </CardContent>
    </Card>
  );
}
