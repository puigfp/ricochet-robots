import { useState } from "react";
import "./App.css";

import { useFibonnacci } from "./worker/useFibonacci";
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
        height={5}
        width={7}
        wallConfiguration={{
          rightWalls: [[], [2], [], [3], []],
          bottomWalls: [[], [], [1], [], [], [], []],
        }}
      />
    </>
  );
}

export default App;
