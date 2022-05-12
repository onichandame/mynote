import { classValidatorResolver } from "@hookform/resolvers/class-validator";
import {
  Button,
  Divider,
  FormControl,
  FormControlLabel,
  Grid,
  Radio,
  RadioGroup,
  TextField,
} from "@mui/material";
import { FC, useCallback, useEffect, useState } from "react";
import { Controller, useForm } from "react-hook-form";
import { useNavigate, useParams } from "react-router-dom";

import { useService } from "../../../backend";
import { CenterRow } from "../../../common";
import { Password, UpdatePasswordInput } from "../../../model";
import { Icon } from "./icon";

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
    setValue,
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
      email: pwd?.email,
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
          navigate(`../`);
        })}
      >
        <Grid container direction="column" spacing={4} alignItems="stretch">
          <Grid item>
            <CenterRow>
              <Icon
                onConfirm={(icon) => {
                  setValue(`icon`, icon);
                }}
                alt={pwd.title}
                src={pwd.icon}
              />
            </CenterRow>
          </Grid>
          <Grid item>
            <CenterRow>
              <TextField
                label="title"
                error={!!errors.title}
                helperText={errors.title?.message}
                InputProps={{
                  sx: { fontSize: (theme) => theme.typography.h4.fontSize },
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
          <Grid item>
            <Grid
              container
              direction="row"
              justifyContent="center"
              alignItems="center"
            >
              <Grid item>
                <Grid container direction="row" spacing={1}>
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
            </Grid>
          </Grid>
          <Grid item>
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
          <Grid item>
            <FormControl error={!!errors.isLocal}>
              <Controller<UpdatePasswordInput>
                control={control}
                name="isLocal"
                render={({ field }) => (
                  <RadioGroup
                    row
                    {...field}
                    onChange={(e) => {
                      const value = e.currentTarget.value === `true`;
                      field.onChange(value);
                      cacheKey &&
                        window.localStorage.setItem(
                          cacheKey,
                          JSON.stringify(getValues())
                        );
                    }}
                  >
                    <FormControlLabel
                      label="Local Only"
                      value={true}
                      control={<Radio />}
                    />
                    <FormControlLabel
                      label="Syncable"
                      value={false}
                      control={<Radio />}
                    />
                  </RadioGroup>
                )}
              />
            </FormControl>
          </Grid>
          <Grid item>
            <Grid container direction="column" spacing={2} alignItems="stretch">
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
                    onClick={() => {
                      navigate(`../`);
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
