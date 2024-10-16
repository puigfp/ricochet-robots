console.log("hello from worker");

self.onmessage = async (ev: MessageEvent) =>  {
  const solver = await import("ricochet-robots-solver/ricochet_robots_solver");
  const input = ev.data;
  console.log("hello from worker", input);
  const result = solver.fib(input);
  console.log("hello from worker", result);
  postMessage(result);
};
