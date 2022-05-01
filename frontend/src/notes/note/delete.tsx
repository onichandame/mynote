import { Button } from "@mui/material";
import { FC, useEffect } from "react";
import { useNavigate, useParams, useSearchParams } from "react-router-dom";

import { useService } from "../../backend";

export const Delete: FC = () => {
  const params = useParams();
  const [searchParams] = useSearchParams();
  const navigate = useNavigate();
  const svc = useService();
  useEffect(() => {
    const id = parseInt(params.id || ``);
    if (id && searchParams.get(`delete`)) {
      svc
        .updateNotes({ deletedAt: new Date() }, { id: { eq: id } })
        .then(() => {
          navigate(`../../`);
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
