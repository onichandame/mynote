import { ContextType, FC, useContext, useState } from 'react'
import { Navigate, Route, Routes, HashRouter } from 'react-router-dom'
import { Box, Toolbar } from '@mui/material'

import { Topbar } from './topbar'
import { Dial } from './dial'
import { CreateNote } from './createNote'
import { Notes } from './notes'
import { Note } from './note'
import { SessionContext, UserContext, UserWatcher } from './auth'
import { Login } from './login'
import { ClientProvider } from './fetcher'

export const App: FC = () => {
  const sessionState = useState<ContextType<typeof SessionContext>[0]>(
    window.localStorage.getItem(`mynote_session`) || ``
  )
  const userState = useState<ContextType<typeof UserContext>[0]>(null)
  return (
    <HashRouter>
      <ClientProvider>
        <SessionContext.Provider value={sessionState}>
          <UserContext.Provider value={userState}>
            <Box sx={{ display: 'flex' }}>
              <Topbar />
              <Box sx={{ flexGrow: 1, padding: theme => theme.spacing(3) }}>
                <Toolbar />
                <Routes>
                  <Route path="login" element={<Login />} />
                  <Route path="/">
                    {
                      (() => {
                        const [user] = useContext(UserContext)
                        return user ? (
                          <>
                            <Route path="/" element={<Notes />} />
                            <Route path="create" element={<CreateNote />} />
                            <Route path=":id" element={<Note />} />
                          </>
                        ) : (
                          <Navigate to="/login" />
                        )
                      }) as FC
                    }
                  </Route>
                </Routes>
              </Box>
              <Dial />
            </Box>
            <UserWatcher />
          </UserContext.Provider>
        </SessionContext.Provider>
      </ClientProvider>
    </HashRouter>
  )
}
