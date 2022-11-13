import { CssBaseline } from "@mui/material"
import { DndProvider } from "react-dnd"
import { HTML5Backend } from "react-dnd-html5-backend"
import { SnackbarProvider } from "notistack"
import { GatsbySSR } from "gatsby"
import React from "react"

import { ClientProvider } from "./src/providers/client"
import { CurrentUserProvider } from "./src/providers/currentUser"
import { SessionProvider } from "./src/providers/session"

export const wrapPageElement: GatsbySSR["wrapPageElement"] = ({ element }) => {
  return (
    <>
      <CssBaseline />
      {element}
    </>
  )
}

export const wrapRootElement: GatsbySSR["wrapRootElement"] = ({ element }) => {
  return (
    <DndProvider backend={HTML5Backend}>
      <SnackbarProvider>
        <SessionProvider>
          <ClientProvider>
            <CurrentUserProvider>{element}</CurrentUserProvider>
          </ClientProvider>
        </SessionProvider>
      </SnackbarProvider>
    </DndProvider>
  )
}
