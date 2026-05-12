import type { GetGameParams } from '@/api/model';
import { getGame } from '@/api/server';
import { mutationOptions } from '@tanstack/react-query';

export function gameMutationOptions() {
  return mutationOptions({
    mutationFn: (params: GetGameParams) => {
      return getGame(params);
    },
  });
}
