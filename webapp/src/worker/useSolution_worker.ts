import { UseSolutionHookInput } from "./useSolution";

self.onmessage = async (ev: MessageEvent) => {
  console.log("worker received event", ev);
  try {
    const { solve, Position } = await import("ricochet-robots-solver");
    const input: UseSolutionHookInput = ev.data;
    const result = solve(
      input.robotPositions.map((value) => new Position(value.row, value.col)),
      input.height,
      input.width,
      { rightWalls: input.rightWalls, bottomWalls: input.bottomWalls },
      new Position(input.target.row, input.target.col),
      input.targetRobot != null ? input.targetRobot : undefined
    );
    console.log("worker computed result", result);
    postMessage({
      result,
      error: null,
    });
  } catch (error) {
    console.log("worker crashed", error);
    postMessage({ result: null, error });
  }
};
