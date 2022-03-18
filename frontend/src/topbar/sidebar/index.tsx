import { NoteAlt, Settings } from "@mui/icons-material";
import {
  Box,
  Divider,
  Drawer,
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
} from "@mui/material";
import { FC, useCallback } from "react";
import { useNavigate } from "react-router-dom";

export const Sidebar: FC<{ open: boolean; setOpen: (_: boolean) => void }> = ({
  open,
  setOpen,
}) => {
  const navigate = useNavigate();
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
          <ListItem disablePadding>
            <ListItemButton onClick={click(() => navigate(`/notes`))}>
              <ListItemIcon>
                <NoteAlt />
              </ListItemIcon>
              <ListItemText primary="Notes" />
            </ListItemButton>
          </ListItem>
        </List>
        <Divider />
        <List>
          <ListItem disablePadding>
            <ListItemButton onClick={click(() => navigate(`/settings`))}>
              <ListItemIcon>
                <Settings />
              </ListItemIcon>
              <ListItemText primary="Settings" />
            </ListItemButton>
          </ListItem>
        </List>
      </Box>
    </Drawer>
  );
};
