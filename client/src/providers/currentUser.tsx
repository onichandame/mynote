import {
  createContext,
  PropsWithChildren,
  useContext,
  useEffect,
  useRef,
  useState,
} from "react"

import { useClient } from "./client"

const CurrentUserContext = createContext<{
  user?: Nullable<User>
  reload: () => void
  loading: boolean
} | null>(null)

export function CurrentUserProvider({ children }: PropsWithChildren) {
  const client = useClient()
  const [loading, setLoading] = useState(true)
  const [user, setUser] = useState<null | User>(null)
  const inited = useRef(false)
  useEffect(() => {
    if (inited.current) setLoading(true)
    else inited.current = true
  }, [client])
  useEffect(() => {
    let active = true
    if (loading) {
      client
        .getSelf()
        .then(self => {
          if (active) setUser(self ?? null)
        })
        .finally(() => {
          if (active) setLoading(false)
        })
    }
    return () => {
      active = false
    }
  }, [client, loading])
  return (
    <CurrentUserContext.Provider
      value={{
        user,
        reload: () => {
          setLoading(true)
        },
        loading,
      }}
    >
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
