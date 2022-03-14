import { useSnackbar } from "notistack";
import { createContext, FC, useContext, useEffect, useState } from "react";

import { useSession } from "../auth";
import { Backend } from "./backend";

const getUrl = () =>
  window.location.protocol.startsWith(`https`)
    ? `wss://`
    : `ws://` + window.location.host + `/graphql`;

const formatError = (e: unknown): string => {
  return Array.isArray(e)
    ? e.map((v) => formatError(v)).join(`; `)
    : e instanceof Error
    ? e.message
    : JSON.stringify(e);
};

const BackendContext = createContext(new Backend(getUrl()));

export const useBackend = () => useContext(BackendContext);

export const BackendProvider: FC = ({ children }) => {
  const session = useSession();
  const { enqueueSnackbar } = useSnackbar();
  const [backend, setBackend] = useState(new Backend(getUrl()));
  useEffect(() => {
    backend.on(`error`, (e) => {
      enqueueSnackbar(formatError(e), { variant: `error` });
    });
    return () => {
      backend.dispose();
    };
  }, [backend]);
  useEffect(() => {
    setBackend(new Backend(getUrl(), session));
  }, [session]);
  return (
    <BackendContext.Provider value={backend}>
      {children}
    </BackendContext.Provider>
  );
};
