import { Note, Report } from "@mui/icons-material"
import { BottomNavigation, BottomNavigationAction, Paper } from "@mui/material"
import { useI18next } from "gatsby-plugin-react-i18next"
import { PropsWithChildren } from "react"

import * as routes from "../routes"
import { Link } from "./link"

export function Dashboard({ children }: PropsWithChildren) {
  const { originalPath } = useI18next()
  return (
    <>
      {children}
      <Paper sx={{ bottom: 0, left: 0, right: 0, position: `fixed` }}>
        <BottomNavigation showLabels value={originalPath}>
          <Link to={routes.NOTES}>
            <BottomNavigationAction
              value={routes.NOTES}
              label="Note"
              icon={<Note />}
            />
          </Link>
          <Link to={routes.REPORTS}>
            <BottomNavigationAction
              value={routes.REPORTS}
              label="Report"
              icon={<Report />}
            />
          </Link>
        </BottomNavigation>
      </Paper>
    </>
  )
}
