import { classValidatorResolver } from "@hookform/resolvers/class-validator";
import { Button, Grid, TextField } from "@mui/material";
import { FC } from "react";
import { useForm } from "react-hook-form";
import { useNavigate } from "react-router-dom";

import { useService } from "../backend";
import { CreatePasswordInput } from "../model";

const resolver = classValidatorResolver(CreatePasswordInput);

export const Create: FC = () => {
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<CreatePasswordInput>({ resolver });
  const navigate = useNavigate();
  const svc = useService();
  return (
    <form
      onSubmit={handleSubmit(async (vals) => {
        await svc.createPassword(vals);
        navigate(`../`);
      })}
    >
      <Grid container direction="column" spacing={2} alignItems="center">
        <Grid item>
          <TextField
            required
            label="Title"
            error={!!errors.title}
            helperText={errors.title?.message}
            {...register(`title`)}
          />
        </Grid>
        <Grid item>
          <TextField
            required
            label="Password"
            error={!!errors.password}
            helperText={errors.password?.message}
            {...register(`password`)}
          />
        </Grid>
        <Grid item>
          <TextField
            label="Username"
            error={!!errors.username}
            helperText={errors.username?.message}
            {...register(`username`)}
          />
        </Grid>
        <Grid item>
          <TextField
            label="Email"
            error={!!errors.email}
            helperText={errors.email?.message}
            {...register(`email`)}
          />
        </Grid>
        <Grid item>
          <TextField
            label="URL"
            error={!!errors.url}
            helperText={errors.url?.message}
            {...register(`url`)}
          />
        </Grid>
        <Grid item>
          <Button type="submit" variant="contained" disabled={isSubmitting}>
            create
          </Button>
        </Grid>
      </Grid>
    </form>
  );
};
