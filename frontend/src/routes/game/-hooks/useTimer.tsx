import { useEffect, useState } from 'react';

const ONE_SECOND = 1_000;

export function useTimer() {
  const [time, setTime] = useState(0);

  useEffect(() => {
    const interval = setInterval(() => {
      setTime((prev) => prev + 1);
    }, ONE_SECOND);

    return () => clearInterval(interval);
  }, []);

  return time;
}
