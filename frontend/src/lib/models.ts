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

export class FreeResponseQuestion {
  prompt: string;
  answers: string[];
  expandedAnswers: string[];
  word_pair: WordDto;

  constructor({ prompt, answers, word_pair }: FreeResponseQuestionDto) {
    this.prompt = prompt;
    this.answers = answers;
    this.expandedAnswers = expandCommented(answers);
    this.word_pair = word_pair;
  }

  isAnswer(guess: string): boolean {
    return this.expandedAnswers.includes(guess);
  }
}

/**
 * Creates a flag array where an elmenent is on if it's part of a bracketed comment.
 *
 * Example: "to (do)" => \[0,0,0,1,1,1,1\]
 */
export function getCommentedMask(commented: string): boolean[] {
  const result: boolean[] = [];
  let depth = 0;
  for (const ch of commented) {
    if (ch === '(') depth++;
    else if (ch === ')') depth--;
    result.push(ch === '(' || ch === ')' || depth !== 0);
  }
  return result;
}

/**
 * Creates a new array of string based off elements in `commented`.
 * For each element that contains bracketed "comments", a new string
 * with those comments removed is inserted after the original.
 * Elements without brackets are left as-is.
 *
 * Example: \["to jump (over)", "to run"\] => \["to jump (over)", "to jump", "to run"\]
 */
function expandCommented(commented: string[]): string[] {
  return commented.flatMap((com): string[] => {
    const newCom = stripComments(com);
    if (newCom !== com) {
      return [com, newCom];
    }
    return [com];
  });
}

/**
 * Strips all bracketed comments from the string.
 *
 * Example: "to jump (over)" returns "to jump"
 */
function stripComments(commented: string): string {
  let result = '';
  let depth = 0;
  for (const ch of commented) {
    if (ch === '(') depth++;
    else if (ch === ')') depth--;
    else if (depth === 0) result += ch;
  }
  return result.replace(/\s+/g, ' ').trim();
}
