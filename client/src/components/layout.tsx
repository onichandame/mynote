import {
  AppBar,
  Avatar,
  Box,
  Divider,
  Drawer,
  FormControl,
  Grid,
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
import { useI18next, useTranslation } from "gatsby-plugin-react-i18next"

import { useCurrentUser } from "../providers/currentUser"
import { Loading } from "./loading"
import { Link } from "./link"
import { NavigationItem } from "./navigationItem"

export function Layout({
  children,
  title,
  isPrivate,
}: PropsWithChildren & { title?: ReactNode; isPrivate?: true }) {
  const { t } = useTranslation()
  const {
    i18n: { language },
    originalPath,
    languages,
  } = useI18next()
  const translate = useCallback((key: string) => t(key, { ns: `layout` }), [t])
  const { user, loading: userLoading } = useCurrentUser()
  const theme = useTheme()
  return (
    <Box sx={{ flexGrow: 1 }}>
      <header>
        <AppBar position="static">
          <Toolbar variant="dense">
            {originalPath !== `/` ? (
              <IconButton onClick={() => window.history.back()}>
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
      <main>
        <Toolbar />
        <Grid container direction="column" alignItems="center">
          <Grid item>
            {isPrivate ? (
              userLoading ? (
                <Loading />
              ) : user ? (
                children
              ) : (
                <Redirect />
              )
            ) : (
              children
            )}
          </Grid>
        </Grid>
      </main>
    </Box>
  )
}

function Ending({ user }: { user?: Nullable<User> }) {
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
        <Box>
          <List>
            {user ? (
              <>
                <NavigationItem
                  to="/settings"
                  icon={<Settings />}
                  title="Settings"
                />
                <Divider />
                <NavigationItem
                  to="logout"
                  icon={<Logout />}
                  title="Log Out"
                  variant="error"
                />
              </>
            ) : (
              <>
                <NavigationItem to="/login" icon={<Login />} title="Log In" />
                <NavigationItem
                  to="/signup"
                  icon={<PersonOutline />}
                  title="Sign Up"
                />
              </>
            )}
          </List>
        </Box>
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
    if (remaining <= 0) navigate("/login")
  }, [remaining])
  return (
    <Typography>
      Please{" "}
      <Link original to="/login">
        log in
      </Link>{" "}
      or{" "}
      <Link original to="/signup">
        sign up
      </Link>{" "}
      to use your private notebook. Redirecting to login page in {remaining}s...
    </Typography>
  )
}
