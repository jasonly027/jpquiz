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
import {
  POS_CATEGORIES,
  N_LEVELS,
  type GameMode,
  type NLevel,
  type PartOfSpeechCategory,
  POS_VIEW,
} from '@/lib/models';
import { cn } from '@/lib/utils';
import type { ComponentProps, Dispatch, SetStateAction } from 'react';
import React, { useId, useState } from 'react';

type GameModePart = (typeof GAME_MODE_PARTS)[number];
const GAME_MODE_PARTS = ['eng', 'kanji', 'kana'] as const;
const GAME_MODE_PARTS_VIEW: Readonly<Record<GameModePart, string>> = {
  eng: 'English',
  kanji: 'Kanji',
  kana: 'Kana',
};

export function FieldGameMode({
  setMode,
}: {
  setMode: Dispatch<SetStateAction<GameMode | undefined>>;
}) {
  const [prompt, setPrompt] = useState<GameModePart>();
  const [answer, setAnswer] = useState<GameModePart>();

  const onPromptChange = (v: GameModePart) => {
    setPrompt(v);
    if (v === answer) {
      setAnswer(undefined);
      setMode(undefined);
      return;
    }
    if (answer !== undefined) {
      setMode((v + 'to' + answer) as GameMode);
    }
  };

  const onAnswerChange = (v: GameModePart) => {
    setAnswer(v);
    if (v === prompt) {
      setPrompt(undefined);
      setMode(undefined);
      return;
    }
    if (prompt !== undefined) {
      setMode((prompt + 'to' + v) as GameMode);
    }
  };

  return (
    <Field orientation="responsive">
      <FieldLabel className="flex-1 text-nowrap">Game Mode</FieldLabel>

      <div className="flex items-baseline gap-3">
        <Select value={prompt ?? ''} onValueChange={onPromptChange}>
          <SelectTrigger className="min-w-24.5">
            <SelectValue placeholder="Prompt" />
          </SelectTrigger>
          <SelectContent>
            <SelectGroup>
              {GAME_MODE_PARTS.map((mode) => (
                <SelectItem key={mode} value={mode}>
                  {GAME_MODE_PARTS_VIEW[mode]}
                </SelectItem>
              ))}
            </SelectGroup>
          </SelectContent>
        </Select>

        <span>to</span>

        <Select value={answer ?? ''} onValueChange={onAnswerChange}>
          <SelectTrigger className="min-w-24.5">
            <SelectValue placeholder="Answer" />
          </SelectTrigger>
          <SelectContent>
            <SelectGroup>
              {GAME_MODE_PARTS.map((mode) => (
                <SelectItem key={mode} value={mode}>
                  {GAME_MODE_PARTS_VIEW[mode]}
                </SelectItem>
              ))}
            </SelectGroup>
          </SelectContent>
        </Select>
      </div>
    </Field>
  );
}

export function FieldNLevel({
  setLevels,
}: {
  setLevels: Dispatch<SetStateAction<NLevel[] | undefined>>;
}) {
  const id = useId();
  const levelsAnchor = useComboboxAnchor();

  return (
    <Field orientation="responsive">
      <FieldLabel htmlFor={id} className="text-nowrap">
        JLPT NLevel
      </FieldLabel>

      <div className="max-w-58 flex-1/4">
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
              {(levels: NLevel[]) => (
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
              {(item: NLevel) => (
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

export function FieldCategory({
  setCategories,
}: {
  setCategories: Dispatch<SetStateAction<PartOfSpeechCategory[] | undefined>>;
}) {
  const id = useId();
  const anchor = useComboboxAnchor();

  return (
    <Field orientation="responsive">
      <FieldLabel htmlFor={id} className="text-nowrap">
        Categories
      </FieldLabel>

      <div className="max-w-58 flex-1/4">
        <Combobox
          id={id}
          items={POS_CATEGORIES}
          onValueChange={(c: PartOfSpeechCategory[]) => setCategories(c)}
          multiple
          autoHighlight
          required
        >
          <ComboboxChips ref={anchor}>
            <ComboboxValue>
              {(categories: PartOfSpeechCategory[]) => (
                <React.Fragment>
                  {categories.map((c) => {
                    return (
                      <ComboboxChip key={c} showRemove={false}>
                        {POS_VIEW[c]}
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
              {(c: PartOfSpeechCategory) => (
                <ComboboxItem key={c} value={c}>
                  {POS_VIEW[c]}
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
