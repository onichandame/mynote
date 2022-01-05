import { createContext, FC, useContext } from "react";
import { Client, createClient, ClientOptions } from "graphql-ws";

const ClientState = createContext<Client | null>(null);

const useClient = () => {
  const client = useContext(ClientState);
  if (!client) throw new Error(`client not initialized`);
  return client;
};
export const ClientProvider: FC<ClientOptions> = ({ children, ...opts }) => {
  const client = createClient(opts);
  return <ClientState.Provider value={client}>{children}</ClientState.Provider>;
};
// Query/Mutation
export const useQuery =
  <
    TData extends Record<string, unknown> = any,
    TVariable extends Record<string, unknown> = any
  >(
    query: string
  ) =>
  async (variables: TVariable) => {
    const client = useClient();
    return new Promise<TData>((r, j) => {
      let result: TData;
      client.subscribe<TData>(
        { query ,variables},
        {
          complete: () => r(result),
          error: (e) => j(e),
          next: (data) => (result = data),
        }
      );
    });
  };
export const useSubscription=<TData,TVariable>(query:string)=>(variables:TVariable)=>{
    const client=useClient()
    return new Promise((r,j)=>{
    })
}