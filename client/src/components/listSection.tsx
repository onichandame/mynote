import { ArrowForwardIos } from "@mui/icons-material"
import {
  Card,
  CardContent,
  List,
  ListItemButton,
  ListItemText,
  Typography,
} from "@mui/material"
import { PropsWithChildren, ReactNode } from "react"
import { Link } from "./link"

export function ListSection({
  children,
  title,
}: PropsWithChildren & { title: string }) {
  return (
    <Card variant="outlined">
      <CardContent>
        <Typography variant="h5">{title}</Typography>
        <List>{children}</List>
      </CardContent>
    </Card>
  )
}

export function ListSectionItem({
  title,
  value,
  to,
}: {
  title: string
  value?: ReactNode
  to?: string
}) {
  const body = (
    <ListItemButton divider>
      <ListItemText>
        <Typography sx={{ color: theme => theme.palette.text.disabled }}>
          {title}
        </Typography>
      </ListItemText>
      {value}
      {to && <ArrowForwardIos sx={{ marginLeft: 2 }} />}
    </ListItemButton>
  )
  return to ? <Link to={to}>{body}</Link> : body
}
