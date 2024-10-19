self.onmessage = async (ev: MessageEvent) =>  {
  console.log("worker received event", ev);
  const solver = await import("ricochet-robots-solver/ricochet_robots_solver");
  const input = ev.data;
  const result = solver.fib(input);
  console.log("worker computed result", result);
  postMessage(result);
};
