import {
  Card,
  CardActionArea,
  CardContent,
  Grid,
  Typography,
} from "@mui/material";
import { FC, useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";

import { useFetcher } from "../request";
import { Note as FullNote } from "./type";

type Note = Pick<FullNote, "title" | "id" | "createdAt">;

export const List: FC = () => {
  const navigate = useNavigate();
  const [notes, setNotes] = useState<Note[]>([]);
  const fetch = useFetcher<{
    listNotes: Note[];
  }>(`query listNotes{
        listNotes{
            id
            createdAt
            title
        }
    }`);
  useEffect(() => {
    let active = true;
    const [promise] = fetch({});
    promise.then((data) => {
      if (active) setNotes(data.listNotes);
    });
    return () => {
      active = false;
    };
  }, []);
  return notes.length ? (
    <Grid container direction="row" spacing={3}>
      {notes.map((note) => (
        <Grid item key={`note${note.id}`}>
          <Card sx={{ minWidth: 275 }}>
            <CardActionArea
              onClick={() => {
                navigate(note.id.toString());
              }}
            >
              <CardContent>
                <Typography
                  sx={{
                    fontSize: `.8rem`,
                    color: (theme) => theme.palette.text.secondary,
                  }}
                >
                  {new Date(note.createdAt).toLocaleDateString()}
                </Typography>
                <Typography variant="h5">{note.title}</Typography>
              </CardContent>
            </CardActionArea>
          </Card>
        </Grid>
      ))}
    </Grid>
  ) : (
    <div>you don't have any notes here, create one now!</div>
  );
};
