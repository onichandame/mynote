import { SnackbarProvider } from "notistack";
import { FC } from "react";

import { BackendProvider } from "./backend";

export const Global: FC = ({ children }) => {
  return (
    <SnackbarProvider maxSnack={3}>
      <BackendProvider>{children}</BackendProvider>
    </SnackbarProvider>
  );
};
