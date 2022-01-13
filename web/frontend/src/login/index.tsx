import { FC, useEffect, useRef } from "react";
import { useNavigate, useSearchParams } from "react-router-dom";
import { useFormik } from "formik";
import * as yup from "yup";
import { Button, Grid, TextField } from "@mui/material";

import { useSessionSetter } from "../auth";
import { useFetcher } from "../request";
import { useSnackbar } from "notistack";

export const Login: FC = () => {
  const { closeSnackbar, enqueueSnackbar } = useSnackbar();
  const setSession = useSessionSetter();
  const navigate = useNavigate();
  const [params] = useSearchParams();
  const login = useFetcher<
    { login: string },
    { name: string; password: string }
  >(
    `mutation login($name:String!,$password:String!){
      login(input:{name:$name,password:$password})
    }`
  );
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
      const [promise, cancel] = login(vals);
      const key = enqueueSnackbar(`logging in`, {
        variant: `info`,
        action: <Button onClick={() => cancel()}>cancel</Button>,
      });
      promise
        .then((data) => {
          setSession(data.login);
          navigate(`/`);
        })
        .catch((e) => {
          enqueueSnackbar(JSON.stringify(e), { variant: `error` });
        })
        .finally(() => {
          closeSnackbar(key);
          helpers.setSubmitting(false);
        });
    },
  });
  const form = useRef<null | HTMLFormElement>(null);
  useEffect(() => {
    if (params) {
      const name = params.get(`name`);
      if (name) formik.setFieldValue(`name`, name);
      const password = params.get(`password`);
      if (password) formik.setFieldValue(`password`, password);
    }
    if (form) {
      const autosubmit = params.get(`autoSubmit`);
      if (autosubmit) form.current?.dispatchEvent(new Event(`submit`));
    }
  }, [form, params]);
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
            value={formik.values.password}
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
            error={!!formik.errors.password}
            helperText={formik.errors.password}
          />
        </Grid>
        <Grid item>
          <Button
            variant="contained"
            type="submit"
            disabled={formik.isSubmitting}
          >
            log in
          </Button>
        </Grid>
      </Grid>
    </form>
  );
};
