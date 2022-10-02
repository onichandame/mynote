import { ArrowForwardIos } from "@mui/icons-material"
import { ListItemButton, ListItemIcon, ListItemText } from "@mui/material"
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
  return (
    <Link to={to}>
      <ListItemButton
        sx={{
          color: theme =>
            variant === `error`
              ? theme.palette.text.secondary
              : theme.palette.text.primary,
        }}
      >
        <ListItemIcon>{icon}</ListItemIcon>
        <ListItemText>{title}</ListItemText>
        <ArrowForwardIos />
      </ListItemButton>
    </Link>
  )
}

type Variant = "error" | "normal"
