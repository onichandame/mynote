import { FC } from 'react'
import { Box, Typography } from '@mui/material'
import { useParams } from 'react-router'
import ReactMarkdown from 'react-markdown'
import { useLiveQuery } from 'dexie-react-hooks'

import { db } from '../db'

export const Note: FC = () => {
  const params = useParams()
  const note = useLiveQuery(
    () => (params.id ? db.notes.get(parseInt(params.id)) : undefined),
    [params.id]
  )
  return (
    <Box sx={{ flexGrow: 1 }}>
      <Typography variant="h3" marginBottom={theme => theme.spacing(3)}>
        {note?.title}
      </Typography>
      <Typography variant="h6">
        <ReactMarkdown children={note?.content || ``} />
      </Typography>
    </Box>
  )
}
