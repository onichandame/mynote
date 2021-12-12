import { ListItemButton, List, ListItemText } from '@mui/material'
import { useQuery } from '@onichandame/react-graphql-ws'
import { FC, useEffect, useState } from 'react'
import { useNavigate } from 'react-router'
import { number } from 'yup/lib/locale'

import { useClient } from '../fetcher'

export const Notes: FC = () => {
  const nav = useNavigate()
  const client = useClient()
  const [notes, setNotes] = useState<{ id: number; title: string }[]>([])
  const fetch = useQuery<{
    notes: { edges: { node: { id: number; title: string } }[] }
  }>({ client, query: `query{notes{edges{node{id title}}}}` })
  useEffect(() => {
    let active = true
    fetch().then(res => {
      if (active) {
        if (res.data) setNotes(res.data.notes.edges.map(v => v.node))
      }
    })
    return () => {
      active = false
    }
  }, [])
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
