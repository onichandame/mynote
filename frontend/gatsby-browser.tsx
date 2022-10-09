import { CssBaseline } from "@mui/material"
import { DndProvider } from "react-dnd"
import { TouchBackend } from "react-dnd-touch-backend"
import { SnackbarProvider } from "notistack"
import { GatsbyBrowser } from "gatsby"
import React from "react"

import { ClientProvider } from "./src/providers/client"
import { CurrentUserProvider } from "./src/providers/currentUser"
import { SessionProvider } from "./src/providers/session"

export const wrapPageElement: GatsbyBrowser["wrapPageElement"] = ({
  element,
}) => {
  return (
    <>
      <CssBaseline />
      {element}
    </>
  )
}

export const wrapRootElement: GatsbyBrowser["wrapRootElement"] = ({
  element,
}) => {
  return (
    <DndProvider backend={TouchBackend}>
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
