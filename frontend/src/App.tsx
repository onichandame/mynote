import { FC } from "react";
import { Box, Grid, Toolbar } from "@mui/material";
import { Route, Routes } from "react-router-dom";

import { TopBar } from "./topbar";
import { Login } from "./login";
import { Signup } from "./signup";
import { Private } from "./private";
import { Logout } from "./logout";

export const App: FC = () => {
  return (
    <div>
      <TopBar />
      <main>
        <Toolbar />
        <Grid container direction="column" alignItems="center" spacing={2}>
          <Grid item m={2}>
            <Routes>
              {/** public routes */}
              <Route path="login" element={<Login />} />
              <Route path="logout" element={<Logout />} />
              <Route path="signup" element={<Signup />} />
              {/** private routes */}
              <Route path="/*" element={<Private />} />
            </Routes>
          </Grid>
        </Grid>
      </main>
    </div>
  );
};
