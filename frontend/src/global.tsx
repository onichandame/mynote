import { SnackbarProvider } from "notistack";
import { FC } from "react";

import { SessionProvider, UserProvider } from "./auth";
import { BackendProvider } from "./backend";

export const Global: FC = ({ children }) => {
  return (
    <SnackbarProvider maxSnack={3}>
      <SessionProvider>
        <BackendProvider>
          <UserProvider>{children}</UserProvider>
        </BackendProvider>
      </SessionProvider>
    </SnackbarProvider>
  );
};
