import { ContextType, createContext, PropsWithChildren, useEffect } from "react"
import { usePersistentState } from "../hooks/persistentState"

const key = `_deviceId`
const DeviceIdContext = createContext<string | null>(null)

export function DeviceIdProvider({ children }: PropsWithChildren) {
  const [id, setId, loading] = usePersistentState<
    ContextType<typeof DeviceIdContext>
  >(key, null)
  useEffect(() => {
    if (!loading && !id) {
      setId(window.crypto.randomUUID())
    }
  }, [loading, id])
  return (
    <DeviceIdContext.Provider value={id}>{children}</DeviceIdContext.Provider>
  )
}

type DeviceId = string | null
