import React, { useState, useEffect } from 'react';
import './App.css';
import * as wasmPkg from 'wasm';

function App() {
  const [sum, setSum] = useState<number>(0);
  const [fib, setFib] = useState<number>(0);

  const init = async() => {
    await wasmPkg.default()

    const sumRes = wasmPkg.add_two_ints(1,2);
    setSum(sumRes);

    const fibRes = wasmPkg.fib(20);
    setFib(fibRes);
  }

  useEffect(() => {
    init();
  }, []);
  
  return (
    <div className="App">
      <div>Sum Results: {sum}</div>
      <div>Fib Results: {fib}</div>
    </div>
  );
}

export default App;
