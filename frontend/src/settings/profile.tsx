import { Button, Grid, TextField } from "@mui/material";
import { useFormik } from "formik";
import * as yup from "yup";
import { FC } from "react";

import { useService, useUser, useUserSetter } from "../backend";

export const Profile: FC = () => {
  const user = useUser();
  const setUser = useUserSetter();
  const svc = useService();
  const schema = yup
    .object()
    .required()
    .shape({
      name: yup.string().default(user?.name).notRequired(),
      email: yup
        .string()
        .email()
        .default(user?.email || undefined)
        .notRequired(),
      avatar: yup
        .string()
        .url()
        .default(user?.avatar || undefined)
        .notRequired(),
    });
  const formik = useFormik({
    validationSchema: schema,
    initialValues: schema.getDefault(),
    onSubmit: async (vals, helpers) => {
      helpers.setSubmitting(true);
      try {
        const user = await svc.updateSelf(vals, { notification: true });
        setUser(user);
      } finally {
      }
    },
  });
  return (
    <form onSubmit={formik.handleSubmit}>
      <Grid container direction="column" alignItems="center" spacing={2}>
        <Grid item>
          <TextField
            label="Username"
            name="name"
            value={formik.values.name}
            error={!!formik.errors.name}
            helperText={formik.errors.name}
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
          />
        </Grid>
        <Grid item>
          <TextField
            label="Email"
            name="email"
            value={formik.values.email}
            error={!!formik.errors.email}
            helperText={formik.errors.email}
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
          />
        </Grid>
        <Grid item>
          <TextField
            label="Avatar Link"
            name="avatar"
            value={formik.values.avatar}
            error={!!formik.errors.avatar}
            helperText={formik.errors.avatar}
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
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
        <Grid item>
          <Button
            onClick={formik.handleReset}
            type="button"
            variant="contained"
            color="warning"
          >
            reset
          </Button>
        </Grid>
      </Grid>
    </form>
  );
};
