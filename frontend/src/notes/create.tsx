import { Button, Grid, TextField } from "@mui/material";
import { useNavigate } from "react-router";
import * as yup from "yup";
import { FC } from "react";
import { useFormik } from "formik";
import { useSnackbar } from "notistack";

import { useService } from "../backend";

export const Create: FC = () => {
  const navigate = useNavigate();
  const { closeSnackbar, enqueueSnackbar } = useSnackbar();
  const svc = useService();
  const schema = yup
    .object()
    .shape({
      title: yup.string().required(),
      content: yup.string().required(),
    })
    .required();
  const formik = useFormik({
    validationSchema: schema,
    initialValues: schema.getDefault(),
    onSubmit: async (vals, helpers) => {
      helpers.setSubmitting(true);
      const key = enqueueSnackbar(`creating note`, {
        variant: `info`,
      });
      try {
        const note = await svc.createNote(vals.title, vals.content);
        enqueueSnackbar(`new note created`, { variant: `success` });
        navigate(`../`);
      } finally {
        closeSnackbar(key);
        helpers.setSubmitting(false);
      }
    },
  });
  return (
    <form onSubmit={formik.handleSubmit}>
      <Grid container direction="column" spacing={2} alignItems="center">
        <Grid item>
          <TextField
            placeholder="Title"
            name="title"
            value={formik.values.title}
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
            error={!!formik.errors.title}
            helperText={formik.errors.title}
          />
        </Grid>
        <Grid item>
          <TextField
            multiline
            placeholder="Content"
            name="content"
            value={formik.values.content}
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
            error={!!formik.errors.content}
            helperText={formik.errors.content}
          />
        </Grid>
        <Grid item>
          <Button type="submit" variant="contained">
            create
          </Button>
        </Grid>
      </Grid>
    </form>
  );
};
