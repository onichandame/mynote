import { NoteAdd } from "@mui/icons-material";
import { Grid, SpeedDialAction } from "@mui/material";
import { FC, useCallback, useEffect, useState } from "react";
import { Link, useNavigate } from "react-router-dom";

import { Actions } from "../../actions";
import { useBackend } from "../../backend";
import { Item } from "./item";
import { Note } from "./type";

export const List: FC = () => {
  const [notes, setNotes] = useState<Note[]>([]);
  const navigate = useNavigate();
  const backend = useBackend();
  const updateList = useCallback(() => {
    const [promise] = fetch({});
    promise.then((data) => {
      setNotes(data.listNotes);
    });
  }, [fetch]);
  useEffect(() => {
    updateList();
  }, [updateList]);
  return (
    <>
      {notes.length ? (
        <Grid
          container
          direction="row"
          spacing={3}
          justifyContent="start"
          flexGrow={1}
        >
          {notes.map((note) => (
            <Grid item key={`note${note.id}`}>
              <Item item={note} />
            </Grid>
          ))}
        </Grid>
      ) : (
        <div>
          you don't have any notes here, <Link to="create">create one now</Link>
          !
        </div>
      )}
      <Actions>
        <SpeedDialAction
          icon={<NoteAdd />}
          tooltipTitle="Create"
          onClick={() => {
            navigate(`create`);
          }}
        />
      </Actions>
    </>
  );
};
