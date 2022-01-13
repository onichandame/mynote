import { createContext, FC, useContext, useEffect } from "react";
import { Client, createClient } from "graphql-ws";
import { useSession } from "../auth";

const ClientState = createContext<Client | null>(null);

export const useClient = () => useContext(ClientState);
export const ClientProvider: FC = ({ children }) => {
  const session = useSession();
  const client = createClient({
    url: window.location.protocol.startsWith(`https`)
      ? `wss://`
      : `ws://` + window.location.host + `/graphql`,
    lazy: false,
    connectionParams: session ? { session } : undefined,
  });
  useEffect(() => {
    client.on(`connected`, () => {
      console.log(`connected`);
    });
    client.on(`closed`, () => {
      console.log(`closed`);
    });
    client.on(`connecting`, () => {
      console.log(`connecting`);
    });
    client.on(`error`, (e) => {
      console.log(e);
    });
    client.on(`opened`, () => {
      console.log(`opened`);
    });
  }, [client]);
  return <ClientState.Provider value={client}>{children}</ClientState.Provider>;
};
