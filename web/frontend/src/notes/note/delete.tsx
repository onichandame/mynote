import { Button } from "@mui/material";
import { useSnackbar } from "notistack";
import { FC, useEffect } from "react";
import { useNavigate, useParams, useSearchParams } from "react-router-dom";

import { useFetcher } from "../../request";

export const Delete: FC = () => {
  const params = useParams();
  const [searchParams] = useSearchParams();
  const navigate = useNavigate();
  const deleteNote = useFetcher<
    {},
    { id: number }
  >(`mutation deleteNote($id:Int!){
        deleteNote(id:$id)
    }`);
  const { closeSnackbar, enqueueSnackbar } = useSnackbar();
  useEffect(() => {
    const id = parseInt(params.id || ``);
    if (id && searchParams.get(`delete`)) {
      const [promise, cancel] = deleteNote({ id });
      const key = enqueueSnackbar(`deleting note ${id}`, {
        variant: `info`,
        action: <Button onClick={cancel}>cancel</Button>,
      });
      promise
        .then(() => {
          navigate(`../../`);
        })
        .catch((e) => {
          enqueueSnackbar(JSON.stringify(e), { variant: `error` });
        })
        .finally(() => {
          closeSnackbar(key);
        });
    }
  }, [searchParams]);
  return (
    <div>
      <p>Deleted note cannot be recovered. Are you sure?</p>
      <Button
        onClick={() => {
          navigate(`./?delete=true`);
        }}
      >
        delete
      </Button>
    </div>
  );
};
