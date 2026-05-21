import type { QuizPreState } from '../../-hooks/useQuiz';
import { useGetMultiChoice } from '@/api/server';
import { Button } from '@/components/ui/button';
import { Card, CardContent } from '@/components/ui/card';
import {
  Combobox,
  ComboboxChip,
  ComboboxChips,
  ComboboxChipsInput,
  ComboboxContent,
  ComboboxEmpty,
  ComboboxItem,
  ComboboxList,
  ComboboxValue,
  useComboboxAnchor,
} from '@/components/ui/combobox';
import {
  Field,
  FieldGroup,
  FieldLabel,
  FieldLegend,
  FieldSet,
} from '@/components/ui/field';
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import type {
  GameMode,
  MultiChoiceQuestion,
  NLevel,
  PartOfSpeechCategory,
} from '@/lib/models';
import React, {
  useState,
  type Dispatch,
  type SetStateAction,
  type SubmitEventHandler,
} from 'react';

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

  // TODO: ADD ERRORS ON SUBMIT

  const onSubmit: SubmitEventHandler = (e) => {
    e.preventDefault();
    if (!mode || !levels || !categories) return;
    getGame.mutate({ params: { mode, levels, pos: categories } });
  };

  return (
    <Card variant="outline" className="w-full max-w-xl">
      <CardContent>
        <form onSubmit={onSubmit}>
          <FieldSet className="grow">
            <FieldLegend className="w-full text-center font-semibold">
              Create a Game
            </FieldLegend>

            <FieldGroup>
              {/* Game Mode selector */}
              <Field orientation="responsive">
                <FieldLabel
                  htmlFor="gameModeSelect"
                  className="flex-1 text-nowrap"
                >
                  Game Mode
                </FieldLabel>

                <div>
                  <GameModeComboBox id="gameModeSelect" setMode={setMode} />
                </div>
              </Field>

              {/* NLevel selector */}
              <Field orientation="responsive">
                <FieldLabel htmlFor="nLevelCombo" className="text-nowrap">
                  JLPT NLevel
                </FieldLabel>

                <div className="max-w-58 flex-1/4">
                  <NLevelComboBox id="nLevelCombo" setLevels={setLevels} />
                </div>
              </Field>

              {/* Category selector */}
              <Field orientation="responsive">
                <FieldLabel htmlFor="categoryCombo" className="text-nowrap">
                  Categories
                </FieldLabel>

                <div className="max-w-58 flex-1/4">
                  <CategoryComboBox
                    id="categoryCombo"
                    setCategories={setCategories}
                  />
                </div>
              </Field>

              {/* Submit */}
              <Field>
                <Button
                  disabled={getGame.isPending}
                  className="max-w-40 self-center"
                >
                  Create Game
                </Button>
              </Field>
            </FieldGroup>
          </FieldSet>
        </form>
      </CardContent>
    </Card>
  );
}

const GAME_MODES: { name: string; value: GameMode }[] = [
  {
    name: 'English to Kana',
    value: 'engtokana',
  },
  {
    name: 'English to Kanji',
    value: 'engtokanji',
  },
  {
    name: 'Kana to English',
    value: 'kanatoeng',
  },
  {
    name: 'Kana to Kanji',
    value: 'kanatokanji',
  },
  {
    name: 'Kanji to English',
    value: 'kanjitoeng',
  },
  {
    name: 'Kanji to Kana',
    value: 'kanjitokana',
  },
];

function GameModeComboBox({
  id,
  setMode,
}: {
  id: string;
  setMode: Dispatch<SetStateAction<GameMode | undefined>>;
}) {
  return (
    <Select name="mode" onValueChange={(v) => setMode(v as GameMode)} required>
      <SelectTrigger id={id} className="">
        <SelectValue placeholder="Select Game Mode" />
      </SelectTrigger>
      <SelectContent>
        <SelectGroup>
          {GAME_MODES.map(({ name, value }) => (
            <SelectItem key={value} value={value}>
              {name}
            </SelectItem>
          ))}
        </SelectGroup>
      </SelectContent>
    </Select>
  );
}

const N_LEVELS = ['N1', 'N2', 'N3', 'N4', 'N5'] as const satisfies NLevel[];

function NLevelComboBox({
  id,
  setLevels,
}: {
  id: string;
  setLevels: Dispatch<SetStateAction<NLevel[] | undefined>>;
}) {
  const levelsAnchor = useComboboxAnchor();

  return (
    <Combobox
      id={id}
      items={N_LEVELS}
      onValueChange={setLevels}
      multiple
      autoHighlight
      required
    >
      <ComboboxChips ref={levelsAnchor}>
        <ComboboxValue>
          {(levels: typeof N_LEVELS) => (
            <React.Fragment>
              {levels.map((level) => (
                <ComboboxChip key={level} showRemove={false}>
                  {level}
                </ComboboxChip>
              ))}
              <ComboboxChipsInput
                placeholder={levels.length > 0 ? '' : 'Select NLevel(s)'}
                className="max-w-xs min-w-5"
              />
            </React.Fragment>
          )}
        </ComboboxValue>
      </ComboboxChips>

      <ComboboxContent anchor={levelsAnchor}>
        <ComboboxEmpty>No NLevel found.</ComboboxEmpty>
        <ComboboxList>
          {(item) => (
            <ComboboxItem key={item} value={item}>
              {item}
            </ComboboxItem>
          )}
        </ComboboxList>
      </ComboboxContent>
    </Combobox>
  );
}

const CATEGORIES: { name: string; value: PartOfSpeechCategory }[] = [
  { name: 'Nouns', value: 'nouns' },
  { name: 'Verbs', value: 'verbs' },
  { name: 'Adjectives', value: 'adjectives' },
  { name: 'Adverbs', value: 'adverbs' },
  { name: 'Expressions', value: 'expressions' },
  { name: 'Conjunctions', value: 'conjunctions' },
  { name: 'Other', value: 'other' },
];

function CategoryComboBox({
  id,
  setCategories,
}: {
  id: string;
  setCategories: Dispatch<SetStateAction<PartOfSpeechCategory[] | undefined>>;
}) {
  const anchor = useComboboxAnchor();

  return (
    <Combobox
      id={id}
      items={CATEGORIES}
      onValueChange={(c: typeof CATEGORIES) =>
        setCategories(c.map(({ value }) => value))
      }
      multiple
      autoHighlight
      required
    >
      <ComboboxChips ref={anchor}>
        <ComboboxValue>
          {(categories: typeof CATEGORIES) => (
            <React.Fragment>
              {categories.map(({ name, value }) => {
                return (
                  <ComboboxChip key={value} showRemove={false}>
                    {name}
                  </ComboboxChip>
                );
              })}
              <ComboboxChipsInput
                placeholder={categories.length > 0 ? '' : 'Select Categories'}
                className="max-w-xs min-w-5"
              />
            </React.Fragment>
          )}
        </ComboboxValue>
      </ComboboxChips>

      <ComboboxContent anchor={anchor}>
        <ComboboxEmpty>No category found.</ComboboxEmpty>
        <ComboboxList>
          {(category: (typeof CATEGORIES)[number]) => (
            <ComboboxItem key={category.value} value={category}>
              {category.name}
            </ComboboxItem>
          )}
        </ComboboxList>
      </ComboboxContent>
    </Combobox>
  );
}
