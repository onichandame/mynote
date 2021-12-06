import { FC } from 'react'
import { Route, Routes, HashRouter } from 'react-router-dom'
import { Box, Toolbar } from '@mui/material'

import { Topbar } from './topbar'
import { Dial } from './dial'
import { CreateNote } from './createNote'
import { Notes } from './notes'
import { Note } from './note'

export const App: FC = () => {
  return (
    <HashRouter>
      <Box sx={{ display: 'flex' }}>
        <Topbar />
        <Box sx={{ flexGrow: 1, padding: theme => theme.spacing(3) }}>
          <Toolbar />
          <Routes>
            <Route path="createNote" element={<CreateNote />} />
            <Route path="/">
              <Route path="/" element={<Notes />} />
              <Route path=":id" element={<Note />} />
            </Route>
          </Routes>
        </Box>
        <Dial />
      </Box>
    </HashRouter>
  )
}
