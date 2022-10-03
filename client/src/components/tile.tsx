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
import { ReactNode } from "react"
import { Link } from "./link"

export function Tile({
  title,
  description,
  icon,
  linkText,
  link,
}: {
  title: string
  description?: string
  icon?: ReactNode
  linkText?: string
  link?: string
}) {
  const body = (
    <>
      <CardContent>
        <Box display="flex" alignItems="center">
          <Box flexGrow={1} flexShrink={1}>
            <Typography variant="h5">{title}</Typography>
            {description && (
              <Typography>Customize your nickname and avatar</Typography>
            )}
          </Box>
          {icon && (
            <Box flexGrow={0} flexShrink={0}>
              <CardMedia>{icon}</CardMedia>
            </Box>
          )}
        </Box>
      </CardContent>
      {linkText && (
        <>
          <Divider />
          <CardActions sx={{ color: theme => theme.palette.primary.main }}>
            Manage your account data
          </CardActions>
        </>
      )}
    </>
  )
  return (
    <Card variant="outlined">
      {link ? (
        <Link to={link}>
          <CardActionArea>{body}</CardActionArea>
        </Link>
      ) : (
        body
      )}
    </Card>
  )
}
