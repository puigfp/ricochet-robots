import { useEffect, useState } from "react";

// This hook is a toy example for delegating an expensive computation to a web worker
export const useWorkerResult =<P, R>(createWorker: () => Worker, input: P): R | null => {
  const [result, setResult] = useState<R | null>(null);

  // Whenever the input changes, we terminate the currently running worker and start a new one
  useEffect(() => {
    // Reset result
    setResult(null);

    // Start a web worker
    const worker = createWorker();

    // Be prepared to receive the result
    worker.onmessage = (ev) => {
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
