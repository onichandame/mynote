import { Button, Grid, TextField } from "@mui/material";
import { useNavigate } from "react-router";
import * as yup from "yup";
import { FC } from "react";
import { useFormik } from "formik";

import { useService } from "../backend";

export const Create: FC = () => {
  const navigate = useNavigate();
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
      try {
        await svc.createNote(vals, { notification: true });
        navigate(`../`);
      } finally {
        helpers.setSubmitting(false);
      }
    },
  });
  return (
    <form onSubmit={formik.handleSubmit}>
      <Grid container direction="column" spacing={2} alignItems="center">
        <Grid item>
          <TextField
            label="Title"
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
            label="Content"
            name="content"
            value={formik.values.content}
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
            error={!!formik.errors.content}
            helperText={formik.errors.content}
          />
        </Grid>
        <Grid item>
          <Button
            type="submit"
            variant="contained"
            disabled={formik.isSubmitting}
          >
            create
          </Button>
        </Grid>
      </Grid>
    </form>
  );
};
