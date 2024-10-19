import { useEffect, useState } from "react";

export const useElapsedTime = (input: number, finished: boolean, timeout: number): number => {
  const [start, setStart] = useState<number>(Date.now());
  const [elapsed, setElapsed] = useState<number>(0);

  // Reset timer on input change
  useEffect(() => {
    setStart(Date.now());
    setElapsed(0);
  }, [input]);

  // While we're not finished, regularly update elapsed time
  useEffect(() => {
    if (finished) {
      return;
    }

    const updateElapsedTime = () => {setElapsed(Date.now() - start)};
    const intervalId = setInterval(updateElapsedTime, timeout);
    return () => {
      // Update elapsed time one last time
      updateElapsedTime();

      // Clear background task
      clearInterval(intervalId);
    };
  }, [start, finished, timeout]);

  return elapsed;
};
