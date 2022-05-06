import { Button, Grid, TextField } from "@mui/material";
import { useForm } from "react-hook-form";
import { classValidatorResolver } from "@hookform/resolvers/class-validator";
import { useNavigate } from "react-router";
import { FC } from "react";

import { useService } from "../backend";
import { CreateNoteInput } from "../model";

const resolver = classValidatorResolver(CreateNoteInput);
export const Create: FC = () => {
  const navigate = useNavigate();
  const svc = useService();
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<CreateNoteInput>({ resolver });
  return (
    <form
      onSubmit={handleSubmit(async (vals) => {
        await svc.createNote(vals);
        navigate(`../`);
      })}
    >
      <Grid container direction="column" spacing={2} alignItems="stretch">
        <Grid item>
          <TextField
            fullWidth
            label="Title"
            error={!!errors.title}
            helperText={errors.title?.message}
            {...register(`title`)}
          />
        </Grid>
        <Grid item>
          <TextField
            multiline
            fullWidth
            label="Content"
            error={!!errors.content}
            helperText={errors.content?.message}
            {...register(`content`)}
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
