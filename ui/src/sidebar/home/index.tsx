import { FC } from 'react'

import { DeleteAll } from './deleteAll'
import { Sync } from './sync'

export const Home: FC = () => {
  return (
    <>
      <DeleteAll />
      <Sync />
    </>
  )
}
