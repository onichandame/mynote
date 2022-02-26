import { useSnackbar } from "notistack";
import { SpeedDialAction } from "@mui/material";
import { Delete, NoteAdd, NoteAlt } from "@mui/icons-material";
import { FC, useEffect, useState } from "react";
import { useNavigate, Routes, Route, useParams } from "react-router-dom";

import { useFetcher } from "../../request";
import { Note as NoteType } from "./type";
import { Detail } from "./detail";
import { Delete as DeleteNote } from "./delete";
import { Actions } from "../../actions";
import { Update } from "./update";

export const Note: FC = () => {
  const { enqueueSnackbar } = useSnackbar();
  const [note, setNote] = useState<NoteType | null>(null);
  const navigate = useNavigate();
  const params = useParams();
  const fetch = useFetcher<
    { getNote: NoteType },
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
  return note ? (
    <>
      <Routes>
        <Route path="/" element={<Detail note={note} />} />
        <Route path="update" element={<Update note={note} />} />
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
