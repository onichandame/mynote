import { useTheme } from "@mui/material"
import { Link as ILink } from "gatsby-plugin-react-i18next"
import { ComponentPropsWithRef } from "react"

export function Link({
  original,
  ...props
}: { original?: boolean } & ComponentPropsWithRef<typeof ILink>) {
  const theme = useTheme()
  return (
    <ILink
      {...props}
      {...(original
        ? {}
        : {
            style: {
              textDecoration: `none`,
              color: theme.palette.text.primary,
              ...props.style,
            },
          })}
    />
  )
}
