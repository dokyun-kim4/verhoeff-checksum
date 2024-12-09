import logo from './logo.svg';
import './App.css';
import Generate from "./components/Generate"
import Validate from "./components/Validate"
import { useEffect, useState } from "react";

function App() {
  let layout = <Validate/>;

  return (
    <div className="App">
      {layout}
    </div>
  );
}

export default App;
