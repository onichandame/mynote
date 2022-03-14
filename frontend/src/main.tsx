import { CssBaseline } from "@mui/material";
import React from "react";
import ReactDOM from "react-dom";
import { HashRouter } from "react-router-dom";

import { App } from "./App";
import { Global } from "./global";

ReactDOM.render(
  <React.StrictMode>
    <CssBaseline />
    <HashRouter>
      <Global>
        <App />
      </Global>
    </HashRouter>
  </React.StrictMode>,
  document.getElementById("root")
);
