import { Assessment, NoteAlt } from "@mui/icons-material"
import { BottomNavigation, BottomNavigationAction } from "@mui/material"
import { useI18next } from "gatsby-plugin-react-i18next"
import { ComponentProps, forwardRef, PropsWithChildren } from "react"
import { useTranslateScoped } from "../hooks/translate"

import * as routes from "../routes"
import { Link } from "./link"

export function Dashboard({ children }: PropsWithChildren) {
  const { originalPath } = useI18next()
  const translate = useTranslate()
  return (
    <>
      {children}
      <BottomNavigation
        showLabels
        value={originalPath.replace(/\/$/, ``)}
        sx={{ bottom: 0, left: 0, right: 0, position: `fixed` }}
      >
        <TabItem
          value={routes.MEMO}
          label={translate(`memoTitle`)}
          icon={<NoteAlt />}
          to={routes.MEMO}
        />
        <TabItem
          value={routes.REPORT}
          label={translate(`reportTitle`)}
          icon={<Assessment />}
          to={routes.REPORT}
        />
      </BottomNavigation>
    </>
  )
}

function TabItem(
  props: Omit<ComponentProps<typeof BottomNavigationAction>, "LinkComponent"> &
    ComponentProps<typeof Link>
) {
  const MyLink = forwardRef<HTMLAnchorElement, ComponentProps<typeof Link>>(
    (props, ref) => <Link original ref={ref} {...props} />
  )
  return <BottomNavigationAction LinkComponent={MyLink} {...props} />
}

function useTranslate() {
  return useTranslateScoped(`dashboard`)
}
