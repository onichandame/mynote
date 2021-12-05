import { Menu } from '@mui/icons-material'
import { AppBar, Drawer, IconButton, Toolbar, Typography } from '@mui/material'
import { Box } from '@mui/system'
import { FC, useState } from 'react'
import { Link } from 'react-router-dom'

import { Sidebar } from '../sidebar'

export const Topbar: FC = () => {
  const [sidebarOpen, setSidebarOpen] = useState(false)
  return (
    <>
      <AppBar position="fixed">
        <Toolbar>
          <Typography
            variant="h6"
            component={Link}
            to="/"
            sx={{
              textDecoration: `none`,
              color: theme => theme.palette.text.primary,
            }}
          >
            My Notes
          </Typography>
          <Box sx={{ flexGrow: 1 }} />
          <IconButton size="large" onClick={() => setSidebarOpen(true)}>
            <Menu />
          </IconButton>
        </Toolbar>
      </AppBar>
      <Drawer
        anchor="right"
        open={sidebarOpen}
        onClose={() => setSidebarOpen(false)}
      >
        <Sidebar close={() => setSidebarOpen(false)} />
      </Drawer>
    </>
  )
}
