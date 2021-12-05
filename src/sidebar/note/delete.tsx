import { Delete as DeleteIcon } from '@mui/icons-material'
import {
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogContentText,
  DialogTitle,
  ListItemButton,
  ListItemIcon,
  ListItemText,
} from '@mui/material'
import { useLiveQuery } from 'dexie-react-hooks'
import { FC, useContext, useState } from 'react'
import { useNavigate } from 'react-router-dom'

import { db } from '../../db'
import { Close } from '../close'
import { ID } from './id'

export const Delete: FC = () => {
  const close = useContext(Close)
  const [diagOpen, setDiagOpen] = useState(false)
  const id = useContext(ID)
  const note = useLiveQuery(() => db.notes.get(parseInt(id)), [id])
  const nav = useNavigate()
  return (
    <>
      <ListItemButton
        onClick={() => {
          setDiagOpen(true)
        }}
      >
        <ListItemIcon>
          <DeleteIcon color="secondary" />
        </ListItemIcon>
        <ListItemText>Delete</ListItemText>
      </ListItemButton>
      <Dialog open={diagOpen} onClose={() => setDiagOpen(false)}>
        <DialogTitle>Delete</DialogTitle>
        <DialogContent>
          <DialogContentText>
            Do you wish to delete note {note?.title}?
          </DialogContentText>
        </DialogContent>
        <DialogActions>
          <Button color="inherit" onClick={() => close()}>
            cancel
          </Button>
          <Button
            color="secondary"
            onClick={() => {
              db.notes.delete(parseInt(id))
              close()
              nav(`..`)
            }}
          >
            Delete
          </Button>
        </DialogActions>
      </Dialog>
    </>
  )
}
