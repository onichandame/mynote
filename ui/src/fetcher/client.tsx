import { createContext, FC, useContext } from 'react'
import { Client, createClient } from 'graphql-ws'

const ClientContext = createContext<Client>(null as any)

export const ClientProvider: FC = ({ children }) => {
  return (
    <ClientContext.Provider
      value={createClient({
        url: `${window.location.protocol === `https` ? `wss` : `ws`}//${
          window.location.host
        }/${window.location.pathname
          .split(`/`)
          .filter(v => !!v)
          .slice(0, -1)
          .concat(`graphql`)
          .join(`/`)}`,
        lazy: false,
      })}
    >
      {children}
    </ClientContext.Provider>
  )
}

export const useClient = () => {
  const client = useContext(ClientContext)
  return client
}

export { ClientContext }
