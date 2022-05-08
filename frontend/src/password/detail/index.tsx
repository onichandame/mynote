import { classValidatorResolver } from "@hookform/resolvers/class-validator";
import { Cloud, CloudOff } from "@mui/icons-material";
import {
  Badge,
  Button,
  Divider,
  FormControl,
  FormControlLabel,
  Grid,
  Radio,
  RadioGroup,
  Skeleton,
  TextField,
  Tooltip,
  Typography,
} from "@mui/material";
import { FC, useCallback, useEffect, useState } from "react";
import { Controller, useForm } from "react-hook-form";
import { useParams } from "react-router-dom";

import { useService } from "../../backend";
import { Password, UpdatePasswordInput } from "../../model";
import { Delete } from "./delete";
import { TextDisplay } from "./textDisplay";

const resolver = classValidatorResolver(UpdatePasswordInput);

export const Detail: FC = () => {
  const [editing, setEditing] = useState(false);
  const svc = useService();
  const params = useParams();
  const [cacheKey, setCacheKey] = useState(``);
  const [pwd, setPwd] = useState<Password | null>(null);
  const updatePwd = useCallback(async () => {
    if (params.id) {
      const pwd = (
        await svc.listPasswords({
          id: { eq: parseInt(params.id) },
          deletedAt: { null: true },
        })
      ).edges[0].node;
      setPwd(pwd);
    }
  }, [svc, params]);
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
  const resetForm = useCallback(() => {
    reset({
      title: pwd?.title,
      password: pwd?.password,
      username: pwd?.username,
      email: pwd?.email,
      url: pwd?.url,
      isLocal: pwd?.isLocal,
    });
  }, [reset, pwd]);
  useEffect(() => {
    updatePwd();
  }, [updatePwd]);
  useEffect(() => {
    const cache = cacheKey && window.localStorage.getItem(cacheKey);
    if (cache) reset(JSON.parse(cache));
    else if (pwd) resetForm();
  }, [pwd, cacheKey, resetForm, reset]);
  useEffect(() => {
    if (params.id)
      setCacheKey([`cache`, `update`, `password`, params.id].join(`:`));
  }, [params]);
  return pwd ? (
    <form
      onSubmit={handleSubmit(async (vals) => {
        await svc.updatePasswords(vals, { id: { eq: pwd.id } });
        cacheKey && window.localStorage.removeItem(cacheKey);
        await updatePwd();
        setEditing(false);
      })}
    >
      <Grid container direction="column" spacing={4}>
        <Grid item>
          {editing && (
            <TextField
              label="title"
              error={!!errors.title}
              helperText={errors.title?.message}
              InputLabelProps={{ shrink: true }}
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
          )}
          {!editing && (
            <Badge
              badgeContent={
                <Tooltip
                  title={
                    pwd.isLocal
                      ? `Ignored in sync`
                      : `Will sync to other devices`
                  }
                >
                  {pwd.isLocal ? (
                    <CloudOff color="disabled" />
                  ) : (
                    <Cloud color="info" />
                  )}
                </Tooltip>
              }
              anchorOrigin={{ horizontal: `left`, vertical: `top` }}
            >
              <Typography variant="h3">{pwd.title}</Typography>
            </Badge>
          )}
        </Grid>
        <Grid item>
          <Grid
            container
            direction="row"
            justifyContent="space-between"
            alignItems="center"
          >
            <Grid item>
              <Grid
                container
                direction="column"
                border="1px solid"
                borderRadius={2}
                borderColor={(theme) => theme.palette.divider}
                spacing={1}
              >
                <Grid item>
                  {editing && (
                    <TextField
                      label="Username"
                      variant="filled"
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
                  )}
                  {!editing && (
                    <TextDisplay label="Username" value={pwd.username} />
                  )}
                </Grid>
                <Divider />
                <Grid item>
                  {editing && (
                    <TextField
                      label="Password"
                      error={!!errors.password}
                      helperText={errors.password?.message}
                      variant="filled"
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
                  )}
                  {!editing && (
                    <TextDisplay
                      label="Password"
                      value={pwd.password}
                      type="password"
                    />
                  )}
                </Grid>
              </Grid>
            </Grid>
            <Grid item>
              <Grid
                container
                direction="column"
                spacing={2}
                alignItems="stretch"
              >
                {editing && (
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
                          setEditing(false);
                        }}
                        disabled={isSubmitting}
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
                        fullWidth
                        variant="contained"
                        onClick={(e) => {
                          e.stopPropagation();
                          setEditing(true);
                        }}
                      >
                        edit
                      </Button>
                    </Grid>
                    <Grid item>
                      <Delete pwd={pwd} />
                    </Grid>
                  </>
                )}
              </Grid>
            </Grid>
          </Grid>
        </Grid>
        <Grid item>
          {editing && (
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
          )}
          {!editing && <TextDisplay label="Website" value={pwd.url} />}
        </Grid>
        {editing && (
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
        )}
      </Grid>
    </form>
  ) : (
    <div>
      {
        // TODO: refine loading animation
        <Skeleton
          sx={{ bgcolor: (theme) => theme.palette.grey[300] }}
          variant="rectangular"
        />
      }
    </div>
  );
};
