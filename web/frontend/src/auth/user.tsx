import { createContext, FC, useContext, useEffect, useState } from "react";
import { useSessionSetter } from ".";

import { useFetcher } from "../request";
import { useSession } from "./session";

export type User = {
  id: number;
  name: string;
  email?: string;
  avatar?: string;
} | null;

const UserState = createContext<[User, (_: User) => void]>([null, () => {}]);

export const UserProvider: FC = ({ children }) => {
  const userState = useState<User>(null);
  const fetch = useFetcher<{ self: NonNullable<User> }>(
    `query self{self{id name email avatar}}`
  );
  const session = useSession();
  const setSession = useSessionSetter();
  useEffect(() => {
    let cleanup = () => {};
    let active = true;
    if (session) {
      const [promise, stop] = fetch({});
      cleanup = stop;
      promise
        .then((data) => {
          console.log(data);
          if (active) {
            userState[1](data.self);
          }
        })
        .catch((e) => {
          console.log(`bitch`);
          console.error(e);
          if (active) {
            setSession(null);
          }
        });
    } else {
      userState[1](null);
    }
    return () => {
      cleanup();
      active = false;
    };
  }, [session, fetch]);
  return <UserState.Provider value={userState}>{children}</UserState.Provider>;
};
export const useUser = () => useContext(UserState)[0];
export const useUserSetter = () => {
  const [, setUser] = useContext(UserState);
  return (user: User) => setUser(user);
};
