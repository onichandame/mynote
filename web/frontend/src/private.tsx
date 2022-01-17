import { FC } from "react";
import { useLocation, Routes, Route, Navigate } from "react-router-dom";

import { Home } from "./home";
import { useUser } from "./auth";
import { Notes } from "./notes";

export const Private: FC = () => {
  const user = useUser();
  const location = useLocation();
  return user ? (
    <Routes>
      <Route path="notes/*" element={<Notes />} />
      <Route path="/" element={<Home />} />
    </Routes>
  ) : (
    <Navigate
      to={`/login?redirect=${encodeURIComponent(
        location.pathname + location.search + location.hash
      )}`}
    />
  );
};
