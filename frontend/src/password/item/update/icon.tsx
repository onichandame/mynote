import {
  Avatar,
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
  Grid,
  IconButton,
  TextField,
} from "@mui/material";
import { ComponentProps, FC, useEffect, useState } from "react";
import { Controller, useForm } from "react-hook-form";
import { ValidateIfNotEmpty } from "../../../common";

type Props = ComponentProps<typeof Avatar> & {
  onConfirm: (url: string | null) => void;
};

export const Icon: FC<Props> = ({ onConfirm, ...props }) => {
  const [open, setOpen] = useState(false);
  const [icon, setIcon] = useState<null | string>(props.src || null);
  return (
    <>
      <IconButton
        onClick={() => {
          setOpen(true);
        }}
      >
        {
          // TODO: 展示confirm后的icon
        }
        <Avatar {...props} src={icon || undefined} />
      </IconButton>
      <Dialog
        open={open}
        onClose={() => {
          setOpen(false);
        }}
      >
        <DialogTitle>Change Icon</DialogTitle>
        <DialogContent>
          <Grid container direction="column" alignItems="center" spacing={2}>
            <Grid item>
              <Avatar {...props} src={icon || undefined} />
            </Grid>
            <Grid item>
              <TextField
                label="Icon Url"
                variant="filled"
                defaultValue={icon}
                onChange={(e) => {
                  setIcon(e.target.value || null);
                }}
              />
            </Grid>
          </Grid>
        </DialogContent>
        <DialogActions>
          <Button
            onClick={() => {
              setOpen(false);
            }}
            color="secondary"
          >
            cancel
          </Button>
          <Button
            onClick={() => {
              onConfirm(icon);
              setOpen(false);
            }}
          >
            confirm
          </Button>
        </DialogActions>
      </Dialog>
    </>
  );
};
