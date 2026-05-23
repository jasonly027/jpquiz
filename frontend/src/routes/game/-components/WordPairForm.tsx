import { Button } from '@/components/ui/button';
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
import { Field, FieldLabel } from '@/components/ui/field';
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Spinner } from '@/components/ui/spinner';
import type { GameMode, NLevel, PartOfSpeechCategory } from '@/lib/models';
import { cn } from '@/lib/utils';
import type { ComponentProps, Dispatch, SetStateAction } from 'react';
import React from 'react';

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

export function FieldGameMode({
  setMode,
}: {
  setMode: Dispatch<SetStateAction<GameMode | undefined>>;
}) {
  return (
    <Field orientation="responsive">
      <FieldLabel htmlFor="gameModeSelect" className="flex-1 text-nowrap">
        Game Mode
      </FieldLabel>

      <div>
        <Select
          name="mode"
          onValueChange={(v) => setMode(v as GameMode)}
          required
        >
          <SelectTrigger id="gameModeSelect">
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
      </div>
    </Field>
  );
}

const N_LEVELS = ['N1', 'N2', 'N3', 'N4', 'N5'] as const satisfies NLevel[];

export function FieldNLevel({
  setLevels,
}: {
  setLevels: Dispatch<SetStateAction<NLevel[] | undefined>>;
}) {
  const levelsAnchor = useComboboxAnchor();

  return (
    <Field orientation="responsive">
      <FieldLabel htmlFor="nLevelCombo" className="text-nowrap">
        JLPT NLevel
      </FieldLabel>

      <div className="max-w-58 flex-1/4">
        <Combobox
          id="nLevelCombo"
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
      </div>
    </Field>
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

export function FieldCategory({
  setCategories,
}: {
  setCategories: Dispatch<SetStateAction<PartOfSpeechCategory[] | undefined>>;
}) {
  const anchor = useComboboxAnchor();

  return (
    <Field orientation="responsive">
      <FieldLabel htmlFor="categoryCombo" className="text-nowrap">
        Categories
      </FieldLabel>

      <div className="max-w-58 flex-1/4">
        <Combobox
          id="categoryCombo"
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
                    placeholder={
                      categories.length > 0 ? '' : 'Select Categories'
                    }
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
      </div>
    </Field>
  );
}

export type FieldSubmitProps = ComponentProps<'button'>;

export function FieldSubmit({
  disabled,
  className,
  ...props
}: FieldSubmitProps) {
  return (
    <Field>
      <Button
        type="submit"
        className={cn(`max-w-40 self-center`, className)}
        disabled={disabled}
        {...props}
      >
        {disabled && <Spinner data-icon="inline-start" />}
        Create Game
      </Button>
    </Field>
  );
}
