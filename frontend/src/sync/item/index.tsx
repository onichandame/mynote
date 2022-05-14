import { classValidatorResolver } from "@hookform/resolvers/class-validator";
import { Cloud, CloudOff } from "@mui/icons-material";
import {
  Button,
  Checkbox,
  FormControl,
  FormControlLabel,
  Grid,
  MenuItem,
  TextField,
  Typography,
} from "@mui/material";
import { useSnackbar } from "notistack";
import { FC, useCallback, useEffect, useState } from "react";
import { Controller, useForm } from "react-hook-form";
import { useParams } from "react-router-dom";

import { useService } from "../../backend";
import { CenterRow, Loading } from "../../common";
import { Password, Peer, UpdatePeerInput } from "../../model";
import { Delete } from "./delete";

const resolver = classValidatorResolver(UpdatePeerInput);

export const Detail: FC = () => {
  const [peer, setPeer] = useState<Peer | null>(null);
  const [editing, setEditing] = useState(false);
  const { enqueueSnackbar } = useSnackbar();
  const [pwds, setPwds] = useState<Password[]>([]);
  const svc = useService();
  const {
    register,
    control,
    handleSubmit,
    reset,
    formState: { errors, isSubmitting },
  } = useForm<UpdatePeerInput>({ resolver });
  const params = useParams();
  const updatePeer = useCallback(async () => {
    const id = parseInt(params.id || ``);
    if (id) {
      await svc
        .listPeers({ deletedAt: { null: true }, id: { eq: id } })
        .then((conns) => {
          setPeer(conns.edges[0]?.node || null);
        });
    }
  }, [svc, params]);
  useEffect(() => {
    updatePeer();
  }, [updatePeer]);
  useEffect(() => {
    if (peer)
      reset({
        passwordId: peer.passwordId,
        title: peer.title,
        autoSync: peer.autoSync,
      });
  }, [peer]);
  useEffect(() => {
    svc
      .listPasswords()
      .then((conns) => conns.edges.map((v) => v.node))
      .then((pwds) => setPwds(pwds));
  }, [svc]);
  return peer ? (
    <form
      onSubmit={handleSubmit(async (vals) => {
        await svc.updatePeers(vals, { id: { eq: peer.id } });
        setEditing(false);
        await updatePeer();
      })}
    >
      <CenterRow>
        <Grid container direction="column" alignItems="stretch" spacing={2}>
          <Grid item>
            {editing && (
              <TextField
                label="Title"
                error={!!errors.title}
                helperText={errors.title?.message}
                {...register(`title`)}
              />
            )}
            {!editing && (
              <Typography variant="h3" textAlign="center">
                {peer.title}
              </Typography>
            )}
          </Grid>
          {editing && (
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
          )}
          <Grid item>
            {editing && (
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
            )}

            {!editing && (
              <Grid
                container
                direction="row"
                spacing={2}
                justifyContent="center"
              >
                <Grid item>
                  {peer.autoSync ? (
                    <Cloud color="info" />
                  ) : (
                    <CloudOff color="disabled" />
                  )}
                </Grid>
                <Grid item>
                  {peer.autoSync ? `Auto Sync on` : `Auto Sync off`}
                </Grid>
              </Grid>
            )}
          </Grid>
          <Grid item>
            <Grid
              container
              direction="row"
              justifyContent="space-between"
              spacing={2}
            >
              {editing && (
                <>
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
                        setEditing(false);
                      }}
                    >
                      cancel
                    </Button>
                  </Grid>
                </>
              )}
              {!editing && (
                <>
                  <Grid item>
                    <Button
                      variant="contained"
                      color="primary"
                      onClick={() => {
                        setEditing(true);
                      }}
                    >
                      edit
                    </Button>
                  </Grid>
                  <Grid item>
                    <Button
                      variant="contained"
                      color="success"
                      onClick={async () => {
                        await svc.syncFromPeer(peer.id);
                        enqueueSnackbar(`sync successful`, {
                          variant: `success`,
                        });
                      }}
                    >
                      sync
                    </Button>
                  </Grid>
                  <Grid item>
                    <Delete peer={peer} />
                  </Grid>
                </>
              )}
            </Grid>
          </Grid>
        </Grid>
      </CenterRow>
    </form>
  ) : (
    <Loading />
  );
};
