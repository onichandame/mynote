import { SpeedDialAction } from "@mui/material";
import { Delete, NoteAdd, NoteAlt } from "@mui/icons-material";
import { FC, useEffect, useState } from "react";
import { useNavigate, Routes, Route, useParams } from "react-router-dom";

import { Detail } from "./detail";
import { Delete as DeleteNote } from "./delete";
import { Actions } from "../../actions";
import { Update } from "./update";
import { useService } from "../../backend";
import { Note as NoteModel } from "../../model";
import { Form } from "../../common";

export const Note: FC = () => {
  const [note, setNote] = useState<NoteModel | null>(null);
  const navigate = useNavigate();
  const params = useParams();
  const svc = useService();
  useEffect(() => {
    if (!params.id) return;
    const id = parseInt(params.id);
    if (!id) return;
    let active = true;
    svc.getNote(id).then((note) => {
      if (active) setNote(note);
    });
    return () => {
      active = false;
    };
  }, [params]);
  return note ? (
    <>
      <Routes>
        <Route path="/" element={<Detail note={note} />} />
        <Route
          path="update"
          element={
            <Form>
              <Update note={note} />
            </Form>
          }
        />
        <Route path="delete" element={<DeleteNote />} />
      </Routes>
      <Actions>
        <SpeedDialAction
          icon={<NoteAlt />}
          tooltipTitle="update"
          onClick={() => {
            navigate("update");
          }}
        />
        <SpeedDialAction
          icon={<NoteAdd />}
          tooltipTitle="create"
          onClick={() => {
            navigate("../create");
          }}
        />
        <SpeedDialAction
          icon={<Delete />}
          tooltipTitle="delete"
          onClick={() => {
            navigate("delete");
          }}
        />
      </Actions>
    </>
  ) : (
    <div>loading or failed to load note</div>
  );
};
