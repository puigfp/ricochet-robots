import _ from "lodash";
import {
  DndContext,
  DragEndEvent,
  useDraggable,
  useDroppable,
} from "@dnd-kit/core";
import { CSS } from "@dnd-kit/utilities";
import { ReactNode, useCallback, useEffect, useMemo, useState } from "react";
import { robotIcons, targetIcons, wildcardTargetIcon } from "./constants";
import { useSolution } from "../worker/useSolution";
import { Results } from "./Results";

interface RobotProps {
  id: number;
}
const Robot = ({ id }: RobotProps) => {
  const { attributes, listeners, setNodeRef, transform } = useDraggable({
    id: `robot_${id}`,
    data: { robot: { id } },
  });
  const style = {
    transform: CSS.Translate.toString(transform),
  };
  return (
    <div ref={setNodeRef} style={style} {...listeners} {...attributes}>
      {robotIcons[id]}
    </div>
  );
};

const useRobotPositions = (
  initialPositions: { row: number; col: number }[]
): [
  { row: number; col: number }[],
  (index: number, pos: { row: number; col: number }) => void
] => {
  const [robotPositions, setRobotPositions] = useState(initialPositions);
  const setRobotPosition = (i: number, pos: { row: number; col: number }) =>
    setRobotPositions((current) =>
      current.map((value, index) => (i == index ? pos : value))
    );
  return [robotPositions, setRobotPosition];
};

interface TargetProps {
  robot: number | null;
}

const Target = ({ robot }: TargetProps) => {
  const { attributes, listeners, setNodeRef, transform } = useDraggable({
    id: "target",
    data: { target: { robot } },
  });
  const style = {
    transform: CSS.Translate.toString(transform),
  };
  return (
    <div ref={setNodeRef} style={style} {...listeners} {...attributes}>
      {robot !== null ? targetIcons[robot] : wildcardTargetIcon}
    </div>
  );
};

interface SquareProps {
  row: number;
  col: number;
  children: ReactNode;
  topWall: boolean;
  bottomWall: boolean;
  rightWall: boolean;
  leftWall: boolean;
}
const Square = ({
  row,
  col,
  children,
  topWall,
  bottomWall,
  rightWall,
  leftWall,
}: SquareProps) => {
  const { isOver, setNodeRef } = useDroppable({
    id: `square_${row}_${col}`,
    data: { square: { row, col } },
  });
  const emptyBorderColor = "#e3e3e6";
  const wallBorderColor = "black";
  return (
    <div
      ref={setNodeRef}
      style={{
        backgroundColor: isOver ? "grey" : "white",
        width: "30px",
        height: "30px",
        padding: 0,
        margin: 0,
        borderWidth: "2px",
        borderTopColor: topWall ? wallBorderColor : emptyBorderColor,
        borderBottomColor: bottomWall ? wallBorderColor : emptyBorderColor,
        borderRightColor: rightWall ? wallBorderColor : emptyBorderColor,
        borderLeftColor: leftWall ? wallBorderColor : emptyBorderColor,
        borderStyle: "solid",
        display: "flex",
        flexDirection: "column",
        justifyContent: "center",
      }}
    >
      {children}
    </div>
  );
};
interface BoardProps {
  wallConfiguration: {
    bottomWalls: number[][];
    rightWalls: number[][];
  };
  width: number;
  height: number;
}
export const Board = ({ width, height, wallConfiguration }: BoardProps) => {
  const [robotPositions, setRobotPosition] = useRobotPositions([
    { row: 0, col: 0 },
    { row: 0, col: 1 },
    { row: 0, col: 2 },
    { row: 0, col: 3 },
  ]);
  const [targetPosition, setTargetPosition] = useState({ row: 0, col: 0 });
  const [targetRobot, setTargetRobot] = useState<number | null>(0);
  const [selectedMove, setSelectedMove] = useState<number>(0);

  // automatically reset selected move to 0 whenever the input changes
  // (this is a bit hacky because the component tree will be rendered once with the new position
  // and a selected move that can be > 0, but in practice it doesn't cause problems)
  useEffect(
    () => setSelectedMove(0),
    [robotPositions, targetPosition, targetRobot]
  );

  const solutionInput = useMemo(
    () => ({
      robotPositions,
      height,
      width,
      rightWalls: wallConfiguration.rightWalls,
      bottomWalls: wallConfiguration.bottomWalls,
      target: targetPosition,
      targetRobot,
    }),
    [
      robotPositions,
      height,
      width,
      wallConfiguration,
      targetPosition,
      targetRobot,
    ]
  );
  const solution = useSolution(solutionInput);
  const handleDragEnd = useCallback(
    (e: DragEndEvent) => {
      console.log(`Element ${e.active.id} dropped over ${e.over?.id}`);
      console.log(e.active.data);
      if (e.over?.data.current == null) {
        return;
      }
      const nextPosition = {
        col: e.over.data.current.square.col,
        row: e.over.data.current.square.row,
      };
      if (e.active.data.current?.robot != null) {
        const robotId = e.active.data.current.robot.id;
        if (
          robotPositions.some(
            (value) =>
              value.col == nextPosition.col && value.row == nextPosition.row
          )
        ) {
          return;
        }
        setRobotPosition(robotId, nextPosition);
      }
      if (e.active.data.current?.target != null) {
        setTargetPosition(nextPosition);
      }
    },
    [robotPositions, setRobotPosition]
  );
  return (
    <div
      style={{ display: "flex", flexDirection: "row", alignItems: "center" }}
    >
      <div
        style={{ display: "flex", flexDirection: "column", padding: "0em 1em" }}
      >
        <DndContext onDragEnd={handleDragEnd}>
          <div
            style={{
              display: "flex",
              flexDirection: "column",
              alignItems: "center",
            }}
          >
            {_.range(0, height).map((row) => (
              <div key={row} style={{ display: "flex", flexDirection: "row" }}>
                {_.range(0, width).map((col) => (
                  <Square
                    key={col}
                    row={row}
                    col={col}
                    topWall={
                      row == 0 ||
                      (row > 0 &&
                        wallConfiguration.bottomWalls[col].includes(row - 1))
                    }
                    bottomWall={
                      row == height - 1 ||
                      wallConfiguration.bottomWalls[col].includes(row)
                    }
                    leftWall={
                      col == 0 ||
                      (col > 1 &&
                        wallConfiguration.rightWalls[row].includes(col - 1))
                    }
                    rightWall={
                      col == width - 1 ||
                      wallConfiguration.rightWalls[row].includes(col)
                    }
                  >
                    {selectedMove == 0 || solution.result == null
                      ? robotPositions.map((position, index) =>
                          position.col == col && position.row == row ? (
                            <Robot key={index} id={index} />
                          ) : null
                        )
                      : solution.result[selectedMove - 1].robot_positions.map(
                          (position, index) =>
                            position.col == col && position.row == row ? (
                              <Robot key={index} id={index} />
                            ) : null
                        )}
                    {targetPosition.row == row && targetPosition.col == col ? (
                      <Target robot={targetRobot} />
                    ) : null}
                  </Square>
                ))}
              </div>
            ))}
          </div>
        </DndContext>
        <div>
          <button
            style={{ margin: "10px" }}
            onClick={() => {
              setTargetRobot((value) => {
                if (value == null) {
                  return 0;
                } else if (value == 3) {
                  return null;
                } else {
                  return (value + 1) % 4;
                }
              });
            }}
          >
            Change target color
          </button>
        </div>
      </div>
      <div style={{ padding: "0em 1em", minWidth: "20em" }}>
        <p>
          Computation time:{" "}
          {(Math.round(solution.elapsedMilliseconds / 10) / 100).toString()}s
        </p>
        {solution.result != null ? (
          <Results
            moves={solution.result}
            selectedMove={selectedMove}
            setSelectedMove={setSelectedMove}
          />
        ) : null}
        {solution.error != null
          ? `Error: "${solution.error.toString()}"\n${solution.error.stack}`
          : null}
      </div>
    </div>
  );
};
