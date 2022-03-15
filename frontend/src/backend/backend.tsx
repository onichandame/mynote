import { useSnackbar } from "notistack";
import { createContext, FC, useContext, useEffect, useState } from "react";

import { User } from "../model";
import { formatError, getSvcUrl } from "../util";
import { Service } from "./service";

const sessionKey = `mynote_session`;

const BackendContext = createContext<[Backend, (_: Backend) => void]>([
  {
    svc: new Service(getSvcUrl()),
  },
  () => {},
]);

export const useService = () => useContext(BackendContext)[0].svc;
export const useUser = () => useContext(BackendContext)[0].user;
export const useSessionSetter = () => {
  const [, setBackend] = useContext(BackendContext);
  return async (session?: string) => {
    try {
      const svc = new Service(getSvcUrl(), session);
      if (session) {
        const user = await svc.self();
        window.localStorage.setItem(sessionKey, session);
        setBackend({ svc, session, user });
      } else {
        setBackend({ svc });
      }
    } catch (e) {
      console.error(e);
    }
  };
};

export const BackendProvider: FC = ({ children }) => {
  const { enqueueSnackbar } = useSnackbar();
  const backendState = useState<Backend>({
    svc: new Service(getSvcUrl()),
  });
  useEffect(() => {
    backendState[0].svc.on(`error`, (_, e) => {
      console.log(e);
      enqueueSnackbar(formatError(e), { variant: `error` });
    });
    return () => {
      backendState[0].svc.dispose();
    };
  }, [backendState[0]]);
  useEffect(() => {
    const session = window.localStorage.getItem(sessionKey);
    if (session) {
      const svc = new Service(getSvcUrl(), session);
      svc
        .self()
        .then((user) => {
          backendState[1]({ svc, user, session });
        })
        .catch((e) => {
          console.error(e);
          window.localStorage.removeItem(sessionKey);
        });
    }
  }, []);
  return (
    <BackendContext.Provider value={backendState}>
      {children}
    </BackendContext.Provider>
  );
};

type Backend = {
  svc: Service;
  session?: string;
  user?: User;
};
