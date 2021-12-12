import { createContext, useState } from 'react'

export const createStateContext = <T>(def: T) => {
  const tmp = (v: T) => useState<T>(v)
  type State = ReturnType<typeof tmp>
  return createContext<State>([def, () => {}])
}
