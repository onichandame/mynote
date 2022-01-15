import { Grid, Typography } from "@mui/material";
import ReactMarkdown from "react-markdown";
import { useSnackbar } from "notistack";
import { FC, useEffect, useState } from "react";
import { useParams } from "react-router-dom";

import { useFetcher } from "../request";
import { Note as FullNote } from "./type";

type Note = Pick<
  FullNote,
  "id" | "createdAt" | "updatedAt" | "title" | "content"
>;

export const Detail: FC = () => {
  const { enqueueSnackbar } = useSnackbar();
  const [note, setNote] = useState<Note | null>(null);
  const params = useParams();
  const fetch = useFetcher<
    { getNote: Note },
    { id: number }
  >(`query getNote($id:Int!){
        getNote(id:$id){
            id
            createdAt
            updatedAt
            title
            content
        }
    }`);
  useEffect(() => {
    if (!params.id) return;
    const id = parseInt(params.id);
    if (!id) return;
    let active = true;
    const [promise] = fetch({ id });
    promise
      .then((data) => {
        if (active) setNote(data.getNote);
      })
      .catch((e) => {
        enqueueSnackbar(JSON.stringify(e), { variant: `error` });
      });
    return () => {
      active = false;
    };
  }, [params]);
  return (
    <Grid container direction="column" spacing={4}>
      <Grid item>
        <Typography variant="h5">{note?.title}</Typography>
      </Grid>
      <Grid item>
        <ReactMarkdown>{note?.content || ``}</ReactMarkdown>
      </Grid>
    </Grid>
  );
};
