import { Menu, MenuItem } from "@mui/material";
import { FC } from "react";
import { useNavigate } from "react-router-dom";

import { useUser } from "../../backend";

export const Dropdown: FC<{
  anchor: HTMLElement | null;
  setAnchor: (_: HTMLElement | null) => void;
}> = ({ anchor, setAnchor }) => {
  const navigate = useNavigate();
  const user = useUser();
  return (
    <Menu
      anchorEl={anchor}
      keepMounted
      open={!!anchor}
      anchorOrigin={{ horizontal: `center`, vertical: `bottom` }}
      transformOrigin={{ vertical: `top`, horizontal: `center` }}
      onClose={() => setAnchor(null)}
    >
      {user ? (
        <>
          <MenuItem
            onClick={() => {
              setAnchor(null);
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
              setAnchor(null);
              navigate(`/login`);
            }}
          >
            Log in
          </MenuItem>
          <MenuItem
            onClick={() => {
              setAnchor(null);
              navigate(`/signup`);
            }}
          >
            Sign up
          </MenuItem>
        </>
      )}
    </Menu>
  );
};
