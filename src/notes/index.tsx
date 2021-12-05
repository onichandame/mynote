import { useLiveQuery } from 'dexie-react-hooks'
import { ListItemButton, List, ListItemText } from '@mui/material'
import { FC } from 'react'
import { useNavigate } from 'react-router'

import { db } from '../db'

export const Notes: FC = () => {
  const nav = useNavigate()
  const notes = useLiveQuery(() => db.notes.reverse().sortBy(`createdAt`))
  return (
    <List>
      {notes?.map(note => (
        <ListItemButton
          key={note.id}
          onClick={() => {
            nav(`${note.id}`)
          }}
        >
          <ListItemText>{note.title}</ListItemText>
        </ListItemButton>
      ))}
    </List>
  )
}
