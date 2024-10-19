import { useEffect, useState } from "react";

export interface FibonacciHookResult {
  result: number | null;
  elapsedMilliseconds: number;
}

// This hook is a toy example for delegating an expensive computation to a web worker
// (Here, the expensive computation is naive Fibonacci implementation written in Rust and compiled
// to WASM)
export const useFibonnacci = (n: number): FibonacciHookResult => {
  const [result, setResult] = useState<number | null>(null);
  const elapsed = useElapsedTime(n, result != null);

  // Whenever the input changes, we terminate the currently running worker and start a new one
  useEffect(() => {
    // Reset result
    setResult(null);

    // Start a web worker
    const worker = new Worker(
      new URL("./fibonacci_worker.ts", import.meta.url),
      {
        type: "module",
      }
    );

    // Be prepared to receive the result
    worker.onmessage = (ev) => {
      setResult(ev.data);
      worker.terminate();
    };

    // Send input to the worker
    worker.postMessage(n);

    return () => {
      worker.terminate();
    };
  }, [n]);

  return { result, elapsedMilliseconds: elapsed };
};

export const useElapsedTime = (input: number, finished: boolean): number => {
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

    const id = setInterval(() => {
      setElapsed(Date.now() - start);
    }, 50);
    return () => {
      clearInterval(id);
    };
  }, [start, finished]);

  return elapsed;
};
