import { FC } from "react";
import { Route, Routes } from "react-router-dom";

import { useClient } from "./request";
import { ConnectionError } from "./error";
import { TopBar } from "./topbar";
import { Logout } from "./logout";
import { Login } from "./login";
import { Signup } from "./signup";
import { Toolbar } from "@mui/material";

export const App: FC = () => {
  const client = useClient();
  return (
    <div>
      <TopBar />
      <main>
        <Toolbar />
        {client ? (
          <Routes>
            <Route path="/logout" element={<Logout />} />
            <Route path="/login" element={<Login />} />
            <Route path="/signup" element={<Signup />} />
          </Routes>
        ) : (
          <ConnectionError />
        )}
      </main>
    </div>
  );
};
