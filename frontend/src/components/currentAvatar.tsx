import { AccountCircle } from "@mui/icons-material"
import { Avatar } from "@mui/material"
import { useCurrentUser } from "../providers/currentUser"

export function CurrentAvatar() {
  const { user } = useCurrentUser()
  return user ? (
    user.avatar ? (
      <Avatar
        sx={{ bgcolor: theme => theme.palette.primary.dark }}
        alt={user.name}
        src={user.avatar}
      />
    ) : (
      <Avatar sx={{ bgcolor: theme => theme.palette.primary.dark }}>
        {user.name
          .split(` `)
          .map(v => v.substring(0, 1))
          .splice(0, 2)
          .join(``)
          .toUpperCase()}
      </Avatar>
    )
  ) : (
    <Avatar sx={{ bgcolor: theme => theme.palette.primary.dark }}>
      <AccountCircle />
    </Avatar>
  )
}
