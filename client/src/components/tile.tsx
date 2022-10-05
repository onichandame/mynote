import {
  Box,
  Card,
  CardActionArea,
  CardActions,
  CardContent,
  CardMedia,
  Divider,
  Typography,
} from "@mui/material"
import { PropsWithChildren, ReactNode } from "react"
import { Link } from "./link"

export function Tile({
  title,
  description,
  icon,
  linkText,
  link,
  actions,
  children,
}: PropsWithChildren & {
  title?: string
  description?: string
  icon?: ReactNode
  linkText?: string
  link?: string
  actions?: ReactNode
}) {
  if (linkText && actions)
    throw new Error(`a tile must either have a 'to' or 'actions'`)
  const body = (
    <>
      <CardContent>
        <Box display="flex" alignItems="center">
          <Box flexGrow={1} flexShrink={1}>
            {title && <Typography variant="h5">{title}</Typography>}
            {description && (
              <Typography variant="body2">{description}</Typography>
            )}
          </Box>
          {icon && (
            <Box flexGrow={0} flexShrink={0}>
              <CardMedia>{icon}</CardMedia>
            </Box>
          )}
        </Box>
        {children}
      </CardContent>
      {linkText ? (
        <>
          <Divider />
          <CardActions sx={{ color: theme => theme.palette.primary.main }}>
            {linkText}
          </CardActions>
        </>
      ) : actions ? (
        <CardActions sx={{ justifyContent: `flex-end` }}>{actions}</CardActions>
      ) : (
        <></>
      )}
    </>
  )
  return (
    <Card variant="outlined" sx={{ height: `100%` }}>
      {link ? (
        <Link to={link}>
          <CardActionArea sx={{ height: `100%` }}>{body}</CardActionArea>
        </Link>
      ) : (
        body
      )}
    </Card>
  )
}
