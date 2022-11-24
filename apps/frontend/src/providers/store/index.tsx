import {
  createContext,
  PropsWithChildren,
  useContext,
  useEffect,
  useState,
} from "react"
import { getRxStorageDexie } from "rxdb/plugins/dexie"
import { createRxDatabase, RxDatabase } from "rxdb"
import { Collections, memoSchema } from "./collections"

const StoreContext = createContext<Database | null>(null)

export function StoreProvider({ children }: PropsWithChildren) {
  const [db, setDb] = useState<Database | null>(null)
  const [err, setErr] = useState<Error | null>(null)

  useEffect(() => {
    createRxDatabase<Collections>({
      name: `notebook`,
      storage: getRxStorageDexie(),
    })
      .then(async db => {
        await db.addCollections({ memos: { schema: memoSchema } })
        setDb(db)
      })
      .catch(e => {
        setErr(e)
      })
  }, [])

  return <StoreContext.Provider value={db}>{children}</StoreContext.Provider>
}

export const useStore = () => useContext(StoreContext)

type Database = RxDatabase<Collections>
