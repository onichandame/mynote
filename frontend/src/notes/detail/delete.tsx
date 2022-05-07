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
import { Note } from "../../model";

export const Delete: FC<{ note: Note }> = ({ note }) => {
  const svc = useService();
  const navigate = useNavigate();
  const [deleting, setDeleting] = useState(false);
  return (
    <>
      <Button
        variant="contained"
        color="secondary"
        onClick={() => {
          setDeleting(true);
        }}
      >
        delete
      </Button>
      <Dialog
        open={deleting}
        onClose={() => {
          setDeleting(false);
        }}
      >
        <DialogTitle>Delete Note</DialogTitle>
        <DialogContent>
          <DialogContentText>Are you sure?</DialogContentText>
        </DialogContent>
        <DialogActions>
          <Button
            onClick={() => {
              setDeleting(false);
            }}
          >
            no
          </Button>
          <Button
            color="secondary"
            onClick={async () => {
              await svc.updateNotes(
                { deletedAt: new Date() },
                { id: { eq: note.id } }
              );
              setDeleting(false);
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
