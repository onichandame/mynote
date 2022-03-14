import { FC, useEffect } from "react";
import { useNavigate } from "react-router-dom";

import { useSessionSetter } from "../auth";

export const Logout: FC = () => {
  const navigate = useNavigate();
  const setSession = useSessionSetter();
  useEffect(() => {
    console.log(`logout`);
    setSession(null);
    navigate(`/`);
  }, []);
  return <div>logging out</div>;
};
