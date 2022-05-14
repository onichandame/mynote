import {
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
  FormControlLabel,
  Radio,
  RadioGroup,
} from "@mui/material";
import { ComponentProps, FC, useEffect, useState } from "react";
import { useService } from "../backend";

import { Password, SyncFromRemoteInput } from "../model";

export const PwSelectionDialog: FC<
  Pick<ComponentProps<typeof Dialog>, "open" | "onClose"> & {
    setValues: (_: SyncFromRemoteInput) => void;
  }
> = ({ setValues, ...props }) => {
  const [pwds, setPwds] = useState<Password[]>([]);
  const [selected, setSelected] = useState<Password | null>(null);
  const svc = useService();
  useEffect(() => {
    if (props.open) {
      svc
        .listPasswords({ deletedAt: { null: true } })
        .then((conns) => conns.edges.map((v) => v.node))
        .then((pwds) => setPwds(pwds));
    }
  }, [props.open, svc]);
  return (
    <Dialog {...props}>
      <DialogTitle>Select Credentials</DialogTitle>
      <DialogContent>
        <RadioGroup
          name="credential"
          onChange={(e) => {
            setSelected(
              pwds.find((v) => v.id.toString() === e.target.value) || null
            );
          }}
        >
          {pwds.map((pwd) => (
            <FormControlLabel
              value={pwd.id}
              key={pwd.id}
              control={<Radio />}
              label={pwd.title}
            />
          ))}
        </RadioGroup>
      </DialogContent>
      <DialogActions>
        <Button
          onClick={() => props.onClose && props.onClose({}, `escapeKeyDown`)}
        >
          cancel
        </Button>
        <Button
          onClick={() => {
            selected &&
              setValues({
                url: selected.url || ``,
                identity: selected.username || ``,
                password: selected.password,
              });
            props.onClose && props.onClose({}, `escapeKeyDown`);
          }}
        >
          ok
        </Button>
      </DialogActions>
    </Dialog>
  );
};
