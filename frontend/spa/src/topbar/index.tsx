import { Menu } from "@mui/icons-material";
import { Typography, AppBar, IconButton, Toolbar } from "@mui/material";
import { FC } from "react";

export const TopBar: FC = () => {
  return (
    <AppBar position="static">
      <Toolbar>
        <IconButton size="large" edge="start" color="inherit" sx={{ mr: 2 }}>
          <Menu />
        </IconButton>
        <Typography variant="h6" sx={{ flexGrow: 1 }}>
          My Notes
        </Typography>
      </Toolbar>
    </AppBar>
  );
};
