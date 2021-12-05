import { Box, List, ListItem } from '@mui/material'
import { FC } from 'react'
import { Route, Routes } from 'react-router-dom'

import { Close } from './close'
import { Home } from './home'
import { Note } from './note'

export const Sidebar: FC<{ close: () => void }> = ({ close }) => {
  return (
    <Close.Provider value={close}>
      <Box>
        <List>
          <Routes>
            <Route path=":id" element={<Note />} />
            <Route path="*" element={<Home />} />
          </Routes>
        </List>
      </Box>
    </Close.Provider>
  )
}
