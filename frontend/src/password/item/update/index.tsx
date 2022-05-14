import { classValidatorResolver } from "@hookform/resolvers/class-validator";
import {
  Button,
  Checkbox,
  Divider,
  FormControl,
  FormControlLabel,
  Grid,
  TextField,
} from "@mui/material";
import { FC, useCallback, useEffect, useState } from "react";
import { Controller, useForm } from "react-hook-form";
import { useNavigate, useParams } from "react-router-dom";

import { useService } from "../../../backend";
import { CenterRow, IconField } from "../../../common";
import { Password, UpdatePasswordInput } from "../../../model";

const resolver = classValidatorResolver(UpdatePasswordInput);

export const Update: FC<{ pwd: Password }> = ({ pwd }) => {
  const svc = useService();
  const params = useParams();
  const [cacheKey, setCacheKey] = useState(``);
  const {
    control,
    register,
    handleSubmit,
    getValues,
    formState: { errors, isSubmitting },
    reset,
  } = useForm<UpdatePasswordInput>({
    resolver,
  });
  const navigate = useNavigate();
  const resetForm = useCallback(() => {
    reset({
      title: pwd?.title,
      password: pwd?.password,
      username: pwd?.username,
      url: pwd?.url,
      icon: pwd?.icon,
      isLocal: pwd?.isLocal,
    });
  }, [reset, pwd]);
  useEffect(() => {
    const cache = cacheKey && window.localStorage.getItem(cacheKey);
    if (cache) reset(JSON.parse(cache));
    else if (pwd) resetForm();
  }, [pwd, cacheKey, resetForm, reset]);
  useEffect(() => {
    if (params.id)
      setCacheKey([`cache`, `update`, `password`, params.id].join(`:`));
  }, [params]);
  return (
    <CenterRow>
      <form
        onSubmit={handleSubmit(async (vals) => {
          await svc.updatePasswords(vals, { id: { eq: pwd.id } });
          cacheKey && window.localStorage.removeItem(cacheKey);
          navigate(-1);
        })}
      >
        <Grid container direction="column" spacing={4} alignItems="stretch">
          <Grid item>
            <CenterRow>
              <Grid container direction="row" spacing={2} alignItems="center">
                <Grid item>
                  <Controller<UpdatePasswordInput>
                    control={control}
                    name="icon"
                    render={({ field }) => (
                      <IconField
                        onConfirm={(val) => {
                          field.onChange(val);
                          cacheKey &&
                            window.localStorage.setItem(
                              cacheKey,
                              JSON.stringify(getValues())
                            );
                        }}
                        value={
                          typeof field.value === `string` ? field.value : null
                        }
                      />
                    )}
                  />
                </Grid>
                <Grid item>
                  <CenterRow>
                    <TextField
                      label="title"
                      error={!!errors.title}
                      helperText={errors.title?.message}
                      InputProps={{
                        sx: {
                          fontSize: (theme) => theme.typography.h5.fontSize,
                        },
                      }}
                      {...register(`title`, {
                        onChange: () => {
                          cacheKey &&
                            window.localStorage.setItem(
                              cacheKey,
                              JSON.stringify(getValues())
                            );
                        },
                      })}
                    />
                  </CenterRow>
                </Grid>
              </Grid>
            </CenterRow>
          </Grid>
          <Grid item>
            <Grid container direction="row" spacing={3}>
              <Grid item>
                <Grid
                  container
                  direction="column"
                  alignItems="center"
                  spacing={1}
                >
                  <Grid item>
                    <TextField
                      label="Username"
                      error={!!errors.username}
                      InputLabelProps={{ shrink: true }}
                      helperText={errors.username?.message}
                      {...register(`username`, {
                        onChange: () => {
                          cacheKey &&
                            window.localStorage.setItem(
                              cacheKey,
                              JSON.stringify(getValues())
                            );
                        },
                      })}
                    />
                  </Grid>
                  <Divider />
                  <Grid item>
                    <TextField
                      required
                      label="Password"
                      error={!!errors.password}
                      helperText={errors.password?.message}
                      {...register(`password`, {
                        onChange: () => {
                          cacheKey &&
                            window.localStorage.setItem(
                              cacheKey,
                              JSON.stringify(getValues())
                            );
                        },
                      })}
                    />
                  </Grid>
                </Grid>
              </Grid>
              <Grid item>
                <Grid container direction="column">
                  <Grid item>
                    <TextField
                      label="Website"
                      InputLabelProps={{ shrink: true }}
                      error={!!errors.url}
                      helperText={errors.url?.message}
                      {...register(`url`, {
                        onChange: () => {
                          cacheKey &&
                            window.localStorage.setItem(
                              cacheKey,
                              JSON.stringify(getValues())
                            );
                        },
                      })}
                    />
                  </Grid>
                </Grid>
              </Grid>
            </Grid>
          </Grid>
          <Grid item>
            <CenterRow>
              <FormControl error={!!errors.isLocal}>
                <FormControlLabel
                  label="Sync to other machines"
                  control={
                    <Controller<UpdatePasswordInput>
                      control={control}
                      name="isLocal"
                      render={({ field }) => (
                        <Checkbox
                          defaultChecked={!pwd.isLocal}
                          onChange={(e) => {
                            field.onChange(!e.currentTarget.checked);
                          }}
                        />
                      )}
                    />
                  }
                />
              </FormControl>
            </CenterRow>
          </Grid>
          <Grid item>
            <Grid container direction="row" spacing={2} justifyContent="center">
              <>
                <Grid item>
                  <Button
                    fullWidth
                    type="submit"
                    variant="contained"
                    disabled={isSubmitting}
                  >
                    save & exit
                  </Button>
                </Grid>
                <Grid item>
                  <Button
                    fullWidth
                    color="warning"
                    variant="contained"
                    onClick={() => {
                      cacheKey && window.localStorage.removeItem(cacheKey);
                      resetForm();
                    }}
                  >
                    reset
                  </Button>
                </Grid>
                <Grid item>
                  <Button
                    fullWidth
                    color="secondary"
                    variant="contained"
                    onClick={(e) => {
                      e.stopPropagation();
                      navigate(-1);
                    }}
                    disabled={isSubmitting}
                  >
                    cancel
                  </Button>
                </Grid>
              </>
            </Grid>
          </Grid>
        </Grid>
      </form>
    </CenterRow>
  );
};
