import { FC } from "react";
import { Box, Toolbar } from "@mui/material";
import { Route, Routes } from "react-router-dom";

import { TopBar } from "./topbar";
import { Login } from "./login";
import { Signup } from "./signup";
import { Private } from "./private";
import { Logout } from "./logout";
import { CenterRow } from "./common";

export const App: FC = () => {
  return (
    <div>
      <TopBar />
      <Toolbar />
      <main>
        <Box sx={{ p: 2, width: `auto` }}>
          <Routes>
            {/** public routes */}
            <Route
              path="login"
              element={
                <CenterRow>
                  <Login />
                </CenterRow>
              }
            />
            <Route
              path="logout"
              element={
                <CenterRow>
                  <Logout />
                </CenterRow>
              }
            />
            <Route
              path="signup"
              element={
                <CenterRow>
                  <Signup />
                </CenterRow>
              }
            />
            {/** private routes */}
            <Route path="/*" element={<Private />} />
          </Routes>
        </Box>
      </main>
    </div>
  );
};
