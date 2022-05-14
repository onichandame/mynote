import { AccountCircle } from "@mui/icons-material";
import { Avatar, IconButton } from "@mui/material";
import { FC, useState } from "react";

import { useUser } from "../../backend";
import { Dropdown } from "./dropdown";

export const Account: FC = () => {
  const [menuAnchor, setMenuAnchor] = useState<HTMLElement | null>(null);
  const user = useUser();
  return (
    <>
      <IconButton size="large" onClick={(e) => setMenuAnchor(e.currentTarget)}>
        {user ? (
          user.avatar ? (
            <Avatar alt={user.name} src={user.avatar} />
          ) : (
            <Avatar alt={user.name}>
              {user.name
                .split(` `)
                .filter((v) => !!v)
                .slice(0, 2)
                .map((v) => v[0]?.toUpperCase())}
            </Avatar>
          )
        ) : (
          <AccountCircle />
        )}
      </IconButton>
      <Dropdown anchor={menuAnchor} setAnchor={setMenuAnchor} />
    </>
  );
};
