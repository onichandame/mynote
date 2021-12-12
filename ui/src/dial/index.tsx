import { NoteAdd } from '@mui/icons-material'
import { SpeedDial, SpeedDialAction, SpeedDialIcon } from '@mui/material'
import { FC } from 'react'
import { useNavigate } from 'react-router-dom'

export const Dial: FC = () => {
  const nav = useNavigate()
  return (
    <SpeedDial
      ariaLabel="Add note"
      sx={{
        position: `absolute`,
        bottom: theme => theme.spacing(6),
        right: theme => theme.spacing(6),
      }}
      icon={<SpeedDialIcon />}
    >
      <SpeedDialAction
        onClick={() => nav(`/create`)}
        icon={<NoteAdd color="primary" />}
        tooltipTitle="Add note"
        tooltipOpen
      />
    </SpeedDial>
  )
}
