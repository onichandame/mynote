import { Button, Grid, TextField } from "@mui/material";
import { useFormik } from "formik";
import { useSnackbar } from "notistack";
import { FC, useEffect } from "react";
import { Link, useNavigate } from "react-router-dom";
import * as yup from "yup";

import { useUser } from "../auth";
import { useBackend } from "../backend";

export const Signup: FC = () => {
  const { closeSnackbar, enqueueSnackbar } = useSnackbar();
  const navigate = useNavigate();
  const user = useUser();
  const backend = useBackend();
  const schema = yup
    .object()
    .shape({
      name: yup.string().required().min(4).max(20),
      password: yup.string().required().min(6).max(40),
      email: yup.string().notRequired().email(),
      avatar: yup.string().notRequired().url(),
    })
    .required();
  const form = useFormik({
    validationSchema: schema,
    initialValues: schema.getDefault(),
    onSubmit: async (vals, helpers) => {
      helpers.setSubmitting(true);
      enqueueSnackbar(`signing up...`, {
        variant: `info`,
      });
      try {
        await backend.signup(vals);
        navigate(
          `/login?name=${vals.name}&password=${vals.password}&autoSubmit=true`
        );
      } finally {
        helpers.setSubmitting(false);
      }
    },
  });
  useEffect(() => {
    if (user) navigate(`/`);
  }, [user]);
  return (
    <form onSubmit={form.handleSubmit}>
      <Grid container direction="column" alignItems="center" spacing={2}>
        <Grid item>
          <TextField
            placeholder="Username"
            name="name"
            value={form.values.name}
            error={!!form.errors.name}
            helperText={form.errors.name}
            onChange={form.handleChange}
            onBlur={form.handleBlur}
          />
        </Grid>
        <Grid item>
          <TextField
            placeholder="Password"
            name="password"
            value={form.values.password}
            error={!!form.errors.password}
            helperText={form.errors.password}
            onChange={form.handleChange}
            onBlur={form.handleBlur}
          />
        </Grid>
        <Grid item>
          <TextField
            placeholder="Email"
            name="email"
            value={form.values.email}
            error={!!form.errors.email}
            helperText={form.errors.email}
            onChange={form.handleChange}
            onBlur={form.handleBlur}
          />
        </Grid>
        <Grid item>
          <TextField
            placeholder="Avatar"
            name="avatar"
            value={form.values.avatar}
            error={!!form.errors.avatar}
            helperText={form.errors.avatar}
            onChange={form.handleChange}
            onBlur={form.handleBlur}
          />
        </Grid>
        <Grid item>
          <Button
            variant="contained"
            type="submit"
            disabled={form.isSubmitting}
          >
            sign up
          </Button>
        </Grid>
        <Grid item>
          <Link to="/login">log in</Link>
        </Grid>
      </Grid>
    </form>
  );
};
