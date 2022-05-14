import {
  CardHeader,
  Card,
  Menu,
  MenuItem,
  CardActionArea,
  CardContent,
  Typography,
} from "@mui/material";
import { MoreVert } from "@mui/icons-material";
import { FC, useState } from "react";
import { useNavigate } from "react-router-dom";

import { Note } from "../../model";

export const Item: FC<{ item: Note }> = ({ item }) => {
  const navigate = useNavigate();
  const [menuAnchor, setMenuAnchor] = useState<HTMLElement | null>(null);
  return (
    <>
      <Card sx={{ minWidth: 180 }} variant="outlined">
        <CardActionArea
          onClick={() => {
            navigate(item.id.toString());
          }}
        >
          <CardHeader
            title={item.title}
            subheader={new Date(item.createdAt).toLocaleDateString()}
          />
          <CardContent>
            <Typography>
              {item.content.length < 10
                ? item.content
                : `${item.content.slice(0, 7)}...`}
            </Typography>
          </CardContent>
        </CardActionArea>
      </Card>
      <Menu
        anchorEl={menuAnchor}
        keepMounted
        open={!!menuAnchor}
        anchorOrigin={{ horizontal: `center`, vertical: `bottom` }}
        transformOrigin={{ vertical: `top`, horizontal: `center` }}
        onClose={() => setMenuAnchor(null)}
      >
        <MenuItem
          onClick={() => {
            setMenuAnchor(null);
            navigate(`${item.id}/delete`);
          }}
          sx={{ color: (theme) => theme.palette.secondary.main }}
        >
          Delete
        </MenuItem>
      </Menu>
    </>
  );
};
