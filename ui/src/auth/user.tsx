import { useQuery } from '@onichandame/react-graphql-ws'
import { FC, useContext, useEffect } from 'react'
import { SessionContext } from './session'

import { useClient } from '../fetcher'

import { createStateContext } from './state'

export type User = { name: string; id: number }

const UserContext = createStateContext<User | null>(null)
export { UserContext }

export const UserWatcher: FC = () => {
  const client = useClient()
  const [session] = useContext(SessionContext)
  const [, setUser] = useContext(UserContext)
  const update = useQuery<{ getUser: { id: number; name: string } }>({
    client,
    query: `query{getUser{id name}}`,
  })
  useEffect(() => {
    let active = true
    if (session) {
      update().then(res => {
        if (active) {
          if (res.data) {
            setUser(res.data.getUser)
          }
        }
      })
    }
    return () => {
      active = false
    }
  }, [session])
  return <></>
}
