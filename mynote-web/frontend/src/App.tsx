import { FC } from "react";
import { Box, Grid, Toolbar } from "@mui/material";
import { Route, Routes } from "react-router-dom";

import { useClient } from "./request";
import { ConnectionError } from "./error";
import { TopBar } from "./topbar";
import { Login } from "./login";
import { Signup } from "./signup";
import { Private } from "./private";
import { Logout } from "./logout";

export const App: FC = () => {
  const client = useClient();
  return (
    <div>
      <TopBar />
      <main>
        <Toolbar />
        <Grid container direction="column" alignItems="center" spacing={2}>
          <Grid item m={2}>
            {client ? (
              <Routes>
                <Route path="login" element={<Login />} />
                <Route path="logout" element={<Logout />} />
                <Route path="signup" element={<Signup />} />
                <Route path="/*" element={<Private />} />
              </Routes>
            ) : (
              <ConnectionError />
            )}
          </Grid>
        </Grid>
      </main>
    </div>
  );
};
