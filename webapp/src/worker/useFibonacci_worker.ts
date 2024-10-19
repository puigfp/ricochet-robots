import * as library from "ricochet-robots-solver";

self.onmessage = async (ev: MessageEvent) => {
  console.log("worker received event", ev);
  try {
    console.log("library", library);
    const input = ev.data;
    const result = library.fib(input);
    console.log("worker computed result", result);
    postMessage({ result, error: null });
  } catch (error) {
    console.log("worker crashed", error);
    postMessage({ result: null, error})
  }
};
