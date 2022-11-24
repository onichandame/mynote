import { CssBaseline } from "@mui/material"
import { DndProvider } from "react-dnd"
import { HTML5Backend } from "react-dnd-html5-backend"
import { SnackbarProvider } from "notistack"
import { GatsbyBrowser } from "gatsby"
import React from "react"

import { ClientProvider } from "./src/providers/client"
import { CurrentUserProvider } from "./src/providers/currentUser"
import { SessionProvider } from "./src/providers/session"
import { StoreProvider } from "./src/providers/store"

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
    <DndProvider backend={HTML5Backend}>
      <SnackbarProvider>
        <StoreProvider>
          <SessionProvider>
            <ClientProvider>
              <CurrentUserProvider>{element}</CurrentUserProvider>
            </ClientProvider>
          </SessionProvider>
        </StoreProvider>
      </SnackbarProvider>
    </DndProvider>
  )
}
