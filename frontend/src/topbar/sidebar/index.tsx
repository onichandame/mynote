import {
  Divider,
  Box,
  Drawer,
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
} from "@mui/material";
import { FC, Fragment, useCallback } from "react";
import { useNavigate } from "react-router-dom";

import { useUser } from "../../backend";
import { root } from "../path";

export const Sidebar: FC<{ open: boolean; setOpen: (_: boolean) => void }> = ({
  open,
  setOpen,
}) => {
  const navigate = useNavigate();
  const user = useUser();
  const click = useCallback(
    (handler: () => void) => {
      return () => {
        handler();
        setOpen(false);
      };
    },
    [setOpen]
  );
  return (
    <Drawer open={open} onClose={() => setOpen(false)} anchor="left">
      <Box sx={{ width: 250 }} role="presentation">
        <List>
          {root
            .getChildren()
            ?.filter((v) => (user ? v.priv : !v.priv))
            .map((path) => (
              <Fragment key={path.match}>
                <ListItem disablePadding>
                  <ListItemButton onClick={click(() => navigate(path.link))}>
                    <ListItemIcon>{path.icon}</ListItemIcon>
                    <ListItemText primary={path.title} />
                  </ListItemButton>
                </ListItem>
                {<Divider />}
              </Fragment>
            ))}
        </List>
      </Box>
    </Drawer>
  );
};
