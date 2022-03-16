import { Grid } from "@mui/material";
import { FC } from "react";

export const Form: FC = ({ children }) => {
  return (
    <Grid container direction="column" alignItems="center" spacing={2}>
      <Grid item>{children}</Grid>
    </Grid>
  );
};
