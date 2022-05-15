import { classValidatorResolver } from "@hookform/resolvers/class-validator";
import {
  Button,
  Checkbox,
  FormControl,
  FormControlLabel,
  Grid,
  MenuItem,
  TextField,
} from "@mui/material";
import { FC, useEffect, useState } from "react";
import { Controller, useForm } from "react-hook-form";
import { useNavigate } from "react-router-dom";

import { useService } from "../../backend";
import { CenterRow, IconField } from "../../common";
import { Password, Peer, UpdatePeerInput } from "../../model";

const resolver = classValidatorResolver(UpdatePeerInput);

export const Update: FC<{ peer: Peer }> = ({ peer }) => {
  const [pwds, setPwds] = useState<Password[]>([]);
  const svc = useService();
  const navigate = useNavigate();
  const {
    register,
    control,
    handleSubmit,
    reset,
    formState: { errors, isSubmitting },
  } = useForm<UpdatePeerInput>({ resolver });
  useEffect(() => {
    if (peer)
      reset({
        passwordId: peer.passwordId,
        icon: peer.icon,
        title: peer.title,
        autoSync: peer.autoSync,
      });
  }, [peer]);
  useEffect(() => {
    svc
      .listPasswords({ deletedAt: { null: true } })
      .then((conns) => conns.edges.map((v) => v.node))
      .then((pwds) => setPwds(pwds));
  }, [svc]);
  return (
    <form
      onSubmit={handleSubmit(async (vals) => {
        await svc.updatePeers(vals, { id: { eq: peer.id } });
        navigate(-1);
      })}
    >
      <CenterRow>
        <Grid container direction="column" alignItems="stretch" spacing={2}>
          <Grid item>
            <Grid
              container
              direction="row"
              alignItems="center"
              justifyContent="center"
            >
              <Grid item>
                <Controller<UpdatePeerInput>
                  control={control}
                  name="icon"
                  render={({ field }) => (
                    <IconField
                      value={
                        typeof field.value === `string` ? field.value : null
                      }
                      onConfirm={(val) => field.onChange(val)}
                    />
                  )}
                />
              </Grid>
              <Grid item>
                <TextField
                  label="Title"
                  error={!!errors.title}
                  helperText={errors.title?.message}
                  {...register(`title`)}
                />
              </Grid>
            </Grid>
          </Grid>
          <Grid item>
            <Controller<UpdatePeerInput>
              control={control}
              name="passwordId"
              render={({ field }) => (
                <TextField
                  select
                  fullWidth
                  label="Credential"
                  defaultValue={peer.passwordId}
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
            <Grid container direction="row" justifyContent="center">
              <Grid item>
                <FormControl error={!!errors.autoSync}>
                  <FormControlLabel
                    label="Auto Sync"
                    control={
                      <Controller<UpdatePeerInput>
                        control={control}
                        name="autoSync"
                        render={({ field }) => (
                          <Checkbox
                            {...field}
                            defaultChecked={!!peer.autoSync}
                            onChange={(e) => {
                              field.onChange(e.currentTarget.checked);
                            }}
                          />
                        )}
                      />
                    }
                  />
                </FormControl>
              </Grid>
            </Grid>
          </Grid>
          <Grid item>
            <Grid
              container
              direction="row"
              justifyContent="space-between"
              spacing={2}
            >
              <Grid item>
                <Button color="primary" variant="contained" type="submit">
                  Save & Exit
                </Button>
              </Grid>
              <Grid item>
                <Button
                  variant="contained"
                  color="secondary"
                  onClick={() => {
                    navigate(-1);
                  }}
                >
                  cancel
                </Button>
              </Grid>
            </Grid>
          </Grid>
        </Grid>
      </CenterRow>
    </form>
  );
};
