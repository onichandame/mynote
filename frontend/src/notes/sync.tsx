import { Button, Grid, TextField } from "@mui/material";
import { useFormik } from "formik";
import { FC } from "react";
import { useNavigate } from "react-router-dom";
import * as yup from "yup";

import { useService } from "../backend";

export const Sync: FC = () => {
  const svc = useService();
  const navigate = useNavigate();
  const schema = yup
    .object()
    .shape({
      name: yup.string().defined().required().min(4).max(20),
      password: yup.string().required().min(6).max(40),
      url: yup.string().required(),
    })
    .required();
  const formik = useFormik({
    validationSchema: schema,
    initialValues: schema.getDefault(),
    onSubmit: async (vals, helpers) => {
      helpers.setSubmitting(true);
      try {
        await svc.syncNotes(vals.url, vals.name, vals.password, {
          notification: true,
        });
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
            label="Remote Address"
            name="url"
            value={formik.values.url}
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
            error={!!formik.errors.url}
            helperText={formik.errors.url}
          />
        </Grid>
        <Grid item>
          <TextField
            label="Username"
            name="name"
            value={formik.values.name}
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
            error={!!formik.errors.name}
            helperText={formik.errors.name}
          />
        </Grid>
        <Grid item>
          <TextField
            label="Password"
            name="password"
            type="password"
            value={formik.values.password}
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
            error={!!formik.errors.password}
            helperText={formik.errors.password}
          />
        </Grid>
        <Grid item>
          <Button
            type="submit"
            variant="contained"
            disabled={formik.isSubmitting}
          >
            sync
          </Button>
        </Grid>
      </Grid>
    </form>
  );
};
