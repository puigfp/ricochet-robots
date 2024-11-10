import { UseSolutionHookInput } from "./useSolution";

self.onmessage = async (ev: MessageEvent) => {
  console.log("worker received event", ev);
  try {
    const { solve, Position } = await import("ricochet-robots-solver");
    const input: UseSolutionHookInput = ev.data;
    const result = solve(
      input.robotPositions.map(
        (value) => new Position(value.row, value.col)
      ),
      input.height,
      input.width,
      input.rightWalls.flatMap((walls, row) =>
        walls.map((col) => new Position(row, col))
      ),
      input.bottomWalls.flatMap((walls, col) =>
        walls.map((row) => new Position(row, col))
      ),
      new Position(input.target.row, input.target.col),
      input.targetRobot != null ? input.targetRobot : undefined
    );
    console.log("worker computed result", result);
    postMessage({
      result: result.map((move) => ({
        robot: move.robot,
        direction: move.direction,
        robot_positions: move.robot_positions.map(p => ({row: p.x, col: p.y})),
      })),
      error: null,
    });
  } catch (error) {
    console.log("worker crashed", error);
    postMessage({ result: null, error });
  }
};
