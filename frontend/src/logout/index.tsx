import { useSnackbar } from "notistack";
import { FC, useEffect } from "react";
import { useNavigate } from "react-router-dom";

import { useSessionSetter } from "../auth";

export const Logout: FC = () => {
  const navigate = useNavigate();
  const setSession = useSessionSetter();
  const { enqueueSnackbar } = useSnackbar();
  useEffect(() => {
    setSession(undefined);
    enqueueSnackbar(`logout successful`, { variant: `success` });
    navigate(`/`);
  }, []);
  return <div>logging out</div>;
};
