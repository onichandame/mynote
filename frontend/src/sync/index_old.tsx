import { Button, Grid, SpeedDialAction, TextField } from "@mui/material";
import { useForm } from "react-hook-form";
import { classValidatorResolver } from "@hookform/resolvers/class-validator";
import { FC, useState } from "react";
import { useNavigate } from "react-router-dom";

import { useService } from "../backend";
import { SyncFromRemoteInput } from "../model";
import { Actions } from "../actions";
import { Upload } from "@mui/icons-material";
import { PwSelectionDialog } from "./pwSelectionDialog";

const resolver = classValidatorResolver(SyncFromRemoteInput);

export const Sync: FC = () => {
  const [pwSelOpen, setPwSelOpen] = useState(false);
  const svc = useService();
  const navigate = useNavigate();
  const {
    handleSubmit,
    register,
    reset,
    formState: { errors, isSubmitting },
  } = useForm<SyncFromRemoteInput>({ resolver });
  return (
    <>
      <form
        onSubmit={handleSubmit(async (vals) => {
          await svc.syncFromRemote(vals);
          navigate(`../`);
        })}
      >
        <Grid container direction="column" spacing={2} alignItems="center">
          <Grid item>
            <TextField
              required
              label="Remote Address"
              InputLabelProps={{ shrink: true }}
              error={!!errors.url}
              helperText={errors.url?.message}
              {...register(`url`)}
            />
          </Grid>
          <Grid item>
            <TextField
              required
              label="Username/Email"
              InputLabelProps={{ shrink: true }}
              error={!!errors.identity}
              helperText={errors.identity?.message}
              {...register(`identity`)}
            />
          </Grid>
          <Grid item>
            <TextField
              required
              label="Password"
              InputLabelProps={{ shrink: true }}
              type="password"
              error={!!errors.password}
              helperText={errors.password?.message}
              {...register(`password`)}
            />
          </Grid>
          <Grid item>
            <Button type="submit" variant="contained" disabled={isSubmitting}>
              sync
            </Button>
          </Grid>
        </Grid>
      </form>
      <Actions>
        <SpeedDialAction
          icon={<Upload />}
          tooltipTitle="Select from passwords"
          onClick={() => {
            setPwSelOpen(true);
          }}
        />
      </Actions>
      <PwSelectionDialog
        open={pwSelOpen}
        onClose={() => {
          setPwSelOpen(false);
        }}
        setValues={(vals) => {
          reset(vals);
        }}
      />
    </>
  );
};
