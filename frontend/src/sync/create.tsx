import { classValidatorResolver } from "@hookform/resolvers/class-validator";
import {
  Button,
  Checkbox,
  FormControl,
  FormControlLabel,
  FormHelperText,
  Grid,
  MenuItem,
  TextField,
} from "@mui/material";
import { FC, useEffect, useState } from "react";
import { Controller, useForm } from "react-hook-form";
import { useNavigate } from "react-router-dom";
import { useService } from "../backend";

import { Form } from "../common";
import { CreatePeerInput, Password } from "../model";

const resolver = classValidatorResolver(CreatePeerInput);

export const Create: FC = () => {
  const [pwds, setPwds] = useState<Password[]>([]);
  const navigate = useNavigate();
  const svc = useService();
  const {
    control,
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<CreatePeerInput>({
    resolver,
    defaultValues: { autoSync: false }, // TODO: remove after backend implements custom input DTO
  });
  useEffect(() => {
    svc
      .listPasswords()
      .then((conn) => conn.edges.map((v) => v.node))
      .then((pwds) => setPwds(pwds));
  }, [svc]);
  return (
    <Form>
      <form
        onSubmit={handleSubmit(async (vals) => {
          await svc.createPeer(vals);
          navigate(`../`);
        })}
      >
        <Grid container direction="column" spacing={2} alignItems="center">
          <Grid item>
            <TextField
              required
              label="Name"
              error={!!errors.title}
              helperText={errors.title?.message}
              {...register(`title`)}
            />
          </Grid>
          <Grid item width="100%">
            <Controller<CreatePeerInput>
              control={control}
              name="passwordId"
              render={({ field }) => (
                <TextField
                  select
                  fullWidth
                  label="Credential"
                  defaultValue={0}
                  error={!!errors.passwordId}
                  helperText={errors.passwordId?.message}
                  onChange={(e) => {
                    field.onChange(e.target.value);
                  }}
                >
                  {pwds.map((pwd) => (
                    <MenuItem key={pwd.id} value={pwd.id}>
                      {pwd.title}
                    </MenuItem>
                  ))}
                </TextField>
              )}
            />
          </Grid>
          <Grid item>
            <FormControl error={!!errors.autoSync}>
              <FormControlLabel
                label="Auto Sync"
                control={
                  <Controller<CreatePeerInput>
                    control={control}
                    name="autoSync"
                    render={({ field }) => (
                      <Checkbox
                        {...field}
                        onChange={(e) => {
                          field.onChange(e.currentTarget.checked);
                        }}
                      />
                    )}
                  />
                }
              />
              <FormHelperText>{errors.autoSync?.message}</FormHelperText>
            </FormControl>
          </Grid>
          <Grid item>
            <Button variant="contained" type="submit">
              create
            </Button>
          </Grid>
        </Grid>
      </form>
    </Form>
  );
};
