import {
  ListItemButton,
  ListItemIcon,
  ListItemText,
  useTheme,
} from "@mui/material"
import { ReactNode } from "react"

import { Link } from "./link"

export function NavigationItem({
  icon,
  title,
  to,
  variant,
}: {
  icon?: ReactNode
  title: string
  to: string
  /** default to 'normal' */
  variant?: Variant
}) {
  const theme = useTheme()
  const color =
    variant === `error`
      ? theme.palette.secondary.light
      : theme.palette.text.primary
  return (
    <Link key={Math.random().toString(36).substring(2, 6)} to={to}>
      <ListItemButton
        divider
        sx={{
          color,
        }}
      >
        <ListItemIcon color={color}>{icon}</ListItemIcon>
        <ListItemText>
          <div style={{ color }}>{title}</div>
        </ListItemText>
      </ListItemButton>
    </Link>
  )
}

type Variant = "error" | "normal"
