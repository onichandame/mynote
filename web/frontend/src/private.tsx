import { FC } from "react";
import { Routes, Route, Navigate } from "react-router-dom";

import { Home } from "./home";
import { useUser } from "./auth";
import { Notes } from "./notes";

export const Private: FC = () => {
  const user = useUser();
  return user ? (
    <Routes>
      <Route path="notes/*" element={<Notes />} />
      <Route path="/" element={<Home />} />
    </Routes>
  ) : (
    <Navigate to="/login" />
  );
};
