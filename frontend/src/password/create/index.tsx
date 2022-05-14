import { classValidatorResolver } from "@hookform/resolvers/class-validator";
import {
  Button,
  Checkbox,
  FormControl,
  FormControlLabel,
  FormHelperText,
  Grid,
  TextField,
} from "@mui/material";
import { FC } from "react";
import { Controller, useForm } from "react-hook-form";
import { useNavigate } from "react-router-dom";

import { useService } from "../../backend";
import { IconField } from "../../common";
import { CreatePasswordInput } from "../../model";

const resolver = classValidatorResolver(CreatePasswordInput);

export const Create: FC = () => {
  const {
    register,
    handleSubmit,
    control,
    formState: { errors, isSubmitting },
  } = useForm<CreatePasswordInput>({
    resolver,
    defaultValues: { isLocal: false },
  });
  const navigate = useNavigate();
  const svc = useService();
  return (
    <form
      onSubmit={handleSubmit(async (vals) => {
        await svc.createPassword(vals);
        navigate(-1);
      })}
    >
      <Grid container direction="column" spacing={2} alignItems="center">
        <Grid item>
          <Grid container direction="row" spacing={2} justifyContent="center">
            <Grid item>
              <Controller<CreatePasswordInput>
                control={control}
                name="icon"
                render={({ field }) => (
                  <IconField
                    value={typeof field.value === `string` ? field.value : null}
                    onConfirm={(val) => field.onChange(val)}
                  />
                )}
              />
            </Grid>
            <Grid item>
              <TextField
                required
                label="Title"
                error={!!errors.title}
                helperText={errors.title?.message}
                {...register(`title`)}
              />
            </Grid>
          </Grid>
        </Grid>
        <Grid item>
          <TextField
            label="Username"
            error={!!errors.username}
            helperText={errors.username?.message}
            {...register(`username`)}
          />
        </Grid>
        <Grid item>
          <TextField
            required
            label="Password"
            error={!!errors.password}
            helperText={errors.password?.message}
            {...register(`password`)}
          />
        </Grid>
        <Grid item>
          <TextField
            label="Website"
            error={!!errors.url}
            helperText={errors.url?.message}
            {...register(`url`)}
          />
        </Grid>
        <Grid item>
          <FormControl error={!!errors.isLocal}>
            <Controller<CreatePasswordInput>
              control={control}
              name="isLocal"
              render={({ field }) => (
                <FormControl error={!!errors.isLocal}>
                  <FormControlLabel
                    label="Sync to other peers"
                    control={
                      <Controller<CreatePasswordInput>
                        control={control}
                        name="isLocal"
                        render={({ field }) => (
                          <Checkbox
                            defaultChecked={true}
                            onChange={(e) => {
                              field.onChange(!e.currentTarget.checked);
                            }}
                          />
                        )}
                      />
                    }
                  />
                </FormControl>
              )}
            />
            <FormHelperText>{errors.isLocal?.message}</FormHelperText>
          </FormControl>
        </Grid>
        <Grid item>
          <Button type="submit" variant="contained" disabled={isSubmitting}>
            create
          </Button>
        </Grid>
      </Grid>
    </form>
  );
};
