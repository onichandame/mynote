import {
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogContentText,
  DialogTitle,
} from "@mui/material";
import { FC, useState } from "react";
import { useNavigate } from "react-router-dom";

import { useService } from "../../backend";
import { Password } from "../../model";

export const Delete: FC<{ pwd: Password }> = ({ pwd }) => {
  const [deleting, setDeleting] = useState(false);
  const svc = useService();
  const navigate = useNavigate();
  return (
    <>
      <Button
        fullWidth
        color="secondary"
        variant="contained"
        onClick={() => {
          setDeleting(true);
        }}
      >
        delete
      </Button>
      <Dialog open={deleting} onClose={() => setDeleting(false)}>
        <DialogTitle>Delete password</DialogTitle>
        <DialogContent>
          <DialogContentText>Are you sure?</DialogContentText>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setDeleting(false)}>no</Button>
          <Button
            color="secondary"
            onClick={async () => {
              setDeleting(false);
              await svc.updatePasswords(
                { deletedAt: new Date() },
                { id: { eq: pwd.id } }
              );
              navigate(`../`);
            }}
          >
            yes
          </Button>
        </DialogActions>
      </Dialog>
    </>
  );
};
