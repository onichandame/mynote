import { Logout, Menu, Settings } from "@mui/icons-material"
import {
  AppBar,
  Box,
  Divider,
  Drawer,
  FormControl,
  IconButton,
  List,
  MenuItem,
  Select,
  Toolbar,
  Typography,
} from "@mui/material"
import { useI18next } from "gatsby-plugin-react-i18next"
import { useCallback, useState } from "react"

import * as routes from "../../routes"
import { Link } from "../link"
import { NavigationItem } from "../navigationItem"
import { useTranslate } from "./utils"

export function NavBar({ title }: { title: string }) {
  const { originalPath, languages, language } = useI18next()
  const translate = useTranslate()
  return (
    <header>
      <AppBar position="static">
        <Toolbar variant="dense">
          <Hamburger />
          <Typography variant="h5">{title ?? translate(`title`)}</Typography>
          <Box sx={{ flexGrow: 1 }} />
          <FormControl size="small">
            <Select value={language}>
              {languages.map(lang => (
                <Link
                  key={lang}
                  {...{ value: lang }}
                  to={originalPath}
                  language={lang}
                >
                  <MenuItem value={lang} dense>
                    {translate(lang)}
                  </MenuItem>
                </Link>
              ))}
            </Select>
          </FormControl>
        </Toolbar>
      </AppBar>
    </header>
  )
}

function Hamburger() {
  const translate = useTranslate()
  const [drawerOpen, setDrawerOpen] = useState(false)
  const openDrawer = useCallback(() => {
    setDrawerOpen(true)
  }, [setDrawerOpen])
  const closeDrawer = useCallback(() => {
    setDrawerOpen(false)
  }, [setDrawerOpen])
  return (
    <>
      <IconButton
        size="large"
        edge="start"
        color="inherit"
        onClick={openDrawer}
      >
        <Menu />
      </IconButton>
      <Drawer open={drawerOpen} onClose={closeDrawer} anchor="left">
        <List sx={{ minWidth: 200 }}>
          <Toolbar variant="dense" />
          <Divider />
          <NavigationItem
            to={routes.SETTINGS}
            icon={<Settings />}
            title={translate(`settings`)}
          />
        </List>
      </Drawer>
    </>
  )
}
