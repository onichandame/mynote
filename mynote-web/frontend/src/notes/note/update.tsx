import { Button, Grid, TextField } from "@mui/material";
import * as yup from "yup";
import { FC } from "react";
import { useFormik } from "formik";
import { useNavigate } from "react-router-dom";
import { useSnackbar } from "notistack";

import { Note } from "./type";
import { useFetcher } from "../../request";

export const Update: FC<{ note: Note }> = ({ note }) => {
  const navigate = useNavigate();
  const { closeSnackbar, enqueueSnackbar } = useSnackbar();
  const update = useFetcher<
    {},
    { id: number; title?: string; content?: string }
  >(`mutation updateNote($id:Int!,$title:String,$content:String){
      updateNote(id:$id,update:{title:$title,content:$content}){
          id
      }
  }`);
  const schema = yup
    .object()
    .shape({
      title: yup.string().default(note.title).notRequired(),
      content: yup.string().default(note.content).notRequired(),
    })
    .required();
  const formik = useFormik({
    validationSchema: schema,
    initialValues: schema.getDefault(),
    onSubmit: (vals, helpers) => {
      helpers.setSubmitting(true);
      const [promise, cancel] = update({ id: note.id, ...vals });
      const key = enqueueSnackbar(`updating note ${note.id}`, {
        variant: `info`,
        action: <Button onClick={cancel}>cancel</Button>,
      });
      promise
        .then(() => {
          navigate(`../`);
        })
        .catch((e) => {
          enqueueSnackbar(JSON.stringify(e), { variant: `error` });
        })
        .finally(() => {
          closeSnackbar(key);
        });
    },
  });
  return (
    <form onSubmit={formik.handleSubmit}>
      <Grid container direction="column" spacing={3}>
        <Grid item>
          <TextField
            type="text"
            name="title"
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
            value={formik.values.title}
            error={!!formik.errors.title}
            helperText={formik.errors.title}
          />
        </Grid>
        <Grid item>
          <TextField
            multiline
            type="text"
            name="content"
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
            value={formik.values.content}
            error={!!formik.errors.content}
            helperText={formik.errors.content}
          />
        </Grid>
        <Grid item>
          <Button
            variant="contained"
            type="submit"
            disabled={formik.isSubmitting}
          >
            update
          </Button>
        </Grid>
      </Grid>
    </form>
  );
};
