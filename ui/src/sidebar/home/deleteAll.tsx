import { Delete } from '@mui/icons-material'
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
import { FC, useContext, useState } from 'react'

import { db } from '../../db'
import { Close } from '../close'

export const DeleteAll: FC = () => {
  const close = useContext(Close)
  const [diagOpen, setDiagOpen] = useState(false)
  return (
    <>
      <ListItemButton
        onClick={() => {
          setDiagOpen(true)
        }}
      >
        <ListItemIcon>
          <Delete color="secondary" />
        </ListItemIcon>
        <ListItemText>Delete All</ListItemText>
      </ListItemButton>
      <Dialog open={diagOpen} onClose={() => setDiagOpen(false)}>
        <DialogTitle>Delete All</DialogTitle>
        <DialogContent>
          <DialogContentText>
            Do you wish to DELETE all the notes? This action cannot be reverted
          </DialogContentText>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => close()} color="inherit">
            Cancel
          </Button>
          <Button
            onClick={() => {
              db.notes.clear()
              close()
            }}
            color="secondary"
          >
            Delete All
          </Button>
        </DialogActions>
      </Dialog>
    </>
  )
}
