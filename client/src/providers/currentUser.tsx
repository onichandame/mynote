import {
  createContext,
  PropsWithChildren,
  useCallback,
  useContext,
  useEffect,
  useState,
} from "react"

import { useSession } from "../hooks/session"
import { useClient } from "./client"

const CurrentUserContext = createContext<{
  user?: Nullable<User>
  reload: () => void
  loading: boolean
} | null>(null)

export function CurrentUserProvider({ children }: PropsWithChildren) {
  const client = useClient()
  const [session, setSession] = useSession()
  const [user, setUser] = useState<null | User>(null)
  const [loading, setLoading] = useState(true)
  const reload = useCallback(async () => {
    setLoading(true)
    if (session) {
      try {
        setUser(await client.getSelf())
      } catch (e) {
        setSession(null, false)
      }
    }
    setLoading(false)
  }, [session, setUser, client, setSession])
  useEffect(() => {
    reload()
  }, [reload])
  return (
    <CurrentUserContext.Provider value={{ user, reload, loading }}>
      {children}
    </CurrentUserContext.Provider>
  )
}

export function useCurrentUser() {
  const userState = useContext(CurrentUserContext)
  if (!userState)
    throw new Error(
      `useCurrentUser must be called within components under CurrentUserProvider`
    )
  return userState
}
