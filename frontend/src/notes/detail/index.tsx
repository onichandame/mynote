import { Button, Grid, TextField, Typography } from "@mui/material";
import { useForm } from "react-hook-form";
import { classValidatorResolver } from "@hookform/resolvers/class-validator";
import { FC, useCallback, useEffect, useState } from "react";
import { useParams } from "react-router-dom";

import { useService } from "../../backend";
import { Note, UpdateNoteInput } from "../../model";
import ReactMarkdown from "react-markdown";
import { Delete } from "./delete";
import { Loading } from "../../common";

const resolver = classValidatorResolver(UpdateNoteInput);

export const Detail: FC = () => {
  const [editing, setEditing] = useState(false);
  const [note, setNote] = useState<Note | null>(null);
  const params = useParams();
  const [cacheKey, setCacheKey] = useState(``);
  const svc = useService();
  const {
    register,
    handleSubmit,
    reset,
    getValues,
    formState: { errors, isSubmitting },
  } = useForm<UpdateNoteInput>({
    resolver,
  });
  const resetForm = useCallback(() => {
    reset({
      title: note?.title,
      content: note?.content,
      deletedAt: note?.deletedAt,
    });
  }, [reset, note]);
  const updateNote = useCallback(async () => {
    if (!params.id) return;
    const id = parseInt(params.id);
    if (!id) return;
    const note = (
      await svc.listNotes(
        { id: { eq: id }, deletedAt: { null: true } },
        { first: 1 }
      )
    ).edges[0]?.node;
    setNote(note || null);
  }, [svc, params]);
  useEffect(() => {
    updateNote();
  }, [updateNote]);
  useEffect(() => {
    const cache = window.localStorage.getItem(cacheKey);
    if (cache) reset(JSON.parse(cache));
    else if (note) resetForm();
  }, [note, cacheKey, resetForm]);
  useEffect(() => {
    if (params.id)
      setCacheKey([`update`, `cache`, `note`, params.id].join(`:`));
  }, [params]);
  return note ? (
    <form
      onSubmit={handleSubmit(async (vals) => {
        await svc.updateNotes(vals, { id: { eq: note.id } });
        cacheKey && window.localStorage.removeItem(cacheKey);
        await updateNote();
        setEditing(false);
      })}
    >
      <Grid container direction="column" spacing={4}>
        <Grid item>
          {editing && (
            <TextField
              label="title"
              size="small"
              fullWidth
              error={!!errors.title}
              helperText={errors.title?.message}
              InputLabelProps={{ shrink: true }}
              InputProps={{
                sx: { fontSize: (theme) => theme.typography.h4.fontSize },
              }}
              {...register(`title`, {
                onChange: () => {
                  cacheKey &&
                    window.localStorage.setItem(
                      cacheKey,
                      JSON.stringify(getValues())
                    );
                },
              })}
            />
          )}
          {!editing && (
            <Grid
              container
              direction="row"
              justifyContent="space-between"
              alignItems="center"
            >
              <Grid item>
                <Typography variant="h3">{note.title}</Typography>
              </Grid>
              <Grid item>
                <Grid container direction="row" spacing={2}>
                  <Grid item>
                    <Button
                      variant="contained"
                      onClick={() => {
                        setEditing(true);
                      }}
                    >
                      edit
                    </Button>
                  </Grid>
                  <Grid item>
                    <Delete note={note} />
                  </Grid>
                </Grid>
              </Grid>
            </Grid>
          )}
        </Grid>
        <Grid item>
          {editing && (
            <TextField
              label="Content"
              multiline
              fullWidth
              error={!!errors.content}
              helperText={errors.content?.message}
              {...register(`content`, {
                onChange: () => {
                  cacheKey &&
                    window.localStorage.setItem(
                      cacheKey,
                      JSON.stringify(getValues())
                    );
                },
              })}
            />
          )}
          {!editing && <ReactMarkdown>{note.content}</ReactMarkdown>}
        </Grid>
        {editing && (
          <Grid item>
            <Grid container direction="row" justifyContent="end" spacing={3}>
              <Grid item>
                <Button
                  variant="contained"
                  type="submit"
                  disabled={isSubmitting}
                >
                  save & exit
                </Button>
              </Grid>
              <Grid item>
                <Button
                  variant="contained"
                  color="warning"
                  onClick={() => {
                    cacheKey && window.localStorage.removeItem(cacheKey);
                    resetForm();
                  }}
                >
                  reset
                </Button>
              </Grid>
              <Grid item>
                <Button
                  variant="contained"
                  color="secondary"
                  onClick={() => {
                    setEditing(false);
                  }}
                  disabled={isSubmitting}
                >
                  cancel
                </Button>
              </Grid>
            </Grid>
          </Grid>
        )}
      </Grid>
    </form>
  ) : (
    <Loading />
  );
};
