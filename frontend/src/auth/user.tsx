import { createContext, FC, useContext, useEffect, useState } from "react";
import { useSessionSetter } from ".";

import { useBackend } from "../backend";
import { User } from "../model";
import { useSession } from "./session";

const UserState = createContext<[User | null, (_: User | null) => void]>([
  null,
  () => {},
]);

export const UserProvider: FC = ({ children }) => {
  const userState = useState<User | null>(null);
  const backend = useBackend();
  const session = useSession();
  const setSession = useSessionSetter();
  useEffect(() => {
    let active = true;
    if (session) {
      backend
        .self()
        .then((me) => {
          if (active) {
            userState[1](me);
          }
        })
        .catch((e) => {
          if (active) {
            setSession(undefined);
          }
        });
    } else {
      userState[1](null);
    }
    return () => {
      active = false;
    };
  }, [session]);
  return <UserState.Provider value={userState}>{children}</UserState.Provider>;
};

export const useUser = () => useContext(UserState)[0];

export const useUserSetter = () => {
  const [, setUser] = useContext(UserState);
  return (user: User) => setUser(user);
};
