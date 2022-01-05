import { FC, useState } from "react";
import { SessionStateProvider } from "./auth";

export const Global: FC = ({ children }) => {
  return <SessionStateProvider>{children}</SessionStateProvider>;
};
