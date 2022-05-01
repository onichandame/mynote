import { Button, Grid, TextField } from "@mui/material";
import { useForm } from "react-hook-form";
import { classValidatorResolver } from "@hookform/resolvers/class-validator";
import { FC } from "react";
import { useNavigate } from "react-router-dom";

import { useService } from "../backend";
import { SyncFromRemoteInput } from "../model";

const resolver = classValidatorResolver(SyncFromRemoteInput);

export const Sync: FC = () => {
  const svc = useService();
  const navigate = useNavigate();
  const {
    handleSubmit,
    register,
    formState: { errors, isSubmitting },
  } = useForm<SyncFromRemoteInput>({ resolver });
  return (
    <form
      onSubmit={handleSubmit(async (vals) => {
        await svc.syncFromRemote(vals);
        navigate(`../`);
      })}
    >
      <Grid container direction="column" spacing={2} alignItems="center">
        <Grid item>
          <TextField
            label="Remote Address"
            error={!!errors.url}
            helperText={errors.url?.message}
            {...register(`url`)}
          />
        </Grid>
        <Grid item>
          <TextField
            label="Username/Email"
            error={!!errors.identity}
            helperText={errors.identity?.message}
            {...register(`identity`)}
          />
        </Grid>
        <Grid item>
          <TextField
            label="Password"
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
  );
};
