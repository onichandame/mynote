import {
  AppBar,
  Avatar,
  Box,
  Container,
  Divider,
  Drawer,
  FormControl,
  IconButton,
  List,
  MenuItem,
  Select,
  Toolbar,
  Typography,
  useTheme,
} from "@mui/material"
import {
  PropsWithChildren,
  ReactNode,
  useCallback,
  useEffect,
  useReducer,
  useState,
} from "react"
import {
  AccountCircle,
  ArrowBackIos,
  Login,
  Logout,
  PersonOutline,
  Settings,
} from "@mui/icons-material"
import { useI18next } from "gatsby-plugin-react-i18next"

import { useCurrentUser } from "../providers/currentUser"
import { Loading } from "./loading"
import { Link } from "./link"
import { NavigationItem } from "./navigationItem"
import { useTranslateScoped } from "../hooks/translate"
import * as routes from "../routes"

export function Layout({
  children,
  title,
  isPrivate,
  publicOnly,
}: PropsWithChildren & {
  title?: ReactNode
  isPrivate?: true
  publicOnly?: true
}) {
  const {
    i18n: { language },
    originalPath,
    languages,
    navigate,
  } = useI18next()
  const translate = useTranslate()
  const { user, loading: userLoading } = useCurrentUser()
  useEffect(() => {
    if (publicOnly && user) navigate(`/`)
  }, [publicOnly, user, navigate])
  return (
    <Box sx={{ flexGrow: 1 }}>
      <header>
        <AppBar position="static">
          <Toolbar variant="dense">
            {originalPath !== `/` ? (
              <IconButton
                onClick={() =>
                  navigate(
                    originalPath
                      .replace(/\/$/, ``)
                      .split(`/`)
                      .slice(0, -1)
                      .join(`/`)
                  )
                }
              >
                <ArrowBackIos />
              </IconButton>
            ) : (
              <></>
            )}
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
            <Ending />
          </Toolbar>
        </AppBar>
      </header>
      {userLoading ? (
        <Loading />
      ) : (
        <main>
          <Toolbar />
          {isPrivate ? user ? children : <Redirect /> : children}
        </main>
      )}
    </Box>
  )
}

function Ending() {
  const translate = useTranslate()
  const { user } = useCurrentUser()
  const [drawerOpen, setDrawerOpen] = useState(false)
  const openDrawer = useCallback(() => {
    setDrawerOpen(true)
  }, [setDrawerOpen])
  const closeDrawer = useCallback(() => {
    setDrawerOpen(false)
  }, [setDrawerOpen])
  return (
    <>
      <IconButton size="small" onClick={openDrawer}>
        {user ? (
          user.avatar ? (
            <Avatar alt={user.name} src={user.avatar} />
          ) : (
            <Avatar>
              {user.name.split(` `).splice(0, 2).join(``).toUpperCase()}
            </Avatar>
          )
        ) : (
          <Avatar sx={{ bgcolor: theme => theme.palette.primary.dark }}>
            <AccountCircle />
          </Avatar>
        )}
      </IconButton>
      <Drawer open={drawerOpen} onClose={closeDrawer} anchor="right">
        <List sx={{ minWidth: 200 }}>
          <Toolbar variant="dense" />
          <Divider />
          {user ? (
            <>
              <NavigationItem
                to={routes.SETTINGS}
                icon={<Settings />}
                title={translate(`settings`)}
              />
              <Divider />
              <NavigationItem
                to={routes.LOG_OUT}
                icon={<Logout />}
                title={translate(`log out`)}
                variant="error"
              />
            </>
          ) : (
            <>
              <NavigationItem
                to={routes.LOG_IN}
                icon={<Login />}
                title={translate(`log in`)}
              />
              <NavigationItem
                to={routes.SIGN_UP}
                icon={<PersonOutline />}
                title={translate(`sign up`)}
              />
            </>
          )}
          <Divider />
        </List>
      </Drawer>
    </>
  )
}

function Redirect({ countdown }: { countdown?: number }) {
  const { navigate } = useI18next()
  const [remaining, reduceRemaining] = useReducer(
    count => count - 1,
    countdown ?? 5
  )
  useEffect(() => {
    const counter = setInterval(reduceRemaining, 1000)
    return () => clearInterval(counter)
  }, [reduceRemaining])
  useEffect(() => {
    if (remaining <= 0) navigate(routes.LOG_IN)
  }, [remaining])
  return (
    <Container>
      <Typography>
        Please{" "}
        <Link original to={routes.LOG_IN}>
          log in
        </Link>{" "}
        or{" "}
        <Link original to={routes.SIGN_UP}>
          sign up
        </Link>{" "}
        to use your private notebook. Redirecting to login page in {remaining}
        s...
      </Typography>
    </Container>
  )
}

function useTranslate() {
  return useTranslateScoped(`layout`)
}
