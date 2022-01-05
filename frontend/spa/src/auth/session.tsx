import { createContext, FC, useContext, useState } from "react";

const SessionState = createContext<[string, (_: string) => void]>([
  ``,
  () => {},
]);

export const SessionStateProvider: FC = () => {
  const session = useState(``);
  return <SessionState.Provider value={session}></SessionState.Provider>;
};
export const useSession = () => useContext(SessionState)[0];
export const setSession = (session: string) => {
  const [, setter] = useContext(SessionState);
  setter(session);
};
