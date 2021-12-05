import { FC } from 'react'
import { useParams } from 'react-router'

import { Delete } from './delete'
import { ID } from './id'

export const Note: FC = () => {
  const params = useParams()
  return (
    <ID.Provider value={params.id || ``}>
      <Delete />
    </ID.Provider>
  )
}
