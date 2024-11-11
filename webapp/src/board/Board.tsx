import _ from "lodash";
import {
  DndContext,
  DragEndEvent,
  useDraggable,
  useDroppable,
} from "@dnd-kit/core";
import { CSS } from "@dnd-kit/utilities";
import { useCallback, useEffect, useMemo, useState } from "react";
import {
  robotFontSize,
  robotIcons,
  squareBorderWidth,
  squareSize,
  squareTotalSize,
  targetFontSize,
  targetIcons,
  wildcardTargetIcon,
} from "./constants";
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
  topWall: boolean;
  bottomWall: boolean;
  rightWall: boolean;
  leftWall: boolean;
}
const Square = ({
  row,
  col,
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
        width: `${squareSize}px`,
        height: `${squareSize}px`,
        padding: 0,
        margin: 0,
        borderWidth: `${squareBorderWidth}px`,
        borderTopColor: topWall ? wallBorderColor : emptyBorderColor,
        borderBottomColor: bottomWall ? wallBorderColor : emptyBorderColor,
        borderRightColor: rightWall ? wallBorderColor : emptyBorderColor,
        borderLeftColor: leftWall ? wallBorderColor : emptyBorderColor,
        borderStyle: "solid",
        display: "flex",
        flexDirection: "column",
        justifyContent: "center",
      }}
    ></div>
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
  const [robotPositions, setRobotPositions] = useState([
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
  const [transition, setTransition] = useState(true);
  const solution = useSolution(solutionInput);

  const handleDragStart = useCallback(() => {
    // HACK: whenever a robot is moved by drag and drop, we don't want a
    // transition, we want it to teleport to its new position
    setTransition(false);
  }, [setTransition]);
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
        const currentPositions =
          selectedMove == 0 || solution.result == null
            ? robotPositions
            : solution.result[selectedMove - 1].robotPositions;
        if (
          currentPositions.some(
            (value) =>
              value.col == nextPosition.col && value.row == nextPosition.row
          )
        ) {
          return;
        }
        setRobotPositions(
          currentPositions.map((value, index) =>
            index == robotId ? nextPosition : value
          )
        );
      }
      if (e.active.data.current?.target != null) {
        setTargetPosition(nextPosition);
      }
    },
    [solution.result, selectedMove, robotPositions, setRobotPositions]
  );
  return (
    <div
      style={{ display: "flex", flexDirection: "row", alignItems: "center" }}
    >
      <div
        style={{ display: "flex", flexDirection: "column", padding: "0em 1em" }}
      >
        <DndContext onDragStart={handleDragStart} onDragEnd={handleDragEnd}>
          <div
            style={{
              // This `position: "relative"` makes the "absolute" positions of
              // the robots and target be relative to this div instead of the
              // root of the DOM tree.
              position: "relative",
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
                  />
                ))}
              </div>
            ))}
            <div
              style={{
                position: "absolute",
                top: `${
                  squareBorderWidth + squareTotalSize * targetPosition.row
                }px`,
                left: `${
                  squareBorderWidth + squareTotalSize * targetPosition.col
                }px`,
                height: `${squareSize}px`,
                width: `${squareSize}px`,
                display: "flex",
                flexDirection: "column",
                alignItems: "center",
                justifyContent: "center",
                fontSize: `${targetFontSize}px`,
              }}
            >
              <Target robot={targetRobot} />
            </div>
            {(selectedMove == 0 || solution.result == null
              ? robotPositions
              : solution.result[selectedMove - 1].robotPositions
            ).map((value, index) => (
              <div
                style={{
                  position: "absolute",
                  top: `${squareBorderWidth + squareTotalSize * value.row}px`,
                  left: `${squareBorderWidth + squareTotalSize * value.col}px`,
                  // leave some margin so that the user can drag the target that might be under the robot
                  height: `${robotFontSize}px`,
                  width: `${robotFontSize}px`,
                  margin: `${(squareSize - robotFontSize) / 2}px`,
                  display: "flex",
                  flexDirection: "column",
                  alignItems: "center",
                  justifyContent: "center",
                  transition: transition ? "all .1s" : "",
                  textShadow: "1px 1px 1px black",
                  fontSize: `${robotFontSize}px`,
                }}
              >
                <Robot key={index} id={index} />
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
            setTransition={setTransition}
          />
        ) : null}
        {solution.error != null
          ? `Error: "${solution.error.toString()}"\n${solution.error.stack}`
          : null}
      </div>
    </div>
  );
};
