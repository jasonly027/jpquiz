import { useRef, useState } from 'react';

export interface UseHintOptions {
  showHiragana?: boolean;
  minRemaining?: number;
}

export interface UseHintValue {
  hint: string;
  revealOne: () => void;
}

const CENSOR_STR = '_';

export function useHint(value: string, opts?: UseHintOptions): UseHintValue {
  const { showHiragana = false, minRemaining = 0 } = opts ?? {};

  const original = useRef(value);

  const [hint, setHint] = useState(() => createHint(value, showHiragana));

  const revealOne = () => {
    setHint((prev) => {
      const censoredIdxs = prev
        .split('')
        .map((c, idx) => [c, idx] as const)
        .filter(([c]) => c === CENSOR_STR)
        .map(([, idx]) => idx);
      if (censoredIdxs.length <= minRemaining) return prev;

      const idx = censoredIdxs[Math.floor(Math.random() * censoredIdxs.length)];
      if (idx === undefined) return prev;

      return (
        prev.substring(0, idx) +
        original.current[idx]! +
        prev.substring(idx + 1)
      );
    });
  };

  return {
    hint,
    revealOne,
  };
}

function createHint(value: string, showHiragana: boolean) {
  return value
    .split('')
    .map((c) => {
      const shouldShow = showHiragana && isHiraganaChar(c);
      return shouldShow ? c : CENSOR_STR;
    })
    .join('');
}

function isHiraganaChar(value: string): boolean {
  return hiraganaCharMatcher.test(value);
}
const hiraganaCharMatcher = /^[\u3041-\u3096]$/;
