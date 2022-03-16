import { FC, useEffect, useRef } from "react";
import { Link, useNavigate, useSearchParams } from "react-router-dom";
import { useFormik } from "formik";
import * as yup from "yup";
import { Button, Grid, TextField } from "@mui/material";
import { useSnackbar } from "notistack";

import { useService, useUser, useSessionSetter } from "../backend";

export const Login: FC = () => {
  const { closeSnackbar, enqueueSnackbar } = useSnackbar();
  const setSession = useSessionSetter();
  const user = useUser();
  const navigate = useNavigate();
  const [params] = useSearchParams();
  const svc = useService();
  const schema = yup
    .object()
    .shape({
      name: yup.string().defined().required().min(4).max(20),
      password: yup.string().required().min(6).max(40),
    })
    .required();
  const formik = useFormik({
    validationSchema: schema,
    initialValues: schema.getDefault(),
    onSubmit: async (vals, helpers) => {
      helpers.setSubmitting(true);
      const key = enqueueSnackbar(`logging in...`, {
        variant: `info`,
      });
      try {
        const session = await svc.login(vals.name, vals.password);
        enqueueSnackbar(`login successful`, { variant: `success` });
        setSession(session);
      } finally {
        closeSnackbar(key);
        helpers.setSubmitting(false);
      }
    },
  });
  const form = useRef<null | HTMLFormElement>(null);
  const submitButton = useRef<null | HTMLButtonElement>(null);
  useEffect(() => {
    if (params) {
      const name = params.get(`name`);
      if (name) formik.setFieldValue(`name`, name);
      const password = params.get(`password`);
      if (password) formik.setFieldValue(`password`, password);
    }
    if (submitButton) {
      formik.validateForm().then(() => {
        const autosubmit = params.get(`autoSubmit`);
        if (autosubmit) submitButton.current?.click();
      });
    }
  }, [submitButton, params]);
  useEffect(() => {
    if (user) navigate(decodeURIComponent(params.get(`redirect`) || `/`));
  }, [user]);
  return (
    <form ref={form} onSubmit={formik.handleSubmit}>
      <Grid container direction="column" spacing={2} alignItems="center">
        <Grid item>
          <TextField
            placeholder="Username"
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
            placeholder="Password"
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
            ref={submitButton}
            variant="contained"
            type="submit"
            disabled={formik.isSubmitting}
          >
            log in
          </Button>
        </Grid>
        <Grid item>
          <Link to="/signup">sign up</Link>
        </Grid>
      </Grid>
    </form>
  );
};
