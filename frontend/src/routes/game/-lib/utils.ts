import type { GameMode } from '@/lib/models';

export function isEnglishPrompt(mode: GameMode) {
  switch (mode) {
    case 'engtokana':
    case 'engtokanji':
      return true;
    case 'kanatokanji':
    case 'kanatoeng':
    case 'kanjitokana':
    case 'kanjitoeng':
      return false;
  }
}

export function isJapanesePrompt(mode: GameMode) {
  switch (mode) {
    case 'engtokana':
    case 'engtokanji':
      return false;
    case 'kanatokanji':
    case 'kanatoeng':
    case 'kanjitokana':
    case 'kanjitoeng':
      return true;
  }
}

export function isEnglishChoices(mode: GameMode) {
  switch (mode) {
    case 'engtokana':
    case 'engtokanji':
    case 'kanatokanji':
    case 'kanjitokana':
      return false;
    case 'kanatoeng':
    case 'kanjitoeng':
      return true;
  }
}

export function isJapaneseChoices(mode: GameMode) {
  switch (mode) {
    case 'engtokana':
    case 'engtokanji':
    case 'kanatokanji':
    case 'kanjitokana':
      return true;
    case 'kanatoeng':
    case 'kanjitoeng':
      return false;
  }
}

export function getGamePromptFont(mode: GameMode) {
  return isJapanesePrompt(mode)
    ? 'font-game-jp font-normal'
    : 'font-game-eng font-bold';
}

export function getGameChoicesFont(mode: GameMode) {
  return isJapaneseChoices(mode)
    ? 'font-game-jp font-normal'
    : 'font-game-eng font-bold';
}

export function formatTime(secs: number): string {
  const hours = Math.floor(secs / 3600);
  const minutes = Math.floor((secs % 3600) / 60);
  const seconds = secs % 60;

  return [hours, minutes, seconds]
    .map((v) => String(v).padStart(2, '0'))
    .join(':');
}

export function formatAccuracy(value: number): string {
  if (isNaN(value)) return 'N/A';
  if (!isFinite(value)) return '100%';
  return percentFormatter.format(value);
}

const percentFormatter = new Intl.NumberFormat(undefined, {
  style: 'percent',
  maximumFractionDigits: 1,
});
