import { Sync as SyncIcon } from '@mui/icons-material'
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

import { Close } from '../close'

export const Sync: FC = () => {
  const [diagOpen, setDiagOpen] = useState(false)
  const close = useContext(Close)
  const [pc] = useState(new RTCPeerConnection())
  const [chanLabel, setChanLabel] = useState(``)
  return (
    <>
      <ListItemButton
        onClick={() => {
          setDiagOpen(true)
          const datachan = pc.createDataChannel(
            Math.random().toString().substr(2, 6)
          )
          setChanLabel(datachan.label)
          datachan.onopen = e => {
            if (datachan) {
              datachan.send(``)
              datachan.close()
            }
          }
        }}
      >
        <ListItemIcon>
          <SyncIcon color="primary" />
        </ListItemIcon>
        <ListItemText>Sync</ListItemText>
      </ListItemButton>
      <Dialog open={diagOpen} onClose={() => setDiagOpen(false)}>
        <DialogTitle>Sync</DialogTitle>
        <DialogContent>
          <DialogContentText>
            <p>Data channel:</p>
            <p>{chanLabel}</p>
          </DialogContentText>
        </DialogContent>
        <DialogActions>
          <Button color="inherit" onClick={() => close()}>
            cancel
          </Button>
          <Button
            color="primary"
            onClick={() => {
              close()
            }}
          >
            connect to ...
          </Button>
        </DialogActions>
      </Dialog>
    </>
  )
}
