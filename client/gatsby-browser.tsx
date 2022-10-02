import { CssBaseline } from "@mui/material"
import { SnackbarProvider } from "notistack"
import { GatsbyBrowser } from "gatsby"
import React from "react"

import { ClientProvider } from "./src/providers/client"
import { CurrentUserProvider } from "./src/providers/currentUser"

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
    <SnackbarProvider>
      <ClientProvider>
        <CurrentUserProvider>{element}</CurrentUserProvider>
      </ClientProvider>
    </SnackbarProvider>
  )
}
