import type { QuestionStat } from '../-lib/models';

export interface StatsProps<T> {
  stats: QuestionStat<T>[];
}

export function Stats<T>({ stats }: StatsProps<T>) {
  return (
    <div className="flex w-full max-w-4xl flex-col">
      <h2 className="text-center text-lg">Quiz Results</h2>
    </div>
  );
}
