import { classValidatorResolver } from "@hookform/resolvers/class-validator";
import { Button, Grid, TextField } from "@mui/material";
import { FC } from "react";
import { useForm } from "react-hook-form";
import { useNavigate } from "react-router-dom";

import { useService } from "../../backend";
import { Note, UpdateNoteInput } from "../../model";

const resolver = classValidatorResolver(UpdateNoteInput);

export const Update: FC<{ note: Note }> = ({ note }) => {
  const navigate = useNavigate();
  const svc = useService();
  const {
    handleSubmit,
    register,
    formState: { errors, isSubmitting },
  } = useForm<UpdateNoteInput>({
    resolver,
    defaultValues: { title: note.title, content: note.content },
  });
  return (
    <form
      onSubmit={handleSubmit(async (vals) => {
        await svc.updateNotes(vals, { id: { eq: note.id } });
        navigate(`../`);
      })}
    >
      <Grid container direction="column" spacing={3}>
        <Grid item>
          <TextField
            type="text"
            error={!!errors.title}
            helperText={errors.title?.message}
            {...register(`title`)}
          />
        </Grid>
        <Grid item>
          <TextField
            multiline
            type="text"
            error={!!errors.content}
            helperText={errors.content?.message}
            {...register(`content`)}
          />
        </Grid>
        <Grid item>
          <Button variant="contained" type="submit" disabled={isSubmitting}>
            update
          </Button>
        </Grid>
      </Grid>
    </form>
  );
};
