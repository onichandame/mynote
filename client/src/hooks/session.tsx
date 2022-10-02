import { usePersistentState } from "./persistentState"

export function useSession() {
  return usePersistentState<null | string>(`_session`, null)
}
