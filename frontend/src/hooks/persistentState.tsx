import { useEffect, useReducer } from "react"

/** Synchronized state which is automatically stored to local storage */
export function usePersistentState<TData>(
  key: string,
  defaultValue: TData
): PersistentState<TData> {
  const [value, setValue] = useReducer(
    (_old: TData, opts: { value: TData; noPersistent: boolean }) => {
      if (!opts.noPersistent)
        window.localStorage.setItem(key, JSON.stringify(opts.value))
      return opts.value
    },
    defaultValue
  )
  useEffect(() => {
    const savedValue = window.localStorage.getItem(key)
    try {
      if (savedValue)
        setValue({ value: JSON.parse(savedValue), noPersistent: true })
    } catch (e) {
      console.error(e)
      window.localStorage.removeItem(key)
    }
  }, [])
  return [
    value,
    /** @param sync - If true the new value will be saved to local storage. Default to true */
    (value: TData, sync = true) => setValue({ value, noPersistent: !sync }),
  ]
}

export type PersistentState<TData> = [
  TData,
  (value: TData, sync?: boolean) => void
]
