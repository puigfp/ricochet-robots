import { useCallback } from "react";
import { useWorkerResult } from "./useWorkerResult";
import { useElapsedTime } from "./useElapsedTime";

// See https://v3.vitejs.dev/guide/features.html#import-with-query-suffixes
import FibonacciWorker from "./useFibonacci_worker?worker";

export interface FibonacciHookResult {
  result: number | null;
  error: Error | null;
  elapsedMilliseconds: number;
}

export const useFibonnacci = (n: number): FibonacciHookResult => {
  const createWorker = useCallback(() => new FibonacciWorker(), []);
  const result = useWorkerResult<number, number>(createWorker, n);
  const elapsedMilliseconds = useElapsedTime(n, result != null, 500);
  const fibResult =  {
    result: result != null ? result.result : null,
    error: result != null ? result.error : null,
    elapsedMilliseconds,
  };
  return fibResult;
};
