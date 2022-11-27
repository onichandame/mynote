import { createContext, PropsWithChildren, useContext } from "react"

import { PersistentState, usePersistentState } from "../hooks/persistentState"

const SessionContext = createContext<PersistentState<null | string>>([
  null,
  () => {},
  true,
])

export function SessionProvider({ children }: PropsWithChildren) {
  const sessionState = usePersistentState<string | null>(`_session`, null)
  return (
    <SessionContext.Provider value={sessionState}>
      {children}
    </SessionContext.Provider>
  )
}

export function useSession() {
  return useContext(SessionContext)
}
