import { useState } from "react";
import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import "./App.css";

import { useFibonnacci } from "./worker/useFibonacci";
import { Board } from "./board/Board";

function App() {
  const [count, setCount] = useState(0);
  const [input, setInput] = useState("42");
  const fibonacciResult = useFibonnacci(Number(input));

  return (
    <>
      <div>
        <a href="https://vitejs.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
      </div>

      <Board height={5} width={7}/>

      <input value={input} onChange={(e) => setInput(e.target.value)} />
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
          ? `Error: "${fibonacciResult.error.toString()}"\n${fibonacciResult.error.stack}`
          : "Running..."}
      </p>
    </>
  );
}

export default App;
