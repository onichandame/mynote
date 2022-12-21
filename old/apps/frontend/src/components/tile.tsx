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
  onClick,
  children,
}: BasicProps) {
  if (linkText && actions)
    throw new Error(`a tile cannot have both 'to' and 'actions'`)
  if (link && onClick)
    throw new Error(`a tile cannot have both 'link' and 'onClick'`)
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
      ) : onClick ? (
        <CardActionArea onClick={onClick} sx={{ height: `100%` }}>
          {body}
        </CardActionArea>
      ) : (
        body
      )}
    </Card>
  )
}

type BasicProps = PropsWithChildren & {
  title?: string
  description?: string
  icon?: ReactNode
  linkText?: string
  link?: string
  actions?: ReactNode
  onClick?: () => void
}
