import { useTheme } from "@mui/material"
import { Link as ILink } from "gatsby-plugin-react-i18next"
import { ComponentPropsWithRef, forwardRef } from "react"

export const Link = forwardRef<
  HTMLAnchorElement,
  { original?: boolean } & ComponentPropsWithRef<typeof ILink>
>(({ original, ...props }, ref) => {
  const theme = useTheme()
  return (
    <ILink
      ref={ref}
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
})
