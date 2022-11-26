import { Box, Container, Toolbar } from "@mui/material"
import { PropsWithChildren, ReactNode } from "react"

import { NavBar } from "./navbar"

export function Layout({
  children,
  title = `Notebook`,
}: PropsWithChildren & {
  title?: string
}) {
  return (
    <Box sx={{ flexGrow: 1 }}>
      <NavBar title={title} />
      <main>
        <Toolbar />
        <Container>{children}</Container>
      </main>
    </Box>
  )
}
