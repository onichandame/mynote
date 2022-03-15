import { Button } from "@mui/material";
import { useSnackbar } from "notistack";
import { FC, useEffect } from "react";
import { useNavigate, useParams, useSearchParams } from "react-router-dom";

import { useService } from "../../backend";

export const Delete: FC = () => {
  const params = useParams();
  const [searchParams] = useSearchParams();
  const navigate = useNavigate();
  const svc = useService();
  const { closeSnackbar, enqueueSnackbar } = useSnackbar();
  useEffect(() => {
    const id = parseInt(params.id || ``);
    if (id && searchParams.get(`delete`)) {
      const key = enqueueSnackbar(`deleting note ${id}`, {
        variant: `info`,
      });
      svc
        .deleteNote(id)
        .then(() => {
          enqueueSnackbar(`delete note successful`, { variant: `success` });
          navigate(`../../`);
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
