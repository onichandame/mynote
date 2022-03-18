import { Button, Grid, TextField } from "@mui/material";
import { useFormik } from "formik";
import { FC } from "react";
import * as yup from "yup";
import { useService } from "../backend";

export const Security: FC = () => {
  const svc = useService();
  const schema = yup
    .object()
    .shape({
      oldPassword: yup.string().required(),
      newPassword: yup.string().required(),
      newPassword2: yup
        .string()
        .required()
        .when(`newPassword`, (newPassword, schema) =>
          schema.oneOf([newPassword], `passwords do not match`)
        ),
    })
    .required();
  const formik = useFormik({
    validationSchema: schema,
    initialValues: schema.getDefault(),
    onSubmit: async (vals, helpers) => {
      helpers.setSubmitting(true);
      try {
        await svc.changePassword(vals.oldPassword, vals.newPassword, {
          notification: true,
        });
      } finally {
        helpers.setSubmitting(false);
      }
    },
  });
  return (
    <form onSubmit={formik.handleSubmit}>
      <Grid container direction="column" alignItems="center" spacing={2}>
        <Grid item>
          <TextField
            label="Old Password"
            name="oldPassword"
            type="password"
            value={formik.values.oldPassword}
            error={!!formik.errors.oldPassword}
            helperText={formik.errors.oldPassword}
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
          />
        </Grid>
        <Grid item>
          <TextField
            label="New Password"
            name="newPassword"
            type="password"
            value={formik.values.newPassword}
            error={!!formik.errors.newPassword}
            helperText={formik.errors.newPassword}
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
          />
        </Grid>
        <Grid item>
          <TextField
            label="Re-type New Password"
            name="newPassword2"
            type="password"
            value={formik.values.newPassword2}
            error={!!formik.errors.newPassword2}
            helperText={formik.errors.newPassword2}
            onChange={formik.handleChange}
            onBlur={formik.handleBlur}
          />
        </Grid>
        <Grid item>
          <Button
            type="submit"
            variant="contained"
            disabled={formik.isSubmitting}
          >
            submit
          </Button>
        </Grid>
      </Grid>
    </form>
  );
};
