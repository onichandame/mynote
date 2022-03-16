import { FC } from "react";
import { Box, Toolbar } from "@mui/material";
import { Route, Routes } from "react-router-dom";

import { TopBar } from "./topbar";
import { Login } from "./login";
import { Signup } from "./signup";
import { Private } from "./private";
import { Logout } from "./logout";
import { Form } from "./common";

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
                <Form>
                  <Login />
                </Form>
              }
            />
            <Route
              path="logout"
              element={
                <Form>
                  <Logout />
                </Form>
              }
            />
            <Route
              path="signup"
              element={
                <Form>
                  <Signup />
                </Form>
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
