import { useLocalStorage } from '@mantine/hooks';

export function useShowHint() {
  return useLocalStorage({
    defaultValue: true,
    key: 'showHint',
  });
}
