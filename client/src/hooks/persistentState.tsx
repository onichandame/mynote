import { useEffect, useState } from "react"

/** Synchronized state which is automatically stored to local storage */
export function usePersistentState<TData>(key: string, defaultValue: TData) {
  const [value, setValue] = useState<TData>(defaultValue)
  useEffect(() => {
    const savedValue = window.localStorage.getItem(key)
    if (savedValue !== null) {
      setValue(JSON.parse(savedValue))
    }
  }, [])
  return [
    value,
    /** @param sync - If true the update will be persisted. Default is true */
    (val: TData, sync: boolean = true) => {
      setValue(val)
      if (sync) window.localStorage.setItem(key, JSON.stringify(val))
    },
  ] as const
}
