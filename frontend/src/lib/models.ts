import {
  NLevelDto,
  GameMode as GameModeDto,
  PartOfSpeechCategoryDto,
  type MultiChoiceQuestion as MultiChoiceQuestionDto,
  type FreeResponseQuestion as FreeResponseQuestionDto,
  type WordDto,
} from '@/api/model';

export type NLevel = NLevelDto;

export type GameMode = GameModeDto;

export type WordPair = WordDto;

export type PartOfSpeechCategory = PartOfSpeechCategoryDto;

export type MultiChoiceQuestion = MultiChoiceQuestionDto;

export type FreeResponseQuestion = FreeResponseQuestionDto;
