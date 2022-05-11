import {
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogContentText,
  DialogTitle,
} from "@mui/material";
import { FC, useCallback, useState } from "react";
import { useNavigate } from "react-router-dom";

import { useService } from "../../backend";
import { Peer } from "../../model";

export const Delete: FC<{ peer: Peer }> = ({ peer }) => {
  const [open, setOpen] = useState(false);
  const close = useCallback(() => {
    setOpen(false);
  }, [setOpen]);
  const svc = useService();
  const navigate = useNavigate();
  return (
    <>
      <Button
        variant="contained"
        color="secondary"
        onClick={() => {
          setOpen(true);
        }}
      >
        delete
      </Button>
      <Dialog open={open} onClose={close}>
        <DialogTitle>Delete Peer</DialogTitle>
        <DialogContent>
          <DialogContentText>Are you sure?</DialogContentText>
        </DialogContent>
        <DialogActions>
          <Button onClick={close}>no</Button>
          <Button
            color="secondary"
            onClick={() => {
              svc
                .updatePeers({ deletedAt: new Date() }, { id: { eq: peer.id } })
                .finally(() => {
                  navigate(`../`);
                });
            }}
          >
            yes
          </Button>
        </DialogActions>
      </Dialog>
    </>
  );
};
