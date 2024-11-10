import { Dispatch, SetStateAction, useCallback, useEffect } from "react";
import { arrowIcons, robotIcons } from "./constants";

interface ResultsProps {
  moves: { robot: number; direction: number }[];
  selectedMove: number;
  setSelectedMove: Dispatch<SetStateAction<number>>;
  setTransition: Dispatch<SetStateAction<bool>>;
}

export const Results = ({
  moves,
  selectedMove,
  setSelectedMove,
  setTransition,
}: ResultsProps) => {
  let prependedMoves: ({ robot: number; direction: number } | null)[] = [null];
  prependedMoves = prependedMoves.concat(moves);

  const setNextMove = useCallback(() => {
    // HACK: only enable transitions on a +1 or -1 move
    setTransition(true);
    setSelectedMove((current) =>
      current + 1 < prependedMoves.length ? current + 1 : current
    );
  }, [setSelectedMove, setTransition, prependedMoves.length]);
  const setPreviousMove = useCallback(() => {
    // HACK: only enable transitions on a +1 or -1 move
    setTransition(true);
    setSelectedMove((current) => (current > 0 ? current - 1 : current));
  }, [setSelectedMove, setTransition]);

  // Set up event listener on the window object to listen for arrow key presses
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key == "ArrowUp") {
        setPreviousMove();
      } else if (event.key == "ArrowDown") {
        setNextMove();
      }
    };

    // Add event listener to the window
    window.addEventListener("keydown", handleKeyDown);

    // Cleanup the event listener on component unmount
    return () => {
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, [setPreviousMove, setNextMove]);

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
            <div style={{ minWidth: "1.5em" }}>
              {index == selectedMove ? "▶️" : null}
            </div>
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
        <button style={{ margin: "0em 1em" }} onClick={setNextMove}>
          ⬇️ Next
        </button>
        <button style={{ margin: "0em 1em" }} onClick={setPreviousMove}>
          ⬆️ Prev
        </button>
      </div>
    </div>
  );
};
