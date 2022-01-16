import { Grid, Typography } from "@mui/material";
import ReactMarkdown from "react-markdown";
import { FC } from "react";

import { Note } from "./type";

export const Detail: FC<{ note: Note }> = ({ note }) => {
  return (
    <Grid container direction="column" spacing={4}>
      <Grid item>
        <Typography variant="h5">{note.title}</Typography>
      </Grid>
      <Grid item>
        <ReactMarkdown>{note.content}</ReactMarkdown>
      </Grid>
    </Grid>
  );
};
