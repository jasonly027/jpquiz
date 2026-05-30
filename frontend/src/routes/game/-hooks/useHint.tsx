import { useRef, useState } from 'react';

export interface UseHintOptions {
  minRemaining?: number;
  autoRevealFilters?: ((ch: string, idx: number) => boolean)[];
}

export interface UseHintValue {
  hint: string;
  revealOne: () => void;
}

const CENSOR_STR = '_';

export function useHint(value: string, opts?: UseHintOptions): UseHintValue {
  const { minRemaining = 0, autoRevealFilters = [] } = opts ?? {};
  autoRevealFilters.push(isSpecialChar);

  const original = useRef(value);

  const [hint, setHint] = useState(() => createHint(value, autoRevealFilters));

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

function createHint(
  value: string,
  autoRevealFilters: NonNullable<UseHintOptions['autoRevealFilters']>
) {
  return value
    .split('')
    .map((c, idx) => {
      for (const filter of autoRevealFilters) {
        if (filter(c, idx)) {
          return c;
        }
      }
      return CENSOR_STR;
    })
    .join('');
}

function isSpecialChar(c: string): boolean {
  return specialCharMatcher.test(c);
}
const specialCharMatcher = /^[ ,.?!()\-[\];'"/\\:@#$%&*+=<>{}|~`^_]$/;

export function isHiraganaChar(c: string): boolean {
  return hiraganaCharMatcher.test(c);
}
const hiraganaCharMatcher = /^[\u3041-\u3096]$/;
