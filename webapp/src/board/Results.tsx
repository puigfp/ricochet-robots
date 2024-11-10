import { Dispatch, SetStateAction } from "react";
import { arrowIcons, robotIcons } from "./constants";

interface ResultsProps {
  moves: { robot: number; direction: number }[];
  selectedMove: number;
  setSelectedMove: Dispatch<SetStateAction<number>>;
}

export const Results = ({
  moves,
  selectedMove,
  setSelectedMove,
}: ResultsProps) => {
  let prependedMoves: ({ robot: number; direction: number } | null)[] = [null];
  prependedMoves = prependedMoves.concat(moves);

  return (
    <div>
      <p>Found solution in {moves.length} moves!</p>
      <div>
        {prependedMoves.map((value, index) => (
          <div
            key={index}
            onClick={() => setSelectedMove(index)}
            style={{
              display: "flex",
              flexDirection: "row",
              justifyContent: "center",
              margin: ".1em 0em",
            }}
          >
            <div style={{minWidth: "1.5em"}}>{index == selectedMove ? "▶️" : null}</div>
            <div>
              {value != null
                ? `${robotIcons[value.robot]} ${arrowIcons[value.direction]}`
                : "Initial position"}
            </div>
          </div>
        ))}
      </div>
      <div
        style={{
          display: "flex",
          flexDirection: "row",
          justifyContent: "center",
        }}
      >
        <button
          style={{ margin: "0em 1em" }}
          onClick={() =>
            setSelectedMove((current) =>
              current + 1 < prependedMoves.length ? current + 1 : current
            )
          }
        >
          ⬇️ Next
        </button>
        <button
          style={{ margin: "0em 1em" }}
          onClick={() =>
            setSelectedMove((current) => (current > 0 ? current - 1 : current))
          }
        >
          ⬆️ Prev
        </button>
      </div>
    </div>
  );
};
