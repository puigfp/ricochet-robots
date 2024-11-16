// import { useState } from "react";
import "./App.css";

// import { useFibonnacci } from "./worker/useFibonacci";
import { Board } from "./board/Board";

function App() {
  // const [input, setInput] = useState("42");
  // const fibonacciResult = useFibonnacci(Number(input));
  return (
    <>
      {/* <input value={input} onChange={(e) => setInput(e.target.value)} />
      <p>
        Computation time:{" "}
        {(
          Math.round(fibonacciResult.elapsedMilliseconds / 100) / 10
        ).toString()}
        s
      </p>
      <p>
        {fibonacciResult.result != null
          ? fibonacciResult.result.toString()
          : fibonacciResult.error != null
          ? `Error: "${fibonacciResult.error.toString()}"\n${
              fibonacciResult.error.stack
            }`
          : "Running..."}
      </p> */}

      <h1>Ricochet Robots ❤️</h1>
      <Board
        height={16}
        width={16}
        wallConfiguration={{
          rightWalls: [
            [4],
            [9],
            [6],
            [],
            [2, 8],
            [12],
            [4, 13],
            [6, 8],
            [6, 8, 9],
            [4],
            [13],
            [],
            [6, 12],
            [5, 9],
            [2],
            [5, 10],
          ],
          bottomWalls: [
            [2, 9],
            [],
            [3],
            [4, 14],
            [8],
            [5],
            [2, 12],
            [6, 8],
            [4, 6, 8],
            [13],
            [0, 7],
            [],
            [11],
            [5, 12],
            [9],
            [2, 8],
          ],
        }}
      />
    </>
  );
}

export default App;
