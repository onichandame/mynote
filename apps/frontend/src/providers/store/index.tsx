import {
  createContext,
  PropsWithChildren,
  useContext,
  useEffect,
  useState,
} from "react"
import { getRxStorageDexie } from "rxdb/plugins/dexie"
import { createRxDatabase, RxDatabase } from "rxdb"
import { useSnackbar } from "notistack"

import {
  Collections,
  memoMethods,
  memoPreInsert,
  memoPreSave,
  memoSchema,
} from "./collections"

const StoreContext = createContext<Database | null>(null)

export function StoreProvider({ children }: PropsWithChildren) {
  const [db, setDb] = useState<Database | null>(null)
  const { enqueueSnackbar } = useSnackbar()

  useEffect(() => {
    let active = true
    let d: Database | null = null
    ;(async () => {
      try {
        const db = await createRxDatabase<Collections>({
          name: `notebook`,
          storage: getRxStorageDexie(),
        })
        await db.addCollections({
          memos: { schema: memoSchema, methods: memoMethods },
        })
        // TODO: trigger sync
        db.memos.preInsert(memoPreInsert, true)
        db.memos.preSave(memoPreSave, true)
        if (active) {
          setDb(db)
          d = db
        }
      } catch (e) {
        if (active)
          enqueueSnackbar(e instanceof Error ? e.message : JSON.stringify(e), {
            variant: `error`,
            persist: true,
          })
      }
    })()
    return () => {
      active = false
      d?.destroy()
    }
  }, [])

  return <StoreContext.Provider value={db}>{children}</StoreContext.Provider>
}

export const useStore = () => useContext(StoreContext)

type Database = RxDatabase<Collections>
