import _ from "lodash";
import {
  DndContext,
  DragEndEvent,
  useDraggable,
  useDroppable,
} from "@dnd-kit/core";
import { CSS } from "@dnd-kit/utilities";
import { ReactNode, useCallback, useState } from "react";
import { robotIcons } from "./constants";

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

interface SquareProps {
  row: number;
  col: number;
  children: ReactNode;
}
const Square = ({ row, col, children }: SquareProps) => {
  const { isOver, setNodeRef } = useDroppable({
    id: `square_${row}_${col}`,
    data: { square: { row, col } },
  });
  return (
    <div
      ref={setNodeRef}
      style={{
        width: "30px",
        height: "30px",
        padding: 0,
        margin: 0,
        borderWidth: "1px",
        borderColor: isOver ? "black" : "grey",
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
  width: number;
  height: number;
}
export const Board = ({ width, height }: BoardProps) => {
  const [robotPositions, setRobotPosition] = useRobotPositions([
    { row: 0, col: 0 },
    { row: 0, col: 1 },
    { row: 0, col: 2 },
    { row: 0, col: 3 },
  ]);
  const handleDragEnd = useCallback(
    (e: DragEndEvent) => {
      console.log(`Element ${e.active.id} dropped over ${e.over?.id}`);
      console.log(e.active.data);
      if (e.active.data.current?.robot == null || e.over?.data.current == null)
        return;
      const robotId = e.active.data.current.robot.id;
      const nextPosition = {
        col: e.over.data.current.square.col,
        row: e.over.data.current.square.row,
      };
      if (
        robotPositions.some(
          (value) =>
            value.col == nextPosition.col && value.row == nextPosition.row
        )
      ) {
        return;
      }
      setRobotPosition(robotId, nextPosition);
    },
    [robotPositions, setRobotPosition]
  );
  return (
    <DndContext onDragEnd={handleDragEnd}>
      <div style={{ display: "flex", flexDirection: "column" }}>
        {_.range(0, height).map((row) => (
          <div style={{ display: "flex", flexDirection: "row" }}>
            {_.range(0, width).map((col) => (
              <Square row={row} col={col}>
                {robotPositions.map((position, index) =>
                  position.col == col && position.row == row ? (
                    <Robot id={index} />
                  ) : null
                )}
              </Square>
            ))}
          </div>
        ))}
      </div>
    </DndContext>
  );
};
