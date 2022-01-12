import { AccountCircle } from "@mui/icons-material";
import { Avatar, IconButton, Menu, MenuItem } from "@mui/material";
import { FC, useState } from "react";
import { useNavigate } from "react-router-dom";

import { useUser } from "../auth";

export const Account: FC = () => {
  const [menuAnchor, setMenuAnchor] = useState<HTMLElement | null>(null);
  const user = useUser();
  const navigate = useNavigate();
  return (
    <>
      <IconButton size="large" onClick={(e) => setMenuAnchor(e.currentTarget)}>
        {user ? (
          user.avatar ? (
            <Avatar alt={user.name} src={user?.avatar} />
          ) : (
            <Avatar alt={user.name}>
              {user.name
                .split(` `)
                .filter((v) => !!v)
                .slice(0, 2)
                .map((v) => v[0].toUpperCase())}
            </Avatar>
          )
        ) : (
          <AccountCircle />
        )}
      </IconButton>
      <Menu
        anchorEl={menuAnchor}
        keepMounted
        open={!!menuAnchor}
        anchorOrigin={{ horizontal: `right`, vertical: `top` }}
        transformOrigin={{ vertical: `top`, horizontal: `right` }}
        onClose={() => setMenuAnchor(null)}
      >
        {user ? (
          <>
            <MenuItem
              onClick={() => {
                setMenuAnchor(null);
                navigate(`/logout`);
              }}
            >
              Logout
            </MenuItem>
          </>
        ) : (
          <>
            <MenuItem
              onClick={() => {
                setMenuAnchor(null);
                navigate(`/login`);
              }}
            >
              Log in
            </MenuItem>
            <MenuItem
              onClick={() => {
                setMenuAnchor(null);
                navigate(`/signup`);
              }}
            >
              Sign up
            </MenuItem>
          </>
        )}
      </Menu>
    </>
  );
};
