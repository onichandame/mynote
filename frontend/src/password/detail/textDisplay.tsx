import { InputAdornment, TextField } from "@mui/material";
import { ComponentProps, FC } from "react";

import { Clipboard } from "../../common";

/** Readonly text with `copy-to-clipboard` button */
export const TextDisplay: FC<
  Pick<ComponentProps<typeof TextField>, "label" | "type"> & { value?: string }
> = (props) => {
  return (
    <TextField
      {...props}
      variant="standard"
      InputLabelProps={{ shrink: true }}
      InputProps={{
        readOnly: true,
        disableUnderline: true,
        endAdornment: props.value && (
          <InputAdornment position="end">
            <Clipboard value={props.value} />
          </InputAdornment>
        ),
      }}
    />
  );
};
