import { Button, Grid, TextField } from "@mui/material";
import { useFormik } from "formik";
import { FC, useEffect } from "react";
import { Link, useNavigate } from "react-router-dom";
import * as yup from "yup";

import { useService, useUser } from "../backend";

export const Signup: FC = () => {
  const navigate = useNavigate();
  const user = useUser();
  const svc = useService();
  const schema = yup
    .object()
    .shape({
      name: yup.string().required().min(4).max(20),
      password: yup.string().required().min(6).max(40),
      password2: yup
        .string()
        .required()
        .when(`password`, (password, schema) =>
          schema.oneOf([password], `passwords do not match`)
        ),
      email: yup.string().notRequired().email(),
      avatar: yup.string().notRequired().url(),
    })
    .required();
  const form = useFormik({
    validationSchema: schema,
    initialValues: schema.getDefault(),
    onSubmit: async (vals, helpers) => {
      helpers.setSubmitting(true);
      try {
        await svc.signup(vals, { notification: true });
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
            required
            label="Username"
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
            required
            label="Password"
            name="password"
            type="password"
            value={form.values.password}
            error={!!form.errors.password}
            helperText={form.errors.password}
            onChange={form.handleChange}
            onBlur={form.handleBlur}
          />
        </Grid>
        <Grid item>
          <TextField
            required
            label="Re-type Password"
            name="password2"
            type="password"
            value={form.values.password2}
            error={!!form.errors.password2}
            helperText={form.errors.password2}
            onChange={form.handleChange}
            onBlur={form.handleBlur}
          />
        </Grid>
        <Grid item>
          <TextField
            label="Email"
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
            label="Avatar Link"
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
