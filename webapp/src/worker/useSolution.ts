import { useCallback } from "react";
import { useWorkerResult } from "./useWorkerResult";
import { useElapsedTime } from "./useElapsedTime";

// See https://v3.vitejs.dev/guide/features.html#import-with-query-suffixes
import SolutionWorker from "./useSolution_worker?worker";

export interface SolutionHookResult {
  result: { robot: number; direction: number }[] | null;
  error: Error | null;
  elapsedMilliseconds: number;
}

export interface UseSolutionHookInput {
  robotPositions: {row: number, col: number}[];
  height: number;
  width: number;
  rightWalls: number[][];
  bottomWalls: number[][];
  target: {row: number, col: number};
  targetRobot: number | null;
}

export const useSolution = (
  input: UseSolutionHookInput
): SolutionHookResult => {
  const createWorker = useCallback(() => new SolutionWorker(), []);
  const result = useWorkerResult<
    UseSolutionHookInput,
    { robot: number; direction: number }[]
  >(createWorker, input);
  const elapsedMilliseconds = useElapsedTime(input, result != null, 500);
  const solutionResult = {
    result: result != null ? result.result : null,
    error: result != null ? result.error : null,
    elapsedMilliseconds,
  };
  return solutionResult;
};
