import { useEffect, useState } from "react";

// This should be a sum type
export interface WorkerResult<R> {
  result: R | null;
  error: Error | null;
}

// This hook is a toy example for delegating an expensive computation to a web worker
export const useWorkerResult = <P, R>(
  createWorker: () => Worker,
  input: P
): WorkerResult<R> | null => {
  const [result, setResult] = useState<WorkerResult<R> | null>(null);

  // Whenever the input changes, we terminate the currently running worker and start a new one
  useEffect(() => {
    // Reset result
    setResult(null);

    // Start a web worker
    const worker = createWorker();

    // Be prepared to receive the result
    worker.onmessage = (ev) => {
      console.log(ev);
      setResult(ev.data);
      worker.terminate();
    };

    // Send input to the worker
    worker.postMessage(input);

    return () => {
      worker.terminate();
    };
  }, [createWorker, input]);

  return result;
};
