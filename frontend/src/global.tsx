import { SnackbarProvider } from "notistack";
import { FC } from "react";
import { SessionProvider, UserProvider } from "./auth";
import { ClientProvider } from "./request";

export const Global: FC = ({ children }) => {
  return (
    <SessionProvider>
      <ClientProvider>
        <UserProvider>
          <SnackbarProvider maxSnack={3}>{children}</SnackbarProvider>
        </UserProvider>
      </ClientProvider>
    </SessionProvider>
  );
};
